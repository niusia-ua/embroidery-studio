<template>
  <ButtonGroup>
    <Button
      icon="pi pi-minus"
      severity="secondary"
      text
      class="border-noround"
      @click="() => appWindow.minimize()"
    />
    <Button
      :icon="`pi pi-window-${isMaximized ? 'minimize' : 'maximize'}`"
      severity="secondary"
      text
      @click="() => appWindow.toggleMaximize()"
    />
    <Button
      icon="pi pi-times"
      severity="danger"
      text
      class="border-noround"
      @click="() => appWindow.close()"
    />
  </ButtonGroup>
</template>

<script setup lang="ts">
  import { appWindow } from "@tauri-apps/api/window";
  import Button from "primevue/button";
  import ButtonGroup from "primevue/buttongroup";
  import { ref } from "vue";

  // New window is maximized by default.
  const isMaximized = ref(true);

  appWindow.onResized(() => {
    isMaximized.value = !isMaximized.value;
  });
</script>
