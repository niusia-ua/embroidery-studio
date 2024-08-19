import { ref } from "vue";
import { defineStore } from "pinia";
import type { Pattern } from "#/types/pattern";

export const usePatternStore = defineStore("embroidery-pattern", () => {
  const pattern = ref<Pattern>();

  return { pattern };
});
