import type { Pattern } from "#/types/pattern";
import { invoke } from "@tauri-apps/api/tauri";

export const loadPattern = (filePath: string) => invoke<Pattern>("load_pattern", { filePath });
export const createPattern = () => invoke<[string, Pattern]>("create_pattern");
