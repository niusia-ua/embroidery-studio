<template>
  <Listbox
    v-model="selectedPaletteItem"
    :options="props.palette"
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

    <template #option="{ option }">
      <div
        class="w-full h-2rem text-color px-2 py-1 white-space-nowrap overflow-hidden text-overflow-ellipsis select-none"
        :style="{
          backgroundColor: `#${option.color}`,
          color: `${contrastTextColor(option.color)} !important`,
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
  import type { Blend, PaletteItem } from "#/types/pattern";
  import { readTextFile } from "@tauri-apps/api/fs";
  import { resolveResource } from "@tauri-apps/api/path";
  import Button from "primevue/button";
  import Checkbox from "primevue/checkbox";
  import Listbox from "primevue/listbox";
  import Popover from "primevue/popover";
  import ToggleSwitch from "primevue/toggleswitch";
  import { reactive, ref } from "vue";

  interface PalettePanelProps {
    palette: PaletteItem[];
  }

  const props = defineProps<PalettePanelProps>();

  interface VendorInfo {
    title: string;
  }

  const vendorsPath = await resolveResource("resources/colors/vendors.json");
  const vendors: Record<number, VendorInfo> = JSON.parse(await readTextFile(vendorsPath));

  const selectedPaletteItem = ref<PaletteItem | null>(null);

  function contrastTextColor(hex: string) {
    const r = parseInt(hex.substring(0, 2), 16);
    const g = parseInt(hex.substring(2, 4), 16);
    const b = parseInt(hex.substring(4, 6), 16);
    const brightness = r * 0.299 + g * 0.587 + b * 0.114;
    return brightness > 128 ? "black" : "white";
  }

  function paletteItemTitle(pi: PaletteItem) {
    if (paletteSettings.colorOnly) return "";
    const components = [];
    if (paletteSettings.showVendor) components.push(vendors[pi.vendorId]?.title ?? "unknown");
    if (pi.blends.length) {
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

  function blendTitle({ vendorId, number }: Blend) {
    if (paletteSettings.colorOnly) return "";
    const components = [];
    if (paletteSettings.showVendor) components.push(vendors[vendorId]?.title ?? "unknown");
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
