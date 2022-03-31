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
pub fn emit_error(
  file_name: String,
  source_file: String,
  labels: Vec<LabelMessage>,
  error_message: Option<String>,
) {
  let mut files = SimpleFiles::new();
  let error_message = error_message.unwrap_or("Error occurred".to_string());
  let file_id = files.add(file_name, source_file);
  let diagnostic = Diagnostic::error()
    .with_message(error_message)
    // .with_code("E0308")
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
