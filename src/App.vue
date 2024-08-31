<template>
  <BlockUI :blocked="loading" full-screen />
  <div class="h-full flex flex-column">
    <Toolbar
      class="border-noround border-none border-bottom-1 p-0"
      draggable="true"
      @dragstart="() => appWindow.startDragging()"
    >
      <template #start>
        <DropdownTieredMenu
          id="general_menu"
          :button="{ icon: 'pi pi-bars' }"
          :tiered-menu="{ model: menuOptions }"
        />
        <StitchToolSelector />
      </template>

      <template #end>
        <WindowControls />
      </template>
    </Toolbar>

    <Splitter :gutter-size="2" class="h-full border-noround border-none">
      <SplitterPanel :min-size="5" :size="15">
        <PalettePanel :palette="patternStore.pattern?.palette" />
      </SplitterPanel>

      <SplitterPanel :min-size="85" :size="85">
        <CanvasPanel v-if="patternStore.pattern" :pattern="patternStore.pattern" />
        <template v-else>
          <ProgressSpinner v-if="loading" class="absolute top-50 left-50" />
          <Panel v-else header="No pattern loaded" class="w-3 border-none absolute top-50 left-50">
            <p class="m-0">Open a pattern or create a new one to get started.</p>
          </Panel>
        </template>
      </SplitterPanel>
    </Splitter>
  </div>
</template>

<script lang="ts" setup>
  import { open } from "@tauri-apps/api/dialog";
  import { appWindow } from "@tauri-apps/api/window";
  import BlockUI from "primevue/blockui";
  import type { MenuItem } from "primevue/menuitem";
  import Panel from "primevue/panel";
  import ProgressSpinner from "primevue/progressspinner";
  import Splitter from "primevue/splitter";
  import SplitterPanel from "primevue/splitterpanel";
  import Toolbar from "primevue/toolbar";
  import { ref } from "vue";
  import { loadPattern } from "./api/pattern";
  import CanvasPanel from "./components/CanvasPanel.vue";
  import PalettePanel from "./components/PalettePanel.vue";
  import DropdownTieredMenu from "./components/toolbar/DropdownTieredMenu.vue";
  import StitchToolSelector from "./components/toolbar/StitchToolSelector.vue";
  import WindowControls from "./components/toolbar/WindowControls.vue";
  import { usePatternStore } from "./stores/pattern";
  import { studioDocumentDir } from "./utils/path";

  const fileOptions: MenuItem = {
    label: "File",
    icon: "pi pi-file",
    items: [
      {
        label: "Open",
        icon: "pi pi-file",
        command: async () => {
          const file = await open({
            defaultPath: await studioDocumentDir(),
            multiple: false,
            filters: [
              {
                name: "Cross Stitch Pattern",
                extensions: ["xsd", "oxs", "xml"],
              },
            ],
          });
          if (file === null || Array.isArray(file)) return;
          loading.value = true;
          patternStore.pattern = await loadPattern(file);
          loading.value = false;
        },
      },
      {
        label: "Create",
        icon: "pi pi-file-plus",
      },
      {
        label: "Save",
        icon: "pi pi-save",
      },
      {
        label: "Save As",
        icon: "pi pi-copy",
      },
      {
        label: "Close",
        icon: "pi pi-times",
      },
    ],
  };
  const menuOptions = ref<MenuItem[]>([fileOptions]);

  const patternStore = usePatternStore();

  const loading = ref(false);
</script>
