import { invoke } from "@tauri-apps/api/core";
import { deserialize } from "@dao-xyz/borsh";
import { PatternProject } from "#/schemas/pattern/project";
import { PaletteItem } from "#/schemas/pattern/pattern";

export const loadPattern = async (filePath: string) => {
  const bytes = await invoke<[number]>("load_pattern", { filePath });
  return deserialize(new Uint8Array(bytes), PatternProject);
};

export const createPattern = async () => {
  const [key, bytes] = await invoke<[string, number[]]>("create_pattern");
  return { key, pattern: deserialize(new Uint8Array(bytes), PatternProject) };
};

export const savePattern = (patternKey: string, filePath: string) => {
  return invoke<void>("save_pattern", { patternKey, filePath });
};

export const closePattern = (patternKey: string) => invoke<void>("close_pattern", { patternKey });

export const getPatternFilePath = (patternKey: string) => invoke<string>("get_pattern_file_path", { patternKey });

export const addPaletteItem = (patternKey: string, paletteItem: PaletteItem) => {
  return invoke<void>("add_palette_item", { patternKey, paletteItem });
};
