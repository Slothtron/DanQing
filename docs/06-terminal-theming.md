# 06 · 终端主题

丹青对外只有一份现成产物：**Windows Terminal 配色**，写在
`themes/windows-terminal/danqing.schemes.json`（其余生成物都是诊断材料，不入库）。
本文是这份产物的契约：它由哪些令牌解析而来、16 个 ANSI 槽位怎么选色、门禁怎么查。

## 1. 产物形状

文件是可直接粘进 `settings.json` 的一个对象，只有 `schemes` 一个键：

```json
{
  "schemes": [
    {
      "name": "丹青 · 素 · 群青",
      "background": "#E4E1D8",
      "foreground": "#1E2732",
      "cursorColor": "#2E59A7",
      "selectionBackground": "#D1E2FF",
      "black": "#1E2732", "red": "#AB0003", "green": "#2A6E3F", "yellow": "#814B00",
      "blue": "#2E59A7", "purple": "#714778", "cyan": "#186760", "white": "#B4B2AA",
      "brightBlack": "#67707A", "brightRed": "#D12920", "brightGreen": "#3E7F50",
      "brightYellow": "#9E5D00", "brightBlue": "#416CBA", "brightPurple": "#7D5284",
      "brightCyan": "#2D7D76", "brightWhite": "#F5F2E9"
    }
  ]
}
```

字段口径与 Windows Terminal 一致：`name` 必需，其余取 `#rrggbb`；
`cursorColor` 与 `selectionBackground` 在格式里可选，这里一律写出。
字段顺序为 `name`、四项 chrome、8 个基色、8 个亮色。

## 2. 八个方案

四个品牌预设 × 明暗两套 = 8 套。名字统一带 `丹青` 前缀，避免与用户
`settings.json` 里已有的配色撞名。

| 方案名 | 模式 | 底色 | 正文 | 光标 | 选区 |
|---|---|---|---|---|---|
| 丹青 · 素 · 群青 | 素（纸） | `#E4E1D8` | `#1E2732` | `#2E59A7` | `#D1E2FF` |
| 丹青 · 墨 · 群青 | 墨（夜） | `#101318` | `#F5F2E9` | `#416CBA` | `#092964` |
| 丹青 · 素 · 银朱 | 素（纸） | `#E4E1D8` | `#1E2732` | `#D12920` | `#FFD6D0` |
| 丹青 · 墨 · 银朱 | 墨（夜） | `#101318` | `#F5F2E9` | `#D12920` | `#5D0001` |
| 丹青 · 素 · 青黛 | 素（纸） | `#E4E1D8` | `#1E2732` | `#45465E` | `#E0E0EA` |
| 丹青 · 墨 · 青黛 | 墨（夜） | `#101318` | `#F5F2E9` | `#6B6C84` | `#2B2C40` |
| 丹青 · 素 · 檀色 | 素（纸） | `#E4E1D8` | `#1E2732` | `#B26D5D` | `#F9D9D1` |
| 丹青 · 墨 · 檀色 | 墨（夜） | `#101318` | `#F5F2E9` | `#B26D5D` | `#492017` |

模式名取 [`04-theming.md`](./04-theming.md) 里的材料名：浅色是「素」（纸），深色是「墨」（夜）。

## 3. 16 个 ANSI 槽位怎么选色

**一个槽位对一族，基色与亮色取同族两档。** 槽位与色族的对应是固定的，不跨族混搭，
所以色相盘在任何品牌下都保持稳定 —— 终端里 `red` 始终是朱、`blue` 始终是青。

| 槽位 | 色族 | 素（浅色） | 墨（深色） |
|---|---|---|---|
| `black` / `brightBlack` | 墨 `mo` | 900 / 600 | 950 / 500 |
| `red` / `brightRed` | 朱 `zhu` | 700 / 600 | 400 / 200 |
| `green` / `brightGreen` | 绿 `lv` | 700 / 600 | 400 / 200 |
| `yellow` / `brightYellow` | 缃 `xiang` | 700 / 600 | 400 / 200 |
| `blue` / `brightBlue` | 青 `qing` | 700 / 600 | 400 / 200 |
| `purple` / `brightPurple` | 紫 `zi` | 700 / 600 | 400 / 200 |
| `cyan` / `brightCyan` | 苍 `cang` | 700 / 600 | 400 / 200 |
| `white` / `brightWhite` | 素 `su` | 300 / 50 | 200 / 50 |

