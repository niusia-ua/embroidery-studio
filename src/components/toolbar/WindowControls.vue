<template>
  <ButtonGroup>
    <Button icon="pi pi-minus" severity="secondary" text class="rounded-none" @click="() => appWindow.minimize()" />
    <Button
      :icon="`pi pi-window-${isMaximized ? 'minimize' : 'maximize'}`"
      severity="secondary"
      text
      @click="() => appWindow.toggleMaximize()"
    />
    <Button icon="pi pi-times" severity="danger" text class="rounded-none" @click="() => appWindow.close()" />
  </ButtonGroup>
</template>

<script setup lang="ts">
  import { ref } from "vue";
  import Button from "primevue/button";
  import ButtonGroup from "primevue/buttongroup";
  import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";

  // New window is maximized by default.
  const isMaximized = ref(true);

  const appWindow = getCurrentWebviewWindow();
  appWindow.onResized(() => {
    isMaximized.value = !isMaximized.value;
  });
</script>
