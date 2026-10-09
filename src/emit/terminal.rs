//! Windows Terminal 配色：`themes/windows-terminal/danqing.schemes.json`
//!
//! 这是本仓库**唯一入库**的产物：使用者 clone 下来即可把它粘进 `settings.json`
//! 的 `schemes` 数组，不需要先装 Rust 工具链。其余生成物（报告、快照、展示页数据）
//! 仍然不入库。取舍见 `docs/06-terminal-theming.md`。
//!
//! 字段与取值口径完全按 Windows Terminal 的配色方案格式：`name` 必需，其余为
//! `#rrggbb`，`cursorColor` / `selectionBackground` 可选（本仓库一律写出）。

use serde_json::{Map, Value};

use crate::pipeline::Pipeline;
use crate::terminal::{self, Scheme};

pub fn emit_windows_terminal(p: &Pipeline) -> String {
    let schemes: Vec<Value> = terminal::schemes(p).iter().map(scheme_json).collect();
    let mut doc = Map::new();
    doc.insert("schemes".to_string(), Value::Array(schemes));
    let mut out = serde_json::to_string_pretty(&Value::Object(doc)).unwrap();
    out.push('\n');
    out
}

fn put(m: &mut Map<String, Value>, key: &str, val: &str) {
    m.insert(key.to_string(), Value::String(val.to_string()));
}

/// 一套 scheme 的字段顺序按 Windows Terminal 文档的示例排列：
/// 先 `name` 与四项 chrome，再 8 基色，最后 8 亮色。
fn scheme_json(s: &Scheme) -> Value {
    let mut m = Map::new();
    put(&mut m, "name", &s.name);
    put(&mut m, "background", &s.background);
    put(&mut m, "foreground", &s.foreground);
    put(&mut m, "cursorColor", &s.cursor_color);
    put(&mut m, "selectionBackground", &s.selection_background);
    for (slot, hexv) in &s.ansi {
        put(&mut m, slot, hexv);
    }
    Value::Object(m)
}
