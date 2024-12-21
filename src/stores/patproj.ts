import { defineAsyncComponent, ref } from "vue";
import { defineStore } from "pinia";
import { useConfirm, useDialog } from "primevue";
import { useAppStateStore } from "./state";
import { PatternApi } from "#/api";
import type { PatternKey, PatternProject, PaletteItem } from "#/schemas/pattern";

export const usePatternProjectStore = defineStore("pattern-project", () => {
  const confirm = useConfirm();
  const dialog = useDialog();
  const FabricProperties = defineAsyncComponent(() => import("#/components/dialogs/FabricProperties.vue"));

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
      appStateStore.addOpenedPattern(patproj.value.pattern.info.title, patproj.value.key);
    });
  const createPattern = () => {
    dialog.open(FabricProperties, {
      props: {
        header: "Fabric Properties",
        modal: true,
      },
      onClose: (options) => {
        if (!options?.data) return;
        const { patternProperties, fabric } = options.data;
        handleCommand(async () => {
          patproj.value = await PatternApi.createPattern(patternProperties, fabric);
          appStateStore.addOpenedPattern(patproj.value.pattern.info.title, patproj.value.key);
        });
      },
    });
  };
  const savePattern = (key: PatternKey, path: string) => handleCommand(() => PatternApi.savePattern(key, path));
  const closePattern = (key: PatternKey) =>
    handleCommand(async () => {
      await PatternApi.closePattern(key);
      appStateStore.removeCurrentPattern();
      if (!appStateStore.state.currentPattern) patproj.value = undefined;
      else await loadPattern(appStateStore.state.currentPattern.key);
    });

  return { loading, patproj, addPaletteItem, loadPattern, createPattern, savePattern, closePattern };
});
