//! compile-pipeline 编排入口。

pub mod basic_validate;
pub mod config_decode;
pub mod config_normalize;
pub mod index_compile;
pub mod listener_plan;
pub mod loop_check;
pub mod priority_flattening;
pub mod rule_set_expand_merge;
pub mod rule_set_load_cache;
pub mod semantic_validate;
pub mod string_interning;

/// 编译流水线占位类型。
pub struct CompilePipeline;
