//! 丹青 CLI：`danqing build | check | verify | serve`
//!
//! 旧流程是 `python tools/build.py`，产物 header 里也写着「重跑 tools/build.py」。
//! Rust 化之后运行时依赖从「Python ≥3.8」降到「无」，一条二进制搞定生成、门禁、
//! 漂移校验和展示页预览。

mod color;
mod emit;
mod gate;
mod model;
mod pipeline;
mod serve;
mod terminal;
mod util;

use std::path::{Path, PathBuf};

use gate::{print_failures, run_gates, Check};
use model::{arr, s};
use pipeline::Pipeline;
use serde_json::{json, Map, Value};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let cmd = args.first().map(|s| s.as_str()).unwrap_or("build");

    match cmd {
        "-h" | "--help" | "help" => print_help(),
        "-V" | "--version" => println!("danqing {}", env!("CARGO_PKG_VERSION")),
        "serve" => {
            let port = args.iter().position(|a| a == "--port").and_then(|i| args.get(i + 1)).and_then(|p| p.parse().ok()).unwrap_or(3788u16);
            if let Err(e) = serve::run(&locate_root(), port) {
                eprintln!("[err] {e}");
                std::process::exit(1);
            }
        }
        "build" | "check" | "verify" | "" => {
            let root = locate_root();
            match run(&root, cmd) {
                Ok(()) => {}
                Err(e) => {
                    eprintln!("[err] {e}");
                    std::process::exit(1);
                }
            }
        }
        other => {
            eprintln!("[err] 未知子命令：{other}（试试 danqing --help）");
            std::process::exit(2);
        }
    }
}

fn print_help() {
    println!(
        "丹青 · 中国传统色彩设计系统 —— 令牌构建器 v{}\n\
        \n\
        用法：\n\
        \x20 danqing build       生成终端配色 + 跑对比度门禁（失败退出码 1）\n\
        \x20 danqing check        只跑门禁，不写盘\n\
        \x20 danqing verify       重新生成并与磁盘上的产物比对，查产物漂移\n\
        \x20 danqing serve        起一个静态服务预览配色参考页（默认 127.0.0.1:3788）\n\
        \x20 danqing --version\n\
        \n\
        真源：tokens/source.json（人手维护的唯一文件）+ data/chinese-colors.json\n\
        产物：themes/windows-terminal/danqing.schemes.json（入库，clone 即可用）\n\
        \x20      dist/reports/** + showcase/data.js + tokens/danqing.tokens.json（不入库）\n\
        产物一律禁止手改，改了会被下次生成覆盖；不入库的那几项在 clone / 拉取后\n\
        跑一次 danqing build 重建。",
        env!("CARGO_PKG_VERSION")
    );
}

/// 从当前目录向上找 `tokens/source.json`，最多走 4 层。
fn locate_root() -> PathBuf {
    let start = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let mut dir = Some(start.as_path());
    for _ in 0..5 {
        if let Some(d) = dir {
            if d.join("tokens").join("source.json").is_file() {
                return d.to_path_buf();
            }
            dir = d.parent();
        }
    }
    start
}

fn run(root: &Path, cmd: &str) -> Result<(), String> {
    let source_text = read(root.join("tokens").join("source.json"))?;
    let palette_text = read(root.join("data").join("chinese-colors.json"))?;
    let source: Value = serde_json::from_str(&source_text).map_err(|e| format!("tokens/source.json 解析失败：{e}"))?;
    let palette: Value = serde_json::from_str(&palette_text).map_err(|e| format!("data/chinese-colors.json 解析失败：{e}"))?;

    let p = pipeline::build(source, &palette);
    let checks = run_gates(&p, &p.families, &p.steps);
    let failures: Vec<&Check> = checks.iter().filter(|c| !c.ok()).collect();

    println!(
        "[ok] 传统色 {} 条 / 色族 {} × {} 级 = {}",
        p.palette_flat.len(),
        p.families.len(),
        p.steps.len(),
        p.families.len() * p.steps.len()
    );
    let default_name = &arr(&p.source, "brands")
        .iter()
        .find(|b| b["id"] == p.default_brand)
        .unwrap()["name"];
    println!("[ok] 品牌预设 {} 个（默认 {}）", p.entries.len(), default_name.as_str().unwrap());
    for (b, roles) in &p.brand_conflicts {
        println!("[warn] 品牌 {b} 与状态色同族：{}（已记入报告，须附图标/文案）", roles.join(", "));
    }
    println!("[ok] 对比度门禁 {} 项，失败 {} 项", checks.len(), failures.len());
    print_failures(&checks);

    if cmd == "check" {
        return finish(failures.is_empty());
    }

    let outputs = collect_outputs(&p, &checks);

    if cmd == "verify" {
        let mut drifted = 0usize;
        let mut missing = 0usize;
        for (rel, body) in &outputs {
            let path = root.join(rel);
            match std::fs::read_to_string(&path) {
                Ok(existing) if normalize(&existing) == normalize(body) => {}
                Ok(_) => {
                    drifted += 1;
                    println!("  [DRIFT] {rel}");
                }
                Err(_) => {
                    missing += 1;
                    println!("  [MISSING] {rel}");
                }
            }
        }
        // 只有终端配色入库，其余产物（报告、快照、展示页数据）都不入版本库，
        // 所以「dist/ 整个不存在」是刚 clone 下来的正常状态，不是漂移——
        // 分开报，否则新人会被一堆 MISSING 误导成「真源坏了」。
        if !root.join("dist").is_dir() {
            println!("[fail] dist/ 不存在 —— 这是新检出的工作区，请先跑 `danqing build`");
            std::process::exit(1);
        }
        if drifted + missing > 0 {
            println!(
                "[fail] {drifted} 个产物漂移 / {missing} 个缺失，请本地跑 `danqing build` 后提交"
            );
            std::process::exit(1);
        }
        println!("[ok] {} 个产物均与真源同步", outputs.len());
        return finish(failures.is_empty());
    }

    for (rel, body) in &outputs {
        let path = root.join(rel);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| format!("无法创建目录 {:?}：{e}", parent))?;
        }
        std::fs::write(&path, body).map_err(|e| format!("写入失败 {rel}：{e}"))?;
    }

    println!("[ok] 产物：");
    for (rel, body) in &outputs {
        println!("  - {rel}  ({} B)", body.len());
    }
    finish(failures.is_empty())
}

