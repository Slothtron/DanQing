//! 流水线：色族构建 → 角色派生 → 引用解析 → 门禁 → 产物。
//!
//! 这是旧 `tools/build.py` 的主干，拆成了带类型的结构。规则本身一行没改，
//! 包括那些「凑出来但确实正确」的细节（比如 `on` 只在纯白与纯墨两个极端里挑）。

use std::collections::HashMap;

use serde_json::{Map, Value};

use crate::util::numf as util_numf;

use crate::color::{contrast, hex_to_rgb, oklch_to_rgb_clamped, rgb_to_hex, rgb_to_oklch};
use crate::util::numf;

// ───────────────────────────────────────────── 数据形态

pub struct PaletteEntry {
    pub hex: String,
    pub name: String,
    pub pinyin: String,
    pub group: String,
}

pub struct FamilyStep {
    pub hex: String,
    pub name: String,
    pub pinyin: String,
    pub nearest_hex: String,
    pub nearest_group: String,
    pub anchor: bool,
}

pub struct Family {
    pub id: String,
    pub name: String,
    pub pinyin: String,
    pub hue: String,
    pub anchor_hex: String,
    pub anchor_name: String,
    pub anchor_pinyin: String,
    pub intent: String,
    pub connotation: String,
    pub alternatives: Vec<String>,
    pub anchor_step: String,
    pub anchor_index: usize,
    /// 与 `steps` 顺序对应的（键, 级）序列。
    pub steps: Vec<(String, FamilyStep)>,
}

impl Family {
    pub fn step(&self, key: &str) -> &FamilyStep {
        self.steps
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v)
            .unwrap_or_else(|| panic!("色族 {} 无第 {} 级", self.id, key))
    }
}

/// 一族色派生出的语义角色。
///
/// 其中若干字段当前没有落到任何产物里（`text_step` / `contrast_*` 等），但它们是
/// 旧实现角色表的一部分，保留下来是为了将来写报告或做手工复核时不用回头改模型。
#[allow(dead_code)]
#[derive(Clone)]
pub struct Roles {
    pub fill: String,
    pub fill_step: i64,
    pub fill_retreat: i64,
    pub fill_hover: String,
    pub fill_active: String,
    pub on: String,
    pub text: String,
    pub text_step: i64,
    pub text_retreat: i64,
    pub subtle: String,
    pub border: String,
    pub contrast_on_fill: f64,
    pub contrast_text_bg: f64,
    pub contrast_text_subtle: f64,
    pub contrast_fill_bg: f64,
}

impl Roles {
    /// 复刻 `resolve_refs` 里那道 `key.endswith((...))` 守卫：只有是这七个后缀之一
    /// 才允许从角色表里取值，其它情况一律回落去看是不是 `@family.step` 引用。
    pub fn str_field(&self, key: &str) -> Option<&String> {
        const SUFFIXES: [&str; 7] = ["fill", "fillHover", "fillActive", "on", "text", "subtle", "border"];
        if !SUFFIXES.iter().any(|s| key.ends_with(s)) {
            return None;
        }
        match key {
            "fill" => Some(&self.fill),
            "fillHover" => Some(&self.fill_hover),
            "fillActive" => Some(&self.fill_active),
            "on" => Some(&self.on),
            "text" => Some(&self.text),
            "subtle" => Some(&self.subtle),
            "border" => Some(&self.border),
            _ => None,
        }
    }
}

#[allow(dead_code)]
pub struct ChartFix {
    /// `"raw"`（参与了自动修正）或 `"nofix"`（非图表色号，原样放过）。
    pub kind: String,
    pub raw: String,
    pub fixed: String,
}

/// 一个品牌在两种模式下的解析结果。
pub struct Entry {
    pub id: String,
    pub light: Value,
    pub dark: Value,
    pub roles_light: HashMap<String, Roles>,
    pub roles_dark: HashMap<String, Roles>,
}

