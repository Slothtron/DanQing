# 04 · 主题、品牌与扩展

## 1. 品牌：只换一个属性

品牌在丹青里是一个**受约束的开关**：它只重新解析 `primary.*`、`focus.ring`、`text.link` 三类令牌，
其余 70 余个语义令牌与全部 121 个原语保持不变。

内置四个品牌预设，全部取自非保留色族（无状态色冲突）：

| `data-brand` | 品牌 | 色族 | 浅色填充 / 其上文字 | 深色填充 / 其上文字 | 适合 |
|---|---|---|---|---|---|
| `qing`（默认） | 群青 | 青 | `#2E59A7` / 白 | `#416CBA` / 白 | 工具、效率、金融、企业后台 |
| `zhu` | 银朱 | 朱 | `#D12920` / 白 | `#D12920` / 白 | 内容、文化、社区、生活方式 |
| `dai` | 青黛 | 黛 | `#45465E` / 白 | `#6B6C84` / 白 | 阅读、写作、设计工具、专业软件 |
| `tan` | 檀色 | 檀 | `#B26D5D` / **墨** | `#B26D5D` / **墨** | 文创、出版、手作、在地生活 |

```html
<html data-theme="light" data-brand="zhu">
```

```css
/* 只改这几个变量也能临时切品牌（不推荐，优先用 data-brand） */
:root { --dq-primary-default: var(--dq-cn-tan-500); --dq-primary-on: #0E1116; }
```

### 1.1 新增品牌

在 `tokens/source.json` 的 `brands` 数组里追加一项：

```jsonc
{
  "id": "cui", "name": "翠微", "pinyin": "Cuì Wēi", "family": "cui",
  "desc": "……", "audience": "……"
}
```

`family` 必须指向一个已存在的**非保留族**（保留族见 `reservedFamilies.map`）。
生成器会自动：

1. 跳过锚点之外的推导，直接复用该族已有的色阶；
2. 用「锚点保真 + 可读性回退」规则推导浅/深两套 `fill/on/text/hover/active/subtle/border`；
3. 把新品牌加入全部门禁检查；
4. 检测是否与状态色同族并写入报告。

## 2. 浅色与深色：两套材料，不是反相

| | 浅色（素 / 纸） | 深色（墨 / 夜） |
|---|---|---|
| 感知隐喻 | 在纸上写字 | 夜里看屏幕 |
| 层次实现 | 靠**白度**与阴影 | 靠**提亮**与 1px 描边 |
| 页面底 | 凝脂 `#F5F2E9` | `#14171C` |
| 卡片 | `#FFFFFF` | 瑾瑜 `#1E2732` |
| 正文 | 瑾瑜 `#1E2732` | 素阶 50 `#F5F2E9` |
| 描边 | 墨 8–28% | 月白 9–30% |
| 主色填充 | 偏深（压白字） | 偏浅（压墨字） |

**不要**用 `filter: invert()` 或 `@media (prefers-color-scheme: dark)` 里翻转浅色值来实现深色主题。
深色下 `text.primary` 不是「浅色的反相」，而是「素材本身换了」——素与墨是两个色族。

### 2.1 主题与品牌是两个正交的开关

主题（素 / 墨）与品牌互不替代，各自独立：

```html
<html data-theme="dark" data-brand="dai">
```

约定：`data-theme` 只取 `light` / `dark`，`data-brand` 取品牌 id。不要用 `class="dark"` 这类
自定义开关，也不要让 `prefers-color-scheme` 直接参与样式计算——系统级偏好应该由应用读取后
写属性，否则用户手动选的「浅色」会被系统主题覆盖。

Web 上自己铺样式层时，四段覆盖的**书写顺序**决定优先级能否成立：

```css
:root { /* 浅色基座 + 默认品牌 */ }
[data-brand="zhu"] { /* 浅色品牌覆盖 · 特异度 0,1,0 */ }
[data-theme="dark"] { /* 深色基座 · 0,1,0，必须排在浅色品牌块之后 */ }
[data-theme="dark"][data-brand="zhu"] { /* 深色品牌覆盖 · 0,2,0 */ }
```

两个点必须成立：深色基座排在浅色品牌块之后，否则 `data-theme="dark"` 下的默认品牌盖不掉
浅色品牌；深色品牌块用复合选择器，0,2,0 的特异度才能一次性压过前面两块。另外只把浅色
写进 `:root`——深浅两套令牌都塞进 `:root`，浅色默认值会被深色那套覆盖掉。

## 3. 响应式与密度

### 3.1 四档屏宽（逻辑像素 / dp / pt）

| 档 | 屏宽 | 导航形态 | 列数 |
|---|---|---|---|
| `compact` | ≤ 599 | 底部标签栏（3–5 项，高 56–64） | 1 |
| `medium` | 600–839 | 图标导航轨（宽 80） | 1 |
| `expanded` | 840–1199 | 展开式侧边导航（宽 232） | 2 |
| `large` | ≥ 1200 | 侧边导航 + 二级栏 | 3 |

断点只描述**导航与列数**，不描述具体组件。组件不应该有「移动版 / 桌面版」两套皮肤，
而应该在断点间连续适应（用 `clamp()` / `minmax()` / 弹性尺寸）。

```css
.grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(160px, 1fr)); gap: var(--dq-space-4); }
.content { width: min(100% - var(--dq-space-8), var(--dq-container-content)); margin-inline: auto; }
```

