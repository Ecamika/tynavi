# AGENTS.md

## 项目概述
- **tynavi**（全称 **type navigator**）是从 onebot-api 的 `selector` feature 中提取并重新设计的通用 **Selector** 模式库。
- 不是 workspace，单 crate 结构。
- Rust **edition 2024**，要求工具链 >= 1.85。
- 主 crate 默认启用 `full` feature，按需集成 `serde_json`、`tungstenite`、`http`、`axum`、`reqwest` 与 `derive` 宏支持。
- 核心 Selector API 可在 `default-features = false` 下以最小依赖使用，但仓库当前并非“只有标准库源码”的纯零依赖状态。
- 仓库包含 `macros/` 过程宏子 crate，用于提供 `#[derive(Selector)]` 与 `#[selector]`。

## 构建 / 检查 / 测试
```bash
cargo check      # 快速编译检查
cargo build      # 完整构建
cargo clippy     # 代码检查
cargo fmt        # 格式化（使用 .rustfmt.toml）
cargo test       # 运行单元测试
```

## 代码格式
- `.rustfmt.toml` 配置：**硬制表符**，`tab_spaces = 2`，`edition = "2024"`。
- 提交前运行 `cargo fmt`。

## Git 提交规则
- git 提交信息必须使用中文。
- 除非用户明确说明排除某些路径，否则 git 提交时必须包含当前工作区内所有**未被 `.gitignore` 列出**的改动文件。
- git 提交前必须读取本次改动涉及的 `changelogs/` 记录，并与 git 中已修改的文件逐项对比。
- 若发现某些文件存在 git 修改，但对应变更未在 `changelogs/` 中提及，必须先将这些遗漏同步追加到当天的 changelog 后再提交。
- 当工作区内存在多个逻辑独立的变更时（如同时包含新功能、bug 修复和文档更新），**必要时可将提交拆分为多个独立的 commit**，每个 commit 只包含一个逻辑变更，以保持提交历史的清晰和可回滚性。

## 核心架构

### Selector 设计
- `Selector<'a, Current, Parent>` 是核心类型，相比 onebot-api 的原版 `Selector<'a, T>` 增加了 **父节点追踪** 能力。
- **不可变 API**：所有过滤方法返回 `Self`（快照语义），而非 `&mut self`。
- 通过 `Snapshot` + `Unmatch` + `Copy` 组合成 `SelectorInstance` trait，使父节点可安全回溯。

### 关键 Trait
| Trait | 作用 |
|-------|------|
| `Snapshot` | 创建自身快照，支持不可变链式调用 |
| `Unmatch` | 将当前 Selector 置为未匹配状态 |
| `SelectorInstance` | `Snapshot + Unmatch + Copy` 的组合约束 |
| `AsSelector` | 为类型提供 `.as_selector()` 入口 |

### 与 onebot-api Selector 的主要差异
| 特性 | onebot-api | tynavi |
|------|-----------|--------|
| 类型签名 | `Selector<'a, T>` | `Selector<'a, Current, Parent>` |
| 可变性 | `&mut self` | 返回 `Self`（不可变） |
| 父节点追踪 | 无 | 有（支持 `backtrack()` / `up()`） |
| 生成方式 | `#[derive(Selector)]` 宏 | 同时支持手动 impl 与 `derive` / `selector` 宏 |
| 比较操作 | 仅基础 `eq/gt/lt/ge/le` | 完整 + `not_*` + `cond_*`，并覆盖整数、无符号整数与浮点数 |

### 核心方法
- `route_to(extractor)` — 通过提取器路由到子类型
- `replace(v)` / `map(f)` — 替换或映射当前游标
- `filter(f)` / `cond_filter(cond, f)` / `filter_async(f)` — 过滤（含条件与异步变体）
- `extract(f)` / `cond_extract(cond, f)` / `extract_async(f)` — 提取值
- `inspect(f)` / `inspect_cursor(f)` — 检查当前匹配态或游标并返回原快照
- `select()` — 获取 `Option<&Current>`
- `is_matched()` / `require_matched()` — 是否匹配 / 转为 `Result`
- `backtrack()` / `up()` — 返回父节点
- `or_a_parent_a()` 等 `or_*` 组合方法 — 合并两个 Selector 的游标并选择保留哪一侧 parent

### 已实现类型扩展
- **数字类型**：`i8`/`i16`/`i32`/`i64`/`i128`/`isize`、`u8`/`u16`/`u32`/`u64`/`u128`/`usize`、`f32`/`f64`
  - 每个都有：`eq`, `not_eq`, `gt`, `not_gt`, `lt`, `not_lt`, `ge`, `not_ge`, `le`, `not_le`
  - 每个都有条件变体：`cond_*`
- **字符串类型**：`&str`, `str`, `String`
  - `starts_with`, `ends_with`, `contains`, `contains_char`, `empty` 及条件变体
- **标准库容器/指针**：`Option<T>`、`Result<T, E>`、`HashMap<K, V>`、`&[T]`、`Box<T>`、`Rc<T>`、`Arc<T>`
  - `flatten`、`ok`/`err`、`keyof`、`first`/`last`/`indexof`、`as_ref` 等路由能力
- **生态扩展**：`http`、`axum`、`tungstenite`、`serde_json`、`reqwest`
  - 由 feature 控制，默认 `full` 会全部启用

## 关键约定
- `()` 实现了 `Snapshot` + `Unmatch`，作为根父节点使用。
- 所有 `cond_*` 方法在 `condition = false` 时直接返回 `snapshot()`，不执行过滤逻辑。
- 异步方法使用 `AsyncFnOnce`（edition 2024 特性）。

## 与 onebot-api 的关系
- tynavi 的 Selector 是 onebot-api `selector` feature 的**通用化重构**。
- onebot-api 使用 proc-macro（`onebot-api-macros`）为事件类型自动生成 Selector 方法。
- tynavi 将核心库与宏 crate 分离：可手动实现以保持最小依赖，也可启用 `derive` 复用自动生成能力。

## Changelog rule

**无论对项目做出何种修改（`src/`、`tests/`、`.github/`、`AGENTS.md` 等），都必须在 `changelogs/` 文件夹下记录变更。**

### 文件命名规则
- 以日期命名：`YYYY-MM-DD.md`
- 同一天的所有变更合并到同一个文件中

### 记录流程
每次变更必须按以下步骤记录：

1. **定位文件**：在 `changelogs/` 文件夹下搜索以**今天日期**命名的 markdown 文件 `YYYY-MM-DD.md`。
2. **若文件不存在**：
   - 创建文件 `changelogs/YYYY-MM-DD.md`
   - 文件顶部写入一级标题 `# YYYY-MM-DD 变更记录`
   - 按照本规范的文件结构模板，写入完整的变更条目
3. **若文件已存在**：
   - **重新读取文件**：在追加前必须重新读取该文件的完整内容，确认文件未被其他会话修改（防止历史记录被覆盖）
   - 在文件末尾（最后一个条目之后）追加新的变更条目
   - **禁止修改、删除或覆盖文件中已有的任何历史条目**
4. **条目格式**：每个条目必须严格按照本规范的二级标题、三级标题、列表格式书写。

### 文件结构模板
```markdown
# YYYY-MM-DD 变更记录

## [变更类型] 变更简要说明（一句话概括）

### 变更内容
- 具体做了什么
- 关键代码/逻辑改动点

### 涉及文件
- `文件路径` [文件修改类型] 对该文件内修改的说明

### 影响
- 对调用方、行为或性能的影响

### 原因
- 变更的动机和背景
```

### 一级标题
统一格式为 `# YYYY-MM-DD 变更记录`，例如：
```markdown
# 2026-05-31 变更记录
```

### 二级标题
统一格式为 `## [变更类型] 变更简要说明`。

变更类型使用 **git commit message type**，允许多类型复用，破坏性变更追加 `[BREAKING]`：

| 类型 | 说明 |
|------|------|
| `[feat]` | 新增功能、模块、API、trait |
| `[fix]` | 修复 Bug、逻辑错误 |
| `[docs]` | 文档、注释、README、changelog 本身的修改 |
| `[style]` | 代码格式、缩进、分号、空行等不影响逻辑的修改 |
| `[refactor]` | 重构（行为不变，内部结构优化） |
| `[perf]` | 性能优化 |
| `[test]` | 新增或修改测试代码 |
| `[chore]` | 构建脚本、工具配置、依赖版本升级等杂项 |
| `[build]` | 构建系统或外部依赖的修改（如 Cargo.toml、Makefile） |
| `[ci]` | CI/CD 配置修改（如 `.github/workflows/`） |
| `[revert]` | 回滚之前的提交 |

