use std::io::Write;

use anyhow::Result;

use crate::pattern::PatternProject;

pub fn parse_pattern(file_path: std::path::PathBuf) -> Result<PatternProject> {
  log::info!("Parsing the EMBPROJ pattern file");
  let file = std::fs::File::open(&file_path)?;
  let mut zip = zip::ZipArchive::new(file)?;

  Ok(PatternProject {
    file_path,
    pattern: {
      let mut file = zip.by_name("pattern")?;
      borsh::from_reader(&mut file)?
    },
    display_settings: {
      let mut file = zip.by_name("display_settings")?;
      borsh::from_reader(&mut file)?
    },
    print_settings: {
      let mut file = zip.by_name("print_settings")?;
      borsh::from_reader(&mut file)?
    },
  })
}

pub fn save_pattern(patproj: &PatternProject) -> Result<()> {
  log::info!("Saving the EMBPROJ pattern file");
  let file = std::fs::OpenOptions::new()
    .create(true)
    .write(true)
    .truncate(true)
    .open(&patproj.file_path)?;
  let mut zip = zip::ZipWriter::new(file);
  let options = zip::write::SimpleFileOptions::default();

  zip.start_file("pattern", options)?;
  zip.write_all(&borsh::to_vec(&patproj.pattern).unwrap())?;

  zip.start_file("display_settings", options)?;
  zip.write_all(&borsh::to_vec(&patproj.display_settings).unwrap())?;

  zip.start_file("print_settings", options)?;
  zip.write_all(&borsh::to_vec(&patproj.print_settings).unwrap())?;

  zip.finish()?;
  Ok(())
}
