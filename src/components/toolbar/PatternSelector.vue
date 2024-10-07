<template>
  <Select
    v-model="appState.state.currentPattern"
    :options="appState.state.openedPatterns"
    option-label="title"
    pt:root:class="border-0 rounded-none bg-transparent"
    @change="() => emit('switch', appState.state.currentPattern!.key)"
  />
</template>

<script setup lang="ts">
  import Select from "primevue/select";
  import { useAppStateStore } from "#/stores/state";

  const emit = defineEmits<{ (e: "switch", key: string): void }>();

  const appState = useAppStateStore();
</script>
