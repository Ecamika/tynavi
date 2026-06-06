# Selector 宏使用说明

本目录记录 `tynavi` 提供的两个 Selector 过程宏：

- `#[derive(Selector)]`：为已有结构体或枚举生成 tynavi 风格的 selector API。
- `#[selector]`：仅用于 enum，先把多字段变体改写为单字段 payload 变体，再自动派生 `Selector`。

## 启用方式

宏通过主 crate 的 `derive` feature 暴露。默认不启用该 feature 时，`tynavi` 不引入 `syn`、`quote`、`proc-macro2` 等过程宏依赖。

```toml
[dependencies]
tynavi = { version = "0.1", features = ["derive"] }
```

使用时通常导入：

```rust
use tynavi::{Selector, selector};
use tynavi::traits::AsSelector;
```

`Selector` 是 derive 宏，`selector` 是属性宏。

## `#[derive(Selector)]`

`#[derive(Selector)]` 不会改写原始类型，只会追加生成代码。

对类型 `Data`，宏会生成：

- `impl AsSelector<'a, Data, ()> for Data`
- `pub trait DataSelector<'a, P: SelectorInstance>`
- `impl<'a, P: SelectorInstance> DataSelector<'a, P> for Selector<'a, Data, P>`
- 对 enum 额外生成类型自身的 `is_*` / `as_*` 辅助方法

### 类型级参数

#### `rename = "..."`

修改生成的 selector trait 名称。

```rust
#[derive(Selector)]
#[selector(rename = "UserSelector")]
struct User {
	id: i64,
}
```

默认会生成 `UserSelector`；上例显式指定后仍叫 `UserSelector`。如果写成：

```rust
#[derive(Selector)]
#[selector(rename = "AccountSelector")]
struct User {
	id: i64,
}
```

则生成的 trait 名为 `AccountSelector`。

## 单元结构体、无字段元组结构体、无字段类型

单元结构体和无字段元组结构体不会生成字段方法，只生成 `AsSelector` 和空 selector trait/impl。

```rust
#[derive(Selector)]
struct Unit;

#[derive(Selector)]
struct TupleUnit();

let unit = Unit;
assert!(unit.as_selector().is_matched());

let tuple = TupleUnit();
assert!(tuple.as_selector().is_matched());
```

## 命名结构体

命名结构体会为每个字段生成四组方法：`inspect`、`filter`、`extract`、`route`。

```rust
#[derive(Selector)]
struct User {
	id: i64,
	name: String,
}
```

对字段 `id` 生成：

```rust
fn id_inspect(&self, f: impl FnOnce(&i64, &Self)) -> Self;
fn cond_id_inspect(&self, condition: bool, f: impl FnOnce(&i64, &Self)) -> Self;
async fn id_inspect_async(&self, f: impl AsyncFnOnce(&i64, &Self)) -> Self;
async fn cond_id_inspect_async(&self, condition: bool, f: impl AsyncFnOnce(&i64, &Self)) -> Self;

fn id_filter(&self, f: impl FnOnce(&i64, &Self) -> bool) -> Self;
fn cond_id_filter(&self, condition: bool, f: impl FnOnce(&i64, &Self) -> bool) -> Self;
async fn id_filter_async(&self, f: impl AsyncFnOnce(&i64, &Self) -> bool) -> Self;
async fn cond_id_filter_async(&self, condition: bool, f: impl AsyncFnOnce(&i64, &Self) -> bool) -> Self;

fn id_extract<T>(&self, f: impl FnOnce(&i64, &Self) -> T) -> Option<T>;
fn cond_id_extract<T>(&self, condition: bool, f: impl FnOnce(&i64, &Self) -> T) -> Option<T>;
async fn id_extract_async<T>(&self, f: impl AsyncFnOnce(&i64, &Self) -> T) -> Option<T>;
async fn cond_id_extract_async<T>(&self, condition: bool, f: impl AsyncFnOnce(&i64, &Self) -> T) -> Option<T>;

fn route_id(&self) -> Selector<'_, i64, Self>;
```

实际使用：

```rust
let user = User {
	id: 1,
	name: "alice".to_string(),
};

let matched = user.as_selector().id_filter(|id, _| *id > 0);
assert!(matched.is_matched());

let len = user.as_selector().name_extract(|name, _| name.len());
assert_eq!(len, Some(5));

let id = user.as_selector().route_id().eq(&1);
assert!(id.is_matched());
```

