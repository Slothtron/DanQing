//! 一次完整构建的全部中间状态：真源 → 色族 → 品牌语义 → 门禁检查项。
//!
//! 与旧实现的一处结构性差异（结果等价）：Python 用 `resolved_all["__base__"]` 指向
//! 默认品牌那个**同一个 dict 对象**；Rust 里没有共享可变的便宜写法，改为记一个索引
//! `base_idx`。语义上仍然是「基座就是默认品牌的那一份」。

use serde_json::Value;

use crate::model::{
    arr, build_families, build_semantic, deref, obj, s, ChartFix, Entry, Family,
    PaletteEntry,
};

pub struct Pipeline {
    pub source: Value,
    pub families: Vec<Family>,
    pub palette_flat: Vec<PaletteEntry>,
    pub steps: Vec<i64>,
    pub entries: Vec<Entry>,
    pub base_idx: usize,
    pub default_brand: String,
    /// 强调色保留族（紫）。留在这里是为了让「accent 走保留族」这件事在模型层可见。
    #[allow(dead_code)]
    pub accent_family: String,
    /// `(模式, [(键, 修正记录)])`，保持插入顺序。
    pub chart_fix: Vec<(String, Vec<(String, ChartFix)>)>,
    pub brand_conflicts: Vec<(String, Vec<String>)>,
    /// 保留族映射，按真源顺序（accent / success / warning / danger / info）。
    pub reserved_map: Vec<(String, String)>,
}

impl Pipeline {
    pub fn base(&self) -> &Entry {
        &self.entries[self.base_idx]
    }

    /// `extensions.reading`（若存在）。
    pub fn reading_ext(&self) -> Option<&Value> {
        self.source
            .get("extensions")
            .and_then(Value::as_object)
            .and_then(|e| e.get("reading"))
    }

    /// 迭代「所有品牌 + 末尾的 `__base__`」，等价于 Python 的 `resolved_all.items()`。
    ///
    /// `标签` 复用 Python 的规则：`__base__` 的行显示成 `<默认品牌id>（默认）`。
    pub fn iter_with_base(&self) -> Vec<(&str, &Entry)> {
        let mut out: Vec<(&str, &Entry)> = self.entries.iter().map(|e| (e.id.as_str(), e)).collect();
        out.push(("__base__", self.base()));
        out
    }
}

pub fn build(source: Value, palette: &Value) -> Pipeline {
    let steps: Vec<i64> = {
        let ramp = obj(&source, "ramp");
        ramp["steps"].as_array().unwrap().iter().map(|x| x.as_i64().unwrap()).collect()
    };

    let (families, palette_flat) = build_families(&source, palette);

    let default_brand = arr(&source, "brands")
        .iter()
        .find(|b| b.get("defaultDevice").and_then(Value::as_bool).unwrap_or(false))
        .map(|b| s(b, "id").to_string())
        .expect("真源里没有标记 defaultDevice 的品牌");
    let accent_family = s(&source["reservedFamilies"]["map"], "accent").to_string();

    // 这两个值在旧实现里算出来传给了 role_set，但 role_set 并未使用。保留计算是为了
    // 真源里 `mo` / `su` 中性族缺失时能像旧实现一样立刻报错，而不是悄悄换个结果。
    let _ink_light = family_of(&families, "mo").step("900").hex.clone();
    let _ink_dark = family_of(&families, "su").step("50").hex.clone();

    let reserved_map: Vec<(String, String)> = obj(&source["reservedFamilies"], "map")
        .iter()
        .map(|(k, v)| (k.clone(), v.as_str().unwrap().to_string()))
        .collect();

    let mut entries: Vec<Entry> = Vec::new();
    let mut chart_fix: Vec<(String, Vec<(String, ChartFix)>)> = Vec::new();
    let mut brand_conflicts: Vec<(String, Vec<String>)> = Vec::new();
    let mut base_idx = 0usize;

    for brand in arr(&source, "brands") {
        let bid = s(brand, "id").to_string();
        let fam_id = s(brand, "family").to_string();
        let mut light: Option<Value> = None;
        let mut dark: Option<Value> = None;
        let mut roles_light = None;
        let mut roles_dark = None;

        for mode in ["light", "dark"] {
            let bg_base = deref(obj(&source["semantic"], mode)["bg"]["base"].as_str().unwrap(), &families);
            let (mut sem, fixes, roles) =
                build_semantic(&source, &families, mode, &fam_id, &accent_family, &steps, &bg_base);

            // 把自动修正后的图表色写回（同时更新 chart_fix 记录）
            let chart_map = sem.as_object_mut().unwrap();
            let chart = chart_map.get_mut("chart").unwrap().as_object_mut().unwrap();
            for (k, fix) in &fixes {
                *chart.get_mut(k).unwrap() = Value::String(fix.fixed.clone());
            }
            upsert_fix(&mut chart_fix, mode, fixes);

            if mode == "light" {
                light = Some(sem);
                roles_light = Some(roles);
            } else {
                dark = Some(sem);
                roles_dark = Some(roles);
            }
        }

        let mut clash: Vec<String> = reserved_map
            .iter()
            .filter(|(_, f)| *f == fam_id)
            .map(|(r, _)| r.clone())
            .collect();
        clash.sort();
        if !clash.is_empty() {
            brand_conflicts.push((bid.clone(), clash));
        }

        if bid == default_brand {
            base_idx = entries.len();
        }
        entries.push(Entry {
            id: bid,
            light: light.unwrap(),
            dark: dark.unwrap(),
            roles_light: roles_light.unwrap(),
            roles_dark: roles_dark.unwrap(),
        });
    }

    Pipeline {
        source,
        families,
        palette_flat,
        steps,
        entries,
        base_idx,
        default_brand,
        accent_family,
        chart_fix,
        brand_conflicts,
        reserved_map,
    }
}

fn family_of<'a>(families: &'a [Family], id: &str) -> &'a Family {
    families
        .iter()
        .find(|f| f.id == id)
        .unwrap_or_else(|| panic!("真源缺少中性色族 {id}"))
}

fn upsert_fix(
    chart_fix: &mut Vec<(String, Vec<(String, ChartFix)>)>,
    mode: &str,
    fixes: Vec<(String, ChartFix)>,
) {
    match chart_fix.iter_mut().find(|(m, _)| m == mode) {
        Some((_, items)) => *items = fixes,
        None => chart_fix.push((mode.to_string(), fixes)),
    }
}