impl Entry {
    pub fn sem(&self, mode: &str) -> &Value {
        if mode == "light" {
            &self.light
        } else {
            &self.dark
        }
    }
    pub fn roles(&self, mode: &str) -> &HashMap<String, Roles> {
        if mode == "light" {
            &self.roles_light
        } else {
            &self.roles_dark
        }
    }
}

// ───────────────────────────────────────────── JSON 访问器

pub fn obj<'a>(v: &'a Value, key: &str) -> &'a Map<String, Value> {
    v.get(key).and_then(Value::as_object).unwrap_or_else(|| panic!("缺少对象字段 {key}"))
}

/// 从已经取出来的对象里再取一层子对象（为避免到处写 `["x"].as_object().unwrap()`）。
pub fn sub<'a>(v: &'a Map<String, Value>, key: &str) -> &'a Map<String, Value> {
    v.get(key).and_then(Value::as_object).unwrap_or_else(|| panic!("缺少对象字段 {key}"))
}

pub fn arr<'a>(v: &'a Value, key: &str) -> &'a Vec<Value> {
    v.get(key).and_then(Value::as_array).unwrap_or_else(|| panic!("缺少数组字段 {key}"))
}

pub fn s<'a>(v: &'a Value, key: &str) -> &'a str {
    v.get(key).and_then(Value::as_str).unwrap_or_else(|| panic!("缺少字符串字段 {key}"))
}

pub fn opt_s<'a>(v: &'a Value, key: &str) -> &'a str {
    v.get(key).and_then(Value::as_str).unwrap_or("")
}

pub fn to_strings(v: &Value) -> Vec<String> {
    v.as_array()
        .map(|a| a.iter().map(|x| x.as_str().unwrap().to_string()).collect())
        .unwrap_or_default()
}

// ───────────────────────────────────────────── 色族

