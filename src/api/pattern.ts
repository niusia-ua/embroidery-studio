import { invoke } from "@tauri-apps/api/tauri";
import type { Pattern } from "#/types/pattern";

export const loadPattern = (filePath: string) => invoke<Pattern>("load_pattern", { filePath });
export const createPattern = () => invoke<[string, Pattern]>("create_pattern");
export const savePattern = (patternKey: string, filePath: string) => {
  return invoke<Pattern>("save_pattern", { patternKey, filePath });
};
export const closePattern = (patternKey: string) => invoke<Pattern>("close_pattern", { patternKey });
