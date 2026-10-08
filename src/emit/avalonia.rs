//! Avalonia 端：`gen/avalonia/Tokens.axaml`（基座）+ `gen/avalonia/brands/<id>.axaml`（品牌 overlay）

use crate::emit::{android_color, avl_key, mix_toward, with_alpha, GENERATOR};
use crate::model::{arr, flat_semantic, obj, s, sub};
use crate::pipeline::Pipeline;
use crate::util::{cap_first, num};

const HEAD: [&str; 2] = [
    r#"<ResourceDictionary xmlns="https://github.com/avaloniaui""#,
    r#"                    xmlns:x="http://schemas.microsoft.com/winfx/2006/xaml">"#,
];

pub fn emit_avalonia(p: &Pipeline) -> String {
    let source = &p.source;
    let sc = obj(source, "scales");
    let meta = &source["meta"];
    let base = p.base();
    let varying = p.brand_varying_keys();

    let mut l: Vec<String> = HEAD.iter().map(|x| x.to_string()).collect();
    l.push(format!(
        "  <!-- {} {} · Avalonia 令牌基座（生成物，勿手改） -->",
        s(meta, "name"),
        s(meta, "nameEn")
    ));
    l.push(format!(
        "  <!-- 真源：tokens/source.json v{} · 生成器：{} -->",
        s(meta, "version"),
        GENERATOR
    ));
    l.push("  <!-- 面层禁止直接引用 SystemControl*/Card*/TextFill* 等框架键，一律走 Dq 语义键 -->".to_string());
    l.push("  <!-- 品牌无关：primary.* / text.link / focus.ring 在 brands/<id>.axaml，须与基座同时合并恰好一支 -->".to_string());

    // tier1 原语：无语义，仅供 tier2 引用
    for fam in &p.families {
        l.push(format!(
            "  <!-- {} {} · 锚点 {} {} -->",
            fam.name, fam.pinyin, fam.anchor_name, fam.anchor_hex
        ));
        for step in &p.steps {
            l.push(format!(
                r#"  <SolidColorBrush x:Key="DqCn{}{step}" Color="{}" />"#,
                crate::util::capitalize(&fam.id),
                fam.step(&step.to_string()).hex
            ));
        }
    }

    // 品牌识别色：品牌的身份色，不随深浅模式变，故刻意不进 ThemeDictionaries
    for bmeta in arr(source, "brands") {
        let bid = s(bmeta, "id");
        l.push(format!("  <!-- 品牌 {} {}：{} -->", bmeta["name"].as_str().unwrap(), bmeta["pinyin"].as_str().unwrap(), s(bmeta, "desc")));
        l.push(format!(
            r#"  <SolidColorBrush x:Key="DqBrandSwatch{}" Color="{}" />"#,
            cap_first(bid),
            p.entries
                .iter()
                .find(|e| e.id == bid)
                .unwrap()
                .sem("light")["primary"]["default"]
                .as_str()
                .unwrap()
        ));
    }

    // tier2 语义：深浅两套，品牌色已摘出
    l.push("  <ResourceDictionary.ThemeDictionaries>".to_string());
    for mode in ["light", "dark"] {
        let label = if mode == "light" { "Light" } else { "Dark" };
        l.push(format!(r#"    <ResourceDictionary x:Key="{label}">"#));
        for (k, v) in flat_semantic(base.sem(mode)) {
            if varying.contains(&k) {
                continue;
            }
            l.push(format!(
                r#"      <SolidColorBrush x:Key="{}" Color="{}" />"#,
                avl_key(&k),
                android_color(&v)
            ));
        }
        l.push("    </ResourceDictionary>".to_string());
    }
    l.push("  </ResourceDictionary.ThemeDictionaries>".to_string());

    // tier3 扩展：纸色与系统明暗解耦（用户选择），故不进 ThemeDictionaries
    if let Some(ext) = p.reading_ext() {
        l.push(format!("  <!-- tier3 扩展 {}：{} -->", s(ext, "id"), s(ext, "desc")));
        for paper in ext["papers"].as_array().unwrap() {
            let pid = cap_first(paper["id"].as_str().unwrap());
            l.push(format!(
                "  <!-- {} {} · {} -->",
                paper["name"].as_str().unwrap(),
                paper["pinyin"].as_str().unwrap(),
                paper["desc"].as_str().unwrap()
            ));
            for (key, suffix) in [("bg", "Bg"), ("text", "Text"), ("sub", "Sub"), ("widget", "Widget")] {
                let raw = paper[key].as_str().unwrap();
                let color = if key == "widget" { android_color(raw) } else { raw.to_string() };
                l.push(format!(r#"  <SolidColorBrush x:Key="DqReading{pid}{suffix}" Color="{color}" />"#));
            }
        }
        let fs = obj(ext, "fontSize");
        l.push(format!(r#"  <x:Double x:Key="DqReadingSizeMin">{}</x:Double>"#, num(&fs["min"])));
        l.push(format!(r#"  <x:Double x:Key="DqReadingSizeMax">{}</x:Double>"#, num(&fs["max"])));
        l.push(format!(r#"  <x:Double x:Key="DqReadingSizeDefault">{}</x:Double>"#, num(&fs["default"])));
        for (prefix, key) in [("Measure", "measure"), ("LineHeight", "lineHeight"), ("Indent", "indent")] {
            for (k, v) in obj(ext, key) {
                l.push(format!(
                    r#"  <x:Double x:Key="DqReading{prefix}{}">{}</x:Double>"#,
                    cap_first(k),
                    num(v)
                ));
            }
        }
    }

    // 字体族：Avalonia FontFamily 直接吃逗号分隔的候选栈
    let typo = sub(sc, "typography");
    for (fk, fam_spec) in sub(typo, "family") {
        l.push(format!("  <!-- {} / {} -->", fam_spec["cn"].as_str().unwrap(), fam_spec["en"].as_str().unwrap()));
        l.push(format!(
            r#"  <FontFamily x:Key="DqFontFamily{}">{}</FontFamily>"#,
            cap_first(fk),
            s(fam_spec, "stack")
        ));
    }
    for (k, v) in sub(typo, "weight") {
        l.push(format!(r#"  <x:Double x:Key="DqFontWeight{}">{}</x:Double>"#, cap_first(k), num(v)));
    }

    // 尺度
    for (k, v) in sub(sc, "radius") {
        l.push(format!(r#"  <CornerRadius x:Key="DqRadius{}">{}</CornerRadius>"#, cap_first(k), num(v)));
    }
    for (k, v) in sub(sc, "space") {
        l.push(format!(r#"  <x:Double x:Key="DqSpace{k}">{}</x:Double>"#, num(v)));
    }
    for (k, v) in sub(sc, "borderWidth") {
        l.push(format!(r#"  <x:Double x:Key="DqBorderWidth{}">{}</x:Double>"#, cap_first(k), num(v)));
    }
    for (k, v) in sub(typo, "scale") {
        l.push(format!(r#"  <x:Double x:Key="DqFontSize{}">{}</x:Double>"#, cap_first(k), num(&v["size"])));
    }
    for (k, v) in sub(sc, "opacity") {
        l.push(format!(r#"  <x:Double x:Key="DqOpacity{}">{}</x:Double>"#, cap_first(k), num(v)));
    }
    for (k, v) in sub(sc, "icon") {
        l.push(format!(r#"  <x:Double x:Key="DqIcon{}">{}</x:Double>"#, cap_first(k), num(v)));
    }
    l.push(format!(r#"  <x:Double x:Key="DqTouchMin">{}</x:Double>"#, num(&sc["touch"]["min"])));
    l.push(format!(
        r#"  <x:Double x:Key="DqContainerProse">{}</x:Double>"#,
        num(&sc["container"]["prose"])
    ));
    l.push("</ResourceDictionary>".to_string());

    format!("{}\n", l.join("\n"))
}

/// 一支品牌一个 overlay 文件。返回 `(品牌 id, 文件内容)`。
pub fn emit_avalonia_brand(p: &Pipeline) -> Vec<(String, String)> {
    let varying = p.brand_varying_keys();
    let mut out: Vec<(String, String)> = Vec::new();

    for bmeta in arr(&p.source, "brands") {
        let bid = s(bmeta, "id").to_string();
        let entry = p.entries.iter().find(|e| e.id == bid).unwrap();
        let mut l: Vec<String> = HEAD.iter().map(|x| x.to_string()).collect();
        l.push(format!(
            "  <!-- {} · 品牌 overlay「{} {}」（生成物，勿手改） -->",
            s(&p.source["meta"], "name"),
            bmeta["name"].as_str().unwrap(),
            bmeta["pinyin"].as_str().unwrap()
        ));
        l.push(format!("  <!-- {} -->", s(bmeta, "desc")));
        l.push(format!("  <!-- 适用：{} -->", s(bmeta, "audience")));
        l.push("  <!-- 须与 Tokens.axaml 基座一起合并，且一次只合并一支 -->".to_string());
        l.push("  <ResourceDictionary.ThemeDictionaries>".to_string());

        for mode in ["light", "dark"] {
            let label = if mode == "light" { "Light" } else { "Dark" };
            l.push(format!(r#"    <ResourceDictionary x:Key="{label}">"#));
            for (k, v) in flat_semantic(entry.sem(mode)) {
                if !varying.contains(&k) {
                    continue;
                }
                l.push(format!(
                    r#"      <SolidColorBrush x:Key="{}" Color="{}" />"#,
                    avl_key(&k),
                    android_color(&v)
                ));
            }

            let prim = &entry.sem(mode)["primary"];
            let fill = prim["default"].as_str().unwrap().to_string();
            let on = prim["on"].as_str().unwrap().to_string();
            l.push("      <!-- FluentTheme 兼容：原生控件（ToggleSwitch / ComboBox / 焦点环等）跟随品牌 -->".to_string());
            l.push("      <!-- Fluent 要的是「1 基色 + Light1..3 + Dark1..3」的渐层，丹青给的是语义 7 角色，形状不同；此处按混合比例派生近似值 -->".to_string());
            l.push(format!(r#"      <Color x:Key="SystemAccentColor">{fill}</Color>"#));
            for (i, t) in [0.15f64, 0.30, 0.45].iter().enumerate() {
                l.push(format!(
                    r#"      <Color x:Key="SystemAccentColorLight{}">{}</Color>"#,
                    i + 1,
                    mix_toward(&fill, (1.0, 1.0, 1.0), *t)
                ));
            }
            for (i, t) in [0.15f64, 0.30, 0.45].iter().enumerate() {
                l.push(format!(
                    r#"      <Color x:Key="SystemAccentColorDark{}">{}</Color>"#,
                    i + 1,
                    mix_toward(&fill, (0.0, 0.0, 0.0), *t)
                ));
            }
            let subtle = prim["subtle"].as_str().unwrap();
            let text = prim["text"].as_str().unwrap();
            l.push(format!(r#"      <SolidColorBrush x:Key="AccentFillColorDefaultBrush" Color="{fill}" />"#));
            l.push(format!(
                r#"      <SolidColorBrush x:Key="AccentFillColorSecondaryBrush" Color="{}" />"#,
                with_alpha(&fill, 0xDD)
            ));
            l.push(format!(
                r#"      <SolidColorBrush x:Key="AccentFillColorTertiaryBrush" Color="{}" />"#,
                with_alpha(&fill, 0x80)
            ));
            l.push(format!(
                r#"      <SolidColorBrush x:Key="AccentFillColorDisabledBrush" Color="{subtle}" />"#
            ));
            l.push(format!(
                r#"      <SolidColorBrush x:Key="AccentTextFillColorPrimaryBrush" Color="{text}" />"#
            ));
            l.push(format!(
                r#"      <SolidColorBrush x:Key="AccentTextFillColorSecondaryBrush" Color="{text}" />"#
            ));
            l.push(format!(
                r#"      <SolidColorBrush x:Key="AccentTextFillColorTertiaryBrush" Color="{}" />"#,
                mix_toward(&fill, (1.0, 1.0, 1.0), 0.25)
            ));
            l.push(format!(
                r#"      <SolidColorBrush x:Key="TextOnAccentFillColorPrimaryBrush" Color="{on}" />"#
            ));
            l.push(format!(
                r#"      <SolidColorBrush x:Key="TextOnAccentFillColorSecondaryBrush" Color="{}" />"#,
                with_alpha(&on, 0xE6)
            ));
            l.push(format!(
                r#"      <SolidColorBrush x:Key="TextOnAccentFillColorDisabledBrush" Color="{}" />"#,
                with_alpha(&on, 0x66)
            ));
            l.push("    </ResourceDictionary>".to_string());
        }
        l.push("  </ResourceDictionary.ThemeDictionaries>".to_string());
        l.push("</ResourceDictionary>".to_string());
        out.push((bid, format!("{}\n", l.join("\n"))));
    }
    out
}
