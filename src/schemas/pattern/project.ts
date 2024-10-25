import { BorshSchema } from "borsher";
import { PatternSchema } from "./pattern";
import { DisplaySettingsSchema } from "./display";
import { PrintSettingsSchema } from "./print";

export const PatternProjectSchema = BorshSchema.Struct({
  pattern: PatternSchema,
  displaySettings: DisplaySettingsSchema,
  printSettings: PrintSettingsSchema,
});
