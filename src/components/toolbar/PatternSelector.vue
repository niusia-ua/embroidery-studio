<template>
  <Select
    v-model="appState.state.currentPattern"
    :options="appState.state.openedPatterns"
    option-label="title"
    pt:root:class="border-none border-noround surface-ground"
    @change="() => emit('switch', appState.state.currentPattern!.key)"
  />
</template>

<script setup lang="ts">
  import { useAppStateStore } from "#/stores/state";
  import Select from "primevue/select";

  const emit = defineEmits<{ (e: "switch", key: string): void }>();

  const appState = useAppStateStore();

  // watch(
  //   () => appState.state.currentPattern,
  //   (newPattern) => {
  //     if (newPattern) emit("switch", newPattern.key);
  //   },
  // );
</script>
