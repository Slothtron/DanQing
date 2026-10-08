//! 展示页数据源：`showcase/data.js`
//!
//! 展示页（`showcase/index.html`）不持有任何色值，全部从这个生成文件读。
//! 这样「改了真源忘了更新展示页」在数据层面就不可能发生。

use serde_json::{json, Map, Value};

use crate::emit::REDO_TIP;
use crate::gate::Check;
use crate::model::{arr, s};
use crate::pipeline::Pipeline;

pub fn emit_showcase_data(p: &Pipeline, checks: &[Check]) -> String {
    let source = &p.source;
    let base = p.base();

    // tier1 色族
    let reserved_ids: Vec<&str> = p.reserved_map.iter().map(|(_, f)| f.as_str()).collect();
    let mut fam_list: Vec<Value> = Vec::new();
    for fam in &p.families {
        let mut steps = Vec::new();
        for step in &p.steps {
            let st = fam.step(&step.to_string());
            let mut m = Map::new();
            m.insert("step".to_string(), Value::String(step.to_string()));
            m.insert("hex".to_string(), Value::String(st.hex.clone()));
            m.insert("name".to_string(), Value::String(st.name.clone()));
            m.insert("pinyin".to_string(), Value::String(st.pinyin.clone()));
            m.insert("nearestHex".to_string(), Value::String(st.nearest_hex.clone()));
            m.insert("nearestGroup".to_string(), Value::String(st.nearest_group.clone()));
            m.insert("anchor".to_string(), Value::Bool(st.anchor));
            steps.push(Value::Object(m));
        }
        fam_list.push(json!({
            "id": fam.id,
            "name": fam.name,
            "pinyin": fam.pinyin,
            "hue": fam.hue,
            "intent": fam.intent,
            "connotation": fam.connotation,
            "anchorName": fam.anchor_name,
            "anchorPinyin": fam.anchor_pinyin,
            "anchorHex": fam.anchor_hex,
            "anchorStep": fam.anchor_step,
            "alternatives": fam.alternatives,
            "reserved": reserved_ids.contains(&fam.id.as_str()),
            "steps": steps,
        }));
    }

    // 品牌
    let mut brand_list: Vec<Value> = Vec::new();
    for b in arr(source, "brands") {
        let bid = s(b, "id");
        let entry = p.entries.iter().find(|e| e.id == bid).unwrap();
        let fam_name = &p.families.iter().find(|f| f.id == s(b, "family")).unwrap().name;
        let conflict = p
            .brand_conflicts
            .iter()
            .find(|(id, _)| id == bid)
            .map(|(_, roles)| roles.clone())
            .unwrap_or_default();
        brand_list.push(json!({
            "id": bid,
            "name": b["name"],
            "pinyin": b["pinyin"],
            "desc": b.get("desc").and_then(Value::as_str).unwrap_or(""),
            "audience": b.get("audience").and_then(Value::as_str).unwrap_or(""),
            "default": b.get("defaultDevice").and_then(Value::as_bool).unwrap_or(false),
            "family": s(b, "family"),
            "familyName": fam_name,
            "light": entry.sem("light")["primary"].clone(),
            "dark": entry.sem("dark")["primary"].clone(),
            "conflict": conflict,
        }));
    }

    // 门禁分组
    let mut groups: Vec<(&str, Vec<Value>)> = Vec::new();
    let mut order: Vec<&str> = Vec::new();
    for c in checks {
        if !order.contains(&c.group.as_str()) {
            order.push(c.group.as_str());
            groups.push((c.group.as_str(), Vec::new()));
        }
        let slot = groups.iter_mut().find(|(g, _)| *g == c.group.as_str()).unwrap();
        slot.1.push(json!({
            "label": c.label,
            "ratio": c.ratio,
            "req": c.req,
            "ok": c.ok(),
        }));
    }

    let chart_fix: Vec<(String, Value)> = p
        .chart_fix
        .iter()
        .map(|(mode, items)| {
            let mut m = Map::new();
            for (k, fix) in items {
                m.insert(k.clone(), json!({ "raw": fix.raw, "fixed": fix.fixed }));
            }
            (mode.clone(), Value::Object(m))
        })
        .collect();
    let mut chart_fix_map = Map::new();
    for (mode, v) in chart_fix {
        chart_fix_map.insert(mode, v);
    }

    let mut semantic = Map::new();
    semantic.insert("light".to_string(), base.sem("light").clone());
    semantic.insert("dark".to_string(), base.sem("dark").clone());

    let data = json!({
        "meta": source["meta"],
        "steps": p.steps,
        "families": fam_list,
        "brands": brand_list,
        "defaultBrand": p.default_brand,
        "reserved": source["reservedFamilies"],
        "semantic": semantic,
        "scales": source["scales"],
        "gates": {
            "total": checks.len(),
            "failed": checks.iter().filter(|c| !c.ok()).count(),
            "groups": groups.iter().map(|(name, items)| json!({ "name": name, "items": items })).collect::<Vec<_>>(),
        },
        "chartFix": chart_fix_map,
    });

    let payload = serde_json::to_string(&data).unwrap();
    format!(
        "// 丹青 · 展示页数据（生成物，勿手改；{}）\nwindow.DQ = {};\n",
        REDO_TIP, payload
    )
}
