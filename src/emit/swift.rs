//! SwiftUI 端：`gen/swift/DesignTokens.swift`

use crate::emit::android_color;
use crate::model::{flat_semantic, obj, s, sub};
use crate::pipeline::Pipeline;
use crate::util::{cam, capitalize, fstr, num};

pub fn emit_swift(p: &Pipeline) -> String {
    let source = &p.source;
    let sc = obj(source, "scales");
    let name = s(&source["meta"], "name");
    let base = p.base();

    let mut l: Vec<String> = Vec::new();
    l.push(format!("// {name} —— SwiftUI 令牌（生成物，勿手改）"));
    l.push("// 建议：把语义色导出为 Assets.xcassets 颜色集以获得自动 Appearance 切换；".to_string());
    l.push("// 下方语义枚举值即颜色集的字面值。".to_string());
    l.push("import SwiftUI".to_string());
    l.push(String::new());
    l.push("public enum Dq {".to_string());
    l.push(String::new());

    l.push("  // MARK: tier1 原语 · 中国传统色族".to_string());
    for fam in &p.families {
        l.push(format!(
            "  /// {} {} · 锚点 {} {}",
            fam.name, fam.pinyin, fam.anchor_name, fam.anchor_hex
        ));
        l.push(format!("  public enum {} {{", capitalize(&fam.id)));
        for step in &p.steps {
            let st = fam.step(&step.to_string());
            l.push(format!(r#"    public static let s{step} = Color(hex: "{}")  // {}"#, st.hex, st.name));
        }
        l.push("  }".to_string());
    }

    for mode in ["light", "dark"] {
        l.push(format!("  // MARK: tier2 语义 · {mode}"));
        l.push(format!("  public enum Semantic{} {{", capitalize(mode)));
        for (k, v) in flat_semantic(base.sem(mode)) {
            l.push(format!(
                r#"    public static let {} = Color(hex: "{}")"#,
                cam(&k),
                android_color(&v)
            ));
        }
        l.push("  }".to_string());
    }

    l.push("  // MARK: 品牌主色".to_string());
    l.push("  public enum Brand {".to_string());
    for entry in &p.entries {
        l.push(format!("    public enum {} {{", capitalize(&entry.id)));
        for mode in ["light", "dark"] {
            for (k2, v2) in entry.sem(mode)["primary"].as_object().unwrap() {
                l.push(format!(
                    r#"      public static let {mode}{} = Color(hex: "{}")"#,
                    capitalize(k2),
                    v2.as_str().unwrap()
                ));
            }
        }
        l.push("    }".to_string());
    }
    l.push("  }".to_string());

    l.push("  // MARK: 尺度".to_string());
    for (k, v) in sub(sc, "space") {
        l.push(format!("  public static let space{k}: CGFloat = {}", num(v)));
    }
    for (k, v) in sub(sc, "radius") {
        l.push(format!("  public static let radius{}: CGFloat = {}", capitalize(&cam(k)), num(v)));
    }
    for (k, v) in sub(sc, "icon") {
        l.push(format!("  public static let icon{}: CGFloat = {}", capitalize(&cam(k)), num(v)));
    }
    l.push(format!(
        "  public static let touchMin: CGFloat = {}",
        num(&sc["touch"]["min"])
    ));
    for (k, v) in obj(&sc["motion"], "duration") {
        l.push(format!(
            "  public static let duration{}: Double = {}",
            capitalize(&cam(k)),
            fstr(numf_owned(v) / 1000.0)
        ));
    }
    l.push("}".to_string());
    l.push(String::new());

    l.push("public extension Color {".to_string());
    l.push("  init(hex: String) {".to_string());
    l.push(r##"    let s = hex.hasPrefix("#") ? String(hex.dropFirst()) : hex"##.to_string());
    l.push("    var v: UInt64 = 0".to_string());
    l.push("    Scanner(string: s).scanHexInt64(&v)".to_string());
    l.push(r#"    let a = s.count == 8 ? Double((v >> 24) & 0xFF) / 255 : 1"#.to_string());
    l.push(r#"    let r = Double((v >> 16) & 0xFF) / 255"#.to_string());
    l.push(r#"    let g = Double((v >> 8) & 0xFF) / 255"#.to_string());
    l.push(r#"    let b = Double(v & 0xFF) / 255"#.to_string());
    l.push(r#"    self.init(.sRGB, red: r, green: g, blue: b, opacity: a)"#.to_string());
    l.push("  }".to_string());
    l.push("}".to_string());
    l.push(String::new());

    l.join("\n")
}

fn numf_owned(v: &serde_json::Value) -> f64 {
    crate::util::numf(v)
}
