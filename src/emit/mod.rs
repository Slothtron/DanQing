//! 各端产物生成器。
//!
//! 丹青是一套**色彩**设计系统，不是样式层：这里只保留终端配色这一条对外生成线，
//! 以及配套的报告与展示页数据。平台的组件样式（CSS / SwiftUI / Compose / …）
//! 不属于本系统该产出的东西——真要落地，由各自的工程从语义令牌自行派生。

pub mod report;
pub mod showcase;
pub mod terminal;

/// 生成器署名，写进产物与快照的元信息。
pub const GENERATOR: &str = "danqing build";

/// 报告头部的署名。与 GENERATOR 同值，保留独立常量是为了报告措辞能单独演进。
pub const REPORT_GENERATOR: &str = GENERATOR;

/// 产物头部那句「改了真源要重跑什么」的提示。
pub const REDO_TIP: &str = "改 tokens/source.json 后重跑 danqing build";
