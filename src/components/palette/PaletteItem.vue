<template>
  <div
    v-tooltip="{ value: paletteItemTitle(paletteItem), showDelay: 200 }"
    class="h-8 w-full px-2 py-1"
    :style="{
      backgroundColor: `#${paletteItem.color}`,
      color: `${textColor} !important`,
      boxShadow: selected ? `inset 0 0 0 2px #${paletteItem.color}, inset 0 0 0 4px ${textColor}` : '',
    }"
  >
    <p v-show="!displayOptions.colorOnly" class="overflow-hidden text-ellipsis whitespace-nowrap">
      {{ paletteItemTitle(paletteItem, displayOptions) }}
    </p>
  </div>
</template>

<script setup lang="ts">
  import { computed } from "vue";
  import { contrastColor } from "#/utils/color";
  import { paletteItemTitle, type PaletteDisplayOptions } from "#/utils/paletteItem";
  import type { PaletteItem } from "#/types/pattern/pattern";

  interface PaletteItemProps {
    paletteItem: PaletteItem;
    displayOptions: PaletteDisplayOptions;
    selected: boolean;
  }

  const { paletteItem, displayOptions, selected } = defineProps<PaletteItemProps>();

  const textColor = computed(() => contrastColor(paletteItem.color));
</script>
