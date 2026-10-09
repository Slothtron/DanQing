//! 终端主题：把真源 `terminal` 段解析成 Windows Terminal 的配色方案。
//!
//! 真源里 `chrome` 用 `sys.<role>[.<key>]` 指语义层，`palette` 用 `@<family>.<step>`
//! 指原语层。本模块是这两套引用**唯一**的解析处——门禁与产物都从这里取值，
//! 免得两边各解析一遍、日后悄悄分叉。

use serde_json::{Map, Value};

use crate::model::{arr, deref, obj, s, Family};
use crate::pipeline::Pipeline;

/// Windows Terminal 的 16 个 ANSI 槽位，按官方文档列出的字段顺序。
pub const ANSI_ORDER: [&str; 16] = [
    "black",
    "red",
    "green",
    "yellow",
    "blue",
    "purple",
    "cyan",
    "white",
    "brightBlack",
    "brightRed",
    "brightGreen",
    "brightYellow",
    "brightBlue",
    "brightPurple",
    "brightCyan",
    "brightWhite",
];

/// 一套已解析的配色方案。`ansi` 按 [`ANSI_ORDER`] 排列。
pub struct Scheme {
    /// 品牌 id，如 `qing`。
    pub brand: String,
    pub brand_name: String,
    /// `light` / `dark`。
    pub mode: String,
    /// 模式中文名，如 `素`。
    pub mode_name: String,
    /// 写进 `settings.json` 的 `name`，如 `丹青 · 素 · 群青`。
    pub name: String,
    pub background: String,
    pub foreground: String,
    pub cursor_color: String,
    pub selection_background: String,
    pub ansi: Vec<(String, String)>,
    /// 与底色同侧的中性槽位，不参与同底对比门禁。
    pub exempt: Vec<String>,
}

impl Scheme {
    /// 除豁免槽位外，其余 16 槽位都要与底色保持可读对比。
    pub fn gated_slots(&self) -> impl Iterator<Item = &(String, String)> {
        self.ansi.iter().filter(|(slot, _)| !self.exempt.contains(slot))
    }
}

fn tstr<'a>(t: &'a Map<String, Value>, key: &str) -> &'a str {
    t.get(key)
        .and_then(Value::as_str)
        .unwrap_or_else(|| panic!("terminal 段缺少字符串字段 {key}"))
}

fn tobj<'a>(t: &'a Map<String, Value>, key: &str) -> &'a Map<String, Value> {
    t.get(key)
        .and_then(Value::as_object)
        .unwrap_or_else(|| panic!("terminal 段缺少对象字段 {key}"))
}

/// `sys.bg.canvas` → 语义层里已解析的 `#RRGGBB`。
fn resolve_chrome(sem: &Value, path: &str) -> String {
    let rest = path
        .strip_prefix("sys.")
        .unwrap_or_else(|| panic!("终端 chrome 引用须以 sys. 开头：{path}"));
    let mut node = sem;
    for part in rest.split('.') {
        node = node
            .get(part)
            .unwrap_or_else(|| panic!("语义层无 {path}"));
    }
    node.as_str()
        .unwrap_or_else(|| panic!("{path} 不是字面色值"))
        .to_string()
}

/// `@mo.900` → `#1E2732`。原文返回即说明引用没解析成功。
fn resolve_slot(families: &[Family], raw: &str) -> String {
    let out = deref(raw, families);
    if out == raw {
        panic!("终端槽位引用无法解析：{raw}（应为 @色族.色阶）");
    }
    out
}

/// 全部品牌 × 模式的配色方案，品牌按真源顺序、模式明前暗后。
pub fn schemes(p: &Pipeline) -> Vec<Scheme> {
    let t = obj(&p.source, "terminal");
    let prefix = tstr(t, "schemePrefix");
    let modes = tobj(t, "modes");
    let chrome = tobj(t, "chrome");
    let palette = tobj(t, "palette");
    let neutrals = tobj(t, "backgroundSideNeutrals");

    let mut out = Vec::new();
    for brand in arr(&p.source, "brands") {
        let bid = s(brand, "id");
        let bname = s(brand, "name");
        let entry = p
            .entries
            .iter()
            .find(|e| e.id == bid)
            .unwrap_or_else(|| panic!("流水线里没有品牌条目 {bid}"));
        for mode in ["light", "dark"] {
            let sem = entry.sem(mode);
            let pal = palette
                .get(mode)
                .and_then(Value::as_object)
                .unwrap_or_else(|| panic!("terminal.palette 缺少 {mode}"));
            let mut ansi = Vec::new();
            for slot in ANSI_ORDER {
                let raw = pal
                    .get(slot)
                    .and_then(Value::as_str)
                    .unwrap_or_else(|| panic!("terminal.palette.{mode} 缺少槽位 {slot}"));
                ansi.push((slot.to_string(), resolve_slot(&p.families, raw)));
            }
            let mode_name = tstr(modes, mode);
            out.push(Scheme {
                brand: bid.to_string(),
                brand_name: bname.to_string(),
                mode: mode.to_string(),
                mode_name: mode_name.to_string(),
                name: format!("{prefix} · {mode_name} · {bname}"),
                background: resolve_chrome(sem, tstr(chrome, "background")),
                foreground: resolve_chrome(sem, tstr(chrome, "foreground")),
                cursor_color: resolve_chrome(sem, tstr(chrome, "cursorColor")),
                selection_background: resolve_chrome(sem, tstr(chrome, "selectionBackground")),
                ansi,
                exempt: neutrals
                    .get(mode)
                    .and_then(Value::as_array)
                    .unwrap_or_else(|| panic!("terminal.backgroundSideNeutrals 缺少 {mode}"))
                    .iter()
                    .map(|v| v.as_str().unwrap().to_string())
                    .collect(),
            });
        }
    }
    out
}
