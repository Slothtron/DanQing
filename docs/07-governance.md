# 07 · 治理

## 1. 真源与生成物

```
人手维护                          机器生成（改了会被覆盖）
─────────────────────────         ────────────────────────────────────────────────────
tokens/source.json       ──┐
data/chinese-colors.json ──┼──▶  danqing build  ──▶  themes/windows-terminal/*.json（8 套配色，**入库**）
                           │                          tokens/danqing.tokens.json（已解析快照，不入库）
                           │                          dist/reports/**.md（3 份报告，不入库）
                           │                          showcase/data.js（参考页数据，不入库）
                           └──▶  退出码 0 = 全部门禁通过
```

**唯一人手可改的文件是 `tokens/source.json`**（以及 `data/chinese-colors.json` 这份原始色库）。
`src/` 下的 Rust 生成器是代码，当然也可以改，但改它意味着改规则，需要走下面的变更流程。

`themes/windows-terminal/**` 是**入库**的：它是唯一对外交付的现成产物，clone 下来就该直接能用，
不必先装 Rust 工具链跑一次构建。其余产物（快照、报告、参考页数据）**一律不入版本库**
（见 `.gitignore`）。纪律是：

- clone / 拉取后跑一次 `danqing build` 重建那几份不入库的产物；终端配色本身已在仓库里
- 改了真源却忘了重跑 → 本地 `danqing verify` 会报出漂移并退出码 1，终端配色同样会被它抓到
- CI 上另以 `git diff --exit-code themes/` 守门：入库的产物必须与真源一致
- **永远不要**为了「先上线」手改产物——下次生成即被覆盖，且这种改动不会进入门禁

## 2. 变更流程

| 变更类型 | 需要动 | 必须做的事 |
|---|---|---|
| 调一个色值 | `tokens/source.json` | 跑生成器；查看 `ramp-report.md` 确认色阶与命名合理；确认门禁 0 失败 |
| 加一个色族 | `source.json` 的 `families` | 同上，并确认新族的锚点是真实传统色原值（可从 `data/chinese-colors.json` 反查） |
| 加一个品牌 | `source.json` 的 `brands` | 确认 `family` 不是保留族；跑生成器；确认冲突检测为空 |
| 加一个语义角色 | `source.json` 的 `semantic.light` **和** `semantic.dark` | 两套模式必须同时给出；加入 `run_gates` 的检查项 |
| 加一个扩展 | `source.json` 的 `extensions` +（若要下拉到产物）`src/emit/` 下对应模块 | 确认不引入扩展时核心仍完整可用 |
| 改终端某个槽位的色阶 | `source.json` 的 `terminal.palette.{mode}.{slot}` | 跑生成器；门禁会立刻告诉你新色阶与底色的对比是否还够 |
| 改终端 chrome 取哪个令牌 | `source.json` 的 `terminal.chrome` | 值写 `sys.<角色>[.<键>]`；跑生成器确认门禁 |
| 加一套终端模式 | `source.json` 的 `terminal.modes` / `palette` / `backgroundSideNeutrals` 三处 | 三处必须同时补齐，否则生成器会明确指出缺哪一个 |
| 改算法（色阶/回退/门禁） | `src/model.rs` / `src/gate.rs` | 需要在 PR 里给出**前后对比**：121 个原语的 diff 摘要 + 门禁项数变化 + 是否出现新的紧项 |
| 改尺度令牌 | `source.json` 的 `scales` | 尺度变更属于破坏性变更，需全端同步并升 major |

## 3. CI 接入

```yaml
# .github/workflows/design-tokens.yml
name: design-tokens
on: [push, pull_request]
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: 编译生成器
        run: cargo build --release
      - name: 生成令牌并执行门禁
        run: ./target/release/danqing build
      - name: 入库产物必须与真源一致
        run: git diff --exit-code themes/
      - name: 上传报告
        if: always()
        uses: actions/upload-artifact@v4
        with: { name: danqing-reports, path: dist/reports }
```

两条命令足以挡住 90% 的回归：

1. `danqing build` — 生成 + 门禁（含对比度与终端配色），失败退出码 1
2. `git diff --exit-code themes/` — 终端配色入库，因此「改了真源没重跑」在 CI 上是可见的 diff

