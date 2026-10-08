//! 工具函数：复刻 Python 语义的舍入、数字格式化与字符串变换。
//!
//! 为什么要单独提 unfaithful 的舍入：旧实现是 Python，`round()` 是**四舍六入五取偶**
//! （banker's rounding），而 Rust 的 `f64::round` 是**远离零**。色阶每个月级的最终
//! 量化是 `round(v * 255)`，一旦有值恰好落在 `n + 0.5`，两种舍入会差 1/255，
//! 于是整个色阶跟着偏一个台阶。为了「重构 ≠ 换行为」，这里严格复刻 Python 语义。

use serde_json::Value;

/// Python `round(x)`（ndigits=0）：远离一半时向最近取整，恰好一半时向偶数取整。
pub fn py_round(v: f64) -> f64 {
    if !v.is_finite() {
        return v;
    }
    let r = v.round();
    if (v - r).abs() == 0.5 && r % 2.0 != 0.0 {
        r - v.signum()
    } else {
        r
    }
}

/// Python `round(x, 2)`：对比度报告与展示页数据用。先把精确到两位的小数取侧面不再
/// 依赖 `*100` 的整数乘法（那样会在 `2.675` 这类值上与 Python 分道扬镳），而是走
/// 同样会遇到的浮点，但关键在于**之后的输出格式化必须补 `.0`**。
pub fn py_round2(v: f64) -> f64 {
    if !v.is_finite() {
        return v;
    }
    py_round(v * 100.0) / 100.0
}

/// 复刻 Python `str(float)` / `json` 对数字的输出：整数值不带小数点是 int，
/// 但 float 即使是整数也要带 `.0`（Python `repr(2.0) == "2.0"`，Rust 的
/// `format!("{}", 2.0)` 只会给 `"2"`）。
pub fn fstr(v: f64) -> String {
    if v.is_finite() && v.fract() == 0.0 && v.abs() < 1e16 {
        if v == 0.0 && v.is_sign_negative() {
            return "-0.0".to_string();
        }
        return format!("{v:.1}");
    }
    format!("{v}")
}

/// 取一个 JSON 数字在 Python 语境下的字面表示：int 走 int 分支，float 走 float 分支。
/// 生成器里到处出现 `f"...{v}px"`，旧实现里 `v` 来自 `json.loads`，整型就是整型。
pub fn num(v: &Value) -> String {
    match v {
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                return i.to_string();
            }
            if let Some(u) = n.as_u64() {
                return u.to_string();
            }
            fstr(n.as_f64().unwrap())
        }
        other => panic!("期望数字，实际为 {other:?}"),
    }
}

/// 取 JSON 数字的 f64 形态（用于计算与 RGBA 分量）。
pub fn numf(v: &Value) -> f64 {
    match v {
        Value::Number(n) => n.as_f64().unwrap(),
        other => panic!("期望数字，实际为 {other:?}"),
    }
}

/// Python `json.dumps(s, ensure_ascii=False)` 对字符串的输出（用于 TS 里的键名）。
pub fn jlabel(s: &str) -> String {
    serde_json::to_string(&Value::String(s.to_string())).unwrap()
}

/// 只把首字母大写，保留词内原有大小写（bodySm → BodySm）。
///
/// 不可用 Rust 的 `to_uppercase` 处理整个串，也不可用 Python 的 `str.capitalize`：
/// 前者依赖 Unicode 而这里只关心 ASCII 首字符，后者会把词内大写压平
/// （bodySm → Bodysm），生成出来的键名会与真源对不上。
pub fn cap_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(first) => {
            let up: String = first.to_uppercase().collect();
            format!("{up}{}", c.as_str())
        }
        None => String::new(),
    }
}

/// Python `str.capitalize()`：首字母大写 + **其余全部小写**。
///
/// 旧实现在 Flutter / Compose / Swift 的尺度键上用的是这个（会压平词内大写，
/// 例如 `bodySm` → `Bodysm`）。这是历史产物里的既定键名，为了不破坏下游引用，
/// 移植时如实保留。
pub fn capitalize(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars();
    if let Some(first) = chars.next() {
        out.extend(first.to_uppercase());
        out.extend(chars.flat_map(|c| c.to_lowercase()));
    }
    out
}

/// `a-b-c` / `a_b_c` → `abcDef`：去掉分隔符，首段原样保留，其余段首字母大写
/// （确切说是 Python 的 `capitalize`，会压平词内大写——见 [`capitalize`]）。
pub fn cam(s: &str) -> String {
    let binding = s.replace('-', "_");
    let parts: Vec<&str> = binding.split('_').collect();
    let head = parts.first().copied().unwrap_or("");
    let mut out = head.to_string();
    for p in &parts[1..] {
        out.push_str(&capitalize(p));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_half_to_even_like_python() {
        assert_eq!(py_round(2.5), 2.0);
        assert_eq!(py_round(3.5), 4.0);
        assert_eq!(py_round(-2.5), -2.0);
        assert_eq!(py_round(2.6), 3.0);
        assert_eq!(py_round(-0.5), 0.0);
    }

    #[test]
    fn number_display_like_python() {
        assert_eq!(fstr(2.0), "2.0");
        assert_eq!(fstr(0.08), "0.08");
        assert_eq!(fstr(1.75), "1.75");
        assert_eq!(num(&serde_json::from_str::<Value>("4").unwrap()), "4");
        assert_eq!(num(&serde_json::from_str::<Value>("0.5").unwrap()), "0.5");
    }

    #[test]
    fn cap_first_keeps_inner_case() {
        assert_eq!(cap_first("bodySm"), "BodySm");
        assert_eq!(cap_first("scrimStrong"), "ScrimStrong");
        assert_eq!(capitalize("bodySm"), "Bodysm");
        assert_eq!(cam("body-sm"), "bodySm");
    }
}
