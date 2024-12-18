<template>
  <div class="relative h-full">
    <Listbox
      v-model="appState.state.selectedPaletteItemIndex"
      :options="props.palette"
      :option-value="(pi: PaletteItem) => props.palette?.indexOf(pi)"
      scroll-height="100%"
      empty-message="No palette items found"
      :dt="{ list: { header: { padding: '4px 8px' } } }"
      pt:root:class="flex flex-col h-full rounded-none border-0"
      :pt:root:style="{ background: dt('splitter.background') }"
      pt:list-container:class="grow"
      pt:list:class="grid gap-1"
      :pt:list:style="{
        gridTemplateColumns: `repeat(${props.palette?.length ? paletteDisplayOptions.columnsNumber : 1}, minmax(0px, 1fr))`,
      }"
      pt:option:class="p-0"
      @option-dblclick="({ value }) => emit('removePaletteItem', value)"
    >
      <template #header>
        <div class="flex min-h-9 w-full items-center justify-between">
          <div class="text-color">Palette</div>
          <ButtonGroup v-if="props.palette !== undefined">
            <Button
              severity="primary"
              :icon="`pi ${showPaletteCatalog ? 'pi-minus' : 'pi-plus'}`"
              size="small"
              text
              @click="showPaletteCatalog = !showPaletteCatalog"
            />
            <Button
              severity="secondary"
              icon="pi pi-cog"
              size="small"
              text
              @click="(e) => paletteSettingsPopover!.toggle(e)"
            />
          </ButtonGroup>
        </div>
      </template>

      <template #option="{ option, selected }">
        <PalItem :palette-item="option" :selected="selected" :display-options="paletteDisplayOptions" />
      </template>
    </Listbox>

    <Listbox
      v-if="showPaletteCatalog"
      :model-value="props.palette?.map((pi) => ({ brand: pi.brand, number: pi.number }))"
      :options="selectedPalette"
      :option-value="(pi: PaletteItem) => ({ brand: pi.brand, number: pi.number })"
      :multiple="true"
      scroll-height="100%"
      empty-message="No palette items found"
      :dt="{ list: { header: { padding: '4px 8px' } } }"
      pt:root:class="flex flex-col h-full rounded-none border-0"
      :pt:root:style="{ background: dt('splitter.background') }"
      pt:list-container:class="grow"
      pt:list:class="grid gap-1"
      :pt:list:style="{
        gridTemplateColumns: `repeat(${selectedPalette.length ? paletteCatalogDisplayOptions.columnsNumber : 1}, minmax(0px, 1fr))`,
      }"
      pt:option:class="p-0"
      class="absolute left-full top-0 z-10 w-max"
      @option-dblclick="
        ({ value }) => {
          if (!props.palette?.find((pi) => pi.brand === value.brand && pi.number === value.number)) {
            emit('addPaletteItem', value);
          }
        }
      "
    >
      <template #header>
        <Select
          v-model="selectedPaletteCatalogItem"
          :options="[...paletteCatalog.keys()]"
          :loading="loadingPalette"
          placeholder="Select a Palette"
          size="small"
          class="w-full"
        />
      </template>

      <template #option="{ option }">
        <PalItem
          :palette-item="option"
          :selected="
            props.palette?.find((pi) => pi.brand === option.brand && pi.number === option.number) !== undefined
          "
          :display-options="paletteCatalogDisplayOptions"
        />
      </template>
    </Listbox>
  </div>

  <Popover ref="paletteSettingsPopover">
    <div class="card">
      <div class="flex items-center pb-4">
        <ToggleSwitch v-model="paletteDisplayOptions.colorOnly" input-id="color-only" />
        <label for="color-only" class="ml-2">Color only</label>
      </div>

      <div class="flex flex-col gap-2">
        <div class="flex items-center">
          <Checkbox
            v-model="paletteDisplayOptions.showBrand"
            input-id="show-brand"
            name="show-brand"
            binary
            :disabled="paletteDisplayOptions.colorOnly"
          />
          <label for="show-brand" class="ml-2">Show floss brand</label>
        </div>

        <div class="flex items-center">
          <Checkbox
            v-model="paletteDisplayOptions.showNumber"
            input-id="show-number"
            binary
            :disabled="paletteDisplayOptions.colorOnly"
          />
          <label for="show-number" class="ml-2">Show color number</label>
        </div>

        <div class="flex items-center">
          <Checkbox
            v-model="paletteDisplayOptions.showName"
            input-id="show-name"
            binary
            :disabled="paletteDisplayOptions.colorOnly"
          />
          <label for="show-name" class="ml-2">Show color name</label>
        </div>

        <IftaLabel>
          <InputNumber
            v-model="paletteDisplayOptions.columnsNumber"
            input-id="columns-number"
            size="small"
            mode="decimal"
            :min="1"
            :max="6"
            :allow-empty="false"
            show-buttons
          />
          <label for="columns-number">Columns number</label>
        </IftaLabel>
      </div>
    </div>
  </Popover>
