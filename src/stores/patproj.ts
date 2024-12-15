import { ref } from "vue";
import { defineStore } from "pinia";
import { useConfirm } from "primevue";
import { useAppStateStore } from "./state";
import { PatternApi } from "#/api";
import type { PatternProject } from "#/schemas/pattern/project";
import type { PaletteItem } from "#/types/pattern/pattern";

export const usePatternProjectStore = defineStore("pattern-project", () => {
  const confirm = useConfirm();
  const appStateStore = useAppStateStore();

  const loading = ref(false);
  const patproj = ref<PatternProject>();

  async function addPaletteItem(pi: PaletteItem) {
    if (!patproj.value || !appStateStore.state.currentPattern) return;
    await PatternApi.addPaletteItem(appStateStore.state.currentPattern.key, pi);
    patproj.value.pattern.palette.push(pi);
  }

  async function handleCommand(command: () => Promise<void>) {
    try {
      loading.value = true;
      await command();
    } catch (err) {
      confirm.require({
        header: "Error",
        message: err as string,
        icon: "pi pi-info-circle",
        acceptLabel: "OK",
        acceptProps: { outlined: true },
        rejectLabel: "Cancel",
        rejectProps: { severity: "secondary", outlined: true },
      });
    } finally {
      loading.value = false;
    }
  }

  const loadPattern = (path: string) =>
    handleCommand(async () => {
      patproj.value = await PatternApi.loadPattern(path);
      appStateStore.addOpenedPattern(patproj.value.pattern.info.title, path);
    });
  const createPattern = () =>
    handleCommand(async () => {
      const { key, pattern } = await PatternApi.createPattern();
      patproj.value = pattern;
      appStateStore.addOpenedPattern(patproj.value.pattern.info.title, key);
    });
  const savePattern = (key: string, path: string) => handleCommand(() => PatternApi.savePattern(key, path));
  const closePattern = (key: string) =>
    handleCommand(async () => {
      await PatternApi.closePattern(key);
      appStateStore.removeCurrentPattern();
      if (!appStateStore.state.currentPattern) patproj.value = undefined;
      else await loadPattern(appStateStore.state.currentPattern.key);
    });

  return { loading, patproj, addPaletteItem, loadPattern, createPattern, savePattern, closePattern };
});
