//! 色彩：OKLab / OKLCh 转换、sRGB 线性化、色域收缩、WCAG 对比度与 alpha 合成。
//!
//! 这一整份是从旧 `tools/build.py` 逐行搬过来的，**包括那些看起来别扭的写法**，
//! 因为它们会直接落到最终的十六进制值上：
//!
//! - `contrast()` 会把背景色先四舍五入到 8 位再参与后续计算（量化误差被算进去了）；
//! - `rgb_to_hex()` 用 Python 的 `round()`，即四舍六入五取偶；
//! - 色域收缩是固定 28 次二分，不是迭代到收敛。
//!
//! 任何「顺手优化」都会让 121 个原语集体偏移一个台阶，所以这里一律不优化。

use crate::util::py_round;

pub type Rgb = (f64, f64, f64);

// ───────────────────────────────────────────── sRGB <-> 线性

pub fn srgb_to_linear(c: f64) -> f64 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

pub fn linear_to_srgb(c: f64) -> f64 {
    if c <= 0.0031308 {
        c * 12.92
    } else {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    }
}

// ───────────────────────────────────────────── hex <-> RGB

pub fn hex_to_rgb(h: &str) -> Rgb {
    let h = h.trim_start_matches('#');
    let h = if h.len() == 3 {
        h.chars().flat_map(|c| [c, c]).collect::<String>()
    } else {
        h.to_string()
    };
    let b = h.as_bytes();
    let chan = |i: usize| i64::from_str_radix(std::str::from_utf8(&b[i..i + 2]).unwrap(), 16).unwrap() as f64 / 255.0;
    (chan(0), chan(2), chan(4))
}

pub fn rgb_to_hex(rgb: Rgb) -> String {
    let parts = [rgb.0, rgb.1, rgb.2].map(|v| format!("{:02X}", (py_round(v * 255.0) as i64).clamp(0, 255)));
    format!("#{}", parts.concat())
}

// ───────────────────────────────────────────── OKLab

pub fn rgb_to_oklab(rgb: Rgb) -> Rgb {
    let (r, g, b) = (srgb_to_linear(rgb.0), srgb_to_linear(rgb.1), srgb_to_linear(rgb.2));
    let l = 0.412_221_470_8 * r + 0.536_332_536_3 * g + 0.051_445_992_9 * b;
    let m = 0.211_903_498_2 * r + 0.680_699_545_1 * g + 0.107_396_956_6 * b;
    let s = 0.088_302_461_9 * r + 0.281_718_837_6 * g + 0.629_978_700_5 * b;
    let cbrt = |v: f64| v.abs().powf(1.0 / 3.0).copysign(v);
    let (l_, m_, s_) = (cbrt(l), cbrt(m), cbrt(s));
    (
        0.210_454_255_3 * l_ + 0.793_617_785_0 * m_ - 0.004_072_046_8 * s_,
        1.977_998_495_1 * l_ - 2.428_592_205_0 * m_ + 0.450_593_709_9 * s_,
        0.025_904_037_1 * l_ + 0.782_771_766_2 * m_ - 0.808_675_766_0 * s_,
    )
}

pub fn oklab_to_rgb(lab: Rgb) -> Rgb {
    let (l, a, b) = lab;
    let l_ = l + 0.396_337_777_4 * a + 0.215_803_757_3 * b;
    let m_ = l - 0.105_561_345_8 * a - 0.063_854_172_8 * b;
    let s_ = l - 0.089_484_177_5 * a - 1.291_485_548_0 * b;
    let (l, m, s) = (l_ * l_ * l_, m_ * m_ * m_, s_ * s_ * s_);
    let r = 4.076_741_662_1 * l - 3.307_711_591_3 * m + 0.230_969_929_2 * s;
    let g = -1.268_438_004_6 * l + 2.609_757_401_1 * m - 0.341_319_396_5 * s;
    let bb = -0.004_196_086_3 * l - 0.703_418_614_7 * m + 1.707_614_701_0 * s;
    (linear_to_srgb(r), linear_to_srgb(g), linear_to_srgb(bb))
}

pub fn rgb_to_oklch(rgb: Rgb) -> Rgb {
    let (l, a, b) = rgb_to_oklab(rgb);
    (l, (a * a + b * b).sqrt(), b.atan2(a))
}

fn in_gamut(rgb: Rgb, tol: f64) -> bool {
    let all = [rgb.0, rgb.1, rgb.2];
    all.iter().all(|v| -tol <= *v && *v <= 1.0 + tol)
}

