//! Android 端：`gen/android/colors.xml` 与 `gen/android/dimens.xml`

use crate::emit::android_color;
use crate::model::{flat_semantic, obj, s, sub};
use crate::pipeline::Pipeline;
use crate::util::num;

pub fn emit_android(p: &Pipeline) -> (String, String) {
    let source = &p.source;
    let sc = obj(source, "scales");
    let prefix = s(&source["meta"], "prefix");
    let name = s(&source["meta"], "name");
    let base = p.base();

    let mut cols: Vec<String> = Vec::new();
    cols.push(r#"<?xml version="1.0" encoding="utf-8"?>"#.to_string());
    cols.push(format!("<!-- {name} · 颜色（生成物，勿手改） -->"));
    cols.push("<resources>".to_string());
    for fam in &p.families {
        cols.push(format!("  <!-- {} {} · 锚点 {} -->", fam.name, fam.pinyin, fam.anchor_name));
        for step in &p.steps {
            cols.push(format!(
                r#"  <color name="{prefix}_cn_{}_{step}">{}</color>"#,
                fam.id,
                fam.step(&step.to_string()).hex
            ));
        }
    }
    for mode in ["light", "dark"] {
        let suf = if mode == "light" { "" } else { "_dark" };
        cols.push(format!("  <!-- 语义 · {mode} -->"));
        for (k, v) in flat_semantic(base.sem(mode)) {
            cols.push(format!(
                r#"  <color name="{prefix}_{}{suf}">{}</color>"#,
                k.replace('-', "_"),
                android_color(&v)
            ));
        }
    }
    cols.push("</resources>".to_string());

    let mut dims: Vec<String> = Vec::new();
    dims.push(r#"<?xml version="1.0" encoding="utf-8"?>"#.to_string());
    dims.push(format!("<!-- {name} · 尺度（生成物，勿手改） -->"));
    dims.push("<resources>".to_string());
    for (k, v) in sub(sc, "space") {
        dims.push(format!(r#"  <dimen name="{prefix}_space_{k}">{}dp</dimen>"#, num(v)));
    }
    for (k, v) in sub(sc, "radius") {
        dims.push(format!(r#"  <dimen name="{prefix}_radius_{k}">{}dp</dimen>"#, num(v)));
    }
    for (k, v) in obj(&sc["typography"], "scale") {
        dims.push(format!(r#"  <dimen name="{prefix}_font_{k}">{}sp</dimen>"#, num(&v["size"])));
    }
    dims.push(format!(
        r#"  <dimen name="{prefix}_touch_min">{}dp</dimen>"#,
        num(&sc["touch"]["min"])
    ));
    dims.push("</resources>".to_string());

    (format!("{}\n", cols.join("\n")), format!("{}\n", dims.join("\n")))
}
