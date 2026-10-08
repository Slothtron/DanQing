//! 门禁：WCAG 对比度。失败即让整个构建非零退出。
//!
//! 关键点：**所有前景色的 alpha 都先合成到背景上再算**。直接用原始 alpha 值去算
//! 会得到「看上去能过、实际压在界面上读不清」的结果——这也是旧实现的做法，
//! 移植时保持不变。

use serde_json::Value;

use crate::color::contrast;
use crate::model::Family;
use crate::pipeline::Pipeline;

pub struct Check {
    pub group: String,
    pub label: String,
    pub ratio: f64,
    pub req: f64,
}

impl Check {
    pub fn ok(&self) -> bool {
        self.ratio >= self.req
    }
}

pub fn run_gates(pipeline: &Pipeline, _families: &[Family], _steps: &[i64]) -> Vec<Check> {
    use crate::util::py_round2;

    let mut checks = Vec::new();
    let base = &pipeline.entries[pipeline.base_idx];

    macro_rules! need {
        ($label:expr, $fg:expr, $bg:expr, $req:expr, $group:expr) => {
            checks.push(Check {
                group: $group.to_string(),
                label: $label,
                ratio: py_round2(contrast(&$fg, &$bg)),
                req: $req,
            })
        };
    }

    for mode in ["light", "dark"] {
        let sm = base.sem(mode);
        let bg = sm["bg"]["base"].as_str().unwrap().to_string();
        let surf = sm["surface"]["default"].as_str().unwrap().to_string();
        let get = |role: &str, key: &str| sm[role][key].as_str().unwrap().to_string();

        need!(format!("{mode} · 正文 / 页面底"), get("text", "primary"), bg.clone(), 4.5, "文字");
        need!(format!("{mode} · 正文 / 卡面"), get("text", "primary"), surf.clone(), 4.5, "文字");
        need!(format!("{mode} · 次要文字 / 卡面"), get("text", "secondary"), surf.clone(), 4.5, "文字");
        need!(format!("{mode} · 三级文字 / 卡面"), get("text", "tertiary"), surf, 3.0, "文字");

        for role in ["accent", "success", "warning", "danger", "info"] {
            let r = &sm[role];
            let pick = |k: &str| r[k].as_str().unwrap().to_string();
            need!(format!("{mode} · {role} 填充上的文字"), pick("on"), pick("default"), 4.5, "状态色");
            need!(format!("{mode} · {role} 文字 / 页面底"), pick("text"), bg.clone(), 4.5, "状态色");
            need!(format!("{mode} · {role} 填充 / 页面底"), pick("default"), bg.clone(), 3.0, "状态色");
            // 浅底深字：选中态与徽标的主要形态，只查页面底会漏掉这一组
            need!(format!("{mode} · {role} 文字 / 浅底"), pick("text"), pick("subtle"), 4.5, "状态色");
        }
        let chart = sm["chart"].as_object().unwrap();
        for (k, v) in chart.iter() {
            if k.starts_with('c') {
                need!(format!("{mode} · 图表 {k} / 页面底"), v.as_str().unwrap().to_string(), bg.clone(), 3.0, "数据可视化");
            }
        }
    }

    // 默认品牌与基座同源，不重复计
    for entry in &pipeline.entries {
        for mode in ["light", "dark"] {
            let sm = entry.sem(mode);
            let bg = sm["bg"]["base"].as_str().unwrap().to_string();
            let p = &sm["primary"];
            let pick = |k: &str| p[k].as_str().unwrap().to_string();
            let lbl = |what: &str| format!("{mode} · 品牌 {} {what}", entry.id);
            need!(lbl("填充上的文字"), pick("on"), pick("default"), 4.5, "品牌");
            need!(lbl("文字 / 页面底"), pick("text"), bg.clone(), 4.5, "品牌");
            need!(lbl("填充 / 页面底"), pick("default"), bg, 3.0, "品牌");
            need!(lbl("文字 / 浅底"), pick("text"), pick("subtle"), 4.5, "品牌");
        }
    }

    if let Some(ext) = pipeline.source.get("extensions").and_then(Value::as_object).and_then(|e| e.get("reading")) {
        for paper in ext["papers"].as_array().unwrap() {
            let name = paper["name"].as_str().unwrap();
            need!(
                format!("ext.reading · {name} 正文"),
                paper["text"].as_str().unwrap().to_string(),
                paper["bg"].as_str().unwrap().to_string(),
                4.5,
                "扩展"
            );
            need!(
                format!("ext.reading · {name} 次要文字"),
                paper["sub"].as_str().unwrap().to_string(),
                paper["bg"].as_str().unwrap().to_string(),
                4.5,
                "扩展"
            );
        }
    }

    checks
}

pub fn print_failures(checks: &[Check]) {
    for c in checks.iter().filter(|c| !c.ok()) {
        println!("  [FAIL] [{}] {}: {}:1 < {}:1", c.group, c.label, c.ratio, c.req);
    }
}
