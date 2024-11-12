import { invoke } from "@tauri-apps/api/core";
import { borshDeserialize } from "borsher";
import { PatternProjectSchema } from "#/schemas/";
import type { PatternProject } from "#/types/pattern/project";
import type { PaletteItem } from "#/types/pattern/pattern";

export const loadPattern = async (filePath: string) => {
  const bytes = await invoke<Uint8Array>("load_pattern", { filePath });
  return borshDeserialize<PatternProject>(PatternProjectSchema, bytes);
};

export const createPattern = async () => {
  const [key, bytes] = await invoke<[string, Uint8Array]>("create_pattern");
  return { key, pattern: borshDeserialize<PatternProject>(PatternProjectSchema, bytes) };
};

export const savePattern = (patternKey: string, filePath: string) => {
  return invoke<void>("save_pattern", { patternKey, filePath });
};

export const closePattern = (patternKey: string) => invoke<void>("close_pattern", { patternKey });

export const getPatternFilePath = (patternKey: string) => invoke<string>("get_pattern_file_path", { patternKey });

export const addPaletteItem = (patternKey: string, paletteItem: PaletteItem) => {
  return invoke<void>("add_palette_item", { patternKey, paletteItem });
};
