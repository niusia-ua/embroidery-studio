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
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { onUnmounted, ref } from "vue";
  import ButtonGroup from "primevue/buttongroup";
  import Button from "primevue/button";

  // New window is maximized by default.
  const isMaximized = ref(true);

  const appWindow = getCurrentWindow();
  const unlistenResized = await appWindow.onResized(() => {
    isMaximized.value = !isMaximized.value;
  });

  onUnmounted(() => {
    unlistenResized();
  });
</script>
