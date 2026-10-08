//! TypeScript 端：`gen/ts/danqing.ts`

use crate::model::{obj, s, sub};
use crate::pipeline::Pipeline;
use crate::util::{jlabel, num};

fn jl(k: &str) -> String {
    jlabel(k)
}

pub fn emit_ts(p: &Pipeline) -> String {
    let source = &p.source;
    let name = s(&source["meta"], "name");
    let sc = obj(source, "scales");
    let base = p.base();

    let mut l: Vec<String> = Vec::new();
    l.push(format!("// {name} —— TypeScript 令牌（生成物，勿手改）"));
    l.push("/* eslint-disable */".to_string());
    l.push(String::new());
    l.push("export const cn = {".to_string());
    for fam in &p.families {
        l.push(format!("  {}: {{", jl(&fam.id)));
        for step in &p.steps {
            let key = step.to_string();
            let st = fam.step(&key);
            l.push(format!(
                "    {}: {},   // {} {}",
                jl(&key),
                jl(&st.hex),
                st.name,
                st.pinyin
            ));
        }
        l.push("  },".to_string());
    }
    l.push("} as const;".to_string());
    l.push(String::new());

    for mode in ["light", "dark"] {
        l.push(format!("export const {mode} = {{"));
        for (role, val) in base.sem(mode).as_object().unwrap() {
            if let Some(inner) = val.as_object() {
                l.push(format!("  {role}: {{"));
                for (k2, v2) in inner {
                    l.push(format!("    {k2}: {},", jl(v2.as_str().unwrap())));
                }
                l.push("  },".to_string());
            } else {
                l.push(format!("  {role}: {},", jl(val.as_str().unwrap())));
            }
        }
        l.push("} as const;".to_string());
        l.push(String::new());
    }

    l.push("export const brands = {".to_string());
    for entry in &p.entries {
        l.push(format!("  {}: {{", jl(&entry.id)));
        for mode in ["light", "dark"] {
            l.push(format!("    {mode}: {{ primary: {{"));
            for (k2, v2) in entry.sem(mode)["primary"].as_object().unwrap() {
                l.push(format!("      {k2}: {},", jl(v2.as_str().unwrap())));
            }
            l.push("    } },".to_string());
        }
        l.push("  },".to_string());
    }
    l.push("} as const;".to_string());
    l.push(String::new());

    let pairs: Vec<(&str, Vec<(String, String)>)> = vec![
        ("space", sub(sc, "space").iter().map(|(k, v)| (k.clone(), num(v))).collect()),
        ("radius", sub(sc, "radius").iter().map(|(k, v)| (k.clone(), num(v))).collect()),
        (
            "duration",
            obj(&sc["motion"], "duration")
                .iter()
                .map(|(k, v)| (k.clone(), num(v)))
                .collect(),
        ),
        (
            "easing",
            obj(&sc["motion"], "easing")
                .iter()
                .map(|(k, v)| (k.clone(), jl(v.as_str().unwrap())))
                .collect(),
        ),
        (
            "elevation",
            sub(sc, "elevation")
                .iter()
                .map(|(k, v)| (k.clone(), jl(v.as_str().unwrap())))
                .collect(),
        ),
        (
            "fontSize",
            obj(&sc["typography"], "scale")
                .iter()
                .map(|(k, v)| (k.clone(), num(&v["size"])))
                .collect(),
        ),
        (
            "breakpoint",
            sub(sc, "breakpoint")
                .iter()
                .map(|(k, v)| {
                    let key = if v.as_object().unwrap().contains_key("max") {
                        &v["max"]
                    } else {
                        &v["min"]
                    };
                    (k.clone(), num(key))
                })
                .collect(),
        ),
    ];
    for (group, items) in pairs {
        l.push(format!("export const {group} = {{"));
        for (k, v) in items {
            l.push(format!("  {}: {v},", jl(&k)));
        }
        l.push("} as const;".to_string());
    }

    l.push(String::new());
    l.push("export type BrandId = keyof typeof brands;".to_string());
    l.push("export type SemanticRole = keyof typeof light;".to_string());
    l.push(String::new());

    l.join("\n")
}
