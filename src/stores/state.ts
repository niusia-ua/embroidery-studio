import { defineStore } from "pinia";
import { reactive } from "vue";

export const useAppStateStore = defineStore("embroidery-studio-state", () => {
  const state = reactive({});

  return { state };
});
