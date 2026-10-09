//! 三份报告：`dist/reports/ramp-report.md`、`contrast-report.md`、`tokens-summary.md`
//!
//! 报告是给人看的，也是 CI 里最容易被扫一眼就跳过的一层。它和产物一样是生成物，
//! 门禁失败时行尾的 ❌ 就是让 CI 日志里那一眼有东西可看。
//!
//! 报告不入库：它们是诊断材料，不是交付物。唯一入库的产物是终端配色。

use crate::gate::Check;
use crate::model::{flat_semantic, s};
use crate::util::numf;
use crate::emit::REPORT_GENERATOR;
use crate::pipeline::Pipeline;
use crate::util::fstr;

pub struct Reports {
    pub ramp: String,
    pub contrast: String,
    pub summary: String,
}

pub fn emit_reports(p: &Pipeline, checks: &[Check]) -> Reports {
    Reports {
        ramp: ramp_report(p),
        contrast: contrast_report(p, checks),
        summary: tokens_summary(p),
    }
}

fn ramp_report(p: &Pipeline) -> String {
    let source = &p.source;
    let meta = &source["meta"];
    let default_id = &p.default_brand;
    let failures_count = 0; // 该计数属于对比度报告，此处不需要

    let mut l: Vec<String> = Vec::new();
    l.push(format!("# 色阶生成报告 · {}", s(meta, "name")));
    l.push(String::new());
    // Python 版这里硬编码的是仓库相对路径，不是 GENERATOR 常量——保持一致，比对通过后再统一。
    l.push(format!("> 本文件由 `{}` 生成，勿手改。", REPORT_GENERATOR));
    l.push(String::new());
    l.push(format!("- 调色板源：{}", meta["palette"]["source"].as_str().unwrap()));
    l.push(format!(
        "- 传统色条目：{}（去重后 {}）",
        numf(&meta["palette"]["total"]) as i64,
        p.palette_flat.len()
    ));
    l.push(format!(
        "- 色族：{} 个 × {} 级 = {} 级",
        p.families.len(),
        p.steps.len(),
        p.families.len() * p.steps.len()
    ));
    l.push("- 生成方法：OKLab L 等距网格，锚点级写回传统色原值，其余级在 OKLCh 恒色相下派生并降彩度收进 sRGB".to_string());
    l.push(String::new());

    let reserved: Vec<&str> = p.reserved_map.iter().map(|(_, f)| f.as_str()).collect();
    for fam in &p.families {
        l.push(format!(
            "## {} {} · `{}` — {}",
            fam.name, fam.pinyin, fam.id, fam.intent
        ));
        l.push(String::new());
        l.push(format!("> {}", fam.connotation));
        l.push(String::new());
        l.push(format!(
            "锚点：**{} `{}`**（第 {} 级）　备用：{}",
            fam.anchor_name,
            fam.anchor_hex,
            fam.anchor_step,
            fam.alternatives.join(" / ")
        ));
        l.push(String::new());
        l.push("| 级 | 色值 | 最近传统色 | 拼音 | 传统色原值 | 色系 |".to_string());
        l.push("|---|---|---|---|---|---|".to_string());
        for step in &p.steps {
            let st = fam.step(&step.to_string());
            let mark = if st.anchor { " ◀ 锚点" } else { "" };
            l.push(format!(
                "| {step}{mark} | `{}` | {} | {} | `{}` | {} |",
                st.hex, st.name, st.pinyin, st.nearest_hex, st.nearest_group
            ));
        }
        l.push(String::new());
    }

    l.push("## 品牌预设与「锚点保真 + 可读性回退」".to_string());
    l.push(String::new());
    l.push("每个品牌的主色从传统色锚点出发；若该级无法同时满足「与底色 ≥3:1」与「填充上文字 ≥4.5:1」，则沿色阶向外回退到最近的达标级（回退步数已记录）。".to_string());
    l.push(String::new());
    l.push("| 品牌 | 色族 | 模式 | 填充级 | 回退 | 填充 | 其上文字 | 文字色 | 对比度 |".to_string());
    l.push("|---|---|---|---|---|---|---|---|---|".to_string());

    for (bid, data) in p.iter_with_base() {
        let tag = if bid == "__base__" {
            format!("{default_id}（默认）")
        } else {
            bid.to_string()
        };
        let fam_id = if bid == "__base__" {
            default_id.clone()
        } else {
            p.source["brands"]
                .as_array()
                .unwrap()
                .iter()
                .find(|b| b["id"] == *bid)
                .map(|b| s(b, "family").to_string())
                .unwrap()
        };
        let fam = p.families.iter().find(|f| f.id == fam_id).unwrap();
        for mode in ["light", "dark"] {
            let rs = &data.roles(mode)[&fam_id];
            l.push(format!(
                "| {tag} | {} | {mode} | {} | {:+} | `{}` | `{}` | `{}` | {}:1 |",
                fam.name,
                rs.fill_step,
                rs.fill_retreat,
                rs.fill,
                rs.on,
                rs.text,
                fstr(rs.contrast_on_fill)
            ));
        }
    }
    l.push(String::new());

    if !p.brand_conflicts.is_empty() {
        l.push("## 品牌 × 状态色 冲突检测".to_string());
        l.push(String::new());
        let mut pairs: Vec<(&str, &str)> = p.reserved_map.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
        pairs.sort();
        l.push(format!(
            "品牌主色与某状态色落到同一色族时，颜色不再具备独立语义。本系统通过「保留族」机制**从结构上避免**该问题：状态色占用 {}，品牌只在非保留族中取色。若新增品牌触发了冲突，将出现在下表。",
            pairs.iter().map(|(k, v)| format!("`{k}`→`{v}`")).collect::<Vec<_>>().join("、")
        ));
        l.push(String::new());
        l.push("| 品牌 | 冲突角色 | 说明 |".to_string());
        l.push("|---|---|---|".to_string());
        for (b, roles) in &p.brand_conflicts {
            for r in roles {
                l.push(format!("| {b} | {r} | 同族：颜色不足以区分，须附图标/文案 |"));
            }
        }
        l.push(String::new());
    } else {
        l.push("## 品牌 × 状态色 冲突检测".to_string());
        l.push(String::new());
        let uniq = dedup_map_values(&p.reserved_map);
        let names: Vec<String> = uniq
            .iter()
            .map(|fid| {
                let fam = p.families.iter().find(|f| f.id == *fid).unwrap();
                format!("`{fid}`({})", fam.name)
            })
            .collect();
        l.push(format!(
            "**无冲突。** 所有品牌预设均取自非保留族，与 {} 五个状态色族互不重叠。",
            names.join("、")
        ));
        l.push(String::new());
    }

    l.push("## 图表色字面值修正".to_string());
    l.push(String::new());
    l.push("图表色为字面值，若与底色对比不足 3:1 则在 OKLCh 中调整明度，修正记录如下。".to_string());
    l.push(String::new());
    l.push("| 模式 | 键 | 原始 | 修正后 |".to_string());
    l.push("|---|---|---|---|".to_string());
    for (mode, items) in &p.chart_fix {
        for (k, fix) in items {
            if fix.raw.to_uppercase() != fix.fixed.to_uppercase() {
                l.push(format!("| {mode} | {k} | `{}` | `{}` |", fix.raw, fix.fixed));
            }
        }
    }
    l.push(String::new());

    let _ = reserved;
    let _ = failures_count;
    l.join("\n")
}

