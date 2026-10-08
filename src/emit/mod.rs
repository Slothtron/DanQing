//! 各端产物生成器。每一个 `emit_*` 对应旧 `tools/build.py` 里的同名函数。

use crate::color::{hex_to_rgb, parse_color, rgb_to_hex};
use crate::util::{cap_first, py_round};

pub mod android;
pub mod avalonia;
pub mod compose;
pub mod css;
pub mod dtcg;
pub mod flutter;
pub mod report;
pub mod scss;
pub mod showcase;
pub mod swift;
pub mod tailwind;
pub mod ts;

/// 生成器署名，写进产物头部注释。
pub const GENERATOR: &str = "danqing build";

/// 色阶报告头部的署名。Python 时代这里是硬编码的相对路径，
/// 与 GENERATOR 不是同一个值；现在二者归一。
pub const REPORT_GENERATOR: &str = GENERATOR;

/// 产物头部那句「改了真源要重跑什么」的提示。
pub const REDO_TIP: &str = "改 tokens/source.json 后重跑 danqing build";

/// `#RRGGBB` 原样，`rgba(...)` → `#AARRGGBB`（Avalonia / Android / Swift 用）。
pub fn android_color(v: &str) -> String {
    if v.starts_with('#') {
        return v.to_string();
    }
    let (rgb, a) = parse_color(v);
    let ch = |x: f64| py_round(x * 255.0) as i64;
    format!("#{:02X}{:02X}{:02X}{:02X}", py_round(a * 255.0) as i64, ch(rgb.0), ch(rgb.1), ch(rgb.2))
}

/// Dart `Color` 字面量。
pub fn dart_color(v: &str) -> String {
    if let Some(h) = v.strip_prefix('#') {
        return if h.len() == 8 {
            format!("Color(0x{h})")
        } else {
            format!("Color(0xFF{h})")
        };
    }
    let (rgb, a) = parse_color(v);
    let ch = |x: f64| py_round(x * 255.0) as i64;
    let a3 = py_round(a * 1000.0) / 1000.0;
    format!(
        "Color.fromRGBO({}, {}, {}, {})",
        ch(rgb.0),
        ch(rgb.1),
        ch(rgb.2),
        crate::util::fstr(a3)
    )
}

/// Kotlin Compose `Color` 字面量。
pub fn kt_color(v: &str) -> String {
    if let Some(h) = v.strip_prefix('#') {
        return if h.len() == 8 {
            format!("Color(0x{h})")
        } else {
            format!("Color(0xFF{h})")
        };
    }
    let (rgb, a) = parse_color(v);
    let ch = |x: f64| py_round(x * 255.0) as i64;
    format!(
        "Color(red = {}, green = {}, blue = {}, alpha = {})",
        ch(rgb.0),
        ch(rgb.1),
        ch(rgb.2),
        py_round(a * 255.0) as i64
    )
}

/// Avalonia 资源键：`bg-base` → `DqBgBase`，`primary-fillHover` → `DqPrimaryFillHover`。
pub fn avl_key(k: &str) -> String {
    let parts = k.replace('-', "_");
    let mut out = String::from("Dq");
    for part in parts.split('_') {
        out.push_str(&cap_first(part));
    }
    out
}

/// 朝目标色线性混合（用于派生 Fluent 需要的渐层强调色）。
pub fn mix_toward(hexv: &str, target: (f64, f64, f64), t: f64) -> String {
    let rgb = hex_to_rgb(hexv);
    rgb_to_hex((
        rgb.0 + (target.0 - rgb.0) * t,
        rgb.1 + (target.1 - rgb.1) * t,
        rgb.2 + (target.2 - rgb.2) * t,
    ))
}

/// `#RRGGBB` → `#AARRGGBB`。
pub fn with_alpha(hexv: &str, alpha: i64) -> String {
    format!("#{alpha:02X}{}", hexv.trim_start_matches('#'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn android_color_handles_both_forms() {
        assert_eq!(android_color("#FFFFFF"), "#FFFFFF");
        assert_eq!(android_color("rgba(30,39,50,0.08)"), "#141E2732");
    }

    #[test]
    fn avl_key_keeps_inner_caps() {
        assert_eq!(avl_key("bg-base"), "DqBgBase");
        assert_eq!(avl_key("primary-fillHover"), "DqPrimaryFillHover");
    }
}
