import type { Pattern } from "#/types/pattern";
import { invoke } from "@tauri-apps/api/tauri";

export function loadPattern(filePath: string) {
  return invoke<Pattern>("load_pattern", { filePath });
}
