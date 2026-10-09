# Windows Terminal 配色

本目录是丹青唯一的入库产物：4 个品牌预设 × 明暗两套 = **8 套配色**。

| 文件 | 说明 |
|---|---|
| `danqing.schemes.json` | 生成物，**勿手改**（下次 `danqing build` 会覆盖，`danqing verify` 会报漂移） |

## 用法

把文件里的 `schemes` 数组粘进 Windows Terminal 的 `settings.json`：

```jsonc
{
  "schemes": [ /* … 8 套 … */ ],
  "profiles": { "defaults": { "colorScheme": "丹青 · 墨 · 群青" } },
  "theme": { "dark": "丹青 · 墨 · 群青", "light": "丹青 · 素 · 群青" }
}
```

八套的名字：`丹青 · {素|墨} · {群青|银朱|青黛|檀色}`。素是浅色（纸），墨是深色（夜）。

## 改配色

改 `../tokens/source.json` 的 `terminal` 段后重跑 `danqing build`。
16 个 ANSI 槽位与色族的对应、取阶规则、门禁口径见 [`../../docs/06-terminal-theming.md`](../../docs/06-terminal-theming.md)。
