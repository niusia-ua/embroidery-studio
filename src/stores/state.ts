import { defineStore } from "pinia";
import { reactive } from "vue";

export interface AppState {
  selectedTool: string;
}

export const useAppStateStore = defineStore("embroidery-studio-state", () => {
  const state = reactive<AppState>({
    selectedTool: "Full Stitch",
  });

  return { state };
});
