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
  import { Point } from "pixi.js";
  import { CanvasService, type CanvasSize } from "#/services/canvas/canvas.service";
  import { AddStitchEventStage, EventType } from "#/services/canvas/events.types";
  import type { AddStitchData, RemoveStitchData } from "#/services/canvas/events.types";
  import { useAppStateStore } from "#/stores/state";
  import { StitchesApi, HistoryApi } from "#/api";
  import {
    PartStitchDirection,
    FullStitch,
    LineStitch,
    NodeStitch,
    PartStitch,
    FullStitchKind,
    PartStitchKind,
    LineStitchKind,
    NodeStitchKind,
    type Stitch,
    type StitchKind,
  } from "#/schemas/pattern/pattern";
  import { PatternProject } from "#/schemas/pattern/project";

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

    // A start point is needed to draw the lines.
    // An end point is needed to draw all the other kinds of stitches (in addition to lines).
    const { stage, start, end, alt, fixed }: AddStitchData = (e as CustomEvent).detail;
    const { x, y } = adjustStitchCoordinate(end, tool);

    switch (tool) {
      case FullStitchKind.Full:
      case FullStitchKind.Petite: {
        const full: FullStitch = { x, y, palindex, kind: tool };
        prevStitchState ??= { full };
        if (fixed && "full" in prevStitchState) {
          full.x = Math.trunc(x) + (prevStitchState.full.x - Math.trunc(prevStitchState.full.x));
          full.y = Math.trunc(y) + (prevStitchState.full.y - Math.trunc(prevStitchState.full.y));
        }
        await StitchesApi.addStitch(patternKey, { full });
        break;
      }

      case PartStitchKind.Half:
      case PartStitchKind.Quarter: {
        const [fracX, fracY] = [end.x % 1, end.y % 1];
        const direction =
          (fracX < 0.5 && fracY > 0.5) || (fracX > 0.5 && fracY < 0.5)
            ? PartStitchDirection.Forward
            : PartStitchDirection.Backward;
        const part: PartStitch = { x, y, palindex, kind: tool, direction };
        prevStitchState ??= { part };
        if (fixed && "part" in prevStitchState) {
          part.direction = prevStitchState.part.direction;
          if (tool === PartStitchKind.Quarter) {
            part.x = Math.trunc(x) + (prevStitchState.part.x - Math.trunc(prevStitchState.part.x));
            part.y = Math.trunc(y) + (prevStitchState.part.y - Math.trunc(prevStitchState.part.y));
          }
        }
        await StitchesApi.addStitch(patternKey, { part });
        break;
      }

      case LineStitchKind.Back: {
        const [_start, _end] = [adjustStitchCoordinate(start, tool), adjustStitchCoordinate(end, tool)];
        if (_start.equals(new Point()) || _end.equals(new Point())) return;
        const line: LineStitch = { x: [_start.x, _end.x], y: [_start.y, _end.y], palindex, kind: tool };
        if (stage === AddStitchEventStage.Continue && prevStitchState && "line" in prevStitchState) {
          line.x[0] = prevStitchState.line.x[1];
          line.y[0] = prevStitchState.line.y[1];
        }
        if (line.x[0] === line.x[1] && line.y[0] === line.y[1]) return;
        prevStitchState = { line };
        if (stage === AddStitchEventStage.Continue) await StitchesApi.addStitch(patternKey, { line });
        break;
      }

      case LineStitchKind.Straight: {
        const [_start, _end] = orderPoints(start, end);
        const { x: x1, y: y1 } = adjustStitchCoordinate(_start, tool);
        const { x: x2, y: y2 } = adjustStitchCoordinate(_end, tool);
        const line: LineStitch = { x: [x1, x2], y: [y1, y2], palindex, kind: tool };
        if (stage === AddStitchEventStage.End) await StitchesApi.addStitch(patternKey, { line });
        else canvasService.drawLine(line, props.patproj.pattern.palette[palindex]!, true);
        break;
      }

      case NodeStitchKind.FrenchKnot:
      case NodeStitchKind.Bead: {
        const node: NodeStitch = {
          x,
          y,
          palindex,
          kind: tool,
          rotated: alt,
        };
        if (stage === AddStitchEventStage.End) await StitchesApi.addStitch(patternKey, { node });
        else canvasService.drawNode(node, props.patproj.pattern.palette[palindex]!, true);
        break;
      }
    }

    if (stage === AddStitchEventStage.End) prevStitchState = undefined;
  });

  canvasService.addEventListener(EventType.RemoveStitch, async (e) => {
    const data: RemoveStitchData = (e as CustomEvent).detail;
    const patternKey = appStateStore.state.currentPattern!.key;
    await StitchesApi.removeStitch(patternKey, data);
  });

  function adjustStitchCoordinate({ x, y }: Point, tool: StitchKind): Point {
    const [intX, intY] = [Math.trunc(x), Math.trunc(y)];
    const [fracX, fracY] = [x - intX, y - intY];
    switch (tool) {
      case FullStitchKind.Full:
      case PartStitchKind.Half: {
        return new Point(intX, intY);
      }
      case FullStitchKind.Petite:
      case PartStitchKind.Quarter: {
        return new Point(fracX > 0.5 ? intX + 0.5 : intX, fracY > 0.5 ? intY + 0.5 : intY);
      }
      case LineStitchKind.Back: {
        if (fracX <= 0.25 && fracY <= 0.25) return new Point(intX, intY); // top-left
        if (fracX >= 0.75 && fracY <= 0.25) return new Point(intX + 1, intY); // top-right
        if (fracX <= 0.25 && fracY >= 0.75) return new Point(intX, intY + 1); // bottom-left
        if (fracX >= 0.75 && fracY >= 0.75) return new Point(intX + 1, intY + 1); // bottom-right
        return new Point(); // to not handle it
      }
      case LineStitchKind.Straight:
      case NodeStitchKind.FrenchKnot:
      case NodeStitchKind.Bead: {
        return new Point(
          fracX > 0.5 ? intX + 1 : fracX > 0.25 ? intX + 0.5 : intX,
          fracY > 0.5 ? intY + 1 : fracY > 0.25 ? intY + 0.5 : intY,
        );
      }
    }
  }

  /** Orders points so that is no way to draw two lines with the same coordinates. */
  function orderPoints(start: Point, end: Point): [Point, Point] {
    if (start.y < end.y || (start.y === end.y && start.x < end.x)) return [start, end];
    else return [end, start];
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
  whenever(keys.ctrl_z!, async () => await HistoryApi.undo(appStateStore.state.currentPattern!.key));
  whenever(keys.ctrl_y!, async () => await HistoryApi.redo(appStateStore.state.currentPattern!.key));

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
