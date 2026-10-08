# 07 · 治理

## 1. 真源与生成物

```
人手维护                          机器生成（改了会被覆盖）
─────────────────────────         ────────────────────────────────────
tokens/source.json       ──┐
data/chinese-colors.json ──┼──▶  danqing build  ──▶  tokens/danqing.tokens.json（已解析快照）
                           │                          gen/**（15 种端产物）
                           │                          gen/reports/**.md（3 份报告）
                           └──▶  退出码 0 = 全部门禁通过
```

**唯一人手可改的文件是 `tokens/source.json`**（以及 `data/chinese-colors.json` 这份原始色库）。
`src/` 下的 Rust 生成器是代码，当然也可以改，但改它意味着改规则，需要走下面的变更流程。

`gen/` 与 `tokens/danqing.tokens.json` 应当视为构建产物。建议做法：

- 提交到仓库（便于各端直接取用、也在 PR 里可见变更）
- 在 `gen/README` 或文件头注明生成物
- **永远不要**为了「先上线」手改 `gen/`——下次生成即被覆盖，且这种改动不会进入门禁

## 2. 变更流程

| 变更类型 | 需要动 | 必须做的事 |
|---|---|---|
| 调一个色值 | `tokens/source.json` | 跑生成器；查看 `ramp-report.md` 确认色阶与命名合理；确认门禁 0 失败 |
| 加一个色族 | `source.json` 的 `families` | 同上，并确认新族的锚点是真实传统色原值（可从 `data/chinese-colors.json` 反查） |
| 加一个品牌 | `source.json` 的 `brands` | 确认 `family` 不是保留族；跑生成器；确认冲突检测为空 |
| 加一个语义角色 | `source.json` 的 `semantic.light` **和** `semantic.dark` | 两套模式必须同时给出；加入 `run_gates` 的检查项 |
| 加一个扩展 | `source.json` 的 `extensions` + `src/emit/` 下对应端的 emit 段 | 确认不引入扩展时核心仍完整可用 |
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
      - name: 上传报告
        if: always()
        uses: actions/upload-artifact@v4
        with: { name: danqing-reports, path: gen/reports }
```

一条命令足以挡住 90% 的回归：

1. `danqing build` — 生成 + 门禁（含对比度），失败退出码 1

`danqing verify` 留给本地：改了 `source.json` 却忘了重跑时它会报出产物漂移（退出码 1）。
CI 上不跑它——`gen/` 不入版本库，新检出的工作区本来就没有产物，跑它只会全量报缺失。

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
| 文档、报告、展示页 | **patch** | 补文档 |

当前 `2.3.0`。`2.0.0 → 2.1.0` 新增了 `cang`/`zi`/`tan` 三个色族与保留族机制；
`2.1.0 → 2.2.0` 引入了双极端 `on` 色择优、图表色自动修正、DTCG 输出。
`2.2.0 → 2.3.0` 生成器由 Python 重写为 Rust（`danqing` 单二进制），运行时依赖降到零；
产物统一改用 LF 换行，并修正了 `prefers-reduced-motion` 下 `--dq-dur-normal` 未生效的问题。

## 5. 给应用的升级指引

应用不该直接依赖色值，因此升级通常是安全的。但有三类需要注意：

1. **品牌切换后 `on` 色可能变化**：如果应用里有 `color: #fff` 压在主色上，
   换成檀色品牌后就是白字压赭褐，对比度不达标。**必须**引用 `--dq-primary-on`。
2. **深色模式的填充可能变化**：暗色下品牌填充级可能外移（如青族 `-1` 级），
   如果应用里用了主色的「相邻级」做渐变，需要重新确认。
3. **`text.tertiary` 的 alpha 曾从 0.48 提到 0.55**（为过 3:1 门禁）。
   若应用里有自定义的「更淡文字」，需要自己重新验证。

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
git diff --stat gen/             # 看影响面：应新增 11 个原语 + 1 组 scale 不变
```

评审要点：

- 锚点色是否**逐字**取自色库（不能自己调一个更顺眼的色值）
- 色相与既有 11 族是否区分得开（用 `ramp-report.md` 的 OKLab 数值判断，不靠眼睛）
- 是否与保留族冲突（若要用作品牌）
- 门禁是否 0 失败，且新色族的紧项是否有合理解释

## 7. 反模式（见过就会踩）

| 反模式 | 后果 | 正确做法 |
|---|---|---|
| 在 `gen/` 里手改色值 | 下次生成被覆盖，且不进门禁 | 改 `source.json` |
| 应用里写 `--dq-cn-qing-700` | 换品牌失效 | 写 `--dq-primary-default` |
| 把品牌色用作页面背景大面积铺 | 中式审美重留白，且深色下刺眼 | 品牌色面积 < 10% |
| 复用状态保留族做品牌 | 语义失效 | 新增非保留族 |
| 只用颜色表示状态 | 色盲用户无法使用 | 附图标或文案 |
| 深色主题用反相生成 | 视觉质量低，层次混乱 | 手工定义深色语义值 |
| 在组件里写 px 字号 | 字号缩放失效 | 用 `scale.typography` |
| 为了「好看」调低文字透明度 | 跌破对比度门禁 | 调门禁允许范围内的值，或换字号/字重 |
