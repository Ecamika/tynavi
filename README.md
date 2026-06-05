# tynavi

`tynavi`（全称 `Type Navigator`）是一个零依赖的 Rust Selector 模式库，专注于两件事：

- 用不可变、可链式的方式在数据结构中导航
- 在选择子链路中保留父节点快照，支持回溯

它来自 [`onebot-api`](https://github.com/Ecamika/onebot-api) 的 `selector` feature（1.2.0 ~ 1.2.5），但被重构为一个更通用独立库

## 特性

- 零外部依赖
- Rust `edition = "2024"`
- 要求工具链 `>= 1.85`
- `Selector<'a, Current, Parent>` 同时追踪当前节点和父节点
- 所有过滤 API 返回 `Self`，适合不可变链式调用
- 支持 `backtrack()` / `up()` 回到父节点
- 内置数字、字符串和智能指针类型扩展
- 提供同步与异步过滤/提取接口

## 设计概念

核心类型：

```rust
Selector<'a, Current, Parent>
```

- `Current`：当前游标指向的类型
- `Parent`：父节点快照类型
- `cursor: Option<&'a Current>`：当前是否匹配

和常见的可变 Selector 设计不同，`tynavi` 的方法不会原地修改自身，而是返回新的快照，这使得链式调用更稳定，也让“当前节点”和“父节点”可以安全一起传递

## 这个库解决什么问题

`tynavi` 最初来自 `onebot-api` 的事件处理场景（1.2.0 ~ 1.2.5），它主要解决的不是“完全替代模式匹配”，而是替代那些在深层嵌套事件结构中，为了取字段、筛选条件和保留上下文而产生的大量样板 `match`

它尤其适合下面几类问题：

- 深层嵌套枚举或结构体访问过于冗长
- 事件处理中“先取值、再判断、再继续下钻”的流程分散在多层 `match + if` 中
- 中间任意一步不匹配时，需要反复手写失败分支
- 深入某个子字段检查后，还需要回到父事件继续处理
- 相似的事件筛选逻辑难以沉淀成可复用的小段能力

换句话说，`tynavi` 更擅长表达：

- 我要进入哪个字段
- 我要对当前值施加什么条件
- 匹配成功后我要提取什么
- 必要时我要回到哪一层上下文

在这种场景下，链式 Selector 往往比直接展开多层模式匹配更接近业务意图

## 不打算解决什么问题

`tynavi` 也有明确边界，它并不适合替代所有 `match`：

- 当逻辑本身是复杂分支控制流时，`match` 通常更直接
- 当你需要穷尽枚举分支并显式处理每一种情况时，模式匹配更清晰
- 当导航路径本身不稳定、需要大量自定义提取器时，链式调用的可读性未必比手写控制流更好

因此更准确的定位是：

`tynavi` 用来处理“深层数据导航 + 条件筛选 + 结果提取 + 父上下文回退”这类问题，而不是用来取代 Rust 里所有的模式匹配

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
	.route_to(|u| Some(&u.profile))
	.route_to(|p| Some(&p.city))
	.starts_with("Hang")
	.extract(|city| city.to_owned());

assert_eq!(city, Some("Hangzhou".to_owned()));

let age_ok = user
	.as_selector()
	.route_to(|u| Some(&u.profile))
	.route_to(|p| Some(&p.age))
	.ge(&18)
	.is_matched();

assert!(age_ok);
```

## 父节点回溯

`tynavi` 的一个关键能力是父节点追踪

```rust
let profile_selector = user
	.as_selector()
	.route_to(|u| Some(&u.profile));

let city_selector = profile_selector
	.route_to(|p| Some(&p.city))
	.contains("zhou");

let parent = city_selector.backtrack();
assert!(parent.select().is_some());
```

- `backtrack()`：直接返回父节点快照
- `up()`：回到父节点，并在当前未匹配时把未匹配状态向上传递

这让“先深入筛选，再回到上层继续处理”成为可能

## 核心 API

### 路由与转换

- `route_to(extractor)`：进入子字段，父节点变为当前快照
- `replace(v)`：替换当前游标
- `map(f)`：将当前引用映射到另一引用

### 过滤

- `filter(f)`
- `cond_filter(condition, f)`
- `filter_async(f)`
- `cond_filter_async(condition, f)`

约定：

- 当 `condition == false` 时，所有 `cond_*` 方法都直接返回 `snapshot()`
- 过滤失败后，Selector 会进入未匹配状态

### 提取与检查

- `select()`：返回 `Option<&Current>`
- `extract(f)`：提取值，返回 `Option<R>`
- `extract_async(f)`：异步提取
- `is_matched()`：检查当前是否匹配
- `require_matched()`：返回 `SelectorResult<Self>`

### 回溯

- `parent()`：获取父节点快照
- `backtrack()`：返回父节点
- `up()`：向上返回，并传播未匹配状态

## 已内置的类型扩展

### 数字类型

已支持：

- `i8` `i16` `i32` `i64` `i128` `isize`
- `u8` `u16` `u32` `u64` `u128` `usize`

每个数字类型都提供：

- `eq` / `not_eq`
- `gt` / `not_gt`
- `lt` / `not_lt`
- `ge` / `not_ge`
- `le` / `not_le`
- 对应的全部 `cond_*` 变体

### 字符串类型

已支持：

- `&str`
- `String`

提供：

- `starts_with`
- `ends_with`
- `contains`
- 对应的全部 `cond_*` 变体

### 智能指针类型

已支持：

- `Box<T>`
- `Rc<T>`
- `Arc<T>`

提供：

- `as_ref()`：将 `Selector<Box<T>, _>`、`Selector<Rc<T>, _>`、`Selector<Arc<T>, _>` 路由到 `Selector<T, _>`

## 为自定义类型接入

接入一个自定义根类型时，通常只需要实现 `AsSelector`：

```rust
use tynavi::{selector::Selector, traits::AsSelector};

struct Event {
	id: u64,
}

impl<'a> AsSelector<'a, Event, ()> for Event {
	fn as_selector(&'a self) -> Selector<'a, Event, ()> {
		Selector::new(self)
	}
}
```

如果你希望某个字段也拥有更自然的导航入口，可以继续围绕 `Selector<'a, T, P>` 为该类型扩展方法

## 构建与测试

```bash
cargo check
cargo build
cargo clippy
cargo fmt
cargo test
```

项目使用 `.rustfmt.toml`，采用硬制表符格式

## 与 onebot-api Selector 的区别

| 特性 | onebot-api（1.2.0 ~ 1.2.5） | tynavi |
|------|-----------|--------|
| 类型签名 | `Selector<'a, T>` | `Selector<'a, Current, Parent>` |
| 可变性 | `&mut self` 风格 | 返回 `Self` 的不可变快照 |
| 父节点追踪 | 无 | 有 |
| 使用范围 | onebot-api 事件模型 | 任意 Rust 类型 |

## 当前状态

这个库目前已经具备可用的基础 Selector 能力，适合：

- 在嵌套结构中做只读导航与筛选
- 需要保留父节点上下文的链式查询
- 希望避免宏和外部依赖的轻量场景

如果后续继续扩展，比较自然的方向包括更多标准库类型支持，以及为业务模型补充更贴近领域的导航方法
