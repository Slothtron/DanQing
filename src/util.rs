//! 工具函数：复刻 Python 语义的舍入与数字格式化。
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

/// 取 JSON 数字的 f64 形态（用于计算与 RGBA 分量）。
pub fn numf(v: &Value) -> f64 {
    match v {
        Value::Number(n) => n.as_f64().unwrap(),
        other => panic!("期望数字，实际为 {other:?}"),
    }
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
    }
}
