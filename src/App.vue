<template>
  <ConfirmDialog />
  <BlockUI :blocked="loading" full-screen />
  <div class="flex h-full flex-col">
    <Toolbar data-tauri-drag-region class="rounded-none border-0 border-b p-0">
      <template #start>
        <MainMenu />
        <StitchToolSelector />
      </template>

      <template v-if="appStateStore.state.openedPatterns?.length" #center>
        <PatternSelector
          @switch="
            (patternPath) => {
              patternProjectStore.loadPattern(patternPath);
              // TODO: Store the selected palette item per opened pattern.
              appStateStore.state.selectedPaletteItemIndex = undefined;
            }
          "
        />
      </template>

      <template #end>
        <Suspense>
          <WindowControls />
        </Suspense>
      </template>
    </Toolbar>

    <Splitter :gutter-size="2" class="grow overflow-y-auto rounded-none border-0">
      <SplitterPanel :min-size="6" :size="15" pt:root:class="overflow-y-clip overflow-x-visible">
        <Suspense>
          <PalettePanel :palette="patproj?.pattern?.palette" @add-palette-item="patternProjectStore.addPaletteItem" />
        </Suspense>
      </SplitterPanel>

      <SplitterPanel :size="85">
        <ProgressSpinner v-if="loading" class="absolute left-1/2 top-1/2" />
        <Suspense v-if="patproj?.pattern"><CanvasPanel :patproj="patproj" /></Suspense>
        <div v-else class="relative flex h-full w-full items-center justify-center">
          <Panel header="No pattern loaded" class="w-3/12 border-0">
            <p class="m-0">Open a pattern or create a new one to get started.</p>
          </Panel>

          <!-- Credits -->
          <div class="absolute bottom-0 w-full">
            <p class="my-2 text-center text-xs">
              Developed with love in Ukraine | GNU General Public License v3.0 or later
            </p>
          </div>
        </div>
      </SplitterPanel>
    </Splitter>
  </div>
</template>

<script lang="ts" setup>
  import { onMounted } from "vue";
  import { BlockUI, Panel, ConfirmDialog, ProgressSpinner, Splitter, SplitterPanel, Toolbar } from "primevue";
  import MainMenu from "./components/toolbar/MainMenu.vue";
  import CanvasPanel from "./components/CanvasPanel.vue";
  import PalettePanel from "./components/palette/PalettePanel.vue";
  import PatternSelector from "./components/toolbar/PatternSelector.vue";
  import StitchToolSelector from "./components/toolbar/StitchToolSelector.vue";
  import WindowControls from "./components/toolbar/WindowControls.vue";
  import { useAppStateStore } from "./stores/state";
  import { usePreferencesStore } from "./stores/preferences";
  import { usePatternProjectStore } from "./stores/patproj";
  import { storeToRefs } from "pinia";

  const appStateStore = useAppStateStore();
  const preferencesStore = usePreferencesStore();
  const patternProjectStore = usePatternProjectStore();
  const { patproj, loading } = storeToRefs(patternProjectStore);

  onMounted(async () => {
    await preferencesStore.setTheme(preferencesStore.theme);
    const currentPattern = appStateStore.state.currentPattern;
    if (currentPattern) await patternProjectStore.loadPattern(currentPattern.key);
  });
</script>
