<template>
  <Select
    v-model="appState.state.selectedStitchTool"
    :options="tools"
    option-label="name"
    option-value="kind"
    pt:root:class="border-0 rounded-none bg-transparent"
  />
</template>

<script setup lang="ts">
  import { ref } from "vue";
  import Select from "primevue/select";
  import { useAppStateStore } from "#/stores/state";
  import { FullStitchKind, LineStitchKind, NodeStitchKind, PartStitchKind } from "#/schemas/pattern/pattern";

  const appState = useAppStateStore();
  const tools = ref([
    { name: "Full Stitch", kind: FullStitchKind.Full },
    { name: "Petite Stitch", kind: FullStitchKind.Petite },
    { name: "Half Stitch", kind: PartStitchKind.Half },
    { name: "Quarter Stitch", kind: PartStitchKind.Quarter },
    { name: "Back Stitch", kind: LineStitchKind.Back },
    { name: "Straight Stitch", kind: LineStitchKind.Straight },
    { name: "French Knot", kind: NodeStitchKind.FrenchKnot },
    { name: "Bead", kind: NodeStitchKind.Bead },
  ]);
</script>
