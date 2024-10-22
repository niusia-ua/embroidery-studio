import type { DisplaySettings } from "./display";
import type { Pattern } from "./pattern";
import type { PrintSettings } from "./print";

export interface PatternProject {
  pattern: Pattern;
  displaySettings: DisplaySettings;
  printSettings: PrintSettings;
}