## 元组结构体

元组结构体的规则与命名结构体相同，但字段名从 `field_1` 开始。

```rust
#[derive(Selector)]
struct Pair(i64, String);

let pair = Pair(7, "hello".to_string());

assert_eq!(pair.as_selector().route_field_1().select(), Some(&7));
assert!(pair
	.as_selector()
	.route_field_2()
	.contains("hell")
	.is_matched());
```

## 字段级参数

字段级参数写在字段上的 `#[selector(...)]` 中。

### `rename = "..."`

修改生成方法中的字段名部分。

```rust
#[derive(Selector)]
struct User {
	#[selector(rename = "id")]
	user_id: i64,
}

let user = User { user_id: 1 };
assert!(user.as_selector().route_id().eq(&1).is_matched());
```

字段本身仍叫 `user_id`，但宏方法使用 `id`：`id_filter`、`id_extract`、`route_id` 等。

### `skip`

跳过该字段全部方法。

```rust
#[derive(Selector)]
struct Secret {
	#[selector(skip)]
	token: String,
}
```

上例不会生成 `token_inspect`、`token_filter`、`token_extract`、`route_token`。

### `skip(...)`

只跳过指定方法组。允许的参数为：

- `inspect`
- `filter`
- `extract`
- `route`

```rust
#[derive(Selector)]
struct User {
	#[selector(skip(inspect, route))]
	name: String,
}
```

上例不会生成 `name_inspect` 系列和 `route_name`，但仍会生成 `name_filter` 与 `name_extract` 系列。

### `variants(...)`

为枚举字段生成快捷判定方法。字段类型必须自己提供对应的 `is_*` 方法。

```rust
#[derive(Clone, Copy)]
enum Role {
	Admin,
	Member,
}

impl Role {
	fn is_admin(&self) -> bool {
		matches!(self, Self::Admin)
	}

	fn is_member(&self) -> bool {
		matches!(self, Self::Member)
	}
}

#[derive(Selector)]
struct User {
	#[selector(variants(admin, member))]
	role: Role,
}

let user = User { role: Role::Admin };

assert!(user.as_selector().is_admin().is_matched());
assert!(!user.as_selector().is_member().is_matched());
assert!(!user.as_selector().not_is_admin().is_matched());
assert!(user.as_selector().cond_is_member(false).is_matched());
```

对 `admin` 会生成：

- `is_admin()`
- `not_is_admin()`
- `cond_is_admin(condition)`
- `cond_not_is_admin(condition)`

## 枚举

`#[derive(Selector)]` 支持：

- 无变体 enum
- 无数据变体
- 单元素元组变体

多元素元组变体和匿名结构体变体需要使用 `#[selector]` 属性宏。

### 无变体 enum

```rust
#[derive(Selector)]
enum Empty {}
```

只生成 `AsSelector` 和空 selector trait/impl。

### 无数据变体

无数据变体会在 enum 本身生成 `is_variant()`，并在 selector 上生成同名过滤方法。

```rust
#[derive(Selector)]
enum Event {
	Ready,
}

let event = Event::Ready;

assert!(event.is_ready());
assert!(event.as_selector().is_ready().is_matched());
```

### 单元素元组变体

单元素元组变体会生成：

- enum 自身：`is_variant()`
- enum 自身：`as_variant() -> Option<&Payload>`
- selector：`is_variant()`
- selector：`route_variant()`

```rust
#[derive(Selector)]
struct Message {
	text: String,
}

#[derive(Selector)]
enum Event {
	Message(Message),
}

let event = Event::Message(Message {
	text: "hello".to_string(),
});

assert!(event.is_message());
assert!(event.as_message().is_some());

assert!(event
	.as_selector()
	.route_message()
	.route_text()
	.contains("hello")
	.is_matched());
```

### 变体名转换

enum 变体名会转换为 snake_case。

```rust
#[derive(Selector)]
enum Event {
	MetaEvent(Meta),
}

// 生成 is_meta_event / as_meta_event / route_meta_event
```

## `#[selector]` 属性宏

