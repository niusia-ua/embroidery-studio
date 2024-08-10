<template>
  <Menubar :model="menuOptions" />
</template>

<script lang="ts" setup>
  import { ref } from "vue";
  import Menubar from "primevue/menubar";
  import type { MenuItem } from "primevue/menuitem";
  import { open } from "@tauri-apps/api/dialog";
  import { loadPattern } from "./commands/pattern";
  import { studioDocumentDir } from "./utils/common";

  const fileOptions: MenuItem = {
    label: "File",
    icon: "pi pi-file",
    items: [
      {
        label: "Open",
        icon: "pi pi-file",
        command: async () => {
          const file = await open({
            defaultPath: await studioDocumentDir(),
            multiple: false,
            filters: [
              {
                name: "Cross Stitch Pattern",
                extensions: ["xsd"],
              },
            ],
          });
          if (file === null || Array.isArray(file)) return;
          const pattern = await loadPattern(file);
          console.log(pattern);
        },
      },
      {
        label: "Create",
        icon: "pi pi-file-plus",
      },
      {
        label: "Save",
        icon: "pi pi-save",
      },
      {
        label: "Save As",
        icon: "pi pi-copy",
      },
      {
        label: "Close",
        icon: "pi pi-times",
      },
    ],
  };
  const menuOptions = ref<MenuItem[]>([
    {
      icon: "pi pi-bars",
      items: [fileOptions],
    },
  ]);
</script>
