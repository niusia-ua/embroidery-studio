import { invoke } from "@tauri-apps/api/core";
import { borshDeserialize } from "borsher";
import { PatternProjectSchema } from "#/schemas/";
import type { PatternProject } from "#/types/pattern/project";

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