/// 由传统色锚点生成感知均匀色阶。
pub fn build_families(source: &Value, palette: &Value) -> (Vec<Family>, Vec<PaletteEntry>) {
    let ramp = obj(source, "ramp");
    let steps: Vec<i64> = ramp["steps"].as_array().unwrap().iter().map(|x| x.as_i64().unwrap()).collect();
    let grid: Vec<f64> = ramp["lightnessGrid"].as_array().unwrap().iter().map(util_numf).collect();
    let taper = ramp["taper"].as_object().unwrap();
    let floor = numf(&taper["floor"]);
    let slope = numf(&taper["slope"]);
    let exponent = numf(&taper["exponent"]);

    // 传统色打平去重：按 HEX 去重，同色取首次出现（先到先得，决定了最近邻归属）
    let mut flat: Vec<PaletteEntry> = Vec::new();
    let mut seen: Vec<String> = Vec::new();
    let groups = obj(palette, "groups");
    for (group, items) in groups.iter() {
        for item in items.as_array().unwrap() {
            let key = item[0].as_str().unwrap().trim_start_matches('#').to_uppercase();
            let keyed = format!("#{key}");
            if seen.iter().any(|k| k == &keyed) {
                continue;
            }
            seen.push(keyed.clone());
            flat.push(PaletteEntry {
                hex: keyed,
                name: item[1].as_str().unwrap().to_string(),
                pinyin: item[2].as_str().unwrap().to_string(),
                group: group.clone(),
            });
        }
    }
    // 注意必须是 **OKLab 空间**的距离，不是 sRGB：后者会让浅蓝白 `#F2F7FF`
    // 的最近邻算成赤色系的「丹罽」（sRGB 空间里两者都接近 1.0）。
    let flat_lab: Vec<_> = flat
        .iter()
        .map(|e| {
            (
                crate::color::rgb_to_oklab(hex_to_rgb(&e.hex)),
                &e.hex,
                &e.name,
                &e.pinyin,
                &e.group,
            )
        })
        .collect();

    let nearest = |hexv: &str| -> (&str, &str, &str, &str) {
        let lab = crate::color::rgb_to_oklab(hex_to_rgb(hexv));
        let mut best: Option<(&str, &str, &str, &str)> = None;
        let mut best_d = f64::INFINITY;
        for (lab2, h2, n2, p2, g2) in flat_lab.iter() {
            let d = (lab.0 - lab2.0).powi(2) + (lab.1 - lab2.1).powi(2) + (lab.2 - lab2.2).powi(2);
            if d < best_d {
                best = Some((n2.as_str(), p2.as_str(), h2.as_str(), g2.as_str()));
                best_d = d;
            }
        }
        best.unwrap()
    };

    let mut families = Vec::new();
    for fam in arr(source, "families") {
        let fid = s(fam, "id").to_string();
        let anchor_hex = s(fam, "anchorHex");
        let (l_a, c_a, h_a) = rgb_to_oklch(hex_to_rgb(anchor_hex));

        // 锚点归位：选与锚点明度最接近的那一级（并列取靠前的）
        let mut idx = 0usize;
        let mut best_d = f64::INFINITY;
        for (i, g) in grid.iter().enumerate() {
            let d = (g - l_a).abs();
            if d < best_d {
                best_d = d;
                idx = i;
            }
        }
        let upper = f64::max(1.0 - l_a, 0.12);
        let lower = f64::max(l_a, 0.12);

        let mut out_steps = Vec::new();
        for (i, (step, l)) in steps.iter().zip(grid.iter()).enumerate() {
            let hexv = if i == idx {
                anchor_hex.to_uppercase()
            } else {
                let d = if *l > l_a { (l - l_a) / upper } else { (l - l_a) / lower };
                let taper_v = f64::max(floor, 1.0 - slope * d.abs().powf(exponent));
                rgb_to_hex(oklch_to_rgb_clamped(*l, c_a * taper_v, h_a))
            };
            let (nm, pyn, nh, ngrp) = nearest(&hexv);
            out_steps.push((
                step.to_string(),
                FamilyStep {
                    hex: hexv,
                    name: nm.to_string(),
                    pinyin: pyn.to_string(),
                    nearest_hex: nh.to_string(),
                    nearest_group: ngrp.to_string(),
                    anchor: i == idx,
                },
            ));
        }

        families.push(Family {
            id: fid,
            name: s(fam, "name").to_string(),
            pinyin: s(fam, "pinyin").to_string(),
            hue: opt_s(fam, "hue").to_string(),
            anchor_hex: anchor_hex.to_string(),
            anchor_name: s(fam, "anchorName").to_string(),
            anchor_pinyin: opt_s(fam, "anchorPinyin").to_string(),
            intent: s(fam, "intent").to_string(),
            connotation: opt_s(fam, "connotation").to_string(),
            alternatives: fam.get("alternatives").map(to_strings).unwrap_or_default(),
            anchor_step: steps[idx].to_string(),
            anchor_index: idx,
            steps: out_steps,
        });
    }
    (families, flat)
}

// ───────────────────────────────────────────── 角色派生

/// 从锚点向外走：fill 近锚优先；textLight 向更暗（索引增大）；textDark 向更亮（索引减小）。
pub fn step_order(idx: usize, action: &str, n: usize) -> Vec<usize> {
    if action == "fill" {
        let mut seq = vec![idx];
        let mut d = 1;
        while seq.len() < n {
            if idx >= d {
                seq.push(idx - d);
            }
            if idx + d < n {
                seq.push(idx + d);
            }
            d += 1;
        }
        return seq.into_iter().take(n).collect();
    }
    if action == "textLight" {
        let mut seq: Vec<usize> = (idx..n).collect();
        seq.extend((0..idx).rev());
        return seq;
    }
    let mut seq: Vec<usize> = (0..=idx).rev().collect();
    seq.extend((idx + 1)..n);
    seq
}

