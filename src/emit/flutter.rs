//! Flutter 端：`gen/flutter/design_tokens.dart`

use crate::emit::dart_color;
use crate::model::{obj, s, sub};
use crate::pipeline::Pipeline;
use crate::util::{cam, capitalize, num};

pub fn emit_flutter(p: &Pipeline) -> String {
    let source = &p.source;
    let sc = obj(source, "scales");
    let name = s(&source["meta"], "name");
    let base = p.base();

    let mut l: Vec<String> = Vec::new();
    l.push(format!("// {name} —— Flutter 令牌（生成物，勿手改）"));
    l.push("import 'package:flutter/material.dart';".to_string());
    l.push(String::new());
    l.push("class DqCn {".to_string());
    for fam in &p.families {
        l.push(format!("  // {} {}", fam.name, fam.pinyin));
        for step in &p.steps {
            let hex = &fam.step(&step.to_string()).hex;
            l.push(format!("  static const Color {}{step} = Color(0xFF{});", fam.id, &hex[1..]));
        }
    }
    l.push("}".to_string());
    l.push(String::new());

    for mode in ["light", "dark"] {
        let cls = if mode == "light" { "DqLight" } else { "DqDark" };
        l.push(format!("class {cls} {{"));
        for (role, val) in base.sem(mode).as_object().unwrap() {
            if let Some(inner) = val.as_object() {
                for (k2, v2) in inner {
                    l.push(format!(
                        "  static const Color {}{} = {};",
                        cam(role),
                        capitalize(&cam(k2)),
                        dart_color(v2.as_str().unwrap())
                    ));
                }
            } else {
                l.push(format!(
                    "  static const Color {} = {};",
                    cam(role),
                    dart_color(val.as_str().unwrap())
                ));
            }
        }
        l.push("}".to_string());
        l.push(String::new());
    }

    l.push("class DqScale {".to_string());
    for (k, v) in sub(sc, "space") {
        l.push(format!("  static const double space{k} = {};", num(v)));
    }
    for (k, v) in sub(sc, "radius") {
        l.push(format!("  static const double radius{} = {};", capitalize(&cam(k)), num(v)));
    }
    for (k, v) in obj(&sc["typography"], "scale") {
        l.push(format!("  static const double font{} = {};", capitalize(&cam(k)), num(&v["size"])));
    }
    l.push("}".to_string());
    l.push(String::new());

    l.join("\n")
}
