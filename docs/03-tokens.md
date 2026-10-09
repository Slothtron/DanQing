# 03 · 令牌参考

## 1. 引用语法（真源内部）

`tokens/source.json` 里可以用四种写法，生成器在构建时全部解析成字面值：

| 写法 | 含义 | 例 |
|---|---|---|
| `#RRGGBB` / `rgba(r,g,b,a)` | 字面值 | `"#FFFFFF"` |
| `@family.step` | 引用原语色阶 | `"@su.100"` → `#E4E1D8` |
| `@brand.*` | 引用**当前品牌**的派生角色 | `"@brand.fill"` / `"@brand.on"` / `"@brand.text"` |
| `@accent.*` | 引用强调族（保留族 `zi`）的派生角色 | `"@accent.fill"` |
| `rgba(@inkRgb,a)` | 引用当前模式的墨色 RGB 三元组 | `"rgba(@inkRgb,0.08)"` |

`@brand.*` 与 `@accent.*` 支持的键：`fill`、`fillHover`、`fillActive`、`subtle`、`border`、`text`、`on`。

这四个键**不是手写的色值**，而是由生成器 Rust 端 `src/model.rs` 的 `role_set()` 从色族色阶上按「锚点保真 + 可读性回退」规则推导出来的
（规则见 [`02-color-system.md`](./02-color-system.md#4-锚点保真-vs-可读性自动回退机制)）。

## 2. 语义令牌全表（`sys.*`）

80 个角色 × 浅色/深色两套。**应用代码唯一允许直接引用的层。**

| 角色 | 浅色 | 深色 |
|---|---|---|
| `bg.canvas` | `#E4E1D8` | `#101318` |
| `bg.base` | `#F5F2E9` | `#14171C` |
| `bg.sunken` | `#CCC9C1` | `#0E1116` |
| `bg.inverse` | `#1E2732` | `#F5F2E9` |
| `surface.default` | `#FFFFFF` | `#1E2732` |
| `surface.raised` | `#FFFFFF` | `#3D4652` |
| `surface.overlay` | `#FFFFFF` | `#3D4652` |
| `surface.sunken` | `#E4E1D8` | `#0E1116` |
| `surface.variant` | `#CCC9C1` | `#3D4652` |
| `surface.inverse` | `#3D4652` | `#F5F2E9` |
| `border.subtle` | `rgba(30,39,50,0.08)` | `rgba(212,229,239,0.09)` |
| `border.default` | `rgba(30,39,50,0.14)` | `rgba(212,229,239,0.16)` |
| `border.strong` | `rgba(30,39,50,0.28)` | `rgba(212,229,239,0.30)` |
| `border.inverse` | `rgba(255,255,255,0.24)` | `rgba(30,39,50,0.24)` |
| `text.primary` | `#1E2732` | `#F5F2E9` |
| `text.secondary` | `rgba(30,39,50,0.68)` | `rgba(212,229,239,0.72)` |
| `text.tertiary` | `rgba(30,39,50,0.55)` | `rgba(212,229,239,0.56)` |
| `text.disabled` | `rgba(30,39,50,0.34)` | `rgba(212,229,239,0.40)` |
| `text.inverse` | `#F5F2E9` | `#1E2732` |
| `text.link` | `#2E59A7` | `#5A83CC` |
| `primary.default` | `#2E59A7` | `#416CBA` |
| `primary.hover` | `#1A4188` | `#5A83CC` |
| `primary.active` | `#092964` | `#759BDC` |
| `primary.subtle` | `#D1E2FF` | `#092964` |
| `primary.border` | `#2E59A7` | `#416CBA` |
| `primary.text` | `#2E59A7` | `#5A83CC` |
| `primary.on` | `#FFFFFF` | `#FFFFFF` |
| `accent.default` | `#7D5284` | `#9C73A3` |
| `accent.subtle` | `#ECDBEE` | `#3E2043` |
| `accent.border` | `#7D5284` | `#9C73A3` |
| `accent.text` | `#7D5284` | `#9C73A3` |
| `accent.on` | `#FFFFFF` | `#0E1116` |
| `success.default` | `#2A6E3F` | `#3E7F50` |
| `success.subtle` | `#D2E8D6` | `#003917` |
| `success.border` | `#2A6E3F` | `#3E7F50` |
| `success.text` | `#2A6E3F` | `#599468` |
| `success.on` | `#FFFFFF` | `#FFFFFF` |
| `warning.default` | `#C67915` | `#C67915` |
| `warning.subtle` | `#FEDAB8` | `#442500` |
| `warning.border` | `#C67915` | `#C67915` |
| `warning.text` | `#9E5D00` | `#C67915` |
| `warning.on` | `#0E1116` | `#0E1116` |
| `danger.default` | `#8F1D22` | `#B14745` |
| `danger.subtle` | `#FFD6D2` | `#5C000A` |
| `danger.border` | `#8F1D22` | `#B14745` |
| `danger.text` | `#8F1D22` | `#C5635E` |
| `danger.on` | `#FFFFFF` | `#FFFFFF` |
| `info.default` | `#3D8E86` | `#3D8E86` |
| `info.subtle` | `#CBE9E4` | `#003632` |
| `info.border` | `#3D8E86` | `#3D8E86` |
| `info.text` | `#186760` | `#3D8E86` |
| `info.on` | `#0E1116` | `#0E1116` |
| `state.hover` | `rgba(30,39,50,0.04)` | `rgba(212,229,239,0.06)` |
| `state.pressed` | `rgba(30,39,50,0.08)` | `rgba(212,229,239,0.10)` |
| `state.selected` | `rgba(30,39,50,0.12)` | `rgba(212,229,239,0.14)` |
| `state.drag` | `rgba(30,39,50,0.16)` | `rgba(212,229,239,0.18)` |
| `state.disabled` | `rgba(30,39,50,0.05)` | `rgba(212,229,239,0.06)` |
| `overlay.scrim` | `rgba(20,23,28,0.45)` | `rgba(0,0,0,0.62)` |
| `overlay.scrimStrong` | `rgba(20,23,28,0.68)` | `rgba(0,0,0,0.78)` |
| `overlay.glass` | `rgba(255,255,255,0.72)` | `rgba(20,23,28,0.72)` |
| `focus.ring` | `#2E59A7` | `#416CBA` |
| `focus.offset` | `#FFFFFF` | `#14171C` |
| `skeleton.base` | `rgba(30,39,50,0.07)` | `rgba(212,229,239,0.09)` |
| `skeleton.sheen` | `rgba(30,39,50,0.03)` | `rgba(212,229,239,0.04)` |
| `chart.c1` | `#2E59A7` | `#6B8FD4` |
| `chart.c2` | `#3D8E86` | `#4FB3A6` |
| `chart.c3` | `#2A6E3F` | `#4E9C63` |
| `chart.c4` | `#A9862E` | `#D4B36A` |
| `chart.c5` | `#C67915` | `#DE9A45` |
| `chart.c6` | `#D12920` | `#E4665C` |
| `chart.c7` | `#B83570` | `#D9739E` |
| `chart.c8` | `#7D5284` | `#A98BC0` |
| `chart.c9` | `#B26D5D` | `#D69A86` |
| `chart.c10` | `#945635` | `#C08A6B` |
| `chart.grid` | `rgba(30,39,50,0.10)` | `rgba(212,229,239,0.12)` |
| `chart.axis` | `rgba(30,39,50,0.45)` | `rgba(212,229,239,0.45)` |
| `chart.label` | `rgba(30,39,50,0.68)` | `rgba(212,229,239,0.72)` |
| `chart.positive` | `#2A6E3F` | `#599468` |
| `chart.negative` | `#8F1D22` | `#C5635E` |
| `chart.neutral` | `rgba(30,39,50,0.45)` | `rgba(212,229,239,0.45)` |

> 表中的 `primary.*` / `text.link` / `focus.ring` 是**默认品牌（群青）**下的值，会随品牌变化。
> 上表是群青品牌时的展开；其他品牌见 `dist/reports/ramp-report.md` 的品牌表。

### 2.1 角色语义说明

| 角色组 | 用在哪 | 关键约束 |
|---|---|---|
| `bg.*` | 页面级底色。`canvas` 是最外层（窗口/画布），`base` 是内容区，`sunken` 是内凹区域（代码块、井） | 卡片（`surface.default`）必须比 `base` 更「近」——浅色下更白、深色下更亮 |
| `surface.*` | 元素级表面。`default` 卡片、`raised` 抬升浮层、`overlay` 弹层、`variant` 次级填充、`inverse` 反色区块 | `variant` 用于无描边的弱分隔块，不要再叠加 `border` |
| `border.*` | 分隔线与描边。`subtle` 表格行、`default` 卡片描边、`strong` 输入框、`inverse` 反色区 | 用 alpha 而非实色，才能自动适配深浅模式 |
| `text.*` | 文字。`primary` 正文、`secondary` 说明、`tertiary` 占位与辅助、`disabled` 禁用、`inverse` 反色、`link` 链接 | `tertiary` 有 3:1 门禁；`disabled` 不受对比度约束但必须还有其它非颜色线索（如禁用手型） |
| `primary.*` | 主操作与选中态。`default`/`hover`/`active` 三态填充、`subtle` 弱化底（选中行背景）、`border` 描边风格、`text` 文字色、`on` **填充之上的前景色** | `on` 可能是白也可能是墨（取决于品牌），必须引用而不要写 `#FFF` |
| `accent.*` | 强调与标记。徽标、计数、当前项、高亮 | 与 `primary` 不同色相，二者可并存 |
| `success/warning/danger/info.*` | 四个状态色，结构同 `primary` | 必须附图标或文案 |
| `state.*` | 覆盖在任意元素上的交互态层（`::hover`、按下、选中、拖拽中） | 用 `background-image: linear-gradient(var(--dq-state-hover), …)` 或伪元素叠加，不要直接改元素底色 |
| `overlay.*` | 模态遮罩（`scrim`）、强遮罩（`scrimStrong`）、毛玻璃（`glass`） | 毛玻璃必须同时给不含 `backdrop-filter` 的降级底色 |
| `focus.*` | 焦点环与其外圈偏移色 | `focus.offset` 必须等于所在表面色，否则焦点环会与背景粘连 |
| `skeleton.*` | 骨架屏底与流光 | ≤ 3:1 时才不像加载完成的内容 |
| `chart.*` | 数据可视化 | `c1…c10` 已避开红绿相邻；`grid`/`axis`/`label` 是图表专用灰阶 |

## 3. 尺度令牌（`scale.*`）

| 组 | 值 | 说明 |
|---|---|---|
| `space` | `0:0` `1:4` `2:8` `3:12` `4:16` `5:20` `6:24` `8:32` `10:40` `12:48` `16:64` `20:80` `24:96` | 4pt 基准。跳过的数字（7/9/11）为保留位，不要插入新值 |
| `radius` | `none:0` `xs:4` `sm:8` `md:12` `lg:16` `xl:20` `2xl:28` `pill:999` | 卡片用 `md/lg`，按钮用 `sm/md`，药丸用 `pill`，输入框用 `sm` |
| `borderWidth` | `none/hairline:1/thick:2/heavy:4` | 高 DPI 下 1px 需按设备像素比处理 |
| `opacity` | `disabled:0.38` `muted:0.6` `full:1` | |
| `zIndex` | `base:0` `raised:10` `sticky:100` `drawer:400` `modal:500` `popover:600` `toast:700` `tooltip:800` | 只允许从这些值里取，禁止随手写 9999 |
| `icon` | `xs:12` `sm:16` `md:20` `lg:24` `xl:32` `2xl:40` | 图标尺寸由令牌定，不要在组件里写 px |
| `touch` | `min:44` `comfortable:48` | 触控目标下限 |
| `container` | `form:480` `prose:640` `content:960` `wide:1280` `full:1600` | 内容最大宽度 |
| `breakpoint` | `compact:≤599` `medium:600–839` `expanded:840–1199` `large:≥1200` | 屏宽档，见 [`04-theming.md`](./04-theming.md#3-响应式与密度) |
| `elevation` | `0…5` | 五级阴影，见下 |
| `typography` | `family`（ui/serif/mono/number）、`weight`（400/500/600/700）、`letterSpacing`、`scale`（8 档字号） | |
| `motion` | `duration`（instant 80 / fast 140 / normal 220 / slow 320 / deliberate 420）、`easing`（standard / decelerate / accelerate / spring） | |

### 3.1 字号阶梯

| 档 | 字号 | 行高 | 字重 | 用途 |
|---|---|---|---|---|
| `caption` | 12 | 1.5 | 400 | 徽标、时间戳、辅助说明 |
| `bodySm` | 13 | 1.6 | 400 | 元信息、列表副标题 |
| `body` | 14 | 1.65 | 400 | 界面正文（默认） |
| `bodyLg` | 16 | 1.7 | 400 | 正文强调、移动端界面正文 |
| `titleSm` | 18 | 1.45 | 600 | 卡片标题、面板标题 |
| `title` | 22 | 1.35 | 600 | 页面标题 |
| `titleLg` | 28 | 1.25 | 700 | 桌面端页面大标题 |
| `display` | 38 | 1.15 | 700 | 空态标题、首屏 |

字体族（`family`）：

- `ui` — `Inter, "Noto Sans SC", "Source Han Sans SC", "PingFang SC", "Microsoft YaHei", system-ui, sans-serif`
- `serif` — `"Source Serif 4", "Noto Serif SC", "Source Han Serif SC", "Songti SC", SimSun, Georgia, serif`
- `mono` — `"JetBrains Mono", "SFMono-Regular", Consolas, monospace`
- `number` — 需表格数字对齐的场景用，配 `font-variant-numeric: tabular-nums`

### 3.2 阴影层级

```
elev 0: none
elev 1: 0 1px 2px rgba(20,23,28,.06),  0 1px 1px rgba(20,23,28,.04)
elev 2: 0 2px 6px rgba(20,23,28,.08),  0 1px 2px rgba(20,23,28,.04)
elev 3: 0 6px 16px rgba(20,23,28,.10), 0 2px 6px rgba(20,23,28,.05)
elev 4: 0 12px 28px rgba(20,23,28,.14), 0 4px 10px rgba(20,23,28,.06)
elev 5: 0 24px 48px rgba(20,23,28,.18), 0 8px 20px rgba(20,23,28,.08)
```

阴影的色相取墨色而非纯黑（`rgba(20,23,28,·)`），与中式纸质质感一致：暖背景上纯黑阴影会显得脏。
深色模式的层级**不靠阴影**，靠 `surface` 提亮 + 1px `border.subtle`。

## 4. 命名速查

```
tier1  cn.{family}.{step}       →  --dq-cn-qing-700       （仅内部）
tier2  sys.{role}.{variant}     →  --dq-text-primary
tier3  ext.{domain}.{name}      →  --dq-reading-bg
scale  —                        →  --dq-space-4 / --dq-radius-lg / --dq-dur-normal
brand  —                        →  --dq-primary-default（切 data-brand 后重解析）
```

跨端命名转换规则：

| 端 | `sys.text.primary` 的形态 |
|---|---|
| CSS | `--dq-text-primary` |
| SCSS | `dq-color("text", "primary")` 或 `$dq-light` map |
| TypeScript | `light.text.primary` / `brands.qing.light.primary.on` |
| Avalonia | `DqTextPrimary`（`SolidColorBrush`） |
| SwiftUI | `Dq.SemanticLight.textPrimary` |
| Compose | `Dq.SemanticLight.TextPrimary` |
| Flutter | `DqLight.textPrimary` |
| Android XML | `@color/dq_text_primary` / `@color/dq_text_primary_dark` |
| Tailwind | `text.primary`（映射到 CSS 变量） |
