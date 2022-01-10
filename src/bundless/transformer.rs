use std::path::Path;

pub struct TransformOutput {
  pub code: String,
  pub source_map: Option<String>,
}

pub fn typescript_file(file: &Path) -> TransformOutput {
  use std::sync::Arc;
  use swc::common::SourceMap;
  use swc::common::{errors::Handler, FilePathMapping};
  use swc::config::{Options};

  let code_map = Arc::new(SourceMap::new(FilePathMapping::empty()));
  let compiler = swc::Compiler::new(code_map.clone());
  let handler =
    Handler::with_emitter_writer(Box::new(std::io::stderr()), Some(compiler.cm.clone()));
  let file_map = compiler.cm.load_file(&file).unwrap();

  let output = compiler
    .process_js_file(
      file_map,
      &handler,
      &Options {
        ..Default::default()
      },
    )
    .unwrap();

  TransformOutput {
    code: output.code,
    source_map: output.map,
  }
}

pub fn sass_file(file: &Path) -> String {
  // use sass_rs::{compile_file, Options as SassOptions};
  use rsass::{compile_scss_path, output};
  let format = output::Format {
    style: output::Style::Compressed,
    ..Default::default()
  };

  let result = compile_scss_path(&file, format).unwrap();

  std::str::from_utf8(&result).unwrap().to_string()
}