fn dedup_map_values(map: &[(String, String)]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for (_, v) in map {
        if !out.contains(v) {
            out.push(v.clone());
        }
    }
    out
}

fn contrast_report(p: &Pipeline, checks: &[Check]) -> String {
    let failures = checks.iter().filter(|c| !c.ok()).count();
    let mut l: Vec<String> = Vec::new();
    l.push(format!("# WCAG 对比度门禁报告 · {}", s(&p.source["meta"], "name")));
    l.push(String::new());
    l.push(format!("- 检查项：{}　失败：{failures}", checks.len()));
    l.push("- 所有前景色的 alpha 均先合成到背景色再计算（与实际观感一致）".to_string());
    l.push("- 阈值：正文/重要文字 4.5:1；非文本（边框、图形、三级文字）3:1".to_string());
    l.push(String::new());

    for group in group_names(checks) {
        l.push(format!("## {group}"));
        l.push(String::new());
        l.push("| 检查项 | 实测 | 要求 | 结论 |".to_string());
        l.push("|---|---|---|---|".to_string());
        for c in checks.iter().filter(|c| c.group == group) {
            l.push(format!(
                "| {} | {}:1 | ≥{}:1 | {} |",
                c.label,
                fstr(c.ratio),
                fstr(c.req),
                if c.ok() { "✅" } else { "❌" }
            ));
        }
        l.push(String::new());
    }
    l.join("\n")
}

