# Koodo Reader 设计规格提炼

来源：`koodo-reader/koodo-reader`（dev 分支，`git clone --depth 1`），文件路径均为仓库内相对路径。
用途：作为「悦读」本地 H5 阅读器 UI 重构的设计依据。

---

## 1. 色彩令牌系统（核心发现）

Koodo 不维护庞大的色板，而是**以单一中性色 `rgba(75,75,75,1)` 为基准，用 alpha 阶梯派生整套 UI 色**。

依据 `src/utils/reader/themeUtil.ts:108-119`（`generateThemeCSS`）：

| 用途 | 生成规则 | 选择器示例 |
|---|---|---|
| 实心元件（按钮/开关/侧栏激活胶囊） | `rgba(r,g,b,1)` | `.side-menu-selector-container`、`.single-control-switch`、`.previous-chapter-single-container` |
| 输入框底色 | `rgba(r,g,b,0.1)` | `.header-search-box`、`#jumpPage`、`.pin-key` |
| 次要文字 / 占位符 | `color: rgba(r,g,b,0.8)` | `.header-search-box`、`.header-search-text` |
| 主要文字 | `color: rgba(r,g,b,1)` | `.single-control-container`、`.book-list-view` |
| hover 底色 | `rgba(r,g,b,0.035)` | `.side-menu-hover-container`、`.setting-icon-container:hover` |
| 分隔线 | `border-bottom: 1px solid rgba(r,g,b,0.1)` | `.book-content-name`、`.nav-search-list-item` |

配套观察：
- 页面底色 `#f5f5f5`；遮罩 `rgba(0,0,0,.5~.55)`（`src/**/*.css` 高频色统计）
- 图标 hover 底色常量 `rgba(75,75,75,.3)` / `rgba(128,128,128,.15)`

### 主题色（11 色）
`src/constants/themeList.tsx`：

| id | 名称 | 值 |
|---|---|---|
| 0 | default | 中性（#4B4B4B 系） |
| 1 | Blue | `#0179CA` |
| 2 | Green | `#008F91` |
| 3 | Red | `#F16464` |
| 4 | Purple | `#6867D1` |
| 5 | **Orange** | `#F97316` |
| 6 | Pink | `#EC4899` |
| 7 | Yellow | `#EAB308` |
| 8 | Violet | `#8B5CF6` |
| 9 | Sky | `#0EA5E9` |
| 10 | Slate | `#64748B` |

### 阅读区 4 组配色（背景 / 文字）
`src/constants/themeList.tsx:1-12`：

| 预设 | 背景 | 文字 |
|---|---|---|
| 白 | `rgba(255,255,255,1)` | `rgba(0,0,0,1)` |
| 深色 | `rgba(44,47,49,1)` | `rgba(255,255,255,1)` |
| 羊皮纸 | `rgba(233,216,188,1)` | `rgba(89,68,41,1)` |
| 护眼绿 | `rgba(197,231,207,1)` | `rgba(54,80,62,1)` |

---

## 2. 桌面端布局度量

| 元素 | 度量 | 来源 |
|---|---|---|
| 侧边栏宽 | `190px` | `containers/sidebar/sidebar.css` |
| 内容区左边距 | `margin-left: 190px` | `containers/header/header.css` |
| Logo | 宽 `125px`，`top:15px; left:70px` | `sidebar.css` |
| 侧栏项 | 高 `39px`、圆角 `25px` 胶囊、字号 `15px`、`font-weight:500`、上间距 `2px` | `sidebar.css` |
| 侧栏图标 | `font-size:22px`，容器 `30px`，`margin:9px 12px 7px 18px` | `sidebar.css` |
| 书架计数徽标 | `12px`，`opacity:.65` | `sidebar.css` |
| 顶栏高 | `80px` | `header.css` |
| 搜索框容器 | `top:23px; margin-left:40px; width:220px` | `header.css` |
| 图标按钮 | `50×50`，图标 `25px`，hover 变圆形底 | `header.css` |
| 导入按钮 | `138×42`，圆角 `25px`，`font-weight:500` | `header.css` |
| 响应式断点 | `@media (max-width:950px)` / `(max-width:1250px)` | `header.css` |
| 书卡（列表视图） | `min-width:330px; height:186px`，圆角 `10px`，外框 `15px transparent` 作间距 | `lists/cardList/cardList.css` |
| 封面条目 | 封面 `120×170`，圆角 `3px`，外距 `5px 15px 15px`；标题 `15px/500` 两行截断；作者 `13px` `opacity:.8` | `components/bookCoverItem/bookCoverItem.css` |
| 滚动条 | 宽 `5px`，thumb 圆角 `0.5rem` | `cardList.css` |

