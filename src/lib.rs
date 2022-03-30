#![deny(clippy::all)]

use codespan_reporting::{
  diagnostic::{Diagnostic, Label},
  files::SimpleFiles,
  term::{
    self,
    termcolor::{ColorChoice, StandardStream},
  },
};
use napi_derive::napi;

#[cfg(all(
  any(windows, unix),
  target_arch = "x86_64",
  not(target_env = "musl"),
  not(debug_assertions)
))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[napi(object)]
pub struct LabelMessage {
  pub message: String,
  pub start: u32,
  pub end: u32,
}
#[napi]
pub fn plus_100(file_name: String, source_file: String, labels: Vec<LabelMessage>) {
  let mut files = SimpleFiles::new();

  let file_id = files.add(file_name, source_file);
  let diagnostic = Diagnostic::error()
    .with_message("`case` clauses have incompatible types")
    .with_code("E0308")
    .with_labels(
      labels
        .into_iter()
        .map(|l| Label::primary(file_id, l.start as usize..l.end as usize).with_message(l.message))
        .collect(),
    );

  // We now set up the writer and configuration, and then finally render the
  // diagnostic to standard error.

  let writer = StandardStream::stderr(ColorChoice::Always);
  let config = codespan_reporting::term::Config::default();

  term::emit(&mut writer.lock(), &config, &files, &diagnostic).unwrap();
}

// #[macro_use]
// extern crate napi_derive;

// use napi::{CallContext, Env, JsNumber, JsObject, JsUndefined, Property, Result};
// use std::convert::TryInto;

// struct NativeClass {
//   value: i32,
// }

// #[js_function(1)]
// fn test_class_constructor(ctx: CallContext) -> Result<JsUndefined> {
//   let count: i32 = ctx.get::<JsNumber>(0)?.try_into()?;
//   let mut this: JsObject = ctx.this_unchecked();
//   ctx
//     .env
//     .wrap(&mut this, NativeClass { value: count + 100 })?;
//   this.set_named_property("count", ctx.env.create_int32(count)?)?;
//   ctx.env.get_undefined()
// }

// #[js_function(1)]
// fn add_count(ctx: CallContext) -> Result<JsNumber> {
//   let add: i32 = ctx.get::<JsNumber>(0)?.try_into()?;
//   let mut this: JsObject = ctx.this_unchecked();
//   let count: i32 = this.get_named_property::<JsNumber>("count")?.try_into()?;
//   this.set_named_property("count", ctx.env.create_int32(count + add)?)?;
//   this.get_named_property("count")
// }

// #[js_function(1)]
// fn add_native_count(ctx: CallContext) -> Result<JsNumber> {
//   let add: i32 = ctx.get::<JsNumber>(0)?.try_into()?;
//   let this: JsObject = ctx.this_unchecked();
//   let native_class: &mut NativeClass = ctx.env.unwrap(&this)?;
//   native_class.value += add;
//   ctx.env.create_int32(native_class.value)
// }

// #[module_exports]
// pub fn init(mut exports: JsObject, env: Env) -> Result<()> {
//   let test_class = env.define_class(
//     "TestClass",
//     test_class_constructor,
//     &[
//       Property::new("addCount")?.with_method(add_count),
//       Property::new("addNativeCount")?.with_method(add_native_count),
//     ],
//   )?;
//   exports.set_named_property("TestClass", test_class)?;

//   Ok(())
// }
