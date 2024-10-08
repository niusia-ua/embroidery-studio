<template>
  <div ref="canvasContainer" class="h-full"></div>
</template>

<script lang="ts" setup>
  import { onMounted, ref, watch } from "vue";
  import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
  import { borshDeserialize } from "borsher";
  import { CanvasService } from "#/services/canvas";
  import { useAppStateStore } from "#/stores/state";
  import { PartStitchDirection, StitchKind } from "#/schemas/pattern";
  import { emitStitchCreated, emitStitchRemoved } from "#/services/events/pattern";
  import {
    RemovedStitchEventPayloadSchema,
    type RemovedStitchPayload,
    type StitchEventPayload,
  } from "#/schemas/events/pattern";
  import type { FullStitch, Line, Node, PartStitch, Pattern } from "#/schemas/pattern";

  interface CanvasPanelProps {
    pattern: Pattern;
  }

  const props = defineProps<CanvasPanelProps>();

  const appStateStore = useAppStateStore();

  const canvasContainer = ref<HTMLDivElement>();
  const canvasService = new CanvasService();

  onMounted(() => {
    // Resizing the canvas to set its initial size.
    canvasService.resize(canvasContainer.value!.getBoundingClientRect());
    window.addEventListener("resize", () => canvasService.resize(canvasContainer.value!.getBoundingClientRect()));
    canvasContainer.value!.appendChild(canvasService.view as HTMLCanvasElement);
    canvasService.drawPattern(props.pattern);
  });

  watch(
    () => props.pattern,
    (pattern) => canvasService.drawPattern(pattern),
  );

  // A start point is needed to draw the lines.
  // An end point is needed to draw all the other kinds of stitches (in addition to lines).
  canvasService.addEventListener("draw", async (e) => {
    if (!appStateStore.state.selectedPaletteItem) return;

    // @ts-ignore
    const { start, end, modifier } = e.detail;

    const x = Math.trunc(end.x);
    const y = Math.trunc(end.y);

    // Decimal portion of the end coordinates.
    const xdp = end.x - x;
    const ydp = end.y - y;

    // The current pattern is always available here.
    const patternKey = appStateStore.state.currentPattern!.key;
    const palitem = appStateStore.state.selectedPaletteItem;
    const palindex = props.pattern.palette.findIndex((pi) => pi.color === palitem.color);

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
        await emitStitchCreated(patternKey, { full: fullstitch });
        canvasService.drawFullStitch(fullstitch, palitem.color);
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
        await emitStitchCreated(patternKey, { part: partstitch });
        canvasService.drawPartStitch(partstitch, palitem.color);
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
        await emitStitchCreated(patternKey, { line });
        canvasService.drawLine(line, palitem.color);
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
        await emitStitchCreated(patternKey, { node });
        canvasService.drawNode(node, palitem.color);
        break;
      }
    }
  });

  // TODO: Don't duplicate this code.
  canvasService.addEventListener("remove", async (e) => {
    if (!appStateStore.state.selectedPaletteItem) return;

    // @ts-ignore
    const { point } = e.detail;

    const x = Math.trunc(point.x);
    const y = Math.trunc(point.y);

    // Decimal portion of the end coordinates.
    const xdp = point.x - x;
    const ydp = point.y - y;

    // The current pattern is always available here.
    const patternKey = appStateStore.state.currentPattern!.key;
    const palitem = appStateStore.state.selectedPaletteItem;
    const palindex = props.pattern.palette.findIndex((pi) => pi.color === palitem.color);

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
        await emitStitchRemoved(patternKey, { full: fullstitch });
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
        await emitStitchRemoved(patternKey, { part: partstitch });
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
        await emitStitchRemoved(patternKey, { node });
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

  const appWindow = getCurrentWebviewWindow();
  appWindow.listen<Uint8Array>("pattern:stitches:remove", (e) => {
    const { payload } = borshDeserialize<StitchEventPayload<RemovedStitchPayload>>(
      RemovedStitchEventPayloadSchema,
      e.payload,
    );
    if (payload.fullstitches) canvasService.removeFullStitches(payload.fullstitches);
    if (payload.partstitches) canvasService.removePartStitches(payload.partstitches);
    if (payload.line) canvasService.removeLine(payload.line);
    if (payload.node) canvasService.removeNode(payload.node);
  });
</script>
