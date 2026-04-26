//! match-pipeline 编排入口。

pub mod action_dispatch;
pub mod bitmap_short_circuit;
pub mod context_enrich;
pub mod context_normalize;
pub mod dns_cache_lookup;
pub mod ebpf_map_sync;
pub mod first_match_evaluator;
pub mod index_probe;
pub mod ingress_classify;

/// 数据面匹配管线占位。
pub struct MatchPipeline;
