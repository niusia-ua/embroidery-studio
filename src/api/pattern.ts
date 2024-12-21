import { invoke } from "@tauri-apps/api/core";
import { PatternProject, PaletteItem, type PatternKey, PatternProperties, Fabric } from "#/schemas/pattern";

export const loadPattern = async (filePath: string) => {
  const bytes = await invoke<number[]>("load_pattern", { filePath });
  return PatternProject.deserialize(new Uint8Array(bytes));
};

export const createPattern = async (patternProperties: PatternProperties, fabric: Fabric) => {
  const bytes = await invoke<number[]>("create_pattern", { patternProperties, fabric });
  return PatternProject.deserialize(new Uint8Array(bytes));
};

export const savePattern = (patternKey: PatternKey, filePath: string) => {
  return invoke<void>("save_pattern", { patternKey, filePath });
};

export const closePattern = (patternKey: PatternKey) => invoke<void>("close_pattern", { patternKey });

export const getPatternFilePath = (patternKey: PatternKey) => invoke<string>("get_pattern_file_path", { patternKey });

export const addPaletteItem = (patternKey: PatternKey, paletteItem: PaletteItem) => {
  return invoke<void>("add_palette_item", { patternKey, paletteItem });
};
export const removePaletteItem = (patternKey: PatternKey, paletteItem: PaletteItem) => {
  return invoke<void>("remove_palette_item", { patternKey, paletteItem });
};
