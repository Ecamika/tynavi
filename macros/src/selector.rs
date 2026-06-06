use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{
	Attribute, Data, DeriveInput, Field, Fields, GenericParam, Generics, Ident, ItemEnum, LitStr,
	Path, Type, Variant, Visibility, parse_macro_input, parse_quote,
};

#[derive(Default)]
struct SelectorAttr {
	rename: Option<Ident>,
	skip_all: bool,
	skip_inspect: bool,
	skip_filter: bool,
	skip_extract: bool,
	skip_route: bool,
	variants: Vec<Ident>,
	derives: Vec<Path>,
}

struct FieldInfo<'a> {
	method_name: Ident,
	ty: &'a Type,
	access: TokenStream2,
	attrs: SelectorAttr,
}

/// 将 Rust 标识符文本转换为宏方法名使用的 snake_case。
///
/// 流程：
/// 1. 逐字符扫描输入字符串。
/// 2. 遇到大写字符时，在非首字符位置先插入下划线。
/// 3. 将大写字符转为小写并追加到结果中，其他字符原样追加。
/// 4. 返回转换后的方法名片段。
fn to_snake_case(s: &str) -> String {
	let mut result = String::new();
	for (i, c) in s.chars().enumerate() {
		if c.is_uppercase() {
			if i > 0 {
				result.push('_');
			}
			for lower in c.to_lowercase() {
				result.push(lower);
			}
		} else {
			result.push(c);
		}
	}
	result
}

/// 解析一组 `#[selector(...)]` 属性为内部配置结构。
///
/// 流程：
/// 1. 遍历所有属性，只处理路径名为 `selector` 的属性。
/// 2. 识别 `rename = "..."` 并保存为重命名标识符。
/// 3. 识别 `skip` 或 `skip(...)`，记录跳过全部或指定方法组。
/// 4. 识别 `variants(...)`，收集需要生成快捷判定方法的变体名。
/// 5. 识别 `derive(...)`，收集属性宏生成 payload 结构体时需要追加的派生项。
/// 6. 遇到未知属性或格式错误时返回 `syn::Error`，供宏展开为编译错误。
fn parse_selector_attrs(attrs: &[Attribute]) -> syn::Result<SelectorAttr> {
	let mut parsed = SelectorAttr::default();

	for attr in attrs.iter().filter(|attr| attr.path().is_ident("selector")) {
		attr.parse_nested_meta(|meta| {
			if meta.path.is_ident("rename") {
				let value = meta.value()?;
				let lit: LitStr = value.parse()?;
				parsed.rename = Some(Ident::new(&lit.value(), lit.span()));
				return Ok(());
			}

			if meta.path.is_ident("skip") {
				if meta.input.is_empty() {
					parsed.skip_all = true;
					return Ok(());
				}

				meta.parse_nested_meta(|nested| {
					if nested.path.is_ident("inspect") {
						parsed.skip_inspect = true;
					} else if nested.path.is_ident("filter") {
						parsed.skip_filter = true;
					} else if nested.path.is_ident("extract") {
						parsed.skip_extract = true;
					} else if nested.path.is_ident("route") {
						parsed.skip_route = true;
					} else {
						return Err(nested.error("unknown selector skip target"));
					}
					Ok(())
				})?;
				return Ok(());
			}

			if meta.path.is_ident("variants") {
				meta.parse_nested_meta(|nested| {
					let Some(ident) = nested.path.get_ident() else {
						return Err(nested.error("selector variant must be an identifier"));
					};
					parsed.variants.push(ident.clone());
					Ok(())
				})?;
				return Ok(());
			}

			if meta.path.is_ident("derive") {
				meta.parse_nested_meta(|nested| {
					parsed.derives.push(nested.path.clone());
					Ok(())
				})?;
				return Ok(());
			}

			Err(meta.error("unknown selector attribute"))
		})?;
	}

	Ok(parsed)
}

