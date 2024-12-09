import { field, validate } from "@dao-xyz/borsh";
import { Pattern } from "./pattern";
import { DisplaySettings } from "./display";
import { PrintSettings } from "./print";

export class PatternProject {
  @field({ type: Pattern })
  pattern: Pattern;

  @field({ type: DisplaySettings })
  displaySettings: DisplaySettings;

  @field({ type: PrintSettings })
  printSettings: PrintSettings;

  constructor(data: PatternProject) {
    this.pattern = data.pattern;
    this.displaySettings = data.displaySettings;
    this.printSettings = data.printSettings;
  }
}

if (import.meta.env.DEV) validate(PatternProject);
