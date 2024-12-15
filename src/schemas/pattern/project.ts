import { deserialize, field, serialize, validate } from "@dao-xyz/borsh";
import { Pattern } from "./pattern";
import { DisplaySettings } from "./display";
import { PrintSettings } from "./print";

export type PatternKey = string;
export class PatternProject {
  key!: PatternKey;

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

  static deserialize(buffer: Uint8Array) {
    class PatternKey {
      @field({ type: "string" })
      key: string;

      constructor(data: PatternKey) {
        this.key = data.key;
      }
    }

    const patternKey = deserialize(buffer, PatternKey, { unchecked: true });
    const patproj = deserialize(buffer.slice(serialize(patternKey).length), PatternProject);
    patproj.key = patternKey.key;

    return patproj;
  }
}

if (import.meta.env.DEV) validate(PatternProject);
