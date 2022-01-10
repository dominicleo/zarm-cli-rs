use path_calculate::path_absolutize::Absolutize;
use path_calculate::Calculate;
use serde_json::{Map, Value};
use std::default::Default;
use std::env::current_dir;
use std::path::PathBuf;

#[napi(object)]
#[derive(Serialize, Deserialize, Debug)]
pub struct UserOptions {
  /// bundler type
  /// @type "esm" | "cjs"
  pub format: Option<String>,
  /// root directory
  /// @default process.cwd()
  pub root: Option<String>,
  /// source code directory
  /// @default src
  pub input: Option<String>,
  /// output directory
  /// @default dist
  pub output: Option<String>,
  /// ignore specific directories & files via ignore syntax
  pub ignores: Option<Vec<String>>,
  /// configure module resolve alias
  pub alias: Option<Map<String, Value>>,
  /// define global constants for source code, like webpack
  pub define: Option<Map<String, Value>>,
}

#[derive(Debug)]
pub struct ResolvedOptions {
  pub format: String,
  pub root: PathBuf,
  pub input: PathBuf,
  pub output: PathBuf,
  pub ignores: Vec<String>,
  pub alias: Map<String, Value>,
  pub define: Map<String, Value>,
}

impl Default for UserOptions {
  fn default() -> Self {
    UserOptions {
      format: Some("esm".to_string()),
      root: Some(
        current_dir()
          .unwrap()
          .as_absolute_path()
          .unwrap()
          .to_str()
          .unwrap()
          .to_string(),
      ),
      input: Some("src".to_string()),
      output: Some("dist".to_string()),
      ignores: Some(Vec::new()),
      alias: Some(Map::new()),
      define: Some(Map::new()),
    }
  }
}

pub fn resolve_options(user_options: Option<UserOptions>) -> ResolvedOptions {
  let options = user_options.unwrap_or_default();

  let root = PathBuf::from(options.root.unwrap_or_default())
    .absolutize()
    .unwrap()
    .to_path_buf();

  let input = PathBuf::from(&root).join(options.input.unwrap_or_default());

  let output = PathBuf::from(&root).join(options.output.unwrap_or_default());

  ResolvedOptions {
    format: options.format.unwrap_or_default(),
    root,
    input,
    output,
    ignores: options.ignores.unwrap_or_default(),
    alias: options.alias.unwrap_or_default(),
    define: options.define.unwrap_or_default(),
  }
}
