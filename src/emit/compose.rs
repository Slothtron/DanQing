//! Jetpack Compose 端：`gen/compose/DesignTokens.kt`

use crate::emit::kt_color;
use crate::model::{flat_semantic, obj, s, sub};
use crate::pipeline::Pipeline;
use crate::util::{cam, capitalize, num};

pub fn emit_compose(p: &Pipeline) -> String {
    let source = &p.source;
    let sc = obj(source, "scales");
    let name = s(&source["meta"], "name");
    let base = p.base();

    let mut l: Vec<String> = Vec::new();
    l.push(format!("// {name} —— Jetpack Compose 令牌（生成物，勿手改）"));
    l.push("package design.danqing.tokens".to_string());
    l.push(String::new());
    l.push("import androidx.compose.ui.graphics.Color".to_string());
    l.push("import androidx.compose.ui.unit.dp".to_string());
    l.push("import androidx.compose.ui.unit.sp".to_string());
    l.push(String::new());
    l.push("object Dq {".to_string());
    l.push(String::new());

    l.push("  // tier1 原语".to_string());
    for fam in &p.families {
        l.push(format!(
            "  /** {} {} · 锚点 {} {} */",
            fam.name, fam.pinyin, fam.anchor_name, fam.anchor_hex
        ));
        l.push(format!("  object {} {{", capitalize(&fam.id)));
        for step in &p.steps {
            let hex = &fam.step(&step.to_string()).hex;
            l.push(format!("    val s{step} = Color(0xFF{})", &hex[1..]));
        }
        l.push("  }".to_string());
    }

    for mode in ["light", "dark"] {
        l.push(format!("  // tier2 语义 · {mode}"));
        l.push(format!("  object Semantic{} {{", capitalize(mode)));
        for (k, v) in flat_semantic(base.sem(mode)) {
            l.push(format!("    val {} = {}", cam(&k), kt_color(&v)));
        }
        l.push("  }".to_string());
    }

    l.push("  // 品牌主色".to_string());
    l.push("  object Brand {".to_string());
    for entry in &p.entries {
        l.push(format!("    object {} {{", capitalize(&entry.id)));
        for mode in ["light", "dark"] {
            for (k2, v2) in entry.sem(mode)["primary"].as_object().unwrap() {
                l.push(format!(
                    "      val {mode}{} = {}",
                    capitalize(k2),
                    kt_color(v2.as_str().unwrap())
                ));
            }
        }
        l.push("    }".to_string());
    }
    l.push("  }".to_string());

    l.push("  // 尺度".to_string());
    for (k, v) in sub(sc, "space") {
        l.push(format!("  val space{k} = {}.dp", num(v)));
    }
    for (k, v) in sub(sc, "radius") {
        l.push(format!("  val radius{} = {}.dp", capitalize(&cam(k)), num(v)));
    }
    for (k, v) in obj(&sc["typography"], "scale") {
        l.push(format!("  val fontSize{} = {}.sp", capitalize(&cam(k)), num(&v["size"])));
    }
    l.push(format!("  val touchMin = {}.dp", num(&sc["touch"]["min"])));
    for (k, v) in obj(&sc["motion"], "duration") {
        l.push(format!("  const val duration{} = {}", capitalize(&cam(k)), num(v)));
    }
    l.push("}".to_string());
    l.push(String::new());

    l.join("\n")
}