`danqing verify` 留给本地：改了 `source.json` 却忘了重跑时它会报出产物漂移（退出码 1）。
CI 上不必跑它——`dist/` 不入版本库，新检出的工作区本来就没有这部分产物，跑它只会报一堆缺失。

## 4. 版本策略

采用语义化版本，但**版本号的判断标准是令牌的兼容性，不是代码的兼容性**：

| 变更 | 版本位 | 例子 |
|---|---|---|
| 新增色族 / 品牌 / 语义角色 / 扩展 | **minor** | 新增 `cang` 色族；新增 `tan` 品牌 |
| 修改某个原语色值（色阶微调） | **minor**（若影响语义则 major） | 群青锚点从 `#2E59A7` 换成 `#2A54A0` |
| 修改语义角色的值 | **major** | `primary.default` 从 700 级改为 600 级 |
| 删除或重命名语义角色 / 色族 / 品牌 | **major** | 删除 `surface.variant` |
| 修改尺度令牌值 | **major** | `space.4` 从 16 改为 14 |
| 门禁阈值收紧 | **major** | 三级文字从 3:1 提到 4.5:1 |
| 变更产物形态（增删交付的产物类型） | **major** | 不再生成平台样式产物，改为只交付终端配色 |
| 文档、报告、参考页 | **patch** | 补文档 |

当前 `3.0.0`。

## 5. 使用时的注意点

应用不该直接依赖色值，因此换品牌、换版本通常是安全的。三类仍需注意：

1. **品牌切换后 `on` 色可能变化**：如果代码里把主色上的文字写死成白色，换成檀色品牌后
   就是白字压赭褐，对比度不达标。**必须**引用 `sys.primary.on`。
2. **深色模式的填充可能变化**：暗色下品牌填充级可能外移（如青族 `-1` 级），
   如果代码里用了主色的「相邻级」做渐变，需要重新确认。
3. **半透明文字的真实观感**：`text.secondary` / `text.tertiary` 是带 alpha 的墨色，
   真实对比度取决于它合成到哪种背景上。要自定义更淡的文字，得自己复算那一组合。

## 6. 如何参与

新增一个传统色族的最小步骤：

```jsonc
// tokens/source.json → families
{
  "id": "cui", "name": "翠", "pinyin": "Cuì", "hue": "青绿",
  "anchorHex": "#4C8045",           // 必须来自 data/chinese-colors.json（此例为「翠微」）
  "anchorName": "翠微", "anchorPinyin": "Cuì Wēi",
  "intent": "品牌候选 · 主色：青绿山水的绿",
  "connotation": "……",              // 一句说明为什么是它
  "alternatives": ["庭芜绿", "竹青"]
}
```

```bash
danqing build                    # 生成 + 门禁
git diff --stat themes/          # 看影响面：应新增 11 个原语，8 套配色随之变化
```

评审要点：

- 锚点色是否**逐字**取自色库（不能自己调一个更顺眼的色值）
- 色相与既有 11 族是否区分得开（用 `ramp-report.md` 的 OKLab 数值判断，不靠眼睛）
- 是否与保留族冲突（若要用作品牌）
- 门禁是否 0 失败，且新色族的紧项是否有合理解释

## 7. 反模式（见过就会踩）

| 反模式 | 后果 | 正确做法 |
|---|---|---|
| 手改生成出来的文件（含终端配色） | 下次生成被覆盖，且不进门禁 | 改 `source.json` |
| 应用里直接引用原语 `cn.qing.700` | 换品牌失效 | 引用 `sys.primary.default` |
| 把品牌色用作页面背景大面积铺 | 中式审美重留白，且深色下刺眼 | 品牌色面积 < 10% |
| 复用状态保留族做品牌 | 语义失效 | 新增非保留族 |
| 只用颜色表示状态 | 色盲用户无法使用 | 附图标或文案 |
| 深色主题用反相生成 | 视觉质量低，层次混乱 | 手工定义深色语义值 |
| 在组件里写 px 字号 | 字号缩放失效 | 用 `scale.typography` |
| 为了「好看」调低文字透明度 | 跌破对比度门禁 | 调门禁允许范围内的值，或换字号/字重 |
