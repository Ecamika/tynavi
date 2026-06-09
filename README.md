# tynavi

`tynavi`（全称 `Type Navigator`）是一个面向深层数据导航场景的 Rust Selector 模式库。

它从 `onebot-api` 的 `selector` 设计中演化而来，但在这里被拆成了更通用的独立实现：支持不可变链式调用、父节点回溯、标准库容器扩展，以及可选的 derive / attribute 宏自动生成能力。

## 当前状态

- 当前 crate 版本：`0.1.2`
- Rust edition：`2024`
- 最低工具链要求：`>= 1.85`
- 主 crate 默认启用 `full` feature
- `default-features = false` 时可退回仅标准库核心能力

需要特别注意的是：

- 核心 Selector API 本身不依赖第三方运行时库
- 但仓库当前已经包含可选生态扩展依赖与 `tynavi-macros` 过程宏子 crate
- 因为默认 feature 是 `full`，直接引入 `tynavi` 时并不是“默认零依赖扩展面”

如果你想只使用最小核心能力，建议这样声明：

```toml
[dependencies]
tynavi = { version = "0.1.2", default-features = false }
```

## 核心设计

核心类型：

```rust
Selector<'a, Current, Parent>
```

- `Current`：当前游标指向的类型
- `Parent`：父节点快照类型
- `cursor: Option<&'a Current>`：当前是否匹配

与 `onebot-api` 早期可变 Selector 不同，`tynavi` 采用不可变快照语义：

- 过滤方法返回新的 `Self`
- 路由方法返回 `Selector<'a, Child, Self>`
- 父节点快照可通过 `backtrack()` / `up()` 安全返回

这使它很适合表达一类固定问题：

- 从嵌套结构中持续下钻
- 在链路中逐步过滤
- 某一步失败后自动进入未匹配态
- 需要在深入检查后回到上层上下文继续处理

## 快速开始

```rust
use tynavi::{
	selector::Selector,
	traits::AsSelector,
};

struct Profile {
	city: String,
	age: u32,
}

struct User {
	name: String,
	profile: Profile,
}

impl<'a> AsSelector<'a, User, ()> for User {
	fn as_selector(&'a self) -> Selector<'a, User, ()> {
		Selector::new(self)
	}
}

let user = User {
	name: "alice".to_owned(),
	profile: Profile {
		city: "Hangzhou".to_owned(),
		age: 20,
	},
};

let city = user
	.as_selector()
	.route_to(|u, _| Some(&u.profile))
	.route_to(|p, _| Some(&p.city))
	.starts_with("Hang")
	.extract(|city, _| city.to_owned());

assert_eq!(city, Some("Hangzhou".to_owned()));

let adult = user
	.as_selector()
	.route_to(|u, _| Some(&u.profile))
	.route_to(|p, _| Some(&p.age))
	.ge(&18)
	.is_matched();

assert!(adult);
```

## 父节点回溯

```rust
let profile = user.as_selector().route_to(|u, _| Some(&u.profile));

let city = profile
	.route_to(|p, _| Some(&p.city))
	.contains("zhou");

assert!(city.is_matched());
assert!(city.backtrack().select().is_some());
assert!(city.up().select().is_some());
```

- `backtrack()`：直接返回父节点快照
- `up()`：返回父节点，并在当前未匹配时把未匹配状态向上传递

## 已实现的核心 API

### 通用构造 / 路由 / 处理

- `new`
- `with`
- `same_parent`
- `route_to`
- `replace`
- `map`
- `select`
- `extract` / `cond_extract`
- `extract_async` / `cond_extract_async`
- `inspect` / `inspect_cursor`
- `filter` / `cond_filter`
- `filter_async` / `cond_filter_async`
- `require_matched`
- `parent`
- `backtrack`
- `up`
- `or_a_parent_a` / `or_a_parent_b` / `or_b_parent_a` / `or_b_parent_b`

### 标准库类型扩展

当前已内置以下常见类型支持：

- 数值类型：`i8` `i16` `i32` `i64` `i128` `isize` `u8` `u16` `u32` `u64` `u128` `usize` `f32` `f64`
- 字符串类型：`&str`、`str`、`String`
- 容器与指针：`Option<T>`、`Result<T, E>`、`HashMap<K, V>`、`&[T]`、`Box<T>`、`Rc<T>`、`Arc<T>`

