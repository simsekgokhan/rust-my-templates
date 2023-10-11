#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]

// mod doc_test; 
  // todo: failed to resolve: use of undeclared crate or module `doc_test`
mod async_;
mod averaged_collection;
mod cfg;
mod ctor_chain;
mod compile_vs_runtime_read_env_and_file;
mod concurrency;
mod const_fn;
mod drop_aka_dtor;
mod r#enum;
mod error_handling;
mod func_ptr;
mod unit;
mod unit_integration_tests;
mod int_overflow;
mod interior_mutability;
mod iterators;
mod match_;
mod mini_grep;
mod multi_threading_par_iter;
mod never_type_never_returns;
mod newtype_pattern;
mod re_export;
mod serde_json;
mod smart_ptrs;
mod string_concat;
mod temp;
mod type_;
mod traits;
mod trait_object_vs_struct_obj;
mod traits_associated_type_vs_generics;
mod traits_default_type_parameter;
mod traits_same_name_fn_call;
mod traits_supertraits;
mod utils;