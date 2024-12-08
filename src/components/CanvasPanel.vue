<template>
  <canvas
    ref="canvas"
    v-element-size="useThrottleFn((size: CanvasSize) => canvasService.resize(size), 500)"
    class="size-full"
  ></canvas>
</template>

<script lang="ts" setup>
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { onMounted, onUnmounted, useTemplateRef, watch } from "vue";
  import { useMagicKeys, whenever, useThrottleFn } from "@vueuse/core";
  import { vElementSize } from "@vueuse/components";
  import { CanvasService, type CanvasSize } from "#/services/canvas/canvas.service";
  import { AddStitchEventStage, EventType } from "#/services/canvas/events.types";
  import type { AddStitchData, RemoveStitchData } from "#/services/canvas/events.types";
  import { useAppStateStore } from "#/stores/state";
  import * as stitchesApi from "#/api/stitches";
  import * as historyApi from "#/api/history";
  import {
    PartStitchDirection,
    StitchKind,
    FullStitch,
    LineStitch,
    NodeStitch,
    PartStitch,
    type Stitch,
  } from "#/schemas/pattern/pattern";
  import { PatternProject } from "#/schemas/pattern/project";
  import type {} from "#/types/pattern/pattern";
  import type { Point } from "pixi.js";

  interface CanvasPanelProps {
    patproj: PatternProject;
  }

  const props = defineProps<CanvasPanelProps>();

  const appStateStore = useAppStateStore();

  const canvas = useTemplateRef("canvas");
  const canvasService = new CanvasService();

  watch(
    () => props.patproj,
    (patproj) => canvasService.drawPattern(patproj),
  );

  let prevStitchState: Stitch | undefined;
  canvasService.addEventListener(EventType.AddStitch, async (e) => {
    const palindex = appStateStore.state.selectedPaletteItemIndex;
    if (palindex === undefined) return;
    // The current pattern is always available here.
    const patternKey = appStateStore.state.currentPattern!.key;
    const tool = appStateStore.state.selectedStitchTool;
    const kind = tool % 2; // Get 0 or 1.

    // A start point is needed to draw the lines.
    // An end point is needed to draw all the other kinds of stitches (in addition to lines).
    const { stage, start, end, alt, fixed }: AddStitchData = (e as CustomEvent).detail;
    const x = adjustStitchCoordinate(end.x, tool);
    const y = adjustStitchCoordinate(end.y, tool);

    switch (tool) {
      case StitchKind.Full:
      case StitchKind.Petite: {
        const full: FullStitch = { x, y, palindex, kind };
        if (fixed && prevStitchState && "full" in prevStitchState) {
          full.x = Math.trunc(x) + (prevStitchState.full.x - Math.trunc(prevStitchState.full.x));
          full.y = Math.trunc(y) + (prevStitchState.full.y - Math.trunc(prevStitchState.full.y));
        }
        if (prevStitchState && stage !== AddStitchEventStage.Continue) prevStitchState = undefined;
        else prevStitchState ??= { full };
        await stitchesApi.addStitch(patternKey, { full });
        break;
      }

      case StitchKind.Half:
      case StitchKind.Quarter: {
        const [fracX, fracY] = [end.x % 1, end.y % 1];
        const direction =
          (fracX < 0.5 && fracY > 0.5) || (fracX > 0.5 && fracY < 0.5)
            ? PartStitchDirection.Forward
            : PartStitchDirection.Backward;
        const part: PartStitch = { x, y, palindex, kind, direction };
        if (fixed && prevStitchState && "part" in prevStitchState) {
          part.direction = prevStitchState.part.direction;
          if (tool === StitchKind.Quarter) {
            part.x = Math.trunc(x) + (prevStitchState.part.x - Math.trunc(prevStitchState.part.x));
            part.y = Math.trunc(y) + (prevStitchState.part.y - Math.trunc(prevStitchState.part.y));
          }
        }
        if (prevStitchState && stage !== AddStitchEventStage.Continue) prevStitchState = undefined;
        else prevStitchState ??= { part };
        await stitchesApi.addStitch(patternKey, { part });
        break;
      }

      case StitchKind.Back:
      case StitchKind.Straight: {
        const [_start, _end] = orderPoints(start, end);
        const line: LineStitch = {
          x: [adjustStitchCoordinate(_start.x, tool), adjustStitchCoordinate(_end.x, tool)],
          y: [adjustStitchCoordinate(_start.y, tool), adjustStitchCoordinate(_end.y, tool)],
          palindex,
          kind,
        };
        if (stage === AddStitchEventStage.End) await stitchesApi.addStitch(patternKey, { line });
        else canvasService.drawLine(line, props.patproj.pattern.palette[palindex]!, true);
        break;
      }

      case StitchKind.FrenchKnot:
      case StitchKind.Bead: {
        const node: NodeStitch = {
          x,
          y,
          palindex,
          kind,
          rotated: alt,
        };
        if (stage === AddStitchEventStage.End) await stitchesApi.addStitch(patternKey, { node });
        else canvasService.drawNode(node, props.patproj.pattern.palette[palindex]!, true);
        break;
      }
    }
  });

  canvasService.addEventListener(EventType.RemoveStitch, async (e) => {
    const data: RemoveStitchData = (e as CustomEvent).detail;
    const patternKey = appStateStore.state.currentPattern!.key;
    await stitchesApi.removeStitch(patternKey, data);
  });

  function adjustStitchCoordinate(value: number, tool: StitchKind): number {
    const int = Math.trunc(value);
    const frac = value - int;
    switch (tool) {
      case StitchKind.Full:
      case StitchKind.Half: {
        return int;
      }
      case StitchKind.Petite:
      case StitchKind.Quarter: {
        return frac > 0.5 ? int + 0.5 : int;
      }
      case StitchKind.Back:
      case StitchKind.Straight:
      case StitchKind.FrenchKnot:
      case StitchKind.Bead: {
        return frac > 0.5 ? int + 1 : frac > 0.25 ? int + 0.5 : int;
      }
    }
  }

  /** Orders points so that is no way to draw two lines with the same coordinates. */
  function orderPoints(start: Point, end: Point): [Point, Point] {
    const x1 = Math.trunc(start.x);
    const y1 = Math.trunc(start.y);
    const x2 = Math.trunc(end.x);
    const y2 = Math.trunc(end.y);
    if (y1 === y2) return x1 < x2 ? [start, end] : [end, start];
    else return y1 < y2 ? [start, end] : [end, start];
  }

  export interface StitchesRemoveManyPayload {
    fullstitches: FullStitch[];
    partstitches: PartStitch[];
    line?: LineStitch;
    node?: NodeStitch;
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
      for (const full of payload.fullstitches) canvasService.drawFullStitch(full, palette[full.palindex]!);
      for (const part of payload.partstitches) canvasService.drawPartStitch(part, palette[part.palindex]!);
      if (payload.line) canvasService.drawLine(payload.line, palette[payload.line.palindex]!);
      if (payload.node) canvasService.drawNode(payload.node, palette[payload.node.palindex]!);
    },
  );
  const unlistenRemoveOneStitch = await appWindow.listen<Stitch>("stitches:remove_one", ({ payload }) => {
    if ("full" in payload) canvasService.removeFullStitch(payload.full);
    if ("part" in payload) canvasService.removePartStitch(payload.part);
    if ("line" in payload) canvasService.removeLine(payload.line);
    if ("node" in payload) canvasService.removeNode(payload.node);
  });
  const unlistenAddOneStitch = await appWindow.listen<Stitch>("stitches:add_one", ({ payload }) => {
    const palette = props.patproj.pattern.palette;
    if ("full" in payload) canvasService.drawFullStitch(payload.full, palette[payload.full.palindex]!);
    if ("part" in payload) canvasService.drawPartStitch(payload.part, palette[payload.part.palindex]!);
    if ("line" in payload) canvasService.drawLine(payload.line, palette[payload.line.palindex]!);
    if ("node" in payload) canvasService.drawNode(payload.node, palette[payload.node.palindex]!);
  });

  const keys = useMagicKeys();
  whenever(keys.ctrl_z!, async () => await historyApi.undo(appStateStore.state.currentPattern!.key));
  whenever(keys.ctrl_y!, async () => await historyApi.redo(appStateStore.state.currentPattern!.key));

  onMounted(async () => {
    const { width, height } = canvas.value!.getBoundingClientRect();
    await canvasService.init({ width, height, canvas: canvas.value! });
    canvasService.drawPattern(props.patproj);

    window.addEventListener(
      "resize",
      useThrottleFn(() => canvasService.resize(canvas.value!.getBoundingClientRect()), 500),
    );
  });

  onUnmounted(() => {
    canvasService.clearPattern();
    unlistenRemoveManyStitches();
    unlistenAddManyStitches();
    unlistenRemoveOneStitch();
    unlistenAddOneStitch();
  });
</script>