fn finish(pass: bool) -> Result<(), String> {
    if pass {
        Ok(())
    } else {
        std::process::exit(1)
    }
}

fn normalize(s: &str) -> String {
    s.replace("\r\n", "\n")
}

fn read(path: PathBuf) -> Result<String, String> {
    std::fs::read_to_string(&path).map_err(|e| format!("无法读取 {:?}：{e}", path))
}

/// 全部输出。（相对路径, 内容）
fn collect_outputs(p: &Pipeline, checks: &[Check]) -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = Vec::new();

    // 唯一入库的产物：终端配色。放 themes/ 而不是不放库的目录，是为了 clone 下来
    // 就有现成可用的配色，不必先装 Rust 工具链跑一次构建。
    out.push((
        "themes/windows-terminal/danqing.schemes.json".to_string(),
        emit::terminal::emit_windows_terminal(p),
    ));

    let reports = emit::report::emit_reports(p, checks);
    out.push(("dist/reports/ramp-report.md".to_string(), reports.ramp));
    out.push(("dist/reports/contrast-report.md".to_string(), reports.contrast));
    out.push(("dist/reports/tokens-summary.md".to_string(), reports.summary));

    out.push((
        "showcase/data.js".to_string(),
        emit::showcase::emit_showcase_data(p, checks),
    ));
    out.push(("tokens/danqing.tokens.json".to_string(), snapshot(p)));
    out
}

/// 已解析的完整真源快照：`tokens/danqing.tokens.json`。
fn snapshot(p: &Pipeline) -> String {
    let mut families = Map::new();
    for fam in &p.families {
        let mut m = Map::new();
        m.insert("id".into(), Value::String(fam.id.clone()));
        m.insert("name".into(), Value::String(fam.name.clone()));
        m.insert("pinyin".into(), Value::String(fam.pinyin.clone()));
        m.insert("hue".into(), Value::String(fam.hue.clone()));
        m.insert("anchorHex".into(), Value::String(fam.anchor_hex.clone()));
        m.insert("anchorName".into(), Value::String(fam.anchor_name.clone()));
        m.insert("anchorPinyin".into(), Value::String(fam.anchor_pinyin.clone()));
        m.insert("intent".into(), Value::String(fam.intent.clone()));
        m.insert("connotation".into(), Value::String(fam.connotation.clone()));
        m.insert("alternatives".into(), json!(fam.alternatives));
        m.insert("anchorStep".into(), Value::String(fam.anchor_step.clone()));
        m.insert("anchorIndex".into(), json!(fam.anchor_index));
        let mut steps = Map::new();
        for (key, st) in &fam.steps {
            steps.insert(
                key.clone(),
                json!({
                    "hex": st.hex, "name": st.name, "pinyin": st.pinyin,
                    "nearestHex": st.nearest_hex, "nearestGroup": st.nearest_group,
                    "anchor": st.anchor,
                }),
            );
        }
        m.insert("steps".into(), Value::Object(steps));
        families.insert(fam.id.clone(), Value::Object(m));
    }

    let mut meta = p.source["meta"].as_object().unwrap().clone();
    meta.insert("generator".into(), Value::String(emit::GENERATOR.to_string()));

    let mut brand_semantics = Map::new();
    for entry in &p.entries {
        brand_semantics.insert(
            entry.id.clone(),
            json!({
                "light": { "primary": entry.sem("light")["primary"].clone() },
                "dark": { "primary": entry.sem("dark")["primary"].clone() },
            }),
        );
    }

    let mut conflicts = Map::new();
    for (bid, roles) in &p.brand_conflicts {
        conflicts.insert(bid.clone(), json!(roles));
    }

    let mut semantic = Map::new();
    semantic.insert("light".into(), p.base().sem("light").clone());
    semantic.insert("dark".into(), p.base().sem("dark").clone());

    let doc = json!({
        "$schema": "danqing/3.0",
        "meta": meta,
        "ramp": p.source["ramp"].clone(),
        "families": families,
        "reservedFamilies": p.source["reservedFamilies"].clone(),
        "brands": p.source["brands"].clone(),
        "defaultBrand": p.default_brand,
        "semantic": semantic,
        "brandSemantics": brand_semantics,
        "scales": p.source["scales"].clone(),
        "extensions": p.source["extensions"].clone(),
        // 终端映射是生成配置而非令牌，但它属于真源，快照要「完整」就不能漏它。
        "terminal": p.source["terminal"].clone(),
        "brandConflicts": conflicts,
    });

    serde_json::to_string_pretty(&doc).unwrap()
}

#[allow(dead_code)]
fn _assert_prefix(p: &Pipeline) -> &str {
    s(&p.source["meta"], "prefix")
}
