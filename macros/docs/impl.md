# Selector 宏实现流程

本文面向维护者，说明 `tynavi-macros` 中 `#[derive(Selector)]` 和 `#[selector]` 的内部工作流程。

实现入口位于：

- `macros/src/lib.rs`
- `macros/src/selector.rs`

## 总体结构

`macros/src/lib.rs` 注册两个过程宏入口：

```rust
#[proc_macro_derive(Selector, attributes(selector))]
pub fn derive_selector(input: TokenStream) -> TokenStream;

#[proc_macro_attribute]
pub fn selector(attr: TokenStream, item: TokenStream) -> TokenStream;
```

真正的解析和生成逻辑位于 `macros/src/selector.rs`。

## 属性解析

所有 `#[selector(...)]` 配置都会先解析成内部结构 `SelectorAttr`。

该结构记录：

- `rename`
- `skip_all`
- `skip_inspect`
- `skip_filter`
- `skip_extract`
- `skip_route`
- `variants`
- `derives`

解析流程：

1. 遍历输入节点上的属性。
2. 只处理路径名为 `selector` 的属性。
3. 使用 `syn::Attribute::parse_nested_meta` 解析内部参数。
4. 将 `rename`、`skip`、`variants`、`derive` 写入 `SelectorAttr`。
5. 未知参数会返回 `syn::Error`，最终展开为编译错误。

`strip_selector_attrs` 用于从属性列表中移除宏控制属性，防止它们泄漏到最终生成的用户代码里。

## `#[derive(Selector)]` 工作流程

derive 宏入口为 `derive_selector`。

流程：

1. 将输入解析为 `syn::DeriveInput`。
2. 根据 `input.data` 分派：
   - `Data::Struct` -> `generate_struct_selector`
   - `Data::Enum` -> `generate_enum_selector`
   - `Data::Union` -> 编译错误
3. 将生成的 `TokenStream2` 转回 `proc_macro::TokenStream`。

### 结构体生成流程

结构体入口为 `generate_struct_selector`。

流程：

1. 读取结构体名、泛型和类型级 selector 属性。
2. 确定 selector trait 名：
   - 默认：`TypeSelector`
   - 类型级 `rename = "..."`：使用指定名称
3. 调用 `struct_fields` 收集字段信息。
4. 对每个字段调用 `field_methods` 生成方法签名和方法实现。
5. 调用 `generate_as_selector` 生成 `AsSelector` 实现。
6. 调用 `generate_trait_and_impl` 生成 selector trait 和 `Selector<'a, Type, P>` 的 impl。
7. 合并所有 token 后返回。

### 字段收集流程

字段收集由 `struct_fields` 完成。

命名结构体：

1. 遍历所有命名字段。
2. 字段默认方法名为字段名。
3. 字段访问表达式为 `&cursor.field`。
4. 若字段有 `rename`，方法名改为指定名称。

元组结构体：

1. 遍历所有元组字段。
2. 字段默认方法名从 `field_1` 开始。
3. 字段访问表达式为 `&cursor.0`、`&cursor.1` 等。
4. 若字段有 `rename`，方法名改为指定名称。

单元结构体：

1. 没有字段。
2. 返回空字段列表。

### 字段方法生成流程

字段方法由 `field_methods` 生成。

输入是 `FieldInfo`：

- 方法名片段
- 字段类型
- 字段访问 token
- 字段 selector 属性

输出分为两组：

- selector trait 中的方法签名
- selector trait impl 中的方法实现

生成规则：

1. 若没有跳过 inspect，生成 `field_inspect`、`cond_field_inspect`、`field_inspect_async`、`cond_field_inspect_async`。
2. 若没有跳过 filter，生成 `field_filter`、`cond_field_filter`、`field_filter_async`、`cond_field_filter_async`。
3. 若没有跳过 extract，生成 `field_extract`、`cond_field_extract`、`field_extract_async`、`cond_field_extract_async`。
4. 若没有跳过 route，生成 `route_field`。
5. 若有 `variants(...)`，为每个变体生成 `is_*`、`not_is_*`、`cond_is_*`、`cond_not_is_*`。

字段方法的行为直接展开在生成代码中，而不是再次包一层闭包调用核心方法。这样做可以减少 `impl AsyncFnOnce` 场景下的类型推断压力。

### `AsSelector` 生成流程

`generate_as_selector` 负责生成根 selector 入口。

流程：

1. 为 impl 泛型插入内部生命周期 `__tynavi_a`。
2. 保留用户类型原本的泛型参数和 where 子句。
3. 生成 `impl AsSelector<'a, Type, ()> for Type`。
4. `as_selector` 返回 `Selector { cursor: Some(self), parent: () }`。

### selector trait 和 impl 生成流程

`generate_trait_and_impl` 负责生成类型专属 selector trait。

流程：

1. 为 trait 泛型插入生命周期 `__tynavi_a`。
2. 追加父节点泛型 `__TynaviParent: SelectorInstance`。
3. 生成公开 trait，例如 `UserSelector<'a, P>`。
4. trait 继承 `SelectorInstance + Sized`，使 trait 方法可以返回 `Self`。
5. 为 `Selector<'a, User, P>` 实现该 trait。
6. 将前面收集的字段或变体方法放入 trait 和 impl。

