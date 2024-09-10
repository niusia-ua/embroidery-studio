<template>
  <Select
    v-model="appState.state.selectedStitchTool"
    :options="tools"
    option-label="name"
    option-value="kind"
    pt:root:class="border-none border-noround surface-ground"
  />
</template>

<script setup lang="ts">
  import { ref } from "vue";
  import Select from "primevue/select";
  import { useAppStateStore } from "#/stores/state";
  import { FullStitchKind, LineKind, NodeKind, PartStitchKind } from "#/types/pattern";

  const appState = useAppStateStore();
  const tools = ref([
    { name: "Full Stitch", kind: FullStitchKind.Full },
    { name: "Petite Stitch", kind: FullStitchKind.Petite },
    { name: "Half Stitch", kind: PartStitchKind.Half },
    { name: "Quarter Stitch", kind: PartStitchKind.Quarter },
    { name: "Back Stitch", kind: LineKind.Back },
    { name: "Straight Stitch", kind: LineKind.Straight },
    { name: "French Knot", kind: NodeKind.FrenchKnot },
    { name: "Bead", kind: NodeKind.Bead },
  ]);
</script>
