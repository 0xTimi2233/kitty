//! compile-pipeline 编排入口。

pub mod atomic_publish;
pub mod basic_validate;
pub mod config_decode;
pub mod listener_apply_plan;
pub mod loop_check;
pub mod match_index_compile;
pub mod post_publish_maintenance;
pub mod priority_flattening;
pub mod rule_set_decode_verify;
pub mod rule_set_expand_merge;
pub mod rule_set_load_cache;
pub mod rule_set_reference_prune;
pub mod runtime_plan;
pub mod semantic_collect;
pub mod semantic_validate;
pub mod string_interning;
pub mod structural_normalize;

/// 控制面编译管线占位。
pub struct CompilePipeline;
