import { invoke } from "@tauri-apps/api/core";
import type { PatternKey } from "#/schemas/pattern";

export const undo = (patternKey: PatternKey) => invoke<void>("undo", { patternKey });
export const redo = (patternKey: PatternKey) => invoke<void>("redo", { patternKey });