/// 恒色相下降彩度直到落进 sRGB 色域。固定 28 次二分，与旧实现一致。
pub fn oklch_to_rgb_clamped(l: f64, c: f64, h: f64) -> Rgb {
    let clamp = |v: f64| v.max(0.0).min(1.0);
    let mut lo = 0.0f64;
    let mut hi = c;
    let rgb = oklab_to_rgb((l, hi * h.cos(), hi * h.sin()));
    if in_gamut(rgb, 0.0015) {
        return (clamp(rgb.0), clamp(rgb.1), clamp(rgb.2));
    }
    for _ in 0..28 {
        let mid = (lo + hi) / 2.0;
        let rgb = oklab_to_rgb((l, mid * h.cos(), mid * h.sin()));
        if in_gamut(rgb, 0.0015) {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    let rgb = oklab_to_rgb((l, lo * h.cos(), lo * h.sin()));
    (clamp(rgb.0), clamp(rgb.1), clamp(rgb.2))
}

// ───────────────────────────────────────────── 亮度与对比度

pub fn rel_luminance(rgb: Rgb) -> f64 {
    let (r, g, b) = (srgb_to_linear(rgb.0), srgb_to_linear(rgb.1), srgb_to_linear(rgb.2));
    0.2126 * r + 0.7152 * g + 0.0722 * b
}

/// 支持 `#RGB` / `#RRGGBB` / `rgb(r,g,b)` / `rgba(r,g,b,a)`。返回 `(rgb, alpha)`。
pub fn parse_color(v: &str) -> (Rgb, f64) {
    let v = v.trim();
    if let Some(rest) = v.strip_prefix('#') {
        return (hex_to_rgb(rest), 1.0);
    }
    let start = v.find('(').expect("非法颜色：缺少 '('") + 1;
    let end = v.rfind(')').expect("非法颜色：缺少 ')'");
    let parts: Vec<&str> = v[start..end].split(',').map(str::trim).collect();
    let rgb = (
        parts[0].parse::<f64>().unwrap() / 255.0,
        parts[1].parse::<f64>().unwrap() / 255.0,
        parts[2].parse::<f64>().unwrap() / 255.0,
    );
    let alpha = if parts.len() > 3 {
        parts[3].parse::<f64>().unwrap()
    } else {
        1.0
    };
    (rgb, alpha)
}

/// 把含 alpha 的前景合成到背景上，返回不透明 rgb。
pub fn composite(fg: &str, bg: &str) -> Rgb {
    let (f_rgb, f_a) = parse_color(fg);
    let (b_rgb, _) = parse_color(bg);
    if f_a >= 1.0 {
        return f_rgb;
    }
    (
        f_a * f_rgb.0 + (1.0 - f_a) * b_rgb.0,
        f_a * f_rgb.1 + (1.0 - f_a) * b_rgb.1,
        f_a * f_rgb.2 + (1.0 - f_a) * b_rgb.2,
    )
}

fn contrast_raw(fg: &str, bg: &str) -> f64 {
    let l1 = rel_luminance(composite(fg, bg));
    let (b_rgb, b_a) = parse_color(bg);
    let l2 = if b_a < 1.0 {
        rel_luminance(composite(bg, "#FFFFFF"))
    } else {
        rel_luminance(b_rgb)
    };
    let (hi, lo) = (l1.max(l2), l1.min(l2));
    (hi + 0.05) / (lo + 0.05)
}

/// WCAG 对比度。背景先量化成 8 位 hex 再参与——这一步量化是旧实现的一部分。
pub fn contrast(fg: &str, bg: &str) -> f64 {
    let (b_rgb, b_a) = parse_color(bg);
    let base = if b_a < 1.0 {
        composite(bg, "#FFFFFF")
    } else {
        b_rgb
    };
    contrast_raw(fg, &rgb_to_hex(base))
}

/// 在 OKLCh 恒色相下调整 L，直到与 bg 的对比度达标（用于自动修正图表色）。
pub fn fit_lightness(hexv: &str, bg: &str, target: f64, toward_light: bool) -> String {
    let (l, c, h) = rgb_to_oklch(hex_to_rgb(hexv));
    if contrast(hexv, bg) >= target {
        return hexv.to_uppercase();
    }
    let mut best = hexv.to_uppercase();
    let mut best_d = (contrast(hexv, bg) - target).abs();
    for i in 1..=100 {
        let step = i as f64 * if toward_light { 0.010 } else { -0.010 };
        let l2 = (l + step).min(0.995).max(0.06);
        let cand = rgb_to_hex(oklch_to_rgb_clamped(l2, c, h)).to_uppercase();
        let d = (contrast(&cand, bg) - target).abs();
        if d < best_d {
            best = cand.clone();
            best_d = d;
        }
        if contrast(&cand, bg) >= target {
            return cand;
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oklab_roundtrip() {
        for hex in ["#2E59A7", "#D12920", "#F5F2E9", "#1E2732", "#7D5284"] {
            let back = rgb_to_hex(oklab_to_rgb(rgb_to_oklab(hex_to_rgb(hex))));
            assert_eq!(back, hex, "往返不一致: {hex}");
        }
    }

    #[test]
    fn known_contrast() {
        assert!((contrast("#FFFFFF", "#000000") - 21.0).abs() < 0.01);
        assert!((contrast("#000000", "#FFFFFF") - 21.0).abs() < 0.01);
        assert!((contrast("#767676", "#FFFFFF") - 4.54).abs() < 0.02);
    }

    #[test]
    fn alpha_is_composited_before_contrast() {
        // rgba(30,39,50,0.68) 压在 #F5F2E9 上，应与先手工合成再算的结果一致
        let composed = rgb_to_hex(composite("rgba(30,39,50,0.68)", "#F5F2E9"));
        let manual = {
            let f = (30.0 / 255.0, 39.0 / 255.0, 50.0 / 255.0);
            let b = hex_to_rgb("#F5F2E9");
            let a = 0.68;
            rgb_to_hex((
                a * f.0 + (1.0 - a) * b.0,
                a * f.1 + (1.0 - a) * b.1,
                a * f.2 + (1.0 - a) * b.2,
            ))
        };
        assert_eq!(composed, manual);
    }
}
