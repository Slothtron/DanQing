//! W3C DTCG 端：`gen/json/danqing.tokens.json`
//!
//! 可被 Style Dictionary / Figma Tokens 直接消费。

use serde_json::{json, Map, Value};

use crate::model::{obj, s};
use crate::pipeline::Pipeline;

const DIM_UNITS: [&str; 6] = ["px", "ms", "em", "rem", "%", "s"];

/// 去掉开头的 `-?\d+(?:\.\d+)?`，剩下的就是单位。等价于 Python 的
/// `re.sub(r"^-?\d+(?:\.\d+)?", "", v)`。注意必须连小数部分一起吃掉，
/// 否则 `-0.01em` 会退化成 `.01em`。
fn strip_numeric(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut i = usize::from(bytes.first() == Some(&b'-'));
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        i += 1;
    }
    if i < bytes.len() && bytes[i] == b'.' {
        let mut j = i + 1;
        while j < bytes.len() && bytes[j].is_ascii_digit() {
            j += 1;
        }
        i = j;
    }
    s[i..].to_string()
}

fn dim_match(v: &str) -> bool {
    let body = v.strip_prefix('-').unwrap_or(v);
    let pos = body.find(|c: char| !c.is_ascii_digit() && c != '.');
    let Some(pos) = pos else { return false };
    let (num_part, unit) = body.split_at(pos);
    if !DIM_UNITS.contains(&unit) {
        return false;
    }
    if num_part.is_empty() {
        return false;
    }
    let digits = num_part.split('.').collect::<Vec<_>>();
    if digits.len() > 2 {
        return false;
    }
    if digits[0].is_empty() {
        return false;
    }
    digits.iter().all(|d| d.chars().all(|c| c.is_ascii_digit()))
}

fn leaf(v: &Value) -> Value {
    if v.is_array() {
        return json!({ "$value": v.clone() });
    }
    if let Some(str) = v.as_str() {
        if str.starts_with('#') || str.starts_with("rgb") {
            return json!({ "$value": str, "$type": "color" });
        }
        if let Some(rest) = str.strip_prefix("cubic-bezier") {
            let start = rest.find('(').unwrap() + 1;
            let end = rest.rfind(')').unwrap();
            let nums: Vec<Value> = rest[start..end]
                .split(',')
                .map(|x| serde_json::Number::from_f64(x.trim().parse::<f64>().unwrap()).map(Value::Number).unwrap())
                .collect();
            return json!({ "$value": nums, "$type": "cubicBezier" });
        }
        if dim_match(str) {
            let unit = strip_numeric(str);
            let value: f64 = str[..str.len() - unit.len()].parse().unwrap();
            return json!({ "$value": { "value": value, "unit": unit }, "$type": "dimension" });
        }
        return json!({ "$value": str });
    }
    json!({ "$value": v.clone() })
}

fn wrap(d: &Map<String, Value>) -> Value {
    let mut out = Map::new();
    for (k, v) in d.iter() {
        out.insert(k.clone(), if let Some(inner) = v.as_object() { wrap(inner) } else { leaf(v) });
    }
    Value::Object(out)
}

pub fn emit_dtcg(p: &Pipeline) -> String {
    let source = &p.source;
    let meta = &source["meta"];
    let base = p.base();

    let mut doc = Map::new();
    doc.insert(
        "$description".to_string(),
        Value::String(format!(
            "{} · {} · W3C DTCG 格式（生成物，勿手改）",
            s(meta, "name"),
            s(meta, "tagline")
        )),
    );
    doc.insert(
        "$extensions".to_string(),
        json!({
            "danqing": {
                "version": s(meta, "version"),
                "generator": crate::emit::GENERATOR,
                "palette": source["meta"]["palette"]["source"]
            }
        }),
    );

    let mut sys = Map::new();
    sys.insert("light".to_string(), wrap(base.sem("light").as_object().unwrap()));
    sys.insert("dark".to_string(), wrap(base.sem("dark").as_object().unwrap()));

    let mut cn = Map::new();
    for fam in &p.families {
        let mut group = Map::new();
        group.insert(
            "$description".to_string(),
            Value::String(format!(
                "{} {} · 锚点 {} {}（第 {} 级）· {}",
                fam.name, fam.pinyin, fam.anchor_name, fam.anchor_hex, fam.anchor_step, fam.intent
            )),
        );
        for step in &p.steps {
            let key = step.to_string();
            let st = fam.step(&key);
            group.insert(
                key,
                json!({
                    "$value": st.hex,
                    "$type": "color",
                    "$description": format!("{} {}", st.name, st.pinyin)
                        + if st.anchor { "　◀ 锚点" } else { "" }
                }),
            );
        }
        cn.insert(fam.id.clone(), Value::Object(group));
    }

    let mut brand = Map::new();
    for entry in &p.entries {
        let mut per_mode = Map::new();
        for mode in ["light", "dark"] {
            let mut inner = Map::new();
            inner.insert("primary".to_string(), entry.sem(mode)["primary"].clone());
            inner.insert(
                "focus".to_string(),
                json!({ "ring": entry.sem(mode)["focus"]["ring"].clone() }),
            );
            per_mode.insert(mode.to_string(), wrap(&inner));
        }
        brand.insert(entry.id.clone(), Value::Object(per_mode));
    }

    let mut scale = Map::new();
    for (group, gv) in obj(source, "scales") {
        if group == "breakpoint" {
            continue;
        }
        scale.insert(
            group.clone(),
            if let Some(d) = gv.as_object() { wrap(d) } else { leaf(gv) },
        );
    }

    let mut ext = Map::new();
    if let Some(reading) = p.reading_ext() {
        let mut papers = Map::new();
        for paper in reading["papers"].as_array().unwrap() {
            let mut inner = Map::new();
            for key in ["bg", "text", "sub", "widget"] {
                inner.insert(key.to_string(), paper[key].clone());
            }
            papers.insert(paper["id"].as_str().unwrap().to_string(), wrap(&inner));
        }
        let mut reading_out = Map::new();
        reading_out.insert("papers".to_string(), Value::Object(papers));
        for (k, v) in reading.as_object().unwrap() {
            if let Some(d) = v.as_object() {
                reading_out.insert(k.clone(), wrap(d));
            }
        }
        ext.insert("reading".to_string(), Value::Object(reading_out));
    }

    doc.insert("cn".to_string(), Value::Object(cn));
    doc.insert("sys".to_string(), Value::Object(sys));
    doc.insert("brand".to_string(), Value::Object(brand));
    doc.insert("scale".to_string(), Value::Object(scale));
    doc.insert("ext".to_string(), Value::Object(ext));

    format!("{}\n", serde_json::to_string_pretty(&Value::Object(doc)).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dimension_regex_behaves_like_python() {
        assert!(dim_match("16px"));
        assert!(dim_match("-0.01em"));
        assert!(!dim_match("2.0"));
        assert!(!dim_match("cubic-bezier(0.2,0,0,1)"));
        assert!(!dim_match("none"));
        assert!(strip_numeric("-0.01em") == "em");
    }
}
