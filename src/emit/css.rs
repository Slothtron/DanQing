//! Web 端：`gen/css/danqing.css`

use serde_json::Value;

use crate::emit::REDO_TIP;
use crate::model::{obj, s, sub};
use crate::pipeline::Pipeline;
use crate::util::num;

pub fn emit_css(p: &Pipeline) -> String {
    let source = &p.source;
    let prefix = s(&source["meta"], "prefix");
    let meta = &source["meta"];

    let mut l: Vec<String> = Vec::new();
    l.push(format!(
        "/* {} · {} v{} —— Web 令牌（生成物，勿手改；{}） */",
        s(meta, "name"),
        s(meta, "tagline"),
        s(meta, "version"),
        REDO_TIP
    ));
    l.push(String::new());
    l.push(":root {".to_string());
    l.push("  /* ========== tier1 原语 · 中国传统色族 ========== */".to_string());

    for fam in &p.families {
        l.push(format!(
            "  /* {} {} · 锚点 {} {} */",
            fam.name, fam.pinyin, fam.anchor_name, fam.anchor_hex
        ));
        for step in &p.steps {
            let key = step.to_string();
            let st = fam.step(&key);
            let marker = if st.anchor { " /* 锚点 */" } else { "" };
            l.push(format!("  --{prefix}-cn-{}-{key}: {};{marker}", fam.id, st.hex));
        }
    }

    let base = p.base();
    l.push(String::new());
    l.push("  /* ========== tier2 语义 · 浅色（默认） ========== */".to_string());
    for (role, val) in base.sem("light").as_object().unwrap() {
        if let Some(inner) = val.as_object() {
            for (k2, v2) in inner {
                l.push(format!("  --{prefix}-{role}-{k2}: {};", v2.as_str().unwrap()));
            }
        } else {
            l.push(format!("  --{prefix}-{role}: {};", val.as_str().unwrap()));
        }
    }

    let sc = obj(source, "scales");
    l.push(String::new());
    l.push("  /* ========== 尺度 ========== */".to_string());
    for (k, v) in sub(sc, "space") {
        l.push(format!("  --{prefix}-space-{k}: {}px;", num(v)));
    }
    for (k, v) in sub(sc, "radius") {
        l.push(format!("  --{prefix}-radius-{k}: {}px;", num(v)));
    }
    for (k, v) in sub(sc, "borderWidth") {
        l.push(format!("  --{prefix}-bw-{k}: {}px;", num(v)));
    }
    for (k, v) in sub(sc, "opacity") {
        l.push(format!("  --{prefix}-opacity-{k}: {};", num(v)));
    }
    for (k, v) in sub(sc, "zIndex") {
        l.push(format!("  --{prefix}-z-{k}: {};", num(v)));
    }
    for (k, v) in sub(sc, "icon") {
        l.push(format!("  --{prefix}-icon-{k}: {}px;", num(v)));
    }
    for (k, v) in sub(sc, "elevation") {
        l.push(format!("  --{prefix}-elev-{k}: {};", v.as_str().unwrap()));
    }
    let motion = sub(sc, "motion");
    for (k, v) in sub(motion, "duration") {
        l.push(format!("  --{prefix}-dur-{k}: {}ms;", num(v)));
    }
    for (k, v) in sub(motion, "easing") {
        l.push(format!("  --{prefix}-ease-{k}: {};", v.as_str().unwrap()));
    }
    for (k, v) in sub(sc, "container") {
        l.push(format!("  --{prefix}-container-{k}: {}px;", num(v)));
    }
    let typo = sub(sc, "typography");
    for (k, v) in sub(typo, "scale") {
        l.push(format!("  --{prefix}-font-{k}: {}px;", num(&v["size"])));
        l.push(format!("  --{prefix}-lh-{k}: {};", num(&v["lineHeight"])));
        l.push(format!("  --{prefix}-fw-{k}: {};", num(&v["weight"])));
    }
    for (k, v) in sub(typo, "family") {
        l.push(format!("  --{prefix}-family-{k}: {};", s(v, "stack")));
    }
    for (k, v) in sub(typo, "letterSpacing") {
        l.push(format!("  --{prefix}-tracking-{k}: {};", v.as_str().unwrap()));
    }
    l.push("}".to_string());

    // tier3 扩展：阅读纸色（可选引入）
    if let Some(ext) = p.reading_ext() {
        l.push(String::new());
        l.push("/* ========== tier3 扩展 ext.reading（可选 · 长文阅读） ========== */".to_string());
        for paper in ext["papers"].as_array().unwrap() {
            let pid = paper["id"].as_str().unwrap();
            l.push(format!(r#"[data-reading-paper="{pid}"] {{"#));
            l.push(format!("  --{prefix}-reading-bg: {};", paper["bg"].as_str().unwrap()));
            l.push(format!("  --{prefix}-reading-text: {};", paper["text"].as_str().unwrap()));
            l.push(format!("  --{prefix}-reading-sub: {};", paper["sub"].as_str().unwrap()));
            l.push(format!("  --{prefix}-reading-widget: {};", paper["widget"].as_str().unwrap()));
            l.push("}".to_string());
        }
        l.push(":root {".to_string());
        let fs = obj(ext, "fontSize");
        l.push(format!("  --{prefix}-reading-size-min: {}px;", num(&fs["min"])));
        l.push(format!("  --{prefix}-reading-size-max: {}px;", num(&fs["max"])));
        l.push(format!("  --{prefix}-reading-size-default: {}px;", num(&fs["default"])));
        for (k, v) in obj(ext, "measure") {
            l.push(format!("  --{prefix}-reading-measure-{k}: {}px;", num(v)));
        }
        for (k, v) in obj(ext, "lineHeight") {
            l.push(format!("  --{prefix}-reading-lh-{k}: {};", num(v)));
        }
        for (k, v) in obj(ext, "indent") {
            l.push(format!("  --{prefix}-reading-indent-{k}: {}em;", num(v)));
        }
        l.push("}".to_string());
    }

    // 品牌：浅色品牌块（必须排在浅色基座之后）
    l.push(String::new());
    l.push("  /* ========== 品牌主色（浅色） ========== */".to_string());
    for entry in &p.entries {
        l.push(format!(r#"[data-brand="{}"] {{"#, entry.id));
        for (k2, v2) in entry.sem("light")["primary"].as_object().unwrap() {
            l.push(format!("  --{prefix}-primary-{k2}: {};", v2.as_str().unwrap()));
        }
        l.push(format!("  --{prefix}-focus-ring: {};", s_focus(&entry.light)));
        l.push(format!("  --{prefix}-text-link: {};", s_link(&entry.light)));
        l.push("}".to_string());
    }

    // 深色基座：必须位于浅色品牌块之后，才能覆盖默认品牌
    l.push(String::new());
    l.push(r#"[data-theme="dark"] {"#.to_string());
    l.push("  /* ========== tier2 语义 · 深色 ========== */".to_string());
    for (role, val) in base.sem("dark").as_object().unwrap() {
        if let Some(inner) = val.as_object() {
            for (k2, v2) in inner {
                l.push(format!("  --{prefix}-{role}-{k2}: {};", v2.as_str().unwrap()));
            }
        } else {
            l.push(format!("  --{prefix}-{role}: {};", val.as_str().unwrap()));
        }
    }
    l.push("}".to_string());

    l.push(String::new());
    l.push("  /* ========== 品牌主色（深色，复合选择器保证优先级） ========== */".to_string());
    for entry in &p.entries {
        l.push(format!(
            r#"[data-theme="dark"][data-brand="{}"] {{"#,
            entry.id
        ));
        for (k2, v2) in entry.sem("dark")["primary"].as_object().unwrap() {
            l.push(format!("  --{prefix}-primary-{k2}: {};", v2.as_str().unwrap()));
        }
        l.push(format!("  --{prefix}-focus-ring: {};", s_focus(&entry.dark)));
        l.push(format!("  --{prefix}-text-link: {};", s_link(&entry.dark)));
        l.push("}".to_string());
    }

    l.push(String::new());
    l.push("@media (prefers-reduced-motion: reduce) {".to_string());
    l.push("  :root {".to_string());
    for k in obj(&sc["motion"], "duration").keys() {
        l.push(format!("    --{prefix}-dur-{k}: 0ms;"));
    }
    // 旧实现这里漏写了 f 前缀，产物里是一行永远不会生效的 `--{p}-dur-normal`。
    // Rust 化时一并修正：reduced-motion 下其余动效归零，仅保留 ≤80ms 供颜色淡变。
    l.push(format!("    --{prefix}-dur-normal: 80ms;"));
    l.push("  }".to_string());
    l.push("}".to_string());
    l.push(String::new());

    l.join("\n")
}

fn s_focus(sem: &Value) -> String {
    s(&sem["focus"], "ring").to_string()
}

fn s_link(sem: &Value) -> String {
    s(&sem["text"], "link").to_string()
}

#[cfg(test)]
mod tests {
    use super::emit_css;
    use crate::pipeline;
    use serde_json::Value;

    fn digits_of_anchor(s: &str) -> usize {
        s.matches("#").count()
    }

    #[test]
    fn css_contains_one_var_per_primitive() {
        let source: Value = serde_json::from_str(
            &std::fs::read_to_string("tokens/source.json").unwrap(),
        )
        .unwrap();
        let palette: Value = serde_json::from_str(
            &std::fs::read_to_string("data/chinese-colors.json").unwrap(),
        )
        .unwrap();
        let p = pipeline::build(source, &palette);
        let css = emit_css(&p);
        // 121 个原语各一行 `--dq-cn-<族>-<级>`
        assert_eq!(css.matches("--dq-cn-").count(), 121);
        for fam in &p.families {
            for (step, st) in &fam.steps {
                assert!(css.contains(&format!("--dq-cn-{}-{step}: {};", fam.id, st.hex)));
            }
        }
        assert!(digits_of_anchor(&css) > 0);
    }
}
