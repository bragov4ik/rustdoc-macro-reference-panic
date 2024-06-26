# rustdoc panic example
It's a minimal verifiable example for an issue that I encountered.

Using both `macro@...` and `...!` for referencing a macro
results in compiler panic.

```
❯ rustup show
Default host: aarch64-apple-darwin
rustup home:  /Users/bragov4ik/.rustup

stable-aarch64-apple-darwin (default)
rustc 1.79.0 (129f3b996 2024-06-10)
```

Steps to repro:

1. open directory of this project
2. run `cargo doc` or `rustdoc src/main.rs`

Output:

```text
❯ rustdoc ./src/main.rs
error: internal compiler error: src/librustdoc/passes/collect_intra_doc_links.rs:402:17: no resolution for "some_cool_macro!" MacroNS DefId(0:0 ~ main[e129])
  --> ./src/main.rs:1:1
   |
1  | / //! Some module with [macro@some_cool_macro!]
2  | |
3  | | #[macro_export]
4  | | macro_rules! some_cool_macro {
...  |
9  | |     println!("Hello, world!");
10 | | }
   | |_^

thread 'rustc' panicked at src/librustdoc/passes/collect_intra_doc_links.rs:402:17:
Box<dyn Any>
stack backtrace:
   0:        0x1066e9750 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1f3776e0b5c7517d
   1:        0x10672c470 - core::fmt::write::heedef092c8c0962e
   2:        0x1066dfa1c - std::io::Write::write_fmt::h7178e8e2ea928914
   3:        0x1066e95a8 - std::sys_common::backtrace::print::h417292deb95532ed
   4:        0x1066ebb2c - std::panicking::default_hook::{{closure}}::h0cb68f1228c4613a
   5:        0x1066eb820 - std::panicking::default_hook::h24535936bc1f51de
   6:        0x10fbb7828 - <alloc[d07bc629de031c28]::boxed::Box<rustc_driver_impl[17630cf877ef70bb]::install_ice_hook::{closure#0}> as core[fca7800875c611c6]::ops::function::Fn<(&dyn for<'a, 'b> core[fca7800875c611c6]::ops::function::Fn<(&'a core[fca7800875c611c6]::panic::panic_info::PanicInfo<'b>,), Output = ()> + core[fca7800875c611c6]::marker::Sync + core[fca7800875c611c6]::marker::Send, &core[fca7800875c611c6]::panic::panic_info::PanicInfo)>>::call
   7:        0x1066ec524 - std::panicking::rust_panic_with_hook::h5db4d2345b297bed
   8:        0x10fc43198 - std[e84bc52996b8b382]::panicking::begin_panic::<rustc_errors[3520987c35dd2513]::ExplicitBug>::{closure#0}
   9:        0x10fc41ce0 - std[e84bc52996b8b382]::sys_common::backtrace::__rust_end_short_backtrace::<std[e84bc52996b8b382]::panicking::begin_panic<rustc_errors[3520987c35dd2513]::ExplicitBug>::{closure#0}, !>
  10:        0x113c0f750 - std[e84bc52996b8b382]::panicking::begin_panic::<rustc_errors[3520987c35dd2513]::ExplicitBug>
  11:        0x10fc440c0 - <rustc_errors[3520987c35dd2513]::diagnostic::BugAbort as rustc_errors[3520987c35dd2513]::diagnostic::EmissionGuarantee>::emit_producing_guarantee
  12:        0x104f89cc4 - <rustc_errors[3520987c35dd2513]::DiagCtxt>::span_bug::<rustc_span[984a238feadb13f5]::span_encoding::Span, alloc[d07bc629de031c28]::string::String>
  13:        0x105049300 - rustc_middle[e187904418cf7088]::util::bug::opt_span_bug_fmt::<rustc_span[984a238feadb13f5]::span_encoding::Span>::{closure#0}
  14:        0x105049334 - rustc_middle[e187904418cf7088]::ty::context::tls::with_opt::<rustc_middle[e187904418cf7088]::util::bug::opt_span_bug_fmt<rustc_span[984a238feadb13f5]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  15:        0x105047a58 - rustc_middle[e187904418cf7088]::ty::context::tls::with_context_opt::<rustc_middle[e187904418cf7088]::ty::context::tls::with_opt<rustc_middle[e187904418cf7088]::util::bug::opt_span_bug_fmt<rustc_span[984a238feadb13f5]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  16:        0x1052e7c98 - rustc_middle[e187904418cf7088]::util::bug::span_bug_fmt::<rustc_span[984a238feadb13f5]::span_encoding::Span>
  17:        0x104f19314 - <rustdoc[6f79afcbacfec209]::passes::collect_intra_doc_links::LinkCollector>::resolve_path
  18:        0x104f19408 - <rustdoc[6f79afcbacfec209]::passes::collect_intra_doc_links::LinkCollector>::resolve
  19:        0x104f1c408 - <rustdoc[6f79afcbacfec209]::passes::collect_intra_doc_links::LinkCollector>::resolve_with_disambiguator_cached
  20:        0x104f26c68 - <rustdoc[6f79afcbacfec209]::passes::collect_intra_doc_links::LinkCollector>::resolve_links
  21:        0x104f183e8 - rustdoc[6f79afcbacfec209]::passes::collect_intra_doc_links::collect_intra_doc_links
  22:        0x1050c4ffc - <rustc_session[176b2fa42c486007]::session::Session>::time::<rustdoc[6f79afcbacfec209]::clean::types::Crate, rustdoc[6f79afcbacfec209]::core::run_global_ctxt::{closure#6}>
  23:        0x105062438 - rustdoc[6f79afcbacfec209]::core::run_global_ctxt
  24:        0x1050c44d8 - <rustc_session[176b2fa42c486007]::session::Session>::time::<core[fca7800875c611c6]::result::Result<(rustdoc[6f79afcbacfec209]::clean::types::Crate, rustdoc[6f79afcbacfec209]::config::RenderOptions, rustdoc[6f79afcbacfec209]::formats::cache::Cache), rustc_span[984a238feadb13f5]::ErrorGuaranteed>, rustdoc[6f79afcbacfec209]::main_args::{closure#1}::{closure#0}::{closure#0}::{closure#0}>
  25:        0x1050219c8 - <rustc_middle[e187904418cf7088]::ty::context::GlobalCtxt>::enter::<rustdoc[6f79afcbacfec209]::main_args::{closure#1}::{closure#0}::{closure#0}, core[fca7800875c611c6]::result::Result<(), rustc_span[984a238feadb13f5]::ErrorGuaranteed>>
  26:        0x10501c700 - rustc_span[984a238feadb13f5]::create_session_globals_then::<core[fca7800875c611c6]::result::Result<(), rustc_span[984a238feadb13f5]::ErrorGuaranteed>, rustc_interface[617d4417f0fbb0b3]::util::run_in_thread_with_globals<rustc_interface[617d4417f0fbb0b3]::interface::run_compiler<core[fca7800875c611c6]::result::Result<(), rustc_span[984a238feadb13f5]::ErrorGuaranteed>, rustdoc[6f79afcbacfec209]::main_args::{closure#1}>::{closure#1}, core[fca7800875c611c6]::result::Result<(), rustc_span[984a238feadb13f5]::ErrorGuaranteed>>::{closure#0}::{closure#0}::{closure#0}>
  27:        0x104fbd224 - std[e84bc52996b8b382]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[617d4417f0fbb0b3]::util::run_in_thread_with_globals<rustc_interface[617d4417f0fbb0b3]::interface::run_compiler<core[fca7800875c611c6]::result::Result<(), rustc_span[984a238feadb13f5]::ErrorGuaranteed>, rustdoc[6f79afcbacfec209]::main_args::{closure#1}>::{closure#1}, core[fca7800875c611c6]::result::Result<(), rustc_span[984a238feadb13f5]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[fca7800875c611c6]::result::Result<(), rustc_span[984a238feadb13f5]::ErrorGuaranteed>>
  28:        0x104f9aa50 - <<std[e84bc52996b8b382]::thread::Builder>::spawn_unchecked_<rustc_interface[617d4417f0fbb0b3]::util::run_in_thread_with_globals<rustc_interface[617d4417f0fbb0b3]::interface::run_compiler<core[fca7800875c611c6]::result::Result<(), rustc_span[984a238feadb13f5]::ErrorGuaranteed>, rustdoc[6f79afcbacfec209]::main_args::{closure#1}>::{closure#1}, core[fca7800875c611c6]::result::Result<(), rustc_span[984a238feadb13f5]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[fca7800875c611c6]::result::Result<(), rustc_span[984a238feadb13f5]::ErrorGuaranteed>>::{closure#2} as core[fca7800875c611c6]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  29:        0x1066f4edc - std::sys::pal::unix::thread::Thread::new::thread_start::h50a0ef5291b272f3
  30:        0x185de9034 - __pthread_joiner_wake

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-rustdoc&template=ice.md

note: rustc 1.79.0 (129f3b996 2024-06-10) running on aarch64-apple-darwin

query stack during panic:
end of query stack
error: aborting due to 1 previous error
```
