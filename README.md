# 丹青 · 中国传统色彩设计系统

> **Danqing — a Chinese traditional color design system.**
> 版本 `2.3.0` · 与应用无关 · 单一真源 · 多端产物 · 可验证的对比度门禁
> 构建用 **Rust 单二进制**（`danqing`），**不需要 Node.js / Python 运行时**。

丹青取自「丹砂与青雘」——中国古代绘画所用的两种原色颜料，《周礼·考工记》「画缋之事，杂五色」。
作为一个**色彩设计系统**的名字，它指的就是「颜色这件事」本身，不绑定任何产品、行业或平台。

---

## 1. 这个系统解决什么问题

绝大多数「中式配色」实践止步于一张色卡：给一堆传统色名和 HEX，然后让每个开发者自己决定用哪个、怎么落到按钮和文字上。
结果是同一套色卡在不同项目里长出完全不同的界面，且没人能证明它可读。

丹青把这件事做成工程：

| 问题 | 丹青的做法 |
|---|---|
| 色卡只有几十个色，不够撑起一套界面 | 11 个色族，**每族在 OKLab 感知空间生成 11 级色阶**，共 121 个原语，且锚点级写回传统色原值 |
| 色阶没有中文语义，和传统色名失去联系 | 每一级都**反查最接近的传统色名**（OKLab 欧氏距离），如 `qing-700` = 群青、`su-50` = 凝脂 |
| 有了色卡不知道用在哪 | **80 个语义令牌 × 浅深双模式**，`sys.text.primary` / `sys.primary.default` 等，应用代码只引用语义层 |
| 换个品牌主色要重做一遍 | **品牌只驱动 `primary` / `focus` / `link`**，内置 4 个品牌预设，一个 `data-brand` 属性切换 |
| 主色和「危险色」「成功色」撞色 | **保留族机制**：状态色占用固定色族，品牌只在非保留族取色，从结构上避免语义失效 |
| 颜色好看但不可读 | **108 项 WCAG 对比度门禁**，alpha 先合成再计算，不达标退出码非 0 |
| 每端各写一份色值 | 一个真源生成 **15 种产物**：CSS / SCSS / TS / DTCG / Tailwind / Avalonia / SwiftUI / Compose / Flutter / Android XML |
| 换个场景就要改核心 | **扩展机制**：核心不含任何领域概念，长文阅读等场景以 `ext.*` 按需引入 |

## 2. 三层架构

```
tier1 原语 cn.*      11 中国传统色族 × 11 级        ── 无语义，只被 tier2 引用
   ↓
tier2 语义 sys.*     bg/surface/border/text/primary/accent/状态色/state/overlay/chart
                     × light / dark                  ── 应用代码唯一允许引用的层
   ↓
tier3 扩展 ext.*     领域扩展，按需引入              ── 例：ext.reading 长文阅读
```

**铁律：应用代码不得直接使用 tier1。** `color: #2E59A7` 与 `color: var(--dq-cn-qing-700)` 都是错的，
只有 `color: var(--dq-text-primary)` 和 `color: var(--dq-primary-default)` 是对的。

## 3. 工具链（Rust 单二进制）

生成器是 **Rust** 写的，产物本体（CSS / TS / XAML / Swift / Kotlin / Dart / XML）不含任何
运行时依赖。仓库里**没有** `package.json`，也**不需要** Node.js 或 Python——这是 2.2.0 之前
的老做法，2.3.0 起已全部替换为 `danqing` 一个二进制。

```bash
cargo build --release          # 编译出 ./target/release/danqing
cargo install --path .         # 或装到 ~/.cargo/bin，之后直接 danqing
cargo test --release           # 14 项单元测试（色彩 / 舍入 / 键名 / 门禁骨架）
```

| 子命令 | 作用 | 失败退出码 |
|---|---|---|
| `danqing build` | 生成全部端产物 + 跑对比度门禁 | 门禁不过 → 1 |
| `danqing check` | 只跑门禁，不写盘（本地快速自检） | 门禁不过 → 1 |
| `danqing verify` | 重新生成并与磁盘产物比对，查「改了真源忘了重跑」 | 有漂移 → 1 |
| `danqing serve` | 起静态服务预览展示页（`--port`，默认 3788，只绑 127.0.0.1） | — |

