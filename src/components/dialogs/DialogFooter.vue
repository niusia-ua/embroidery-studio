<template>
  <div class="flex w-full items-center justify-end gap-2">
    <Button severity="success" size="small" label="Save" @click="props.save || dialogRef.close" />
    <Button severity="danger" size="small" label="Cancel" @click="props.cancel || dialogRef.close" />
  </div>
</template>

<script setup lang="ts">
  import { Button } from "primevue";
  import type { DynamicDialogInstance } from "primevue/dynamicdialogoptions";
  import { inject, type Ref } from "vue";

  const dialogRef = inject<Ref<DynamicDialogInstance>>("dialogRef")!;

  interface DialogFooterProps {
    save?: () => void;
    cancel?: () => void;
  }

  const props = defineProps<DialogFooterProps>();
</script>