**多类型复用示例：**
```markdown
## [perf][fix][BREAKING] 重构事件分发逻辑并修复内存泄漏
```

> `[BREAKING]` 标记必须放在所有类型之后。

### 三级标题（强制包含）
每个变更条目必须包含以下四个三级标题：

#### `### 变更内容`
- 用无序列表描述具体改动
- 关键代码逻辑、API 签名变更、数据结构变动等

#### `### 涉及文件`
- 文件路径统一以**项目根目录**开始，不使用 `./` 前缀
- 每个文件必须标注**文件修改类型**
- 必须附带对该文件内具体修改的说明

**文件修改类型：**

| 类型 | 说明 |
|------|------|
| `[Added]` | 新增文件 |
| `[Changed]` | 修改文件内容 |
| `[Removed]` | 删除文件 |
| `[Moved]` | 重命名或移动文件（建议注明来源和去向）|

**格式示例：**
```markdown
### 涉及文件
- `src/error.rs` [Changed] 扩展 ServiceRuntimeError 枚举，新增 9 个错误变体
- `Cargo.toml` [Changed] 从 dependencies 中移除 anyhow
- `src/quick_operation.rs` [Added] 新增 6 个 quick operation trait 定义
- `src/event/old_handler.rs` [Removed] 删除已废弃的事件处理模块
```

#### `### 影响`
- 对现有功能、调用方或行为的影响
- 若有 Breaking change，必须明确说明迁移方式

#### `### 原因`
- 变更的动机和背景
- 解决了什么问题或满足了什么需求

### 会话隔离规则
**禁止修改已存在的历史 changelog 条目。当前会话只能向当天的 changelog 文件追加新条目，不得修改、删除或覆盖已有条目（无论日期）。若发现历史 changelog 有遗漏或错误，应在当天的新条目中说明纠正，而非直接修改已有文件。**

### 规范要点
1. **同一天多次变更**：在同个 `YYYY-MM-DD.md` 中按时间顺序追加 `## [类型] 标题` 条目，不要拆成多个文件
2. **文件路径**：所有路径必须用反引号 `` ` `` 包裹，以项目根目录为起点（如 `src/lib.rs`、`README.md`）
3. **标题层级**：严格使用 `#` → `##` → `###`，禁止跨级或多余层级
4. **语言**：标题和正文统一使用中文，技术术语（如 API、trait、crate、git commit type）可保留英文
5. **粒度**：每个 `##` 条目对应一个独立的变更主题，避免一个条目涵盖多个无关修改

## 宏文档规则
**对 `macros/` 目录进行任何修改（新增、删除或更改过程宏及其行为）时，必须同步更新 `macros/docs/` 中的文档。**

- 新增过程宏：在 `macros/docs/` 下创建与宏同名的 markdown 文件（例如 `#[my_macro]` 对应 `my_macro.md`），记录其功能、用法和属性。
- 删除过程宏：从 `macros/docs/` 中删除对应的 markdown 文件。
- 修改宏行为或属性：更新已有 markdown 文件以反映新行为。

---

## Selector API 规范

### 方法分类体系

所有 Selector 方法分为四大类：

| 类别 | 语义 | 签名特征 | 示例 |
|------|------|---------|------|
| **构造方法** | 创建或转换 Selector | 不依赖 cursor 是否匹配 | `new`, `with`, `same_parent`, `replace`, `map` |
| **过滤方法** | 对 cursor 进行条件判断，不满足则 unmatch | 返回 `Self` | `filter`, `eq`, `starts_with`, `any` |
| **路由方法** | 导航到子类型 | 返回 `Selector<'a, Child, Self>` | `route_to`, `first`, `keyof`, `ok`, `flatten` |
| **处理方法** | 查询状态或提取值，不改变 Selector | 返回外部类型 | `select`, `is_matched`, `extract`, `inspect`, `backtrack` |

### 命名规范

#### 类型与文件
- 结构体 / Trait：PascalCase（`Selector`, `SelectorNotMatched`, `Snapshot`）
- 模块 / 文件：snake_case（`hash_map.rs`, `serde_json/`）
- 数字类型模块：精确使用原始类型名（`i32.rs`, `usize.rs`, `f64.rs`）

#### 泛型参数
- 公开 API：语义化命名，`Current` / `C`，`Parent` / `P`，`T`，`E`，`K`，`V`，`R`
- 宏内部：双下划线 + `Tynavi` 前缀防冲突（`__TynaviParent`, `__TynaviOutput`）

