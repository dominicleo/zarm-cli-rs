use path_calculate::path_absolutize::Absolutize;
use regex::Regex;
use std::fs;
use std::io::Result;
use std::path::{Path, PathBuf};
use wax::Glob;

static SASS_REG: &str = r"\.(sass|scss)$";
static TS_REG: &str = r"\.tsx?$";

pub fn is_sass_file(path: &Path) -> bool {
  path.is_file()
    && Regex::new(SASS_REG)
      .unwrap()
      .is_match(path.to_str().unwrap())
}

pub fn is_typescript_file(path: &Path) -> bool {
  path.is_file() && Regex::new(TS_REG).unwrap().is_match(path.to_str().unwrap())
}

pub fn get_files(path: &Path, ignores: &Vec<String>) -> Vec<String> {
  let mut files: Vec<String> = Vec::new();
  let glob = Glob::new("**/*").unwrap();
  let paths = glob.walk(&path, usize::MAX).flatten().map(|entry| entry.into_path());

  for path in paths {
    if path.is_dir() { continue; }
    let file = path.absolutize().unwrap().to_path_buf().to_str().unwrap().to_string();
    let matched = ignores.iter().filter(|ignore| {
      let ignore_glob = Glob::new(ignore.as_str()).unwrap();
      ignore_glob.is_match(file.as_str())
    }).count() > 0;

    if !matched {
      files.push(file);
    }
  }

  files
}

pub fn write_file(path: &Path, contents: &[u8]) -> Result<()> {
  if !PathBuf::from(path).exists() {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
  }

  fs::write(path, contents)
}

pub fn get_output_path(file: &Path, input: &Path, output: &Path) -> PathBuf {
  let path_string = file.to_str().unwrap().to_string();

  let target = path_string.replace(input.to_str().unwrap(), output.to_str().unwrap());

  PathBuf::from(target)
}

pub fn get_display_path(path: &Path, root: &Path) -> String {
  let path_string = path.to_str().unwrap().to_string();
  path_string.replace(&root.to_str().unwrap(), "")
}

pub fn remove_output_path(path: &Path) {
  if PathBuf::from(&path).exists() {
    fs::remove_dir_all(&path).unwrap();
    println!("clean output directory")
  }
}
