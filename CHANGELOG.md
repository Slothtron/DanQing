# 变更记录

格式参考 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/)。
版本号走语义化版本，但**判断标准是令牌与产物的兼容性，不是代码的兼容性**——
规则见 [`docs/07-governance.md`](./docs/07-governance.md#4-版本策略)。

## [3.0.0] - 2026-10-09

只交付终端配色，不再生成平台样式产物。

### 新增

- **Windows Terminal 配色** `themes/windows-terminal/danqing.schemes.json`：
  4 个品牌预设 × 明暗两套 = 8 套，字段与 Windows Terminal 的 `schemes` 格式一致，
  可直接粘进 `settings.json`。这是本仓库唯一对外交付的现成产物。
- 真源新增 `terminal` 段：16 个 ANSI 槽位以 `@色族.色阶` 声明（一槽一族、基色与亮色取同族两档），
  chrome 四项以 `sys.*` 路径声明，另声明与底色同侧的中性槽位免于同底对比门禁。
- 对比度门禁新增「终端」组 **140 项**，合计 108 → **248 项**。
- [`docs/06-terminal-theming.md`](./docs/06-terminal-theming.md)：终端主题契约——槽位与色族的映射表、
  取阶规则、门禁口径、导入方式。
- `showcase/app.css`：配色参考页自包含的页面外壳样式，页面不再依赖生成的令牌样式表。

### 变更

- 产物形态：由「15 种端产物」改为「**一份终端配色** + 三份诊断报告 + 令牌快照 + 参考页数据」。
- 终端配色**入库跟踪**（唯一入库产物）；其余产物仍不入库，报告路径由 `gen/reports/` 迁至 `dist/reports/`。
- 参考页收敛为配色参考浏览器：去掉按钮/输入框/标签/告警/表格/卡片/骨架屏等组件样例与柱状图，
  新增 8 套终端配色预览（伪终端行 + 16 色条 + 光标与选区）。
- `danqing serve` 支持目录请求回落 `index.html`，`/showcase/` 不再 404。
- 令牌快照 `tokens/danqing.tokens.json` 补上 `terminal` 段，`$schema` 提到 `danqing/3.0`。
- `showcase/shots/` 六张截图全部重出为当前页面的分区图。
- 终端配色的光标色取 `sys.primary.default`（而非 `sys.text.primary`）：真源里 `bg.canvas` 与
  `text.primary` 都与品牌无关，光标若不取品牌主色，同一模式下四个品牌只剩选区色不同。

### 移除

- `gen/**` 全部样式产物，及对应的十个发射模块：
  CSS、SCSS、TypeScript、DTCG JSON、Tailwind、Avalonia、SwiftUI、Jetpack Compose、Flutter、Android XML。
- `docs/06-platform-mapping.md`（由 `06-terminal-theming.md` 取代）。
- 只服务于上述产物的死代码：`util` 里的 Python 字符串变换、`model::sub`、`pipeline::brand_varying_keys`、
  `emit` 里的各端色值格式化辅助函数。

**破坏性**：依赖 `gen/**` 的下游（Web 的 `danqing.css`、`danqing.ts`、`Tokens.axaml` 等）需改为自行派生。
各端可读取 `tokens/danqing.tokens.json` 里 80 个语义角色 × 2 模式，组件只引用语义层。

## 更早版本

以下条目按仓库文档与提交历史回填，交代 3.0.0 之前的走向；各版的发布日期未记录。

### 2.3.0

- 生成器由 Python（`tools/build.py`）重写为 Rust 单二进制 `danqing`，运行时依赖降到零。
- 产物换行统一为 LF，不再随平台变化。
- 移除 npm 包壳 `package.json`，改按目录 / git 取用产物。
- 修正 `prefers-reduced-motion` 下 `--dq-dur-normal` 未生效的问题。

### 2.2.0

- 填充上的文字改为在「最亮白」与「最暗墨」两个极端之间择优。
- 图表色自动修正到与页面底 ≥3:1，并记录修正前后值。
- 新增 DTCG（W3C Design Tokens）输出。

### 2.1.0

- 新增 `cang` 苍、`zi` 紫、`tan` 檀 三个色族。
- 新增保留族机制：状态色占用固定色族，品牌只在非保留族取色。
