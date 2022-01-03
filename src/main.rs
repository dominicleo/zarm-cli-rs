use glob::{glob_with, MatchOptions};
use regex::Regex;
use std::fs;
use std::io;
use std::time;
use std::path::{Path, PathBuf};
use sass_rs::{compile_file, Options};
use crate::bundless::Transformer;

fn main() {
  let start = time::Instant::now();
  let cwd = PathBuf::from("examples/library");
  let source = cwd.clone().join("src");
  let target = cwd.clone().join("dist");
  let pattern = source.clone().join("**/*");

  if !source.is_dir() {
    panic!("{} is not a directory", source.to_str().unwrap());
  }

  let options = MatchOptions {
    case_sensitive: true,
    require_literal_separator: false,
    require_literal_leading_dot: false,
  };

  let files = glob_with(&pattern.to_str().unwrap(), options).unwrap();

  if PathBuf::from(&target).exists() {
    fs::remove_dir_all(&target).unwrap();
  }

  for file in files {
    let path = file.as_ref().unwrap();
    let mut output = get_output_path(path, &source, &target);

    if path.is_file() {
      if is_typescript_file(&path) {
        println!("script {:?}", output);
        let contents = fs::read(&path).unwrap();
        write_file(&output, &contents).unwrap();

      } else if is_sass_file(&path) {
        let contents = compile_file(&path, Options::default()).unwrap();

        if contents.len() > 0 {
          println!("sass {:?}", output);
          output.set_extension("css");
          write_file(&output, &contents.as_bytes()).unwrap();
        }
      }
    }
  }

  let end = time::Instant::now();

  println!("done {:?}", end - start);
}

fn get_output_path(path: &PathBuf, source: &PathBuf, target: &PathBuf) -> PathBuf {
  PathBuf::from(
    path
      .canonicalize()
      .unwrap()
      .to_str()
      .unwrap()
      .replace(source.to_str().unwrap(), target.to_str().unwrap()),
  )
}

fn is_sass_file (path: &PathBuf) -> bool {
  Regex::new(r"\.(sass|scss)$").unwrap().is_match(&path.to_str().unwrap())
}

// fn is_ecmascript_file (path: &PathBuf) -> bool {
//   Regex::new(r"\.jsx?$").unwrap().is_match(&path.to_str().unwrap())
// }

fn is_typescript_file(path: &PathBuf) -> bool {
  Regex::new(r"\.tsx?$")
    .unwrap()
    .is_match(&path.to_str().unwrap())
}

fn write_file(path: &Path, contents: &[u8]) -> io::Result<()> {
  if !PathBuf::from(path).exists() {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
  }

  fs::write(path, contents)
}
