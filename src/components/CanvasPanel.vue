<template>
  <div ref="canvasContainer" class="h-full"></div>
</template>

<script lang="ts" setup>
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { onMounted, onUnmounted, ref, watch } from "vue";
  import { useMagicKeys, whenever } from "@vueuse/core";
  import { CanvasService } from "#/services/canvas";
  import { useAppStateStore } from "#/stores/state";
  import * as stitchesApi from "#/api/stitches";
  import * as historyApi from "#/api/history";
  import { PartStitchDirection, StitchKind } from "#/types/pattern/pattern";
  import type { FullStitch, Line, Node, PartStitch } from "#/types/pattern/pattern";
  import type { PatternProject } from "#/types/pattern/project";

  interface CanvasPanelProps {
    patproj: PatternProject;
  }

  const props = defineProps<CanvasPanelProps>();

  const appStateStore = useAppStateStore();

  const canvasContainer = ref<HTMLDivElement>();
  const canvasService = new CanvasService();
  await canvasService.init();

  watch(
    () => props.patproj,
    (patproj) => canvasService.drawPattern(patproj),
  );

  // A start point is needed to draw the lines.
  // An end point is needed to draw all the other kinds of stitches (in addition to lines).
  canvasService.addEventListener("draw", async (e) => {
    if (!appStateStore.state.selectedPaletteItemIndex) return;

    // @ts-expect-error ...
    const { start, end, modifier } = e.detail;

    const x = Math.trunc(end.x);
    const y = Math.trunc(end.y);

    // Decimal portion of the end coordinates.
    const xdp = end.x - x;
    const ydp = end.y - y;

    // The current pattern is always available here.
    const patternKey = appStateStore.state.currentPattern!.key;
    const palindex = appStateStore.state.selectedPaletteItemIndex;

    const tool = appStateStore.state.selectedStitchTool;
    const kind = tool % 2; // Get 0 or 1.
    switch (tool) {
      case StitchKind.Full:
      case StitchKind.Petite: {
        const fullstitch: FullStitch = {
          x: adjustStitchCoordinate(x, xdp, kind),
          y: adjustStitchCoordinate(y, ydp, kind),
          palindex,
          kind,
        };
        await stitchesApi.addStitch(patternKey, { full: fullstitch });
        break;
      }

      case StitchKind.Half:
      case StitchKind.Quarter: {
        const direction =
          (xdp < 0.5 && ydp > 0.5) || (xdp > 0.5 && ydp < 0.5)
            ? PartStitchDirection.Forward
            : PartStitchDirection.Backward;
        const partstitch: PartStitch = {
          x: adjustStitchCoordinate(x, xdp, kind),
          y: adjustStitchCoordinate(y, ydp, kind),
          palindex,
          kind,
          direction,
        };
        await stitchesApi.addStitch(patternKey, { part: partstitch });
        break;
      }

      case StitchKind.Back:
      case StitchKind.Straight: {
        const startX = Math.trunc(start.x);
        const startY = Math.trunc(start.y);

        const line: Line = {
          x: [adjustStitchCoordinate(startX, start.x - startX, kind), adjustStitchCoordinate(x, xdp, kind)],
          y: [adjustStitchCoordinate(startY, start.y - startY, kind), adjustStitchCoordinate(y, ydp, kind)],
          palindex,
          kind,
        };
        await stitchesApi.addStitch(patternKey, { line });
        break;
      }

      case StitchKind.FrenchKnot:
      case StitchKind.Bead: {
        const node: Node = {
          x: adjustStitchCoordinate(x, xdp, kind),
          y: adjustStitchCoordinate(y, ydp, kind),
          palindex,
          kind,
          rotated: modifier,
        };
        await stitchesApi.addStitch(patternKey, { node });
        break;
      }
    }
  });

  // TODO: Don't duplicate this code.
  canvasService.addEventListener("remove", async (e) => {
    if (!appStateStore.state.selectedPaletteItemIndex) return;

    // @ts-expect-error ...
    const { point } = e.detail;

    const x = Math.trunc(point.x);
    const y = Math.trunc(point.y);

    // Decimal portion of the end coordinates.
    const xdp = point.x - x;
    const ydp = point.y - y;

    // The current pattern is always available here.
    // const patternKey = appStateStore.state.currentPattern!.key;
    const palindex = appStateStore.state.selectedPaletteItemIndex;

    const tool = appStateStore.state.selectedStitchTool;
    const kind = tool % 2; // Get 0 or 1.
    switch (tool) {
      case StitchKind.Full:
      case StitchKind.Petite: {
        const fullstitch: FullStitch = {
          x: adjustStitchCoordinate(x, xdp, kind),
          y: adjustStitchCoordinate(y, ydp, kind),
          palindex,
          kind,
        };
        // await emitStitchRemoved(patternKey, { full: fullstitch });
        canvasService.removeFullStitch(fullstitch);
        break;
      }

      case StitchKind.Half:
      case StitchKind.Quarter: {
        const direction =
          (xdp < 0.5 && ydp > 0.5) || (xdp > 0.5 && ydp < 0.5)
            ? PartStitchDirection.Forward
            : PartStitchDirection.Backward;
        const partstitch: PartStitch = {
          x: adjustStitchCoordinate(x, xdp, kind),
          y: adjustStitchCoordinate(y, ydp, kind),
          palindex,
          kind,
          direction,
        };
        // await emitStitchRemoved(patternKey, { part: partstitch });
        canvasService.removePartStitch(partstitch);
        break;
      }

      case StitchKind.FrenchKnot:
      case StitchKind.Bead: {
        const node: Node = {
          x: adjustStitchCoordinate(x, xdp, kind),
          y: adjustStitchCoordinate(y, ydp, kind),
          palindex,
          kind,
          rotated: false,
        };
        // await emitStitchRemoved(patternKey, { node });
        canvasService.removeNode(node);
        break;
      }
    }
  });

  function adjustStitchCoordinate(value: number, decimalPortion: number, tool: StitchKind): number {
    switch (tool) {
      case StitchKind.Full:
      case StitchKind.Half: {
        return value;
      }
      case StitchKind.Petite:
      case StitchKind.Quarter: {
        return decimalPortion > 0.5 ? value + 0.5 : value;
      }
      case StitchKind.Back:
      case StitchKind.Straight:
      case StitchKind.FrenchKnot:
      case StitchKind.Bead: {
        return decimalPortion > 0.5 ? value + 1 : decimalPortion > 0.25 ? value + 0.5 : value;
      }
    }
  }

  export interface StitchesRemoveManyPayload {
    fullstitches: FullStitch[];
    partstitches: PartStitch[];
    line?: Line;
    node?: Node;
  }

  const appWindow = getCurrentWindow();
  const unlistenRemoveManyStitches = await appWindow.listen<StitchesRemoveManyPayload>(
    "stitches:remove_many",
    ({ payload }) => {
      canvasService.removeFullStitches(payload.fullstitches);
      canvasService.removePartStitches(payload.partstitches);
      if (payload.line) canvasService.removeLine(payload.line);
      if (payload.node) canvasService.removeNode(payload.node);
    },
  );
  const unlistenAddManyStitches = await appWindow.listen<StitchesRemoveManyPayload>(
    "stitches:add_many",
    ({ payload }) => {
      const palette = props.patproj.pattern.palette;
      for (const fullstitch of payload.fullstitches) {
        canvasService.drawFullStitch(fullstitch, palette[fullstitch.palindex]!.color);
      }
      for (const partstitch of payload.partstitches) {
        canvasService.drawPartStitch(partstitch, palette[partstitch.palindex]!.color);
      }
      if (payload.line) canvasService.drawLine(payload.line, palette[payload.line.palindex]!.color);
      if (payload.node) canvasService.drawNode(payload.node, palette[payload.node.palindex]!.color);
    },
  );
  const unlistenRemoveOneStitch = await appWindow.listen<stitchesApi.Stitch>("stitches:remove_one", ({ payload }) => {
    if ("full" in payload) canvasService.removeFullStitch(payload.full);
    if ("part" in payload) canvasService.removePartStitch(payload.part);
    if ("line" in payload) canvasService.removeLine(payload.line);
    if ("node" in payload) canvasService.removeNode(payload.node);
  });
  const unlistenAddOneStitch = await appWindow.listen<stitchesApi.Stitch>("stitches:add_one", ({ payload }) => {
    const palette = props.patproj.pattern.palette;
    if ("full" in payload) canvasService.drawFullStitch(payload.full, palette[payload.full.palindex]!.color);
    if ("part" in payload) canvasService.drawPartStitch(payload.part, palette[payload.part.palindex]!.color);
    if ("line" in payload) canvasService.drawLine(payload.line, palette[payload.line.palindex]!.color);
    if ("node" in payload) canvasService.drawNode(payload.node, palette[payload.node.palindex]!.color);
  });

  const keys = useMagicKeys();
  whenever(keys.ctrl_z!, () => historyApi.undo(appStateStore.state.currentPattern!.key));
  whenever(keys.ctrl_y!, () => historyApi.redo(appStateStore.state.currentPattern!.key));

  onMounted(() => {
    // Resizing the canvas to set its initial size.
    canvasService.resize(canvasContainer.value!.getBoundingClientRect());
    window.addEventListener("resize", () => canvasService.resize(canvasContainer.value!.getBoundingClientRect()));
    canvasContainer.value!.appendChild(canvasService.view as HTMLCanvasElement);
    canvasService.drawPattern(props.patproj);
  });

  onUnmounted(() => {
    canvasService.clearPattern();
    unlistenRemoveManyStitches();
    unlistenAddManyStitches();
    unlistenRemoveOneStitch();
    unlistenAddOneStitch();
  });
</script>