</template>

<script setup lang="ts">
  import { onMounted, reactive, ref, useTemplateRef } from "vue";
  import { computedAsync } from "@vueuse/core";
  import {
    Button,
    ButtonGroup,
    Checkbox,
    IftaLabel,
    InputNumber,
    Listbox,
    Popover,
    Select,
    ToggleSwitch,
  } from "primevue";
  import { path } from "@tauri-apps/api";
  import { readDir, readTextFile } from "@tauri-apps/plugin-fs";
  import PalItem from "./PaletteItem.vue";
  import { DEFAULT_PALETTE_DISPLAY_OPTIONS, type PaletteDisplayOptions } from "#/utils/paletteItem";
  import { useAppStateStore } from "#/stores/state";
  import type { PaletteItem, PaletteItemBase } from "#/types/pattern/pattern";
  import { dt } from "@primevue/themes";

  interface PalettePanelProps {
    palette?: PaletteItem[];
  }

  interface PalettePanelEmits {
    (e: "addPaletteItem", pi: PaletteItem): void;
    (e: "removePaletteItem", pi: PaletteItem): void;
  }

  const props = defineProps<PalettePanelProps>();
  const emit = defineEmits<PalettePanelEmits>();

  const appState = useAppStateStore();

  const paletteDisplayOptions = reactive<PaletteDisplayOptions>({ ...DEFAULT_PALETTE_DISPLAY_OPTIONS });
  const paletteSettingsPopover = useTemplateRef("paletteSettingsPopover");

  const paletteCatalogDirPath = await path.resolveResource("resources/palettes");
  const showPaletteCatalog = ref(false);
  const paletteCatalog = ref<Map<string, PaletteItemBase[] | undefined>>(new Map());
  const selectedPaletteCatalogItem = ref("DMC");
  const paletteCatalogDisplayOptions: PaletteDisplayOptions = {
    colorOnly: false,
    showBrand: false,
    showNumber: true,
    showName: false,
    columnsNumber: 4,
  };

  const loadingPalette = ref(false);
  const selectedPalette = computedAsync(
    async () => {
      loadingPalette.value = true;
      const brand = selectedPaletteCatalogItem.value;
      let palette = paletteCatalog.value.get(brand);
      if (palette === undefined) {
        const content = await readTextFile(await path.join(paletteCatalogDirPath, `${brand}.json`));
        palette = JSON.parse(content) as PaletteItemBase[];
        paletteCatalog.value.set(brand, palette);
      }
      loadingPalette.value = false;
      return palette;
    },
    [],
    { lazy: true },
  );

  onMounted(async () => {
    for (const entry of await readDir(paletteCatalogDirPath)) {
      // TODO: allow users to create custom palettes and put them under resources/palettes/<subfolder> or via symlinks.
      if (entry.isFile) {
        // The file name is the brand name.
        const brand = entry.name.split(".")[0]!;
        paletteCatalog.value.set(brand, undefined);
      }
    }
  });
</script>
