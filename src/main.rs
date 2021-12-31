use glob::{glob_with, MatchOptions};
use std::fs;
use std::path::PathBuf;

fn main() {
  let cwd = PathBuf::from("examples/library");
  let source = cwd.clone().join("src");
  let target = cwd.clone().join("dist");
  let pattern = source.clone().join("**/*");

  if (!source.is_dir()) {
    panic!("{} is not a directory", source.to_str().unwrap());
  }

  let options = MatchOptions {
    case_sensitive: true,
    require_literal_separator: false,
    require_literal_leading_dot: false,
  };

  let files = glob_with(&pattern.to_str().unwrap(), options).unwrap();

  fs::remove_dir(&target).unwrap();



  for file in files {
    let path = file.as_ref().unwrap();
    let output = get_output_path(path, &source, &target);

    if (path.is_file()) {
      let content = fs::read_to_string(path).unwrap();
      // println!("{}", content);
    }

    // println!("{:?}", path);
  }
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