### 枚举生成流程

枚举入口为 `generate_enum_selector`。

流程：

1. 读取枚举名、泛型和类型级 selector 属性。
2. 确定 selector trait 名。
3. 遍历枚举变体。
4. 使用 `to_snake_case` 将变体名转换为方法名片段。
5. 为每个变体生成 enum 自身的 `is_*`。
6. 在 selector trait 中生成同名 `is_*` 过滤方法。
7. 对单元素元组变体额外生成 enum 自身的 `as_*` 和 selector 上的 `route_*`。
8. 生成 `AsSelector`、selector trait 和 impl。

变体 payload 判断由 `enum_payload_type` 完成：

- 单元变体：无 payload，只生成 `is_*`。
- 单元素元组变体：返回内部类型，可生成 `as_*` 和 `route_*`。
- 多字段元组变体或匿名结构体变体：返回编译错误，提示使用 `#[tynavi::selector]`。

`enum_variant_pattern` 根据变体形态生成 `matches!` 使用的模式。

## `#[selector]` 属性宏工作流程

属性宏入口为 `selector`。

流程：

1. 将输入 item 解析为 `syn::ItemEnum`。
2. 保存 enum 名称和可见性。
3. 遍历 enum 的每个变体，调用 `rewrite_variant`。
4. 收集 `rewrite_variant` 生成的 payload 结构体 token。
5. 如果 enum 没有显式派生 `Selector`，自动追加 `#[derive(tynavi::Selector)]`。
6. 输出 payload 结构体和改写后的 enum。

### 变体改写流程

`rewrite_variant` 负责处理单个变体。

流程：

1. 解析变体上的 selector 属性。
2. 计算 payload 结构体名称：
   - 优先使用 `rename = "..."`
   - 否则使用 `EnumVariant`
3. 收集 `derive(...)` 中的额外派生项。
4. 移除变体上的 selector 控制属性。
5. 根据字段形态分支：
   - 单元变体：不改写。
   - 单元素元组变体：不改写。
   - 多元素元组变体：生成 tuple payload 结构体，并将原变体改写为 `Variant(Payload)`。
   - 匿名结构体变体：生成 named payload 结构体，并将原变体改写为 `Variant(Payload)`。
6. 返回可选 payload 结构体 token。

### payload 结构体生成规则

payload 名称：

```rust
enum Event {
	Moved(i64, i64),
}
```

默认生成：

```rust
struct EventMoved(pub i64, pub i64);
```

变体上存在 `rename` 时：

```rust
#[selector(rename = "MovedPayload")]
Moved(i64, i64)
```

生成：

```rust
struct MovedPayload(pub i64, pub i64);
```

payload 可见性：

1. payload 结构体使用 enum 的可见性。
2. payload 字段也被同步为 enum 的可见性。
3. 这样改写后，原本可构造的公开 enum 数据仍能在同可见性层级构造。

payload derive：

1. payload 默认派生 `tynavi::Selector`。
2. `derive(...)` 中的额外派生项会追加到 derive 列表。

示例：

```rust
#[selector(rename = "MovedPayload", derive(Debug, Clone, Copy))]
Moved(i64, i64)
```

生成语义：

```rust
#[derive(tynavi::Selector, Debug, Clone, Copy)]
struct MovedPayload(pub i64, pub i64);
```

## 泛型处理

宏通过两个辅助函数处理泛型：

- `add_lifetime`
- `add_lifetime_and_parent`

流程：

1. 克隆用户类型原始泛型。
2. 在生成 impl 时插入内部生命周期 `__tynavi_a`。
3. 在 selector trait/impl 中额外追加 `__TynaviParent`。
4. 使用 `split_for_impl` 保留用户泛型和 where 子句。

这种做法让宏可以支持普通泛型结构：

```rust
#[derive(Selector)]
struct Wrapper<T>
where
	T: Clone,
{
	value: T,
}
```

## 路径与宏卫生

生成代码使用 `::tynavi::...` 绝对路径引用运行时类型和 trait：

- `::tynavi::selector::Selector`
- `::tynavi::traits::AsSelector`
- `::tynavi::traits::SelectorInstance`
- `::tynavi::traits::Snapshot`
- `::tynavi::traits::Unmatch`

主 crate 中通过 `extern crate self as tynavi;` 支持这些路径在 crate 内部和外部都能解析。

## 错误处理

宏中的可恢复错误统一使用 `syn::Error`。

常见错误：

- `#[derive(Selector)]` 用于 union。
- `#[derive(Selector)]` 直接用于多字段 enum 变体。
- `#[selector]` 标注非 enum item。
- `skip(...)` 中出现未知参数。
- `variants(...)` 中出现非标识符参数。

错误最终通过 `into_compile_error()` 展开为 Rust 编译错误。

## 设计边界

- `#[derive(Selector)]` 只追加代码，不改写输入类型。
- `#[selector]` 负责 enum 数据布局改写。
- 字段方法保持 tynavi 的不可变 selector 语义，返回新快照或 unmatch 后的新 selector。
- 宏不生成 onebot-api 风格的可变 `and_filter_*` API。