其中已实现的典型能力包括：

- 数值比较：`eq`、`not_eq`、`gt`、`lt`、`ge`、`le` 及其 `cond_*`
- 字符串过滤：`starts_with`、`ends_with`、`contains`、`contains_char`、`empty`
- 切片导航：`first`、`last`、`indexof`、`find`、`any`、`all`、`none`
- `HashMap` 导航：`keyof`、`find_key`、`find`、`contains_key`、`contains_value`
- `Option` 路由：`flatten`
- `Result` 路由：`ok`、`err`
- 智能指针解引用：`as_ref`

## 生态扩展

当前通过 feature 提供以下扩展：

- `http`
- `axum`
- `tungstenite`
- `serde_json`
- `reqwest`
- `derive`

`full` 会一次性启用：

```toml
["derive", "serde_json", "tungstenite", "http", "axum", "reqwest"]
```

如果只想启用某一类扩展，可以关闭默认 feature 再手动选择：

```toml
[dependencies]
tynavi = { version = "0.1.2", default-features = false, features = ["reqwest"] }
```

### `reqwest` 示例

```rust
use http::Response as HttpResponse;
use reqwest::{Method, Request, Response, ResponseBuilderExt, Url};
use tynavi::traits::AsSelector;

let mut req = Request::new(
	Method::POST,
	Url::parse("https://api.example.com:8443/v1/users?active=true").unwrap(),
);
req.headers_mut().insert("x-token", "secret".parse().unwrap());
*req.body_mut() = Some("payload".into());

assert!(
	req
		.as_selector()
		.url()
		.path()
		.starts_with("/v1/")
		.is_matched()
);

assert!(req.as_selector().body().starts_with(b"pay").is_matched());

let res = Response::from(
	HttpResponse::builder()
		.status(201)
		.url(Url::parse("https://api.example.com/v1/users/42").unwrap())
		.body("hello")
		.unwrap(),
);

assert!(res.as_selector().is_success().is_matched());
assert!(res.as_selector().content_length_eq(5).is_matched());
```

### `serde_json` 示例

```rust
use serde_json::json;
use tynavi::traits::AsSelector;

let value = json!({
	"users": [
		{ "name": "alice" }
	]
});

assert!(
	value
		.as_selector()
		.keyof("users")
		.first()
		.keyof("name")
		.as_str()
		.contains("alice")
		.is_matched()
);
```

## 宏支持

仓库当前已经提供 `tynavi-macros` 子 crate，并通过主 crate 的 `derive` feature 暴露两类宏：

- `#[derive(Selector)]`
- `#[selector]`

它们适合把结构体 / 枚举上的手写导航方法批量生成出来，尤其适用于事件模型或较大的领域对象。

```toml
[dependencies]
tynavi = { version = "0.1.2", default-features = false, features = ["derive"] }
```

使用示例：

```rust
use tynavi::{Selector, selector};
use tynavi::traits::AsSelector;

#[derive(Selector)]
struct Message {
	text: String,
}

#[selector]
enum Event {
	Message(Message),
}

let event = Event::Message(Message {
	text: "hello".to_string(),
});

assert!(event
	.as_selector()
	.route_message()
	.route_text()
	.contains("hello")
	.is_matched());
```

更完整的宏说明见 `macros/docs/selector.md`。

## 与 onebot-api Selector 的差异

| 特性 | onebot-api 旧版 selector | tynavi |
|------|--------------------------|--------|
| 类型签名 | `Selector<'a, T>` | `Selector<'a, Current, Parent>` |
| 可变性 | `&mut self` 风格 | 不可变快照，过滤返回 `Self` |
| 父节点追踪 | 无 | 有，支持 `backtrack()` / `up()` |
| 生成方式 | 主要依赖宏生成 | 可手写扩展，也支持 `derive` / `selector` 宏 |
| 适用范围 | onebot-api 事件模型 | 任意 Rust 类型与若干生态对象 |

## 构建与测试

```bash
cargo check
cargo build
cargo clippy
cargo fmt
cargo test
```

项目使用 `.rustfmt.toml`，采用硬制表符格式。
