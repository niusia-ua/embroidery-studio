<template>
  <div ref="canvasContainer" class="h-full"></div>
</template>

<script lang="ts" setup>
  import { CanvasService } from "#/services/canvas";
  import { useAppStateStore } from "#/stores/state";
  import {
    FullStitchKind,
    LineKind,
    NodeKind,
    PartStitchDirection,
    PartStitchKind,
    type FullStitch,
    type Line,
    type Node,
    type PartStitch,
    type Pattern,
    type StitchKind,
  } from "#/types/pattern";
  import { appWindow } from "@tauri-apps/api/window";
  import { onMounted, ref } from "vue";

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
    window.addEventListener("resize", () =>
      canvasService.resize(canvasContainer.value!.getBoundingClientRect()),
    );

    canvasContainer.value!.appendChild(canvasService.view as HTMLCanvasElement);

    canvasService.drawPattern(props.pattern);
  });

  // A start point is needed to draw the lines.
  // An end point is needed to draw all the other kinds of stitches (in addition to lines).
  canvasService.onDraw(async (start, end, ctrl) => {
    if (!appStateStore.state.selectedPaletteItem) return;

    const x = Math.trunc(end.x);
    const y = Math.trunc(end.y);
    // Decimal portion of the end coordinates.
    const xdp = end.x - x;
    const ydp = end.y - y;

    const palitem = appStateStore.state.selectedPaletteItem;
    const palindex = props.pattern.palette.indexOf(palitem);
    const kind = appStateStore.state.selectedStitchTool;
    switch (kind) {
      case FullStitchKind.Full:
      case FullStitchKind.Petite: {
        const fullstitch: FullStitch = {
          x: adjustStitchCoordinate(x, xdp, kind),
          y: adjustStitchCoordinate(y, ydp, kind),
          palindex,
          kind,
        };
        await appWindow.emit("pattern:stitch:create", { fullstitch });
        canvasService.drawFullStitch(fullstitch, palitem.color);
        break;
      }

      case PartStitchKind.Half:
      case PartStitchKind.Quarter: {
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
        await appWindow.emit("pattern:stitch:create", { partstitch });
        canvasService.drawPartStitch(partstitch, palitem.color);
        break;
      }

      case LineKind.Back:
      case LineKind.Straight: {
        const startX = Math.trunc(start.x);
        const startY = Math.trunc(start.y);

        const line: Line = {
          x: [
            adjustStitchCoordinate(startX, start.x - startX, kind),
            adjustStitchCoordinate(x, xdp, kind),
          ],
          y: [
            adjustStitchCoordinate(startY, start.y - startY, kind),
            adjustStitchCoordinate(y, ydp, kind),
          ],
          palindex,
          kind,
        };
        await appWindow.emit("pattern:stitch:create", { line });
        canvasService.drawLine(line, palitem.color);
        break;
      }

      case NodeKind.FrenchKnot:
      case NodeKind.Bead: {
        const node: Node = {
          x: adjustStitchCoordinate(x, xdp, kind),
          y: adjustStitchCoordinate(y, ydp, kind),
          palindex,
          kind,
          rotated: ctrl,
        };
        await appWindow.emit("pattern:stitch:create", { node });
        canvasService.drawNode(node, palitem.color);
        break;
      }
    }
  });

  function adjustStitchCoordinate(value: number, decimalPortion: number, kind: StitchKind): number {
    if (kind === FullStitchKind.Full || kind === PartStitchKind.Half) return value;
    if (kind === FullStitchKind.Petite || kind === PartStitchKind.Quarter) {
      return decimalPortion > 0.5 ? value + 0.5 : value;
    }
    return decimalPortion > 0.5 ? value + 1 : decimalPortion > 0.25 ? value + 0.5 : value;
  }

  interface EventStitchRemovePayload {
    fullstitches?: FullStitch[];
    partstitches?: PartStitch[];
    line?: Line;
    node?: Node;
  }

  appWindow.listen<EventStitchRemovePayload>("pattern:stitch:remove", ({ payload }) => {
    if (payload.fullstitches) canvasService.removeFullStitches(payload.fullstitches);
    if (payload.partstitches) canvasService.removePartStitches(payload.partstitches);
    if (payload.line) canvasService.removeLine(payload.line);
    if (payload.node) canvasService.removeNode(payload.node);
  });
</script>
