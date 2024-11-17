import { invoke } from "@tauri-apps/api/core";

export const undo = (patternKey: string) => invoke<void>("undo", { patternKey });
export const redo = (patternKey: string) => invoke<void>("redo", { patternKey });