**两套材料的取阶方向相反，但这不是反相。** 素模式的彩色槽取 700/600 —— 深墨一档才压得住
纸底；墨模式取 400/200 —— 提亮一档才浮得出墨底。这与语义层的做法一致：深色不是浅色的
取反，是换了一套材料。

色调 `jiang 绛`、`dai 黛`、`tan 檀` 不进入 ANSI 映射：绛与朱同色相、黛与紫灰相近，
再塞进 16 槽只会让红与紫分辨不开；檀是暖褐，与黄槽抢位。品牌若要更鲜明地体现色相，
扩展位是 `terminal.brands.{id}`，本轮不启用。

## 4. 品牌体现在光标与选区

`background`、`foreground`、`cursorColor`、`selectionBackground` 四项 chrome 按
**品牌 × 模式**从语义层取：

| 字段 | 令牌 | 为什么是它 |
|---|---|---|
| `background` | `sys.bg.canvas` | 终端整屏就是页面画布 |
| `foreground` | `sys.text.primary` | 正文色 |
| `cursorColor` | `sys.primary.default` | 品牌主色最自然的落点，且必须与底色拉得开 |
| `selectionBackground` | `sys.primary.subtle` | 选区是浅底，正文压在其上仍须 ≥4.5:1 |

注意 `sys.bg.canvas` 与 `sys.text.primary` 本身**与品牌无关**，因此同一模式下四个品牌的
底色与正文色相同，品牌差异只落在光标与选区上。这是有意的：终端底色要能长时间盯，
不适合按品牌染上一层重色；而光标与选区正好是「一眼看到这是哪套主题」的位置。

## 5. 门禁口径

终端配色与其余令牌一起过 `danqing build` 的对比度门禁，分组为「终端」，每套方案 17~18 项：

| 检查项 | 阈值 | 依据 |
|---|---|---|
| 正文 / 底色 | ≥ 4.5:1 | WCAG 2.1 AA · 1.4.3 |
| 光标 / 底色 | ≥ 3.0:1 | WCAG 2.1 AA · 1.4.11 |
| 正文 / 选区 | ≥ 4.5:1 | 选区里的文字同样是正文 |
| 每个 ANSI 槽位 / 底色 | ≥ 3.0:1 | 16 色都要能当正文用 |

**豁免**：`terminal.backgroundSideNeutrals` 声明了与底色同侧、用来表达底色本身的槽位 ——
素模式是 `white`、`brightWhite`，墨模式是 `black`。要求它们与底色拉开 3:1 是自相矛盾的
要求：素模式里 `white` 就是纸、墨模式里 `black` 就是墨。其余槽位一律查。

素模式 17 项（16 槽中 14 项 + 3 项 chrome）、墨模式 18 项（15 + 3），四品牌合计 140 项。

## 6. 怎么用

```jsonc
// settings.json
{
  "schemes": [ /* 粘贴 themes/windows-terminal/danqing.schemes.json 的 "schemes" 数组 */ ],
  "profiles": { "defaults": { "colorScheme": "丹青 · 墨 · 群青" } },
  "theme": { "dark": "丹青 · 墨 · 群青", "light": "丹青 · 素 · 群青" }
}
```

`theme` 那个键让 Windows 切换明暗时自动跟着换配色。设置界面里也可以直接打开 JSON 文件粘贴。

## 7. 改这个导出

改 `tokens/source.json` 的 `terminal` 段，然后重跑 `danqing build`：

- 换某槽的色阶 → 改 `terminal.palette.{mode}.{slot}` 的 `@色族.色阶` 引用，门禁会立刻告诉你新色阶是否还够对比；
- 换 chrome 取哪个令牌 → 改 `terminal.chrome`，值写 `sys.<角色>[.<键>]`；
- 加一套模式或品牌 → 品牌加进 `brands` 数组后自动生成；模式需要在 `terminal.modes`、`palette`、`backgroundSideNeutrals` 三处同时补齐。

**不要手改产物 JSON**：下次构建会覆盖，且 `danqing verify` 会报漂移。
