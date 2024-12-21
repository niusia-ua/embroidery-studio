<template>
  <Button
    text
    type="button"
    severity="secondary"
    icon="pi pi-bars"
    aria-haspopup="true"
    aria-controls="main_menu"
    class="rounded-none"
    @click="(e) => menu!.toggle(e)"
  />
  <TieredMenu id="main_menu" ref="menu" :model="menuOptions" popup />
</template>

<script setup lang="ts">
  import { defineAsyncComponent, ref, useTemplateRef } from "vue";
  import { useMagicKeys, whenever } from "@vueuse/core";
  import { Button, TieredMenu, useDialog } from "primevue";
  import type { MenuItem } from "primevue/menuitem";
  import { open, save } from "@tauri-apps/plugin-dialog";
  import { PathApi, PatternApi } from "#/api";
  import { useAppStateStore } from "#/stores/state";
  import { usePreferencesStore } from "#/stores/preferences";
  import { usePatternProjectStore } from "#/stores/patproj";
  import { storeToRefs } from "pinia";

  const dialog = useDialog();
  const FabricProperties = defineAsyncComponent(() => import("#/components/dialogs/FabricProperties.vue"));

  const appStateStore = useAppStateStore();
  const preferencesStore = usePreferencesStore();
  const patternProjectStore = usePatternProjectStore();
  const { patproj } = storeToRefs(patternProjectStore);

  const menu = useTemplateRef("menu");

  const keys = useMagicKeys();
  whenever(keys.ctrl_o!, loadPattern);
  whenever(keys.ctrl_n!, patternProjectStore.createPattern);
  whenever(keys.ctrl_s!, savePattern);
  whenever(keys.ctrl_w!, closePattern);

  const fileOptions: MenuItem = {
    label: "File",
    icon: "pi pi-file",
    items: [
      {
        label: "Open",
        icon: "pi pi-file",
        command: loadPattern,
      },
      {
        label: "Create",
        icon: "pi pi-file-plus",
        command: patternProjectStore.createPattern,
      },
      {
        label: "Save As",
        icon: "pi pi-copy",
        command: savePattern,
      },
      {
        label: "Close",
        icon: "pi pi-times",
        command: closePattern,
      },
    ],
  };
  const editOptions: MenuItem = {
    label: "Edit",
    icon: "pi pi-pencil",
    visible: () => patproj.value !== undefined,
    items: [
      {
        label: "Fabric Properties",
        command: () =>
          dialog.open(FabricProperties, {
            props: {
              header: "Fabric Properties",
              modal: true,
            },
            data: {
              patternProperties: patproj.value!.pattern.properties,
              fabric: patproj.value!.pattern.fabric,
            },
            onClose: (options) => {
              const data = options?.data;
              if (!data) return;
              console.log(data);
            },
          }),
      },
    ],
  };
  const preferencesOptions: MenuItem = {
    label: "Preferences",
    icon: "pi pi-cog",
    items: [
      {
        label: "Theme",
        icon: "pi pi-palette",
        items: [
          {
            label: "Light",
            icon: "pi pi-sun",
            command: () => preferencesStore.setTheme("light"),
          },
          {
            label: "Dark",
            icon: "pi pi-moon",
            command: () => preferencesStore.setTheme("dark"),
          },
          {
            label: "System",
            icon: "pi pi-desktop",
            command: () => preferencesStore.setTheme("system"),
          },
        ],
      },
    ],
  };
  const menuOptions = ref<MenuItem[]>([fileOptions, editOptions, preferencesOptions]);

  async function loadPattern() {
    const path = await open({
      defaultPath: await PathApi.getAppDocumentDir(),
      multiple: false,
      filters: [
        {
          name: "Cross-Stitch Pattern",
          extensions: ["xsd", "oxs", "xml", "embproj"],
        },
      ],
    });
    if (path === null || Array.isArray(path)) return;
    await patternProjectStore.loadPattern(path);
  }

  async function savePattern() {
    const currentPattern = appStateStore.state.currentPattern;
    if (!currentPattern) return;
    const path = await save({
      defaultPath: await PatternApi.getPatternFilePath(currentPattern.key),
      filters: [
        {
          name: "Cross-Stitch Pattern",
          extensions: ["oxs", "embproj"],
        },
      ],
    });
    if (path === null) return;
    await patternProjectStore.savePattern(currentPattern.key, path);
  }

  async function closePattern() {
    const currentPattern = appStateStore.state.currentPattern;
    if (!currentPattern) return;
    await patternProjectStore.closePattern(currentPattern.key);
  }
</script>
