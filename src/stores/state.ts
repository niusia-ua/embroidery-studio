import { FullStitchKind, type PaletteItem, type StitchKind } from "#/types/pattern";
import { defineStore } from "pinia";
import { reactive } from "vue";

export interface AppState {
  selectedStitchTool: StitchKind;
  selectedPaletteItem?: PaletteItem;
}

export const useAppStateStore = defineStore("embroidery-studio-state", () => {
  const state = reactive<AppState>({
    selectedStitchTool: FullStitchKind.Full,
  });

  return { state };
});