/// 从属性列表中移除所有 `#[selector(...)]` 属性。
///
/// 流程：
/// 1. 遍历传入的属性切片。
/// 2. 保留路径名不是 `selector` 的属性。
/// 3. 克隆并返回过滤后的属性列表，用于把非宏控制属性继续转发到生成代码中。
fn strip_selector_attrs(attrs: &[Attribute]) -> Vec<Attribute> {
	attrs
		.iter()
		.filter(|attr| !attr.path().is_ident("selector"))
		.cloned()
		.collect()
}

/// 在用户类型的泛型参数前插入宏内部使用的生命周期。
///
/// 流程：
/// 1. 克隆原始泛型，避免修改 `syn` 解析出的输入节点。
/// 2. 将指定生命周期作为第一个泛型参数插入。
/// 3. 返回新的泛型列表，供 `impl AsSelector` 等生成逻辑使用。
fn add_lifetime(generics: &Generics, lifetime: syn::Lifetime) -> Generics {
	let mut generics = generics.clone();
	generics
		.params
		.insert(0, GenericParam::Lifetime(parse_quote!(#lifetime)));
	generics
}

/// 在用户类型泛型中追加 selector trait 所需的生命周期和父节点类型参数。
///
/// 流程：
/// 1. 先调用 `add_lifetime` 插入宏内部生命周期。
/// 2. 在泛型参数末尾追加 `__TynaviParent`。
/// 3. 为 `__TynaviParent` 添加 `SelectorInstance` 约束，确保生成 impl 可作为 selector 快照使用。
/// 4. 返回新的泛型列表。
fn add_lifetime_and_parent(generics: &Generics, lifetime: syn::Lifetime) -> Generics {
	let mut generics = add_lifetime(generics, lifetime);
	generics
		.params
		.push(parse_quote!(__TynaviParent: ::tynavi::traits::SelectorInstance));
	generics
}

/// 根据派生类型名生成默认 selector trait 名。
///
/// 流程：
/// 1. 接收类型标识符，例如 `Data`。
/// 2. 使用 `format_ident!` 拼接 `Selector` 后缀。
/// 3. 返回 `DataSelector` 形式的标识符。
fn default_trait_name(name: &Ident) -> Ident {
	format_ident!("{name}Selector")
}

/// 为派生类型生成 `AsSelector<'a, T, ()>` 实现。
///
/// 流程：
/// 1. 构造宏内部生命周期 `__tynavi_a`。
/// 2. 将生命周期插入用户类型泛型，并拆分出 `impl` 所需的泛型片段。
/// 3. 保留用户类型自身的类型泛型，用于 `T` 的具体路径。
/// 4. 生成 `as_selector`，让根 selector 的 cursor 指向 `self`，parent 为 `()`。
fn generate_as_selector(name: &Ident, generics: &Generics) -> TokenStream2 {
	let lifetime: syn::Lifetime = parse_quote!('__tynavi_a);
	let as_generics = add_lifetime(generics, lifetime.clone());
	let (as_impl_generics, _, as_where_clause) = as_generics.split_for_impl();
	let (_, ty_generics, _) = generics.split_for_impl();

	quote! {
		impl #as_impl_generics ::tynavi::traits::AsSelector<#lifetime, #name #ty_generics, ()> for #name #ty_generics
		#as_where_clause
		{
			fn as_selector(&#lifetime self) -> ::tynavi::selector::Selector<#lifetime, #name #ty_generics, ()> {
				::tynavi::selector::Selector {
					cursor: Some(self),
					parent: (),
				}
			}
		}
	}
}

/// 为派生类型生成类型专属 selector trait 及其 `Selector` 实现。
///
/// 流程：
/// 1. 构造宏内部生命周期和父节点泛型参数。
/// 2. 将调用方收集好的方法签名放入公开 trait。
/// 3. 为 `Selector<'a, Type, Parent>` 实现该 trait。
/// 4. 将调用方收集好的方法实现放入 impl 块。
/// 5. 返回 trait 与 impl 两部分 token。
fn generate_trait_and_impl(
	name: &Ident,
	generics: &Generics,
	trait_name: &Ident,
	method_sigs: &[TokenStream2],
	method_impls: &[TokenStream2],
) -> TokenStream2 {
	let lifetime: syn::Lifetime = parse_quote!('__tynavi_a);
	let trait_generics = add_lifetime_and_parent(generics, lifetime.clone());
	let (trait_impl_generics, trait_ty_generics, trait_where_clause) = trait_generics.split_for_impl();
	let (_, ty_generics, _) = generics.split_for_impl();

	quote! {
		pub trait #trait_name #trait_generics: ::tynavi::traits::SelectorInstance + Sized
		#trait_where_clause
		{
			#(#method_sigs)*
		}

		impl #trait_impl_generics #trait_name #trait_ty_generics for ::tynavi::selector::Selector<#lifetime, #name #ty_generics, __TynaviParent>
		#trait_where_clause
		{
			#(#method_impls)*
		}
	}
}

/// 为单个结构体字段生成 selector trait 方法签名和实现。
///
/// 流程：
/// 1. 读取字段方法名、字段类型、字段访问表达式和字段属性配置。
/// 2. 根据 `skip` 配置决定是否生成 inspect/filter/extract/route 四组方法。
/// 3. 为 inspect/filter/extract 分别生成普通、条件、异步和条件异步变体。
/// 4. 为 route 生成深入字段的 `Selector<'a, Field, Self>`。
/// 5. 根据 `variants(...)` 额外生成字段枚举快捷判定方法。
/// 6. 分别返回 trait 方法签名列表和 impl 方法实现列表。
fn field_methods(field: &FieldInfo<'_>) -> (Vec<TokenStream2>, Vec<TokenStream2>) {
	let mut sigs = Vec::new();
	let mut impls = Vec::new();
	let name = &field.method_name;
	let ty = field.ty;
	let access = &field.access;
	let lifetime: syn::Lifetime = parse_quote!('__tynavi_a);

	if !(field.attrs.skip_all || field.attrs.skip_inspect) {
		let method = format_ident!("{name}_inspect");
		let cond_method = format_ident!("cond_{name}_inspect");
		let async_method = format_ident!("{name}_inspect_async");
		let cond_async_method = format_ident!("cond_{name}_inspect_async");

		sigs.push(quote! {
			fn #method(&self, f: impl FnOnce(&#lifetime #ty, &Self)) -> Self;
			fn #cond_method(&self, condition: bool, f: impl FnOnce(&#lifetime #ty, &Self)) -> Self;
			async fn #async_method(&self, f: impl AsyncFnOnce(&#lifetime #ty, &Self)) -> Self;
			async fn #cond_async_method(&self, condition: bool, f: impl AsyncFnOnce(&#lifetime #ty, &Self)) -> Self;
		});
		impls.push(quote! {
			fn #method(&self, f: impl FnOnce(&#lifetime #ty, &Self)) -> Self {
				if let Some(cursor) = self.cursor {
					f(#access, self);
				}
				::tynavi::traits::Snapshot::snapshot(self)
			}

			fn #cond_method(&self, condition: bool, f: impl FnOnce(&#lifetime #ty, &Self)) -> Self {
				if condition {
					self.#method(f)
				} else {
					::tynavi::traits::Snapshot::snapshot(self)
				}
			}

			async fn #async_method(&self, f: impl AsyncFnOnce(&#lifetime #ty, &Self)) -> Self {
				if let Some(cursor) = self.cursor {
					f(#access, self).await;
				}
				::tynavi::traits::Snapshot::snapshot(self)
			}

			async fn #cond_async_method(&self, condition: bool, f: impl AsyncFnOnce(&#lifetime #ty, &Self)) -> Self {
				if condition {
					self.#async_method(f).await
				} else {
					::tynavi::traits::Snapshot::snapshot(self)
				}
			}
		});
	}

	if !(field.attrs.skip_all || field.attrs.skip_filter) {
		let method = format_ident!("{name}_filter");
		let cond_method = format_ident!("cond_{name}_filter");
		let async_method = format_ident!("{name}_filter_async");
		let cond_async_method = format_ident!("cond_{name}_filter_async");

		sigs.push(quote! {
			fn #method(&self, f: impl FnOnce(&#lifetime #ty, &Self) -> bool) -> Self;
			fn #cond_method(&self, condition: bool, f: impl FnOnce(&#lifetime #ty, &Self) -> bool) -> Self;
			async fn #async_method(&self, f: impl AsyncFnOnce(&#lifetime #ty, &Self) -> bool) -> Self;
			async fn #cond_async_method(&self, condition: bool, f: impl AsyncFnOnce(&#lifetime #ty, &Self) -> bool) -> Self;
		});
		impls.push(quote! {
			fn #method(&self, f: impl FnOnce(&#lifetime #ty, &Self) -> bool) -> Self {
				if let Some(cursor) = self.cursor
					&& !f(#access, self)
				{
					::tynavi::traits::Unmatch::unmatch(self)
				} else {
					::tynavi::traits::Snapshot::snapshot(self)
				}
			}

			fn #cond_method(&self, condition: bool, f: impl FnOnce(&#lifetime #ty, &Self) -> bool) -> Self {
				if condition {
					self.#method(f)
				} else {
					::tynavi::traits::Snapshot::snapshot(self)
				}
			}

			async fn #async_method(&self, f: impl AsyncFnOnce(&#lifetime #ty, &Self) -> bool) -> Self {
				if let Some(cursor) = self.cursor
					&& !f(#access, self).await
				{
					::tynavi::traits::Unmatch::unmatch(self)
				} else {
					::tynavi::traits::Snapshot::snapshot(self)
				}
			}

			async fn #cond_async_method(&self, condition: bool, f: impl AsyncFnOnce(&#lifetime #ty, &Self) -> bool) -> Self {
				if condition {
					self.#async_method(f).await
				} else {
					::tynavi::traits::Snapshot::snapshot(self)
				}
			}
		});
	}

	if !(field.attrs.skip_all || field.attrs.skip_extract) {
		let method = format_ident!("{name}_extract");
		let cond_method = format_ident!("cond_{name}_extract");
		let async_method = format_ident!("{name}_extract_async");
		let cond_async_method = format_ident!("cond_{name}_extract_async");

		sigs.push(quote! {
			fn #method<__TynaviOutput>(&self, f: impl FnOnce(&#lifetime #ty, &Self) -> __TynaviOutput) -> Option<__TynaviOutput>;
			fn #cond_method<__TynaviOutput>(&self, condition: bool, f: impl FnOnce(&#lifetime #ty, &Self) -> __TynaviOutput) -> Option<__TynaviOutput>;
			async fn #async_method<__TynaviOutput>(&self, f: impl AsyncFnOnce(&#lifetime #ty, &Self) -> __TynaviOutput) -> Option<__TynaviOutput>;
			async fn #cond_async_method<__TynaviOutput>(&self, condition: bool, f: impl AsyncFnOnce(&#lifetime #ty, &Self) -> __TynaviOutput) -> Option<__TynaviOutput>;
		});
		impls.push(quote! {
			fn #method<__TynaviOutput>(&self, f: impl FnOnce(&#lifetime #ty, &Self) -> __TynaviOutput) -> Option<__TynaviOutput> {
				self.cursor.map(|cursor| f(#access, self))
			}

			fn #cond_method<__TynaviOutput>(&self, condition: bool, f: impl FnOnce(&#lifetime #ty, &Self) -> __TynaviOutput) -> Option<__TynaviOutput> {
				if condition {
					self.#method(f)
				} else {
					None
				}
			}

			async fn #async_method<__TynaviOutput>(&self, f: impl AsyncFnOnce(&#lifetime #ty, &Self) -> __TynaviOutput) -> Option<__TynaviOutput> {
				if let Some(cursor) = self.cursor {
					Some(f(#access, self).await)
				} else {
					None
				}
			}

			async fn #cond_async_method<__TynaviOutput>(&self, condition: bool, f: impl AsyncFnOnce(&#lifetime #ty, &Self) -> __TynaviOutput) -> Option<__TynaviOutput> {
				if condition {
					self.#async_method(f).await
				} else {
					None
				}
			}
		});
	}

	if !(field.attrs.skip_all || field.attrs.skip_route) {
		let method = format_ident!("route_{name}");
		sigs.push(quote! {
			fn #method(&self) -> ::tynavi::selector::Selector<#lifetime, #ty, Self>;
		});
		impls.push(quote! {
			fn #method(&self) -> ::tynavi::selector::Selector<#lifetime, #ty, Self> {
				::tynavi::selector::Selector {
					cursor: self.cursor.map(|cursor| #access),
					parent: ::tynavi::traits::Snapshot::snapshot(self),
				}
			}
		});
	}

	for variant in &field.attrs.variants {
		let is_method = format_ident!("is_{variant}");
		let not_method = format_ident!("not_is_{variant}");
		let cond_method = format_ident!("cond_is_{variant}");
		let cond_not_method = format_ident!("cond_not_is_{variant}");
		sigs.push(quote! {
			fn #is_method(&self) -> Self;
			fn #not_method(&self) -> Self;
			fn #cond_method(&self, condition: bool) -> Self;
			fn #cond_not_method(&self, condition: bool) -> Self;
		});
		impls.push(quote! {
			fn #is_method(&self) -> Self {
				if let Some(cursor) = self.cursor
					&& !((#access).#is_method())
				{
					::tynavi::traits::Unmatch::unmatch(self)
				} else {
					::tynavi::traits::Snapshot::snapshot(self)
				}
			}

			fn #not_method(&self) -> Self {
				if let Some(cursor) = self.cursor
					&& (#access).#is_method()
				{
					::tynavi::traits::Unmatch::unmatch(self)
				} else {
					::tynavi::traits::Snapshot::snapshot(self)
				}
			}

			fn #cond_method(&self, condition: bool) -> Self {
				if condition {
					self.#is_method()
				} else {
					::tynavi::traits::Snapshot::snapshot(self)
				}
			}

			fn #cond_not_method(&self, condition: bool) -> Self {
				if condition {
					self.#not_method()
				} else {
					::tynavi::traits::Snapshot::snapshot(self)
				}
			}
		});
	}

	(sigs, impls)
}

/// 将结构体字段解析成统一的字段生成信息。
///
/// 流程：
/// 1. 根据结构体字段形态分支处理命名字段、元组字段和单元结构体。
/// 2. 命名字段使用原字段名作为默认方法名，元组字段使用 `field_1` 起始的索引名。
/// 3. 解析字段上的 `#[selector(...)]` 属性，应用 `rename` 等配置。
/// 4. 生成字段访问表达式，例如 `&cursor.name` 或 `&cursor.0`。
/// 5. 汇总为 `FieldInfo` 列表返回，单元结构体返回空列表。
fn struct_fields(data: &syn::DataStruct) -> syn::Result<Vec<FieldInfo<'_>>> {
	let mut fields = Vec::new();

	match &data.fields {
		Fields::Named(named) => {
			for field in &named.named {
				let field_ident = field.ident.as_ref().expect("named field");
				let attrs = parse_selector_attrs(&field.attrs)?;
				let method_name = attrs
					.rename
					.clone()
					.unwrap_or_else(|| field_ident.clone());
				fields.push(FieldInfo {
					method_name,
					ty: &field.ty,
					access: quote!(&cursor.#field_ident),
					attrs,
				});
			}
		}
		Fields::Unnamed(unnamed) => {
			if unnamed.unnamed.is_empty() {
				return Ok(fields);
			}

			for (idx, field) in unnamed.unnamed.iter().enumerate() {
				let index = syn::Index::from(idx);
				let attrs = parse_selector_attrs(&field.attrs)?;
				let method_name = attrs
					.rename
					.clone()
					.unwrap_or_else(|| format_ident!("field_{}", idx + 1));
				fields.push(FieldInfo {
					method_name,
					ty: &field.ty,
					access: quote!(&cursor.#index),
					attrs,
				});
			}
		}
		Fields::Unit => {}
	}

	Ok(fields)
}

/// 为结构体输入生成完整的 `Selector` derive 展开代码。
///
/// 流程：
/// 1. 读取结构体名称、泛型和类型级 `#[selector(...)]` 属性。
/// 2. 确定 selector trait 名，默认使用 `TypeSelector`，可由 `rename` 覆盖。
/// 3. 通过 `struct_fields` 收集所有字段的生成信息。
/// 4. 对每个字段调用 `field_methods`，合并所有方法签名和实现。
/// 5. 生成 `AsSelector` 实现和类型专属 selector trait/impl。
/// 6. 返回最终 token。
fn generate_struct_selector(
	input: &DeriveInput,
	data: &syn::DataStruct,
) -> syn::Result<TokenStream2> {
	let name = &input.ident;
	let generics = &input.generics;
	let selector_attrs = parse_selector_attrs(&input.attrs)?;
	let trait_name = selector_attrs
		.rename
		.unwrap_or_else(|| default_trait_name(name));

	let mut sigs = Vec::new();
	let mut impls = Vec::new();

	for field in struct_fields(data)? {
		let (field_sigs, field_impls) = field_methods(&field);
		sigs.extend(field_sigs);
		impls.extend(field_impls);
	}

	let as_selector = generate_as_selector(name, generics);
	let trait_impl = generate_trait_and_impl(name, generics, &trait_name, &sigs, &impls);

	Ok(quote! {
		#as_selector
		#trait_impl
	})
}

/// 判断枚举变体是否拥有可直接路由的 payload 类型。
///
/// 流程：
/// 1. 单元变体返回 `Ok(None)`，表示只生成 `is_*` 判定。
/// 2. 单元素元组变体返回内部类型引用，表示可生成 `as_*` 和 `route_*`。
/// 3. 多字段元组变体或匿名结构体变体返回错误，提示调用方使用 `#[tynavi::selector]` 先改写。
fn enum_payload_type(variant: &Variant) -> syn::Result<Option<&Type>> {
	match &variant.fields {
		Fields::Unit => Ok(None),
		Fields::Unnamed(fields) if fields.unnamed.len() == 1 => Ok(Some(&fields.unnamed[0].ty)),
		_ => Err(syn::Error::new_spanned(
			variant,
			"derive(Selector) only supports unit variants and single-field tuple variants; use #[tynavi::selector] to rewrite multi-field variants",
		)),
	}
}

/// 为枚举变体生成 `matches!` 使用的模式。
///
/// 流程：
/// 1. 读取变体名称。
/// 2. 单元变体生成 `Self::Variant`。
/// 3. 元组变体生成 `Self::Variant(..)`。
/// 4. 命名字段变体生成 `Self::Variant { .. }`。
fn enum_variant_pattern(variant: &Variant) -> TokenStream2 {
	let variant_ident = &variant.ident;
	match &variant.fields {
		Fields::Unit => quote!(Self::#variant_ident),
		Fields::Unnamed(_) => quote!(Self::#variant_ident(..)),
		Fields::Named(_) => quote!(Self::#variant_ident { .. }),
	}
}

/// 为枚举输入生成完整的 `Selector` derive 展开代码。
///
/// 流程：
/// 1. 读取枚举名称、泛型和类型级 selector 属性。
/// 2. 确定 selector trait 名，默认使用 `TypeSelector`，可由 `rename` 覆盖。
/// 3. 遍历所有变体，为每个变体生成枚举自身的 `is_*` 方法。
/// 4. 在 selector trait/impl 中生成同名过滤方法，匹配时返回快照，不匹配时 unmatch。
/// 5. 对单元素元组变体额外生成枚举自身的 `as_*` 和 selector 上的 `route_*`。
/// 6. 生成 `AsSelector` 实现和类型专属 selector trait/impl。
/// 7. 返回枚举 inherent impl、`AsSelector` impl 和 selector trait/impl 的组合 token。
fn generate_enum_selector(input: &DeriveInput, data: &syn::DataEnum) -> syn::Result<TokenStream2> {
	let name = &input.ident;
	let generics = &input.generics;
	let selector_attrs = parse_selector_attrs(&input.attrs)?;
	let trait_name = selector_attrs
		.rename
		.unwrap_or_else(|| default_trait_name(name));
	let lifetime: syn::Lifetime = parse_quote!('__tynavi_a);

	let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
	let mut inherent_methods = Vec::new();
	let mut sigs = Vec::new();
	let mut impls = Vec::new();

	for variant in &data.variants {
		let variant_ident = &variant.ident;
		let method_part = parse_selector_attrs(&variant.attrs)?
			.rename
			.unwrap_or_else(|| Ident::new(&to_snake_case(&variant_ident.to_string()), variant_ident.span()));
		let is_method = format_ident!("is_{method_part}");
		let payload = enum_payload_type(variant)?;
		let pattern = enum_variant_pattern(variant);

		inherent_methods.push(quote! {
			pub fn #is_method(&self) -> bool {
				matches!(self, #pattern)
			}
		});

		sigs.push(quote! {
			fn #is_method(&self) -> Self;
		});
		impls.push(quote! {
			fn #is_method(&self) -> Self {
				if let Some(cursor) = self.cursor
					&& cursor.#is_method()
				{
					::tynavi::traits::Snapshot::snapshot(self)
				} else {
					::tynavi::traits::Unmatch::unmatch(self)
				}
			}
		});

		if let Some(payload_ty) = payload {
			let as_method = format_ident!("as_{method_part}");
			let route_method = format_ident!("route_{method_part}");

			inherent_methods.push(quote! {
				pub fn #as_method(&self) -> Option<&#payload_ty> {
					if let Self::#variant_ident(data) = self {
						Some(data)
					} else {
						None
					}
				}
			});

			sigs.push(quote! {
				fn #route_method(&self) -> ::tynavi::selector::Selector<#lifetime, #payload_ty, Self>;
			});
			impls.push(quote! {
				fn #route_method(&self) -> ::tynavi::selector::Selector<#lifetime, #payload_ty, Self> {
					self.route_to(|cursor, _| cursor.#as_method())
				}
			});
		}
	}

	let as_selector = generate_as_selector(name, generics);
	let trait_impl = generate_trait_and_impl(name, generics, &trait_name, &sigs, &impls);

	Ok(quote! {
		impl #impl_generics #name #ty_generics
		#where_clause
		{
			#(#inherent_methods)*
		}

		#as_selector
		#trait_impl
	})
}

/// `#[derive(Selector)]` 的公开入口。
///
/// 流程：
/// 1. 将编译器传入的 `TokenStream` 解析为 `DeriveInput`。
/// 2. 根据输入数据类型分派到结构体或枚举生成逻辑。
/// 3. 对 union 直接产生编译错误，因为 selector 不支持 union。
/// 4. 将生成结果或错误转换回过程宏所需的 `TokenStream`。
pub fn derive_selector(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let result = match &input.data {
		Data::Struct(data) => generate_struct_selector(&input, data),
		Data::Enum(data) => generate_enum_selector(&input, data),
		Data::Union(_) => Err(syn::Error::new_spanned(
			input,
			"Selector cannot be derived for unions",
		)),
	};

	result.unwrap_or_else(syn::Error::into_compile_error).into()
}

/// 检查 enum 是否已经显式声明了 `Selector` derive。
///
/// 流程：
/// 1. 遍历所有 `#[derive(...)]` 属性。
/// 2. 解析 derive 列表中的每个路径。
/// 3. 若路径本身是 `Selector`，或路径最后一段是 `Selector`，则标记为已存在。
/// 4. 返回是否找到，供属性宏决定是否自动补充 derive。
fn has_selector_derive(attrs: &[Attribute]) -> bool {
	attrs
		.iter()
		.filter(|attr| attr.path().is_ident("derive"))
		.any(|attr| {
			let mut found = false;
			let _ = attr.parse_nested_meta(|meta| {
				if meta.path.is_ident("Selector")
					|| meta
						.path
						.segments
						.last()
						.is_some_and(|segment| segment.ident == "Selector")
				{
					found = true;
				}
				Ok(())
			});
			found
		})
}

/// 计算属性宏为多字段变体生成的 payload 结构体名称。
///
/// 流程：
/// 1. 优先使用变体级 `#[selector(rename = "...")]` 指定的名称。
/// 2. 若没有重命名，则拼接枚举名和变体名。
/// 3. 返回生成结构体使用的标识符，例如 `EventMoved`。
fn variant_payload_name(enum_name: &Ident, variant: &Variant, attrs: &SelectorAttr) -> Ident {
	attrs.rename.clone().unwrap_or_else(|| {
		format_ident!(
			"{}{}",
			enum_name,
			variant.ident,
			span = variant.ident.span()
		)
	})
}

/// 将生成 payload 结构体中的字段可见性同步为枚举可见性。
///
/// 流程：
/// 1. 接收待改写字段和枚举可见性。
/// 2. 克隆枚举可见性并赋值给字段。
/// 3. 让原先匿名变体中的数据在改写为结构体后保持同级可访问性。
fn set_field_visibility(field: &mut Field, vis: &Visibility) {
	field.vis = vis.clone();
}

/// 在 `#[selector]` 属性宏中改写单个 enum 变体。
///
/// 流程：
/// 1. 解析变体上的 selector 属性，得到 payload 名称、额外 derive 等配置。
/// 2. 移除变体和生成 payload 上的 selector 控制属性，避免它们泄漏到最终代码。
/// 3. 多元素元组变体会生成同可见性的 tuple payload 结构体，并把变体改为单元素元组变体。
/// 4. 匿名结构体变体会生成同可见性的 named payload 结构体，并把变体改为单元素元组变体。
/// 5. 单元变体和单元素元组变体保持不变，返回 `Ok(None)`。
/// 6. 返回可选的 payload 结构体 token。
fn rewrite_variant(
	enum_name: &Ident,
	enum_vis: &Visibility,
	variant: &mut Variant,
) -> syn::Result<Option<TokenStream2>> {
	let attrs = parse_selector_attrs(&variant.attrs)?;
	let payload_name = variant_payload_name(enum_name, variant, &attrs);
	let payload_derives = attrs.derives;
	let payload_attrs = strip_selector_attrs(&variant.attrs);
	let variant_attrs = strip_selector_attrs(&variant.attrs);
	variant.attrs = variant_attrs;

	match &mut variant.fields {
		Fields::Unnamed(fields) if fields.unnamed.len() > 1 => {
			for field in &mut fields.unnamed {
				set_field_visibility(field, enum_vis);
			}
			let old_fields = fields.clone();
			variant.fields = Fields::Unnamed(parse_quote!((#payload_name)));
			Ok(Some(quote! {
				#(#payload_attrs)*
				#[derive(tynavi::Selector #(, #payload_derives)*)]
				#enum_vis struct #payload_name #old_fields;
			}))
		}
		Fields::Named(fields) => {
			for field in &mut fields.named {
				set_field_visibility(field, enum_vis);
			}
			let old_fields = fields.clone();
			variant.fields = Fields::Unnamed(parse_quote!((#payload_name)));
			Ok(Some(quote! {
				#(#payload_attrs)*
				#[derive(tynavi::Selector #(, #payload_derives)*)]
				#enum_vis struct #payload_name #old_fields
			}))
		}
		Fields::Unit | Fields::Unnamed(_) => Ok(None),
	}
}

/// `#[selector]` 属性宏的公开入口，用于规范化 enum 变体并自动派生 selector。
///
/// 流程：
/// 1. 将被标注的 item 解析为 `ItemEnum`，因此该属性宏只接受 enum。
/// 2. 遍历 enum 的所有变体，调用 `rewrite_variant` 改写多字段变体并收集生成的 payload 结构体。
/// 3. 如果 enum 尚未显式派生 `Selector`，自动追加 `#[derive(tynavi::Selector)]`。
/// 4. 输出所有生成的 payload 结构体和改写后的 enum。
pub fn selector(_attr: TokenStream, item: TokenStream) -> TokenStream {
	let mut item = parse_macro_input!(item as ItemEnum);
	let mut payloads = Vec::new();
	let enum_name = item.ident.clone();
	let enum_vis = item.vis.clone();

	for variant in &mut item.variants {
		match rewrite_variant(&enum_name, &enum_vis, variant) {
			Ok(Some(payload)) => payloads.push(payload),
			Ok(None) => {}
			Err(err) => return err.into_compile_error().into(),
		}
	}

	if !has_selector_derive(&item.attrs) {
		item.attrs.push(parse_quote!(#[derive(tynavi::Selector)]));
	}

	quote! {
		#(#payloads)*
		#item
	}
	.into()
}
