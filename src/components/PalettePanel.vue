<template>
  <Listbox
    v-model="appState.state.selectedPaletteItem"
    :options="props.palette ?? []"
    empty-message="No palette items found"
    scroll-height="100%"
    pt:root:class="bg-transparent"
    pt:list:class="gap-1"
    pt:option:class="p-0"
    class="h-full rounded-none border-0"
  >
    <template #header>
      <div class="w-full flex justify-between items-center">
        <div class="text-color">Palette</div>
        <Button
          type="button"
          severity="secondary"
          icon="pi pi-cog"
          size="small"
          text
          @click="togglePaletteSettingsPopover"
        />
      </div>
    </template>

    <template #option="{ option: pi, selected }">
      <div
        class="w-full h-8 text-color px-2 py-1 whitespace-nowrap overflow-hidden text-ellipsis select-none"
        :style="{
          backgroundColor: `#${pi.color}`,
          color: `${contrastColor(pi.color)} !important`,
          boxShadow: selected ? `inset 0 0 0 2px #${pi.color}, inset 0 0 0 4px ${contrastColor(pi.color)}` : '',
        }"
      >
        <div v-show="!displayOnlyPaletteItemColor">
          {{ paletteItemTitle(pi, paletteItemDisplayOptions) }}
        </div>
      </div>
    </template>
  </Listbox>

  <Popover ref="paletteSettingsPopover">
    <div class="card">
      <div class="flex items-center pb-4">
        <ToggleSwitch v-model="displayOnlyPaletteItemColor" input-id="color-only" />
        <label for="color-only" class="ml-2">Color only</label>
      </div>

      <div class="flex flex-col gap-2">
        <div class="flex items-center">
          <Checkbox
            v-model="paletteItemDisplayOptions.showBrand"
            input-id="show-brand"
            name="show-brand"
            binary
            :disabled="displayOnlyPaletteItemColor"
          />
          <label for="show-brand" class="ml-2">Show floss brand</label>
        </div>

        <div class="flex items-center">
          <Checkbox
            v-model="paletteItemDisplayOptions.showNumber"
            input-id="show-number"
            binary
            :disabled="displayOnlyPaletteItemColor"
          />
          <label for="show-number" class="ml-2">Show color number</label>
        </div>

        <div class="flex items-center">
          <Checkbox
            v-model="paletteItemDisplayOptions.showName"
            input-id="show-name"
            binary
            :disabled="displayOnlyPaletteItemColor"
          />
          <label for="show-name" class="ml-2">Show color name</label>
        </div>
      </div>
    </div>
  </Popover>
</template>

<script setup lang="ts">
  import { reactive, ref } from "vue";
  import Button from "primevue/button";
  import Checkbox from "primevue/checkbox";
  import Listbox from "primevue/listbox";
  import Popover from "primevue/popover";
  import ToggleSwitch from "primevue/toggleswitch";
  import { contrastColor } from "#/utils/color";
  import { paletteItemTitle, type PaletteItemDisplayOptions } from "#/utils/paletteItem";
  import { useAppStateStore } from "#/stores/state";
  import type { PaletteItem } from "#/schemas/pattern";

  interface PalettePanelProps {
    palette?: PaletteItem[];
  }

  const props = defineProps<PalettePanelProps>();

  const appState = useAppStateStore();

  const paletteSettingsPopover = ref<typeof Popover>();
  const paletteItemDisplayOptions = reactive<PaletteItemDisplayOptions>({
    showBrand: true,
    showNumber: true,
    showName: true,
  });
  const displayOnlyPaletteItemColor = ref(false);

  function togglePaletteSettingsPopover(event: Event) {
    paletteSettingsPopover.value!.toggle(event);
  }
</script>
