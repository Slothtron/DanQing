//! Tailwind 预设：`gen/tailwind/danqing.preset.cjs`

use crate::model::{flat_semantic, obj, s, sub};
use crate::pipeline::Pipeline;
use crate::util::num;

pub fn emit_tailwind(p: &Pipeline) -> String {
    let source = &p.source;
    let sc = obj(source, "scales");
    let name = s(&source["meta"], "name");
    let base = p.base();

    let mut l: Vec<String> = Vec::new();
    l.push(format!("// {name} —— Tailwind 预设（生成物，勿手改）"));
    l.push("/** @type {import('tailwindcss').Config} */".to_string());
    l.push("module.exports = {".to_string());
    l.push("  theme: {".to_string());
    l.push("    extend: {".to_string());
    l.push("      colors: {".to_string());
    l.push("        // tier1 原语".to_string());
    for fam in &p.families {
        l.push(format!(r#"        "{}": {{"#, fam.id));
        for step in &p.steps {
            l.push(format!(r#"          {step}: "{}","#, fam.step(&step.to_string()).hex));
        }
        l.push("        },".to_string());
    }
    l.push("        // tier2 语义（浅色；深色请用 CSS 变量或 darkMode 覆盖）".to_string());
    for (k, v) in flat_semantic(base.sem("light")) {
        l.push(format!(
            r#"        "{}": "var(--dq-{k}, {v})","#,
            k.replace('-', ".")
        ));
    }
    l.push("      },".to_string());

    l.push("      spacing: {".to_string());
    for (k, v) in sub(sc, "space") {
        l.push(format!(r#"        "{k}": "{}px","#, num(v)));
    }
    l.push("      },".to_string());

    l.push("      borderRadius: {".to_string());
    for (k, v) in sub(sc, "radius") {
        l.push(format!(r#"        "{k}": "{}px","#, num(v)));
    }
    l.push("      },".to_string());

    l.push("      boxShadow: {".to_string());
    for (k, v) in sub(sc, "elevation") {
        l.push(format!(r#"        "{k}": "{}","#, v.as_str().unwrap()));
    }
    l.push("      },".to_string());

    l.push("      transitionDuration: {".to_string());
    for (k, v) in obj(&sc["motion"], "duration") {
        l.push(format!(r#"        "{k}": "{}ms","#, num(v)));
    }
    l.push("      },".to_string());

    l.push("      transitionTimingFunction: {".to_string());
    for (k, v) in obj(&sc["motion"], "easing") {
        l.push(format!(r#"        "{k}": "{}","#, v.as_str().unwrap()));
    }
    l.push("      },".to_string());

    l.push("      fontSize: {".to_string());
    for (k, v) in obj(&sc["typography"], "scale") {
        l.push(format!(
            r#"        "{k}": ["{}px", {{ lineHeight: "{}", fontWeight: "{}" }}],"#,
            num(&v["size"]),
            num(&v["lineHeight"]),
            num(&v["weight"])
        ));
    }
    l.push("      },".to_string());

    l.push("      screens: {".to_string());
    for (k, v) in sub(sc, "breakpoint") {
        let key = if v.as_object().unwrap().contains_key("min") {
            &v["min"]
        } else {
            &v["max"]
        };
        l.push(format!(r#"        "{k}": "{}px","#, num(key)));
    }
    l.push("      },".to_string());
    l.push("    },".to_string());
    l.push("  },".to_string());
    l.push("};".to_string());
    l.push(String::new());

    l.join("\n")
}