pub fn role_set(fid: &str, families: &[Family], mode: &str, bg_base: &str, steps: &[i64]) -> Roles {
    let fam = families.iter().find(|f| f.id == fid).unwrap();
    let idx = fam.anchor_index;
    let n = steps.len();
    const ON_CANDIDATES: [&str; 2] = ["#FFFFFF", "#0E1116"];

    let hexof = |i: usize| fam.step(&steps[i].to_string()).hex.clone();

    let pick = |action: &str, test: &dyn Fn(&str) -> bool| {
        let seq = step_order(idx, action, n);
        for i in seq.iter() {
            if test(&hexof(*i)) {
                return *i;
            }
        }
        seq[0]
    };

    let fill_ok = |h: &str| {
        if contrast(h, bg_base) < 3.0 {
            return false;
        }
        ON_CANDIDATES.iter().map(|c| contrast(c, h)).fold(0.0, f64::max) >= 4.5
    };
    let fi = pick("fill", &fill_ok);
    let fill = hexof(fi);
    // Python 的 max() 在并列时取首个，这里用严格大于保留同一行为
    let on = ON_CANDIDATES
        .iter()
        .copied()
        .fold(ON_CANDIDATES[0], |best, c| {
            if contrast(c, &fill) > contrast(best, &fill) {
                c
            } else {
                best
            }
        })
        .to_string();

    let subtle = hexof(if mode == "dark" { n - 2 } else { 1 });

    let text_ok = |h: &str| contrast(h, bg_base) >= 4.5 && contrast(h, &subtle) >= 4.5;
    let ti = pick(
        if mode == "light" { "textLight" } else { "textDark" },
        &text_ok,
    );
    let text = hexof(ti);

    let delta = if mode == "dark" { -1isize } else { 1 };
    let clamp_i = |v: isize| (v.max(0) as usize).min(n - 1);
    let hov = hexof(clamp_i(fi as isize + delta));
    let act = hexof(clamp_i(fi as isize + 2 * delta));

    Roles {
        contrast_on_fill: round2(contrast(&on, &fill)),
        contrast_text_bg: round2(contrast(&text, bg_base)),
        contrast_text_subtle: round2(contrast(&text, &subtle)),
        contrast_fill_bg: round2(contrast(&fill, bg_base)),
        fill: fill.clone(),
        fill_step: steps[fi],
        fill_retreat: fi as i64 - idx as i64,
        fill_hover: hov,
        fill_active: act,
        on,
        text,
        text_step: steps[ti],
        text_retreat: ti as i64 - idx as i64,
        subtle,
        border: fill,
    }
}

fn round2(v: f64) -> f64 {
    crate::util::py_round2(v)
}

// ───────────────────────────────────────────── 引用解析

struct RefCtx<'a> {
    families: &'a [Family],
    roles: &'a HashMap<String, Roles>,
    brand_family: &'a str,
    accent_family: &'a str,
    ink_rgb: &'a str,
}

fn resolve_refs(node: &Value, ctx: &RefCtx) -> Value {
    match node {
        Value::Object(map) => {
            let mut out = Map::new();
            for (k, v) in map.iter() {
                out.insert(k.clone(), resolve_refs(v, ctx));
            }
            Value::Object(out)
        }
        Value::Array(list) => Value::Array(list.iter().map(|v| resolve_refs(v, ctx)).collect()),
        Value::String(v) => {
            if let Some(target) = v.strip_prefix('@') {
                let (fam_id, key) = target.split_once('.').unwrap_or((target, ""));
                if fam_id == "brand" {
                    return Value::String(
                        ctx.roles[ctx.brand_family]
                            .str_field(key)
                            .unwrap_or_else(|| panic!("角色表无 {}.{}", ctx.brand_family, key))
                            .clone(),
                    );
                }
                if fam_id == "accent" {
                    return Value::String(
                        ctx.roles[ctx.accent_family]
                            .str_field(key)
                            .unwrap_or_else(|| panic!("角色表无 {}.{}", ctx.accent_family, key))
                            .clone(),
                    );
                }
                if let Some(roles) = ctx.roles.get(fam_id) {
                    if let Some(val) = roles.str_field(key) {
                        return Value::String(val.clone());
                    }
                }
                if let Some(fam) = ctx.families.iter().find(|f| f.id == fam_id) {
                    if fam.steps.iter().any(|(k, _)| k == key) {
                        return Value::String(fam.step(key).hex.clone());
                    }
                }
                panic!("无法解析引用：{v}");
            }
            Value::String(v.replace("@inkRgb", ctx.ink_rgb))
        }
        other => other.clone(),
    }
}