### 3.2 密度

丹青不提供独立的密度令牌组，而是建议产品在此之上做一层**规模乘数**（`compact 0.875` / `comfortable 1` / `spacious 1.125`），
只作用于 `space` 与行高，不影响 `radius`、字号与触控尺寸（触控尺寸在任何密度下都不得低于 44）。

## 4. 扩展机制

核心层不含任何领域概念。领域需求通过 `ext.*` 扩展实现，规则有三条：

1. 扩展令牌**必须**建立在 `sys.*` 之上（颜色可引用语义色，也可新增原语但不得替换语义）；
2. 扩展必须**可选**——不引入扩展时，核心系统完整可用；
3. 扩展的令牌 id 形如 `ext.{domain}.{name}`，CSS 变量为 `--dq-{domain}-{name}`。

### 4.1 内置示例：`ext.reading`（长文阅读）

长文阅读需要「纸色」「正文字号」「行高」「行宽」「段首缩进」这五类参数，它们在任何通用界面里都是噪音，
因此被放进扩展：

```jsonc
"extensions": {
  "reading": {
    "papers": [
      { "id": "paper", "name": "素笺", "bg": "#F5F2E9", "text": "#31322C", "sub": "#6B6A5E" },
      { "id": "white", "name": "缟",   "bg": "#EFEFEC", "text": "#1E2732", "sub": "#5C6470" },
      { "id": "green", "name": "天缥", "bg": "#D5EBE1", "text": "#13393E", "sub": "#3F5F5D" },
      { "id": "night", "name": "夜",   "bg": "#1E2732", "text": "#D4E5EF", "sub": "#93A6B4" }
    ],
    "fontSize": { "min": 14, "max": 36, "default": 19, "step": 1 },
    "lineHeight": { "tight": 1.5, "normal": 1.75, "loose": 2.0 },
    "measure":    { "narrow": 560, "default": 640, "wide": 720 },
    "indent":     { "none": 0, "half": 0.5, "full": 1.0 },
    "letterSpacing": { "tight": "0", "normal": "0.01em", "loose": "0.03em" }
  }
}
```

应用侧消费（`--dq-*` 是应用自己那套变量的命名空间，见 [`01-principles.md`](./01-principles.md#22-令牌-id机器的语言)）：

```css
.reader { background: var(--dq-reading-bg); color: var(--dq-reading-text);
          font-size: var(--dq-reading-size-default); line-height: var(--dq-reading-line-height);
          max-width: var(--dq-reading-measure-default); }
```
```html
<article class="reader" data-reading-paper="night" style="--dq-reading-line-height: 1.75">
```

四套纸色的正文对比度（`dist/reports/contrast-report.md` 的「扩展」分组）：

| 纸色 | 底 / 正文 | 正文对比度 | 次要文字 | 次要对比度 |
|---|---|---|---|---|
| 素笺（宣纸暖白） | `#F5F2E9` / `#31322C` | 11.56:1 | `#6B6A5E` | 4.87:1 |
| 缟（纯白） | `#EFEFEC` / `#1E2732` | 13.10:1 | `#5C6470` | 5.19:1 |
| 天缥（护眼青） | `#D5EBE1` / `#13393E` | 9.99:1 | `#3F5F5D` | 5.59:1 |
| 夜（墨夜） | `#1E2732` / `#D4E5EF` | 11.68:1 | `#93A6B4` | 6.00:1 |

> 四套全部达到 AAA（≥7:1）。注意纸色是**独立于 `data-theme` 的第三维**：
> 一个深色主题的应用里，用户仍可能选择「素笺」这种浅纸来读长文——所以纸色不复用 `bg.*`。
> 这类「领域正交维度」正是应该走扩展而不是塞进核心的典型信号。

### 4.2 如何加自己的扩展

1. 在 `tokens/source.json` → `extensions` 下加一个键（如 `player`）；
2. 不需要下拉到产物就到此为止——扩展令牌与核心令牌一起进快照，应用侧照样能读到；
   确实要产出某个平台的现成文件时，在 `src/emit/` 下加一个模块并接进 `src/main.rs` 的输出表；
3. 如果需要新原语色，加到 `families`，但**不要**把它变成保留族或品牌族；
4. 跑 `danqing build`，门禁会自动覆盖新增部分。

## 5. 主题设计清单

新增一个主题（如「高对比度」「赛博」）时逐项确认：

- [ ] `bg.canvas` / `bg.base` / `surface.default` / `surface.raised` 四层层次是否单调（越「近」越亮或越白）
- [ ] `text.primary` 在 `bg.base` 与 `surface.default` 上是否都 ≥ 4.5:1
- [ ] `text.secondary` 是否 ≥ 4.5:1（注意 alpha 合成后）
- [ ] `text.tertiary` 是否 ≥ 3:1
- [ ] 五个状态色在 `bg.base` 上是否 ≥ 4.5:1（文字用法）与 ≥ 3:1（填充边界）
- [ ] 每个状态色填充上的 `on` 色是否正确（不要假定是白色）
- [ ] `focus.offset` 是否等于所在表面色
- [ ] `chart.c1…c10` 是否都 ≥ 3:1
- [ ] `prefers-reduced-motion` 下动效是否正确降级
- [ ] 跑一次 `danqing build`，确认退出码为 0
