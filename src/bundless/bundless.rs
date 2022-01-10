use std::ops::Sub;
use std::time::{Instant};
use std::path::PathBuf;
use super::options::{resolve_options, UserOptions};
use super::utils::{get_files, get_output_path, get_display_path, write_file, remove_output_path, is_typescript_file};
use super::transformer;

#[napi]
pub fn bundless(user_options: Option<UserOptions>) {
  let start = Instant::now();
  let options = resolve_options(user_options);
  let root = &options.root;
  let input = &options.input;
  let output = &options.output;
  let ignores = &options.ignores;
  let files = get_files(&input, &ignores);

  remove_output_path(&output);

  for file in files {
    let file_path = PathBuf::from(&file);
    let mut output_path = get_output_path(&file_path, &input, &output);

    if is_typescript_file(&file_path) {
      let output = transformer::typescript_file(&file_path);
      output_path.set_extension("js");

      write_file(&output_path, output.code.as_bytes()).unwrap();
      println!("{}", get_display_path(&output_path, root));

    }
    // if is_sass_file(&file_path) {
    //   let code = transformer::sass_file(&file_path);
    //   if code.len() > 0 {
    //     output_path.set_extension("css");
    //     write_file(&output_path, code.as_bytes()).unwrap();
    //     println!("{}", get_display_path(&output_path, root));
    //   }
    // }
  }

  let end = Instant::now();

  println!("{:?}", end.sub(start));

}
