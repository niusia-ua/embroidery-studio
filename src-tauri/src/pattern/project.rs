use borsh::{BorshDeserialize, BorshSerialize};

use super::{display::DisplaySettings, print::PrintSettings, Pattern};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct PatternProject {
  #[borsh(skip)]
  pub file_path: Option<std::path::PathBuf>,
  pub pattern: Pattern,
  pub display_settings: DisplaySettings,
  pub print_settings: PrintSettings,
}