fn group_names(checks: &[Check]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for c in checks {
        if !out.contains(&c.group) {
            out.push(c.group.clone());
        }
    }
    out
}

fn tokens_summary(p: &Pipeline) -> String {
    let source = &p.source;
    let base = p.base();
    let n_cn = p.families.len() * p.steps.len();
    let n_sem = flat_semantic(base.sem("light")).len();
    let scale_groups = source["scales"]
        .as_object()
        .unwrap()
        .values()
        .map(|v| if v.is_object() { v.as_object().unwrap().len() } else { 1 })
        .sum::<usize>();

    let papers = p
        .reading_ext()
        .map(|e| e["papers"].as_array().unwrap().len())
        .unwrap_or(0);

    let mut l: Vec<String> = Vec::new();
    l.push(format!(
        "# 令牌总览 · {} v{}",
        s(&source["meta"], "name"),
        s(&source["meta"], "version")
    ));
    l.push(String::new());
    l.push("| 层 | 令牌数 | 说明 |".to_string());
    l.push("|---|---|---|".to_string());
    l.push(format!("| tier1 原语 `cn.*` | {n_cn} | {} 色族 × {} 级 |", p.families.len(), p.steps.len()));
    l.push(format!("| tier2 语义 `sys.*` | {n_sem} × 2 模式 | 浅色 / 深色各一套 |"));
    l.push(format!(
        "| 品牌 `brand.*` | {} 预设 × 2 模式 × 7 角色 | 仅影响 primary / focus / link |",
        p.entries.len()
    ));
    l.push(format!("| 尺度 `scale.*` | {scale_groups} 组 | 间距/圆角/字号/动效/层级/断点 |"));
    l.push(format!("| tier3 扩展 `ext.reading` | {papers} 纸色 + 6 组参数 | 可选引入 |"));
    l.push(String::new());
    l.push("## 产物".to_string());
    l.push(String::new());
    l.push("| 文件 | 端 | 入库 | 说明 |".to_string());
    l.push("|---|---|---|---|".to_string());
    l.push(format!(
        "| `themes/windows-terminal/danqing.schemes.json` | Windows Terminal | 是 | {} 套配色（{} 品牌 × 明暗），可粘进 `settings.json` 的 `schemes` |",
        p.entries.len() * 2,
        p.entries.len()
    ));
    l.push("| `tokens/danqing.tokens.json` | 全端 | 否 | 已解析的完整真源快照 |".to_string());
    l.push("| `showcase/data.js` | 配色参考页 | 否 | 展示页数据，页面本身不持有色值 |".to_string());
    l.push("| `dist/reports/*.md` | — | 否 | 本文件等三份诊断报告 |".to_string());
    l.push(String::new());
    l.join("\n")
}
