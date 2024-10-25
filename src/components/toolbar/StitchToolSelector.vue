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
  import { StitchKind } from "#/types/pattern/pattern";

  const appState = useAppStateStore();
  const tools = ref([
    { name: "Full Stitch", kind: StitchKind.Full },
    { name: "Petite Stitch", kind: StitchKind.Petite },
    { name: "Half Stitch", kind: StitchKind.Half },
    { name: "Quarter Stitch", kind: StitchKind.Quarter },
    { name: "Back Stitch", kind: StitchKind.Back },
    { name: "Straight Stitch", kind: StitchKind.Straight },
    { name: "French Knot", kind: StitchKind.FrenchKnot },
    { name: "Bead", kind: StitchKind.Bead },
  ]);
</script>
