#![deny(clippy::all)]
#![allow(unused)]

use napi::bindgen_prelude::*;
use std::collections::HashMap;

use codespan_reporting::{
  diagnostic::{self, Diagnostic as RDiagnostic, Label, LabelStyle},
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

#[derive(Clone)]
#[napi(object)]
pub struct LabelInfo {
  pub message: String,
  pub start: u32,
  pub end: u32,
}

#[napi]
pub enum DiagnosticLabelStyle {
  Primary,
  Secondary,
}
impl From<LabelStyle> for DiagnosticLabelStyle {
  fn from(s: LabelStyle) -> Self {
    match s {
      LabelStyle::Primary => Self::Primary,
      LabelStyle::Secondary => Self::Secondary,
    }
  }
}
#[napi(object)]
/// a wrapper of `codespan_reporting::diagnostic::Label`
pub struct DiagnosticLabel {
  pub style: DiagnosticLabelStyle,
  pub file_id: u32,
  pub info: LabelInfo,
}

#[napi]
impl DiagnosticLabel {
  #[napi(factory)]
  pub fn primary(file_id: u32, info: LabelInfo) -> Self {
    Self {
      file_id,
      style: DiagnosticLabelStyle::Primary,
      info,
    }
  }
  #[napi(factory)]
  pub fn secondary(file_id: u32, info: LabelInfo) -> Self {
    Self {
      file_id,
      style: DiagnosticLabelStyle::Secondary,
      info,
    }
  }
}

impl From<Label<usize>> for DiagnosticLabel {
  fn from(l: Label<usize>) -> Self {
    Self {
      style: l.style.into(),
      file_id: l.file_id as u32,
      info: LabelInfo {
        message: l.message,
        start: l.range.start as u32,
        end: l.range.end as u32,
      },
    }
  }
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
pub enum Severity {
  Bug,
  Error,
  Warning,
  Note,
  Help,
}

#[napi]
struct Diagnostic {
  severity: Severity,
  code: Option<String>,
  message: String,
  labels: Vec<DiagnosticLabel>,
  notes: Vec<String>,
}

#[napi]
impl Diagnostic {
  #[napi(factory)]
  pub fn error() -> Self {
    Self {
      severity: Severity::Error,
      code: None,
      message: "".to_string(),
      labels: vec![],
      notes: vec![],
    }
  }
  #[napi(factory)]
  pub fn bug() -> Self {
    Self {
      severity: Severity::Bug,
      code: None,
      message: "".to_string(),
      labels: vec![],
      notes: vec![],
    }
  }
  #[napi(factory)]
  pub fn warning() -> Self {
    Self {
      severity: Severity::Warning,
      code: None,
      message: "".to_string(),
      labels: vec![],
      notes: vec![],
    }
  }
  #[napi(factory)]
  pub fn help() -> Self {
    Self {
      severity: Severity::Help,
      code: None,
      message: "".to_string(),
      labels: vec![],
      notes: vec![],
    }
  }
  #[napi(factory)]
  pub fn note() -> Self {
    Self {
      severity: Severity::Note,
      code: None,
      message: "".to_string(),
      labels: vec![],
      notes: vec![],
    }
  }

  #[napi]
  pub fn with_message(&mut self, message: String) {
    self.message = message;
  }

  #[napi]
  pub fn with_code(&mut self, code: String) {
    self.code = Some(code);
  }

  #[napi]
  pub fn with_labels(&mut self, labels: Vec<DiagnosticLabel>) {
    self.labels = labels;
  }
  #[napi]
  pub fn with_notes(&mut self, notes: Vec<String>) {
    self.notes = notes;
  }
}

#[napi]
pub fn emit_error(
  file_name: String,
  source_file: String,
  labels: Vec<LabelInfo>,
  error_message: Option<String>,
) {
  let mut files = SimpleFiles::new();
  let error_message = error_message.unwrap_or("Error occurred".to_string());
  let file_id = files.add(file_name, source_file);
  let diagnostic = RDiagnostic::error()
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