/// 把 `@family.step` 解成字面色值；非引用原样返回。
pub fn deref(refr: &str, families: &[Family]) -> String {
    if let Some(target) = refr.strip_prefix('@') {
        if !target.starts_with("brand") && !target.starts_with("accent") {
            let (fid, step) = target.split_once('.').unwrap_or((target, ""));
            if let Some(fam) = families.iter().find(|f| f.id == fid) {
                if fam.steps.iter().any(|(k, _)| k == step) {
                    return fam.step(step).hex.clone();
                }
            }
        }
    }
    refr.to_string()
}

pub fn build_semantic(
    source: &Value,
    families: &[Family],
    mode: &str,
    brand_family: &str,
    accent_family: &str,
    steps: &[i64],
    bg_base: &str,
) -> (Value, Vec<(String, ChartFix)>, HashMap<String, Roles>) {
    let mut roles: HashMap<String, Roles> = HashMap::new();
    for fam in arr(source, "families") {
        let fid = s(fam, "id");
        roles.insert(fid.to_string(), role_set(fid, families, mode, bg_base, steps));
    }
    let sem_src = &obj(source, "semantic")[mode];
    let ink_rgb = s(sem_src, "inkRgb");
    let ctx = RefCtx {
        families,
        roles: &roles,
        brand_family,
        accent_family,
        ink_rgb,
    };
    let mut raw = Map::new();
    for (k, v) in sem_src.as_object().unwrap() {
        if k == "inkRgb" {
            continue;
        }
        raw.insert(k.clone(), resolve_refs(v, &ctx));
    }
    let resolved = Value::Object(raw);

    // chart 色自动保证与底色 3:1
    let bg = resolved["bg"]["base"].as_str().unwrap().to_string();
    let toward_light = mode == "dark";
    let mut fixes: Vec<(String, ChartFix)> = Vec::new();
    let chart = resolved["chart"].as_object().unwrap();
    for (k, v) in chart.iter() {
        let vv = v.as_str().unwrap().to_string();
        let is_chart = k.starts_with('c') && k[1..].chars().all(|c| c.is_ascii_digit());
        let fixed = if is_chart {
            crate::color::fit_lightness(&vv, &bg, 3.0, toward_light)
        } else {
            vv.clone()
        };
        fixes.push((
            k.clone(),
            ChartFix {
                kind: if is_chart { "raw" } else { "nofix" }.to_string(),
                raw: vv,
                fixed,
            },
        ));
    }

    (resolved, fixes, roles)
}

// ───────────────────────────────────────────── 扁平化

/// `{"role": {"k": v}}` → `[("role-k", v)]`，保留原 Key 顺序。
pub fn flat_semantic(sem: &Value) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for (role, val) in sem.as_object().unwrap() {
        if let Some(inner) = val.as_object() {
            for (k2, v2) in inner {
                out.push((format!("{role}-{k2}"), v2.as_str().unwrap().to_string()));
            }
        } else {
            out.push((role.clone(), val.as_str().unwrap().to_string()));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fill_order_is_near_anchor_first() {
        assert_eq!(step_order(4, "fill", 11), vec![4, 3, 5, 2, 6, 1, 7, 0, 8, 9, 10]);
    }

    #[test]
    fn text_orders_walk_outward() {
        assert_eq!(step_order(3, "textLight", 6), vec![3, 4, 5, 2, 1, 0]);
        assert_eq!(step_order(3, "textDark", 6), vec![3, 2, 1, 0, 4, 5]);
    }
}
