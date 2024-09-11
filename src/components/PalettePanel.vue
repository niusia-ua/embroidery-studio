<template>
  <Listbox
    v-model="appState.state.selectedPaletteItem"
    :options="props.palette ?? []"
    empty-message="No palette items found"
    scroll-height="100%"
    pt:root:class="surface-ground"
    pt:list:class="gap-1"
    pt:option:class="p-0"
    class="h-full border-noround border-none"
  >
    <template #header>
      <div class="w-full flex justify-content-between align-items-center">
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

    <template #option="{ option, selected }">
      <div
        class="w-full h-2rem text-color px-2 py-1 white-space-nowrap overflow-hidden text-overflow-ellipsis select-none"
        :style="{
          backgroundColor: `#${option.color}`,
          color: `${contrastColor(option.color)} !important`,
          boxShadow: selected ? `inset 0 0 0 2px #${option.color}, inset 0 0 0 4px ${contrastColor(option.color)}` : '',
        }"
      >
        {{ paletteItemTitle(option) }}
      </div>
    </template>
  </Listbox>

  <Popover ref="paletteSettingsPopover">
    <div class="card">
      <div class="flex items-center pb-3">
        <ToggleSwitch v-model="paletteSettings.colorOnly" input-id="color-only" />
        <label for="color-only" class="ml-2">Color only</label>
      </div>

      <div class="flex flex-column gap-2">
        <div class="flex items-center">
          <Checkbox
            v-model="paletteSettings.showVendor"
            input-id="show-vendor"
            name="show-vendor"
            binary
            :disabled="paletteSettings.colorOnly"
          />
          <label for="show-vendor" class="ml-2">Show floss vendor</label>
        </div>

        <div class="flex items-center">
          <Checkbox
            v-model="paletteSettings.showNumber"
            input-id="show-number"
            binary
            :disabled="paletteSettings.colorOnly"
          />
          <label for="show-number" class="ml-2">Show color number</label>
        </div>

        <div class="flex items-center">
          <Checkbox
            v-model="paletteSettings.showName"
            input-id="show-name"
            binary
            :disabled="paletteSettings.colorOnly"
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
  import { useAppStateStore } from "#/stores/state";
  import type { Blend, PaletteItem } from "#/types/pattern";

  interface PalettePanelProps {
    palette?: PaletteItem[];
  }

  const props = defineProps<PalettePanelProps>();

  const appState = useAppStateStore();

  function paletteItemTitle(pi: PaletteItem) {
    if (paletteSettings.colorOnly) return "";
    const components = [];
    if (paletteSettings.showVendor) components.push(pi.brand);
    if (pi.blends) {
      components.push(
        pi.blends
          .map(blendTitle)
          .filter((v) => v.length)
          .join(", "),
      );
      return components.join(": ");
    }
    if (paletteSettings.showNumber) components.push(pi.number);
    // The name can be an empty string. For example, if the palette item is blend.
    if (paletteSettings.showName && pi.name.length) {
      if (!components.length) return pi.name;
      return [components.join(" "), pi.name].join(", ");
    }
    return components.join(" ");
  }

  function blendTitle({ brand, number }: Blend) {
    if (paletteSettings.colorOnly) return "";
    const components = [];
    if (paletteSettings.showVendor) components.push(brand ?? "unknown");
    if (paletteSettings.showNumber) components.push(number);
    return components.join(" ");
  }

  const paletteSettingsPopover = ref<typeof Popover>();
  const paletteSettings = reactive({
    colorOnly: false,
    showVendor: true,
    showNumber: true,
    showName: true,
  });

  function togglePaletteSettingsPopover(event: Event) {
    paletteSettingsPopover.value!.toggle(event);
  }
</script>