`#[selector]` 仅能标注 enum。它会先改写 enum，再自动为改写后的 enum 添加 `#[derive(tynavi::Selector)]`，除非 enum 已经显式派生了 `Selector`。

属性宏的目标是让多字段变体也能通过 `route_*` 路由。它通过生成真实存在于 enum 内部的 payload 结构体来实现这一点。

### 无数据变体和单元素元组变体

这两类变体保持原样。

```rust
#[selector]
enum Event {
	Ready,
	Message(Message),
}
```

展开语义等价于：

```rust
#[derive(Selector)]
enum Event {
	Ready,
	Message(Message),
}
```

### 多元素元组变体

多元素元组变体会改写为单元素元组变体。

```rust
#[selector]
enum Event {
	Moved(i64, i64),
}
```

展开语义等价于：

```rust
#[derive(Selector)]
struct EventMoved(pub i64, pub i64);

#[derive(Selector)]
enum Event {
	Moved(EventMoved),
}
```

使用时需要按改写后的形状构造：

```rust
let event = Event::Moved(EventMoved(1, 2));

assert!(event
	.as_selector()
	.route_moved()
	.route_field_1()
	.eq(&1)
	.is_matched());
```

### 匿名结构体变体

匿名结构体变体也会改写为单元素元组变体。

```rust
#[selector]
enum Event {
	Joined {
		user_id: i64,
		group_id: i64,
	},
}
```

展开语义等价于：

```rust
#[derive(Selector)]
struct EventJoined {
	pub user_id: i64,
	pub group_id: i64,
}

#[derive(Selector)]
enum Event {
	Joined(EventJoined),
}
```

使用时：

```rust
let event = Event::Joined(EventJoined {
	user_id: 1,
	group_id: 2,
});

assert!(event
	.as_selector()
	.route_joined()
	.route_user_id()
	.eq(&1)
	.is_matched());
```

### 变体级 `rename = "..."`

修改生成 payload 结构体的名称，也会影响 derive 宏基于变体属性得到的方法名部分。

```rust
#[selector]
enum Event {
	#[selector(rename = "MovedPayload")]
	Moved(i64, i64),
}

let event = Event::Moved(MovedPayload(1, 2));
```

### 变体级 `derive(...)`

为生成的 payload 结构体追加派生项。

```rust
#[selector]
enum Event {
	#[selector(rename = "MovedPayload", derive(Debug, Clone, Copy))]
	Moved(i64, i64),
}

let payload = MovedPayload(1, 2);
let copied = payload;
println!("{payload:?} {copied:?}");
```

生成语义：

```rust
#[derive(Selector, Debug, Clone, Copy)]
struct MovedPayload(pub i64, pub i64);
```

### 完整示例

```rust
use tynavi::{Selector, selector};
use tynavi::traits::AsSelector;

#[derive(Selector)]
struct Message {
	text: String,
}

#[selector]
enum Event {
	Ready,
	Message(Message),

	#[selector(rename = "MovedPayload", derive(Debug, Clone, Copy))]
	Moved(i64, i64),

	#[selector(rename = "JoinedPayload", derive(Debug, Clone, Copy))]
	Joined {
		user_id: i64,
		group_id: i64,
	},
}

let message = Event::Message(Message {
	text: "hello".to_string(),
});

assert!(message
	.as_selector()
	.route_message()
	.route_text()
	.contains("hello")
	.is_matched());

let moved = Event::Moved(MovedPayload(1, 2));
assert!(moved
	.as_selector()
	.route_moved()
	.route_field_2()
	.eq(&2)
	.is_matched());

let joined = Event::Joined(JoinedPayload {
	user_id: 7,
	group_id: 9,
});
assert!(joined
	.as_selector()
	.route_joined()
	.route_group_id()
	.eq(&9)
	.is_matched());
```

## 限制

- `#[derive(Selector)]` 不改写用户类型。
- `#[derive(Selector)]` 不支持 union。
- `#[derive(Selector)]` 直接用于 enum 时，只支持无数据变体和单元素元组变体。
- `#[selector]` 属性宏只支持 enum。
- `#[selector]` 改写多字段变体后，构造 enum 时也要使用生成的 payload 类型。
- 宏不生成 onebot-api 旧式可变 `and_filter_*` API。
