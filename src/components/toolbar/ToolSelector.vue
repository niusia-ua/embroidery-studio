<template>
  <Select
    v-model="appState.state.selectedTool"
    :options="tools"
    pt:root:class="border-none border-noround surface-ground"
  />
</template>

<script setup lang="ts">
  import { useAppStateStore } from "#/stores/state";
  import Select from "primevue/select";
  import { ref } from "vue";

  const appState = useAppStateStore();
  const tools = ref([
    "Full Stitch",
    "Petite Stitch",
    "Half Stitch",
    "Quarter Stitch",
    "Back Stitch",
    "Straight Stitch",
    "French Knot",
    "Bead",
  ]);
</script>
