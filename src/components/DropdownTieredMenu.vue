<template>
  <Button
    type="button"
    aria-haspopup="true"
    :aria-controls="id"
    severity="secondary"
    class="border-noround"
    text
    v-bind="props.button"
    @click="toggleMenu"
  />
  <TieredMenu :id="id" ref="menu" v-bind="props.tieredMenu" popup />
</template>

<script setup lang="ts">
  import Button, { type ButtonProps } from "primevue/button";
  import TieredMenu, { type TieredMenuProps } from "primevue/tieredmenu";
  import { ref } from "vue";

  interface DropdownTieredMenuProps {
    id: string;
    button?: ButtonProps;
    tieredMenu?: TieredMenuProps;
  }

  const props = defineProps<DropdownTieredMenuProps>();
  const menu = ref<typeof TieredMenu>();

  function toggleMenu(event: Event) {
    menu.value!.toggle(event);
  }
</script>