## 3. 阅读器布局度量

| 元素 | 度量 | 来源 |
|---|---|---|
| 正文页边距 | `left/right:20px; top/bottom:40px` | `containers/viewer/index.css`（`.html-viewer-page`） |
| 页眉/页脚（书名/章节名） | 高 `25px`、字号 `16px`、**`opacity:.3`**、居中；`top:13px` / `bottom:14px` | `containers/pageWidget/pageWidget.css` |
| 底部进度面板 | 宽 `450px`，`left: calc(50% - 225px)`，高 `100px` | `panels/progressPanel/progressPanel.css` |
| 上一章/下一章按钮 | `25×25` 圆形，旋转 `±90°` | 同上 |
| 滑块轨道 | `border-bottom: 2px solid rgba(112,112,112,1)`，高 0 | 同上 |
| 滑块拇指 | `20×20` 圆形，白底 + `2px solid rgba(112,112,112,1)` 边框，`bottom:9px` | 同上 |
| 顶部操作面板 | 宽 `450px` 居中，高 `90px`，按钮 `125×37`、`font-weight:500` | `panels/operationPanel/operationPanel.css` |
| 右侧设置面板 | 宽 `299px` 全高，`animation: fade-right .2s`；标题栏高 `38px`，`margin:4px 22px` | `panels/settingPanel/settingPanel.css` |
| 左侧目录面板 | 宽 `299px` 全高，`fade-left .2s`；头部 `173px`（封面 `91×118`）；tab 行 `line-height:27px`、宽 `23%` 居中 | `panels/navigationPanel/navigationPanel.css` |
| 目录当前项 | 左侧 `3px` 主题色描边 + 底色 hover 变量 | 同上（`.nav-search-list-item-active`） |
| 设置行 | `width: calc(100% - 44px); margin: 0 22px`；标题 `15px/500`、`line-height:27px` | `components/readerSettings/sliderList/sliderList.css` |
| 开关 | `40×20`，圆角 `10px`，旋钮 `16px`、`margin:2px` | `panels/settingPanel/settingPanel.css` |
| 主题色圆点 | `35×35`，圆角 `50%` | `components/readerSettings/themeList/themeList.css` |
| 字号范围 | `TEXT_SIZE_MIN=12` / `TEXT_SIZE_MAX=72` | `src/utils/common.ts` |
| 动效 | `fade-left / fade-right / fade-up / fade-down`，`.1~.2s ease-in-out`；hover 进场 `slidein`（`scale .2→1`，`0.1s`） | 各 CSS |

## 4. 图标体系
`src/assets/styles/global.css`：Koodo 使用 `icomoon` 图标字体（`icon-home-line`、`icon-search`、`icon-trash-line`、`icon-night`、`icon-day`、`icon-setting`、`icon-list`、`icon-grid` 等 60+ 字形）。
本项目不引入字体文件，改用**内联 SVG 描边图标**（`stroke-width:1.8`、`stroke-linecap:round`）保持同一视觉密度。

## 5. 落地映射（本项目采用）
| 设计意图 | 本项目实现 |
|---|---|
| 中性基准色 | `--ink-rgb: 75,75,75`，alpha 阶梯派生 text/hover/field/line |
| 主题色 | 采用 Orange `#F97316`（Koodo 主题色 5 号）；文字场景用 `#C2410C` 保证 AA |
| 阅读区 4 配色 | 直接采用上表 4 组 RGB 值（替换原有自定义值） |
| 侧栏 190px / 项高 39px / 胶囊 25px | 桌面端侧边导航 |
| 950px 断点 | 侧栏折叠为底部标签栏 |
| 页眉页脚 opacity .3 | 阅读页常显运行页眉（章节名）+ 运行页脚（进度） |
| 滑块轨道 2px + 拇指 20px | `input[type=range]` 自定义样式 |
| 设置行 `100% - 44px` / 标题 15px/500 | 阅读设置面板 |
| 目录面板 299px + 左描边当前项 | 目录抽屉 |

### 已知偏差（有意为之）
1. **阅读页限宽**：Koodo 桌面端正文铺满（iframe 分栏），本项目 CSS 多列分页依赖元素宽度做位移计算，故将阅读视口限宽 `780px` 居中，避免超宽屏单行过长。
2. **滑块填充色**：Koodo 轨道为纯灰 `2px` 描线；本项目额外叠加 `--accent` 渐变填充以指示进度（功能优先）。
3. **对比度修正**：Koodo 次要文字 `alpha .8` 在浅底约 3.4:1；本项目提到 `.68`（约 4.6:1）以满足 WCAG AA。