产物一律以 **LF** 换行写出，不随操作系统变化——旧实现在 Windows 上会产出 CRLF、
在 Linux 上产出 LF，同一份真源在两个平台生成的产物互相冲突。

## 4. 五分钟接入

```bash
# 0. 装好生成器（只需一次）
cargo install --path .        # 也可以每次 cargo run --release -- build

# 1. 生成令牌（改 tokens/source.json 后必须重跑）
danqing build
# 门禁不过 → 退出码 1，禁止合入

# 2. Web：引入生成的 CSS
```
```html
<html lang="zh-CN" data-theme="light" data-brand="qing">
  <link rel="stylesheet" href="gen/css/danqing.css" />
```
```css
.btn-primary { background: var(--dq-primary-default); color: var(--dq-primary-on); }
.btn-primary:hover { background: var(--dq-primary-hover); }
```

```js
// 3. TypeScript / 原生
import { light, brands, space, radius } from "./gen/ts/danqing";
```

```csharp
<!-- 4. Avalonia：合并资源词典即可 -->
<ResourceInclude Source="gen/avalonia/Tokens.axaml" />
```

其他端见 [`docs/06-platform-mapping.md`](./docs/06-platform-mapping.md)。

## 5. 目录

| 路径 | 说明 |
|---|---|
| [`tokens/source.json`](./tokens/source.json) | **唯一人手真源**：色族锚点、语义角色、品牌、尺度、扩展 |
| [`tokens/danqing.tokens.json`](./tokens/danqing.tokens.json) | 已解析的完整令牌快照（生成物） |
| [`src/`](./src/) | **Rust 生成器**：色阶 → 角色派生 → 引用解析 → 门禁 → 多端产物 |
| [`data/chinese-colors.json`](./data/chinese-colors.json) | 中国传统色原始数据（386 条 / 8 色系，浏览器实机提取） |
| `gen/` | **全部为生成物，禁止手改** |
| [`docs/`](./docs/) | 设计原则、色彩系统、令牌、主题、无障碍、端映射、治理、溯源 |
| [`showcase/index.html`](./showcase/index.html) | 可视化展示页（色阶长廊 / 语义令牌 / 品牌切换 / 组件 / 门禁表），`danqing serve` 预览 |
| [`reference/koodo/`](./reference/koodo/) | 第三方阅读器设计依据，**仅作参考**，其设计语言不进入本系统 |

## 6. 十分钟读完

1. [`docs/00-overview.md`](./docs/00-overview.md) — 定位、边界、心智模型
2. [`docs/02-color-system.md`](./docs/02-color-system.md) — 11 色族从哪来、色阶怎么算
3. [`docs/03-tokens.md`](./docs/03-tokens.md) — 语义令牌全表与引用语法
4. [`docs/05-accessibility.md`](./docs/05-accessibility.md) — 门禁怎么算、实测多少

## 7. 四条铁律

1. `tokens/source.json` 是唯一人手真源；`gen/` 全部是生成物，**改了会被下次生成覆盖**。
2. 应用只引用 tier2 / tier3；tier1 不得直出到界面。
3. 新增语义角色必须**同时给出浅色与深色两个值**。
4. 状态色（成功/警告/危险/信息/强调）**不得仅靠颜色传达**，必须附图标或文案。

## 8. 许可证

MIT。色值源自公开的中国传统色资料，本系统对其做了色阶推导、语义映射与可读性验证。

## 9. 与 2.2.0 及之前版本的差异

| 项 | 2.2.0 及之前 | 2.3.0 起 |
|---|---|---|
| 生成器 | `python tools/build.py`（仅标准库） | `danqing build`（Rust 单二进制） |
| 运行时依赖 | Python ≥ 3.8 | **无** |
| npm 包壳 | `package.json`（`@danqing/tokens`） | **已移除**；按目录 / git 取用产物 |
| 预览展示页 | `python -m http.server 3788` | `danqing serve`（内置，只绑 127.0.0.1） |
| 产物换行 | 随平台（Windows 产 CRLF） | **恒为 LF** |
| `prefers-reduced-motion` | `--dq-dur-normal` 未生效（漏写 f 前缀） | `--dq-dur-normal: 80ms` 生效 |

产物内容（121 个原语、80 个语义角色 × 2 模式、4 个品牌、108 项门禁）与 2.2.0 **逐字节一致**，
仅上表所列项变化——迁移过程用 Python 版产物做了全量回归比对后才切换。
