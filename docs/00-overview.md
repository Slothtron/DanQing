# 00 · 定位与心智模型

## 1. 这个系统是什么

丹青是一套**色彩与令牌设计系统**，供任意应用开发直接引用。它提供三样东西：

1. **121 个色彩原语** —— 11 个中国传统色族 × 11 级 OKLab 等距色阶，锚点级写回传统色原值，每一级都有可溯源的中文色名。
2. **160 个语义令牌** —— 80 个通用语义角色 × 浅色/深色两套，覆盖背景、表面、描边、文字、主色、五个状态色、交互态、浮层、焦点、骨架屏、数据可视化。
3. **66 组尺度令牌** —— 间距、圆角、描边、图标、触控、容器、阴影层级、字号阶梯、字重、字距、动效、断点。

以及把这些令牌变成**任何技术栈可直接消费的产物**的生成器（15 种输出）与**可执行的对比度门禁**（108 项）。

## 2. 这个系统不是什么

明确边界比功能列表更重要：

| 不是 | 说明 |
|---|---|
| 不是组件库 | 不提供 Button / Card / Dialog 的代码与结构。组件属于产品，颜色与尺度属于设计系统。 |
| 不是设计稿/视觉规范图 | 不规定某个按钮该有多圆、标题该多大。它给的是**可选值与语义角色**，取舍由产品做。 |
| 不含任何领域概念 | 核心层没有「阅读」「播放」「下单」「图表轴」这类词。领域概念一律走 `ext.*` 扩展。 |
| 不是品牌规范 | 它提供 4 个品牌预设与新增品牌的机制，但不替某家公司定品牌。 |
| 不是传统色数据库 | 数据库是 `data/chinese-colors.json`（386 色）。丹青从里面**挑出 11 个锚点**并推导成体系。 |

## 3. 心智模型：站在应用与调色板之间的那一层

```
   中国传统色（386 条，公开资料）
            │  挑选 11 个锚点
            ▼
   ┌──────────────────────────────────────────┐
   │  tier1  原语  cn.{family}.{step}         │  121 个
   │          "青族第 7 级 = 群青 #2E59A7"     │  有中文名，无语义
   └──────────────┬───────────────────────────┘
                  │  按语义角色装配
                  ▼
   ┌──────────────────────────────────────────┐
   │  tier2  语义  sys.{role}.{variant}       │  80 × 2 模式
   │          "正文色 / 主操作填充 / 危险色"     │  应用代码唯一允许直接用的层
   └──────────────┬───────────────────────────┘
                  │  按领域扩展
                  ▼
   ┌──────────────────────────────────────────┐
   │  tier3  扩展  ext.{domain}.*             │  按需
   │          "长文阅读的纸色与行高"            │  例如 ext.reading
   └──────────────────────────────────────────┘
```

关键在于**中间这一层**。没有它，设计师给一张色卡，开发就得自己在 121 个色里挑、自己给它们起名字、自己判断哪个能压得住白字——而这些判断在不同人、不同项目之间必然发散。

有了它，讨论从「这个按钮用哪个蓝」变成「这是主操作还是强调」——前者无法评审，后者可以。

## 4. 应用该怎么用它

### 4.1 认领语义，而不是认领颜色

```diff
- background: #2E59A7;              /* 硬编码原语 */
- background: var(--dq-cn-qing-700); /* 仍是硬编码，只是换了写法 */
+ background: var(--dq-primary-default); /* 认领了「主操作」这个语义 */
```

当产品决定换品牌主色时，第一、二种写法需要全局搜索替换，第三种写法只需改 `<html data-brand="zhu">`。

### 4.2 按目录接入

| 技术栈 | 引入方式 |
|---|---|
| Web / HTML | `<link>` 引入 `gen/css/danqing.css`，用 `data-theme` / `data-brand` 切换 |
| React / Vue / Svelte | 同上（CSS 自定义属性天然可用），或 `import` `gen/ts/danqing.ts` 取字面量 |
| Sass | `@use "gen/scss/danqing"` 取 map 与 `dq-color()` |
| Tailwind | `presets: [require("./gen/tailwind/danqing.preset.cjs")]` |
| DTCG 工具链 | `gen/json/danqing.tokens.json`，可被 Style Dictionary / Figma Tokens 直接读 |
| Avalonia | 合并 `gen/avalonia/Tokens.axaml` |
| SwiftUI | 把 `gen/swift/DesignTokens.swift` 加入 target；建议同时导出为 `Assets.xcassets` 颜色集 |
| Jetpack Compose | 复制 `gen/compose/DesignTokens.kt`，用它构造自定义 `ColorScheme` |
| Flutter | 复制 `gen/flutter/design_tokens.dart` |
| Android XML | `gen/android/colors.xml` + `dimens.xml` |

### 4.3 扩展自己的领域

产品的领域概念（阅读、绘图、播放…）不应该污染核心。做法见 [`04-theming.md`](./04-theming.md#4-扩展机制)：

1. 在 `tokens/source.json` 的 `extensions` 下新增一个键；
2. 在 Rust 生成器 `src/emit/css.rs` 的 `emit_css` 里加一段输出（或在产品侧单独维护一个 `product.tokens.json`）；
3. 扩展令牌以 `ext.*` 命名，且必须建立在 `sys.*` 之上。

## 5. 目录与产物

```
danqing/
├─ tokens/source.json          ← 唯一人手真源，改这里
├─ tokens/danqing.tokens.json  ← 已解析快照（生成）
├─ Cargo.toml                  ← Rust 工程清单
├─ src/                        ← 生成器 + 门禁（编译成单个二进制 `danqing`）
├─ data/chinese-colors.json    ← 386 条传统色原始数据
├─ gen/                        ← 15 种端产物（生成，勿手改）
│  ├─ css/danqing.css          gen/scss/_danqing.scss
│  ├─ ts/danqing.ts            gen/json/danqing.tokens.json  (DTCG)
│  ├─ tailwind/danqing.preset.cjs
│  ├─ avalonia/Tokens.axaml    gen/swift/DesignTokens.swift
│  ├─ compose/DesignTokens.kt  gen/flutter/design_tokens.dart
│  ├─ android/{colors,dimens}.xml
│  └─ reports/{ramp,contrast,tokens-summary}.md
├─ docs/                       ← 本目录
├─ showcase/index.html         ← 可视化展示
└─ reference/koodo/            ← 第三方参考，不进入系统
```

## 6. 下一步

- 想直接看效果 → 打开 [`../showcase/index.html`](../showcase/index.html)
- 想知道颜色怎么来的 → [`02-color-system.md`](./02-color-system.md)
- 想知道令牌怎么用 → [`03-tokens.md`](./03-tokens.md)
- 想知道怎么保证可读 → [`05-accessibility.md`](./05-accessibility.md)
- 想改这个系统 → [`07-governance.md`](./07-governance.md)