#### 生命周期
- 公开 API：`'a`，偶尔 `'b`
- 宏生成：`'__tynavi_a`

### 方法命名与变体规范

#### 语义一致性原则

Selector 方法命名**不应直接照搬底层容器的原始方法名**，而应体现 Selector 的导航与过滤语义。

| 底层 API | Selector 语义命名 | 说明 |
|----------|-------------------|------|
| `Vec::get(index)` | `indexof(index)` | 按索引路由到元素，返回 `Selector<'a, T, Self>` |
| `HashMap::get(key)` | `keyof(key)` | 按键路由到值，返回 `Selector<'a, V, Self>` |
| `Option::unwrap()` | `flatten()` | 解包 `Option<T>` 为 `T` |
| `Result::ok()` | `ok()` | 路由到 `Ok` 变体 |

#### 变体体系（仅过滤方法适用）

过滤方法的完整变体矩阵：

| 变体 | 命名模式 | 适用类别 | 说明 |
|------|---------|---------|------|
| 基础 | `{name}` | 过滤 | 直接执行过滤 |
| 否定 | `not_{name}` | 过滤 | 逻辑取反后过滤 |
| 条件 | `cond_{name}` | 过滤 | `condition == false` 时短路，直接返回 `snapshot()` |
| 条件否定 | `cond_not_{name}` | 过滤 | 条件 + 否定的组合 |
| 异步 | `{name}_async` | 过滤 | 异步闭包版本 |
| 条件异步 | `cond_{name}_async` | 过滤 | 条件 + 异步的组合 |

**关键约束**：
1. **路由方法不提供条件变体**。路由改变 `Current` 类型，条件短路无法通过返回 `Self` 表达，因此路由方法只有基础版本（必要时调用方可通过 `if` 分支自行控制）。
2. **处理方法不提供条件变体**。`extract`、`inspect` 等返回外部类型，其条件语义可由调用方用 `if` 直接表达，无需冗余 API。
3. **构造方法不提供任何变体**。

#### 语义前缀 / 后缀速查

| 前缀/后缀 | 语义 | 所属类别 | 示例 |
|-----------|------|---------|------|
| `cond_` | 条件短路 | 过滤 | `cond_eq`, `cond_filter` |
| `not_` | 逻辑否定 | 过滤 | `not_eq`, `not_contains` |
| `*_async` | 异步闭包 | 过滤 | `filter_async`, `any_async` |
| `is_*` | 类型/状态检查（过滤） | 过滤 | `is_null`, `is_matched` |
| `not_is_*` | 否定类型检查 | 过滤 | `not_is_null` |
| `as_*` | 提取/转换（路由） | 路由 | `as_bool`, `as_ref`, `as_str` |
| `route_*` | 导航到子字段（宏生成） | 路由 | `route_id`, `route_name` |
| `or_a_parent_a` | 组合两 Selector 的 cursor，保留左边 parent | 过滤 | `or_a_parent_a`, `or_a_parent_b` |

#### 参数命名惯例

| 参数名 | 含义 | 出现位置 |
|--------|------|---------|
| `v` | 用于比较的值 | `eq(v)`, `gt(v)` |
| `pat` | 模式/子串 | `starts_with(pat)`, `contains(pat)` |
| `value` | 元素值 | `contains(value)` |
| `list` | 值列表 | `one_of(list)` |
| `condition` | 条件布尔 | 所有 `cond_*` 方法的第一个参数 |
| `f` / `extractor` | 闭包 | `filter(f)`, `route_to(extractor)` |
| `min`, `max` | 范围边界 | `in_open_range(min, max)` |
| `key` | 键 | `keyof(key)`, `contains_key(key)` |
| `index` | 索引 | `indexof(index)` |

### 方法实现模板

#### 过滤方法实现模式
```rust
pub fn {name}(&self, ...) -> Self {
	if let Some(cursor) = self.cursor
		&& !{predicate}(cursor, ...)
	{
		self.unmatch()
	} else {
		self.snapshot()
	}
}
```

#### 条件过滤方法实现模式
```rust
pub fn cond_{name}(&self, condition: bool, ...) -> Self {
	if condition {
		self.{name}(...)
	} else {
		self.snapshot()
	}
}
```

#### 路由方法实现模式
```rust
pub fn {name}(&self, ...) -> Selector<'a, Child, Self> {
	self.route_to(|cursor, _| {extract_child(cursor, ...)})
}
```
