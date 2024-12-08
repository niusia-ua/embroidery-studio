import { invoke } from "@tauri-apps/api/core";
import type { Stitch } from "#/schemas/pattern/pattern";

export const addStitch = (patternKey: string, stitch: Stitch) => invoke<void>("add_stitch", { patternKey, stitch });
export const removeStitch = (patternKey: string, stitch: Stitch) =>
  invoke<void>("remove_stitch", { patternKey, stitch });
