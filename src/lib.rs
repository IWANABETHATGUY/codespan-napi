#![deny(clippy::all)]

use std::collections::HashMap;

use codespan_reporting::{
  diagnostic::{Diagnostic, Label},
  files::{SimpleFile, SimpleFiles},
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
pub struct FileMap {
  files: SimpleFiles<String, String>,
  id_map: HashMap<String, usize>,
}

#[napi]
impl FileMap {
  #[napi(constructor)]
  pub fn new() -> Self {
    FileMap {
      id_map: HashMap::new(),
      files: SimpleFiles::new(),
    }
  }
  #[napi]
  pub fn get_file_id(&self, file_name: String) -> i32 {
    match self.id_map.get(&file_name) {
      Some(v) => *v as i32,
      None => -1,
    }
  }

  #[napi]
  pub fn add_file(&mut self, file_name: String, source_file: String) {
    let id = self.files.add(file_name.clone(), source_file);
    self.id_map.insert(file_name, id);
  }
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
