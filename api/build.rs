use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

fn main() {
  let base_dir = Path::new("./src");
  let root_modules = generate_mod_files(base_dir).unwrap();

  let mut lib_file = File::create(base_dir.join("lib.rs")).unwrap();
  for module in root_modules {
    writeln!(lib_file, "pub mod {};", module).unwrap();
  }
}

fn generate_mod_files(dir: &Path) -> std::io::Result<Vec<String>> {
  let entries = fs::read_dir(dir)?;

  let mut mod_names = vec![];

  for entry in entries {
    let path = entry?.path();
    if path.is_dir() {
      let dir_name = path.file_name().unwrap().to_str().unwrap().to_string();

      let child_mods = generate_mod_files(&path)?;

      // Skip empty directories
      if child_mods.is_empty() {
        continue;
      }
      let mod_file_path = dir.join(format!("{}.rs", dir_name));
      let mut mod_file = File::create(&mod_file_path)?;

      for child_mod in child_mods {
        writeln!(mod_file, "pub mod {};", child_mod)?;
      }
    } else if let Some(file_stem) = path.file_stem() {
      if file_stem != "lib" && file_stem != "mod" && file_stem != "main" {
        mod_names.push(file_stem.to_str().unwrap().to_string());
      }
    }
  }

  // Sort module names to avoid unnecessary changes in the generated code
  mod_names.sort();
  // Avoid to process twice like /domains and domains.rs
  mod_names.dedup();
  Ok(mod_names)
}