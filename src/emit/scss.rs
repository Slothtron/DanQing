//! Sass 端：`gen/scss/_danqing.scss`

use crate::model::{obj, s, sub};
use crate::pipeline::Pipeline;
use crate::util::num;

pub fn emit_scss(p: &Pipeline) -> String {
    let source = &p.source;
    let prefix = s(&source["meta"], "prefix");
    let name = s(&source["meta"], "name");
    let sc = obj(source, "scales");
    let base = p.base();

    let mut l: Vec<String> = Vec::new();
    l.push(format!("// {name} —— SCSS 令牌（生成物，勿手改）"));
    l.push(r#"@use "sass:map";"#.to_string());
    l.push(String::new());
    l.push(format!("${prefix}-cn: ("));
    for fam in &p.families {
        l.push(format!(r#"  "{}": ("#, fam.id));
        for step in &p.steps {
            l.push(format!(r#"    "{step}": {},"#, fam.step(&step.to_string()).hex));
        }
        l.push("  ),".to_string());
    }
    l.push(") !default;".to_string());
    l.push(String::new());

    for mode in ["light", "dark"] {
        l.push(format!("${prefix}-{mode}: ("));
        for (role, val) in base.sem(mode).as_object().unwrap() {
            if let Some(inner) = val.as_object() {
                l.push(format!(r#"  "{role}": ("#));
                for (k2, v2) in inner {
                    l.push(format!(r#"    "{k2}": {},"#, v2.as_str().unwrap()));
                }
                l.push("  ),".to_string());
            } else {
                l.push(format!(r#"  "{role}": {},"#, val.as_str().unwrap()));
            }
        }
        l.push(") !default;".to_string());
        l.push(String::new());
    }

    for (group, suffix) in [("space", "px"), ("radius", "px"), ("breakpoint", "px")] {
        l.push(format!("${prefix}-{group}: ("));
        if group == "breakpoint" {
            for (k, v) in sub(sc, "breakpoint") {
                let key = if v.as_object().unwrap().contains_key("max") {
                    &v["max"]
                } else {
                    &v["min"]
                };
                l.push(format!(r#"  "{k}": {}px,"#, num(key)));
            }
        } else {
            for (k, v) in sub(sc, group) {
                l.push(format!(r#"  "{k}": {}{suffix},"#, num(v)));
            }
        }
        l.push(") !default;".to_string());
        l.push(String::new());
    }

    l.push("// 便捷函数".to_string());
    l.push(format!("@function {prefix}-color($role, $variant: default, $mode: light) {{"));
    l.push(format!(r#"  $m: map.get(${prefix}-light, $role);"#));
    l.push("  @if type-of($m) == map { @return map.get($m, $variant); }".to_string());
    l.push("  @return $m;".to_string());
    l.push("}".to_string());
    l.push(String::new());

    l.join("\n")
}
