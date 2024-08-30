<template>
  <div class="h-full flex flex-column">
    <Toolbar
      class="border-noround border-none border-bottom-1 p-0"
      draggable="true"
      @dragstart="() => appWindow.startDragging()"
    >
      <template #start>
        <DropdownTieredMenu
          id="general_menu"
          :button="{ icon: 'pi pi-bars' }"
          :tiered-menu="{ model: menuOptions }"
        />
        <StitchToolSelector />
      </template>

      <template #end>
        <WindowControls />
      </template>
    </Toolbar>

    <Splitter :gutter-size="2" class="h-full border-noround border-none">
      <SplitterPanel :min-size="5" :size="15">
        <PalettePanel :palette="patternStore.pattern?.palette ?? []" />
      </SplitterPanel>

      <SplitterPanel :min-size="85" :size="85">
        <div ref="root" class="h-full"></div>
      </SplitterPanel>
    </Splitter>
  </div>
</template>

<script lang="ts" setup>
  import { open } from "@tauri-apps/api/dialog";
  import { appWindow } from "@tauri-apps/api/window";
  import { Simple as SimpleCull } from "pixi-cull";
  import { Viewport } from "pixi-viewport";
  import { Application, Container, Graphics, LINE_CAP, Point, Polygon } from "pixi.js";
  import type { MenuItem } from "primevue/menuitem";
  import Splitter from "primevue/splitter";
  import SplitterPanel from "primevue/splitterpanel";
  import Toolbar from "primevue/toolbar";
  import { onMounted, ref } from "vue";
  import { loadPattern } from "./commands/pattern";
  import PalettePanel from "./components/PalettePanel.vue";
  import DropdownTieredMenu from "./components/toolbar/DropdownTieredMenu.vue";
  import StitchToolSelector from "./components/toolbar/StitchToolSelector.vue";
  import WindowControls from "./components/toolbar/WindowControls.vue";
  import { usePatternStore } from "./stores/pattern";
  import { useAppStateStore } from "./stores/state";
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
    type PatternProperties,
    type StitchKind,
  } from "./types/pattern";
  import type { GridSettings } from "./types/view";
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
                extensions: ["xsd", "oxs", "xml"],
              },
            ],
          });
          if (file === null || Array.isArray(file)) return;
          patternStore.pattern = await loadPattern(file);
          drawPattern();
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
  const menuOptions = ref<MenuItem[]>([fileOptions]);

  const root = ref<HTMLElement>();
  const pixi = new Application({ backgroundAlpha: 0 });
  const viewport = new Viewport({ events: pixi.renderer.events });
  const cull = new SimpleCull();
  // The order determines the sequence of layering according to property indices.
  const stages = {
    fabric: new Graphics(),
    fullstitches: new Container(),
    partstitches: new Container(),
    lines: new Container(),
    nodes: new Container(),
    grid: new Graphics(),
  };

  const stateStore = useAppStateStore();
  const patternStore = usePatternStore();

  function resizePixi() {
    const { width, height } = root.value!.getBoundingClientRect();
    pixi.renderer.resize(width, height);
    viewport.resize(width, height);
  }

  onMounted(() => {
    // Configuring the viewport.
    viewport.scale.set(10);
    viewport.drag({ keyToPress: ["ControlLeft"], factor: 2 }).wheel();
    viewport.clampZoom({
      minScale: 1,
      maxScale: 100,
    });

    // Adding stages to the viewport.
    for (const elem of Object.values(stages)) {
      viewport.addChild(elem);
      if (elem instanceof Graphics) cull.add(elem);
      else cull.addList(elem.children);
    }
    pixi.stage.addChild(viewport);

    // Initializing culling.
    cull.cull(viewport.getVisibleBounds());
    pixi.ticker.add(() => {
      if (viewport.dirty) {
        cull.cull(viewport.getVisibleBounds());
        viewport.dirty = false;
      }
    });

    // Resizing the canvas to set its initial size.
    resizePixi();
    window.addEventListener("resize", resizePixi);

    // @ts-expect-error There is type mismatch here, but it is actually working as expected.
    root.value!.appendChild(pixi.view);

    drawPattern();
  });

  function drawPattern() {
    if (!patternStore.pattern) return;
    clearPattern();

    viewport.moveCenter(
      patternStore.pattern.properties.width / 2,
      patternStore.pattern.properties.height / 2,
    );

    drawFabric(patternStore.pattern.properties, patternStore.pattern.fabric.color);
    drawGrid(patternStore.pattern.properties, GRID_SETTINGS);
    for (const fullstitch of patternStore.pattern.fullstitches) drawFullStitch(fullstitch);
    for (const partstitch of patternStore.pattern.partstitches) drawPartStitch(partstitch);
    for (const node of patternStore.pattern.nodes) drawNode(node);
    for (const line of patternStore.pattern.lines) drawLine(line);
  }

  function clearPattern() {
    for (const elem of Object.values(stages)) {
      if (elem instanceof Graphics) elem.clear();
      else elem.removeChildren();
    }
  }

  function drawFabric({ width, height }: PatternProperties, color: string) {
    stages.fabric.beginFill(color).drawRect(0, 0, width, height).endFill();
  }

  const GRID_SETTINGS: GridSettings = {
    majorLinesEveryStitches: 10,
    minorLines: {
      thickness: 0.05,
      color: "000000",
    },
    majorLines: {
      thickness: 0.1,
      color: "000000",
    },
  };

  function drawGrid({ width, height }: PatternProperties, gridSettings: GridSettings) {
    const graphics = stages.grid;
    {
      // Drawing major grid lines.
      const interval = gridSettings.majorLinesEveryStitches;
      const { thickness, color } = gridSettings.majorLines;
      graphics.lineStyle({ width: thickness, color });
      for (let i = 0; i < width / interval; i++) {
        graphics.moveTo(i * interval, 0);
        graphics.lineTo(i * interval, height);
      }
      for (let i = 0; i < height / interval; i++) {
        graphics.moveTo(0, i * interval);
        graphics.lineTo(width, i * interval);
      }
    }
    {
      // Drawing minor grid lines.
      const { thickness, color } = gridSettings.minorLines;
      graphics.lineStyle({ width: thickness, color });
      for (let i = 0; i < width; i++) {
        graphics.moveTo(i, 0);
        graphics.lineTo(i, height);
      }
      for (let i = 0; i < height; i++) {
        graphics.moveTo(0, i);
        graphics.lineTo(width, i);
      }
    }
  }

  const FULL_STITCH_GEOMETRY = {
    Full: new Graphics().beginFill("FFFFFF").drawRect(0, 0, 1, 1).endFill().geometry,
    Petite: new Graphics().beginFill("FFFFFF").drawRect(0, 0, 0.5, 0.5).endFill().geometry,
  };

  function drawFullStitch({ x, y, palindex, kind }: FullStitch) {
    const graphics = new Graphics(FULL_STITCH_GEOMETRY[kind]);
    graphics.tint = patternStore.pattern!.palette[palindex].color;
    graphics.position.set(x, y);
    stages.fullstitches.addChild(graphics);
  }

  const PART_STITCH_GEOMETRY = {
    Half: {
      Forward: new Graphics()
        .beginFill("FFFFFF")
        .drawPolygon(new Polygon([1, 0, 1, 0.25, 0.25, 1, 0, 1, 0, 0.75, 0.75, 0]))
        .endFill().geometry,
      Backward: new Graphics()
        .beginFill("FFFFFF")
        .drawPolygon(new Polygon([0, 0, 0.25, 0, 1, 0.75, 1, 1, 0.75, 1, 0, 0.25]))
        .endFill().geometry,
    },
    Quarter: {
      Forward: new Graphics()
        .lineStyle({ width: 0.01, alignment: 1 })
        .beginFill("FFFFFF")
        .drawPolygon(new Polygon([0.5, 0, 0.5, 0.25, 0.25, 0.5, 0, 0.5, 0, 0.25, 0.25, 0]))
        .endFill().geometry,
      Backward: new Graphics()
        .lineStyle({ width: 0.01, alignment: 1 })
        .beginFill("FFFFFF")
        .drawPolygon(new Polygon([0, 0, 0.25, 0, 0.5, 0.25, 0.5, 0.5, 0.25, 0.5, 0, 0.25]))
        .endFill().geometry,
    },
  };

  function drawPartStitch({ x, y, palindex, direction, kind }: PartStitch) {
    const graphics = new Graphics(PART_STITCH_GEOMETRY[kind][direction]);
    graphics.tint = patternStore.pattern!.palette[palindex].color;
    graphics.position.set(x, y);
    stages.partstitches.addChild(graphics);
  }

  const NODE_GEOMETRY = {
    FrenchKnot: new Graphics()
      .beginFill("FFFFFF")
      .lineStyle({ width: 0.1, color: 0x000000, alignment: 1 })
      .drawCircle(0, 0, 5)
      .endFill().geometry,
    Bead: new Graphics()
      .beginFill("FFFFFF")
      .lineStyle({ width: 0.1, color: 0x000000, alignment: 1 })
      // Set negative coordinates to rotate elements around their center.
      .drawRoundedRect(-3.75, -5, 7.5, 10, 10)
      .endFill().geometry,
  };

  function drawNode({ x, y, palindex, rotated, kind }: Node) {
    const graphics = new Graphics(NODE_GEOMETRY[kind]);
    // Actually, we create node graphics in a larger size so that they have more points.
    // We need to divide the size by 10 to display them in the correct size.
    // This is a workaround to display the graphics in the good quality.
    graphics.height /= 10;
    graphics.width /= 10;
    graphics.tint = patternStore.pattern!.palette[palindex].color;
    graphics.position.set(x, y);
    if (rotated) graphics.angle = 90;
    stages.nodes.addChild(graphics);
  }

  function drawLine({ x, y, palindex }: Line) {
    const start = { x: x[0], y: y[0] };
    const end = { x: x[1], y: y[1] };
    const color = patternStore.pattern!.palette[palindex].color;
    const cap = LINE_CAP.ROUND;
    const graphics = new Graphics()
      // Draw a line with a larger width to make it look like a border.
      .moveTo(start.x, start.y)
      .lineStyle({ width: 0.225, cap })
      .lineTo(end.x, end.y)
      // Draw a line with a smaller width to make it look like a fill.
      .lineStyle({ width: 0.2, color, cap })
      .lineTo(start.x, start.y);
    stages.lines.addChild(graphics);
  }

  // A start point used for drawing lines.
  let startPoint: Point | null = null;

  viewport.addEventListener("mousedown", (e) => (startPoint = viewport.toWorld(e.global)));
  viewport.addEventListener("mouseup", (e) => {
    if (!patternStore.pattern || !stateStore.state.selectedPaletteItem || !startPoint) return;

    const point = viewport.toWorld(e.global);
    if (isOutsideOfPattern(point, patternStore.pattern.properties)) return;

    const x = Math.trunc(point.x);
    const y = Math.trunc(point.y);
    const xr = point.x - x;
    const yr = point.y - y;

    const palindex = patternStore.pattern.palette.indexOf(stateStore.state.selectedPaletteItem);
    const kind = stateStore.state.selectedStitchTool;
    switch (kind) {
      case FullStitchKind.Full:
      case FullStitchKind.Petite: {
        const fullstitch: FullStitch = {
          x: adjustStitchCoordinate(x, xr, kind),
          y: adjustStitchCoordinate(y, yr, kind),
          palindex,
          kind,
        };
        patternStore.pattern.fullstitches.push(fullstitch);
        drawFullStitch(fullstitch);
        break;
      }

      case PartStitchKind.Half:
      case PartStitchKind.Quarter: {
        const direction =
          (xr < 0.5 && yr > 0.5) || (xr > 0.5 && yr < 0.5)
            ? PartStitchDirection.Forward
            : PartStitchDirection.Backward;
        const partstitch: PartStitch = {
          x: adjustStitchCoordinate(x, xr, kind),
          y: adjustStitchCoordinate(y, yr, kind),
          palindex,
          kind,
          direction,
        };
        patternStore.pattern.partstitches.push(partstitch);
        drawPartStitch(partstitch);
        break;
      }

      case LineKind.Back:
      case LineKind.Straight: {
        const startX = Math.trunc(startPoint.x);
        const startY = Math.trunc(startPoint.y);

        const line: Line = {
          x: [
            adjustStitchCoordinate(startX, startPoint.x - startX, kind),
            adjustStitchCoordinate(x, xr, kind),
          ],
          y: [
            adjustStitchCoordinate(startY, startPoint.y - startY, kind),
            adjustStitchCoordinate(y, yr, kind),
          ],
          palindex,
          kind,
        };
        patternStore.pattern.lines.push(line);
        drawLine(line);
        break;
      }

      case NodeKind.FrenchKnot:
      case NodeKind.Bead: {
        const node: Node = {
          x: adjustStitchCoordinate(x, xr, kind),
          y: adjustStitchCoordinate(y, yr, kind),
          palindex,
          kind,
          rotated: e.ctrlKey,
        };
        patternStore.pattern.nodes.push(node);
        drawNode(node);
        break;
      }
    }

    startPoint = null;
  });

  function isOutsideOfPattern({ x, y }: Point, { width, height }: PatternProperties) {
    return x < 0 || y < 0 || x >= width || y >= height;
  }

  function adjustStitchCoordinate(value: number, decimalPortion: number, kind: StitchKind): number {
    if (kind === FullStitchKind.Full || kind === PartStitchKind.Half) return value;
    if (kind === FullStitchKind.Petite || kind === PartStitchKind.Quarter) {
      return decimalPortion > 0.5 ? value + 0.5 : value;
    }
    return decimalPortion > 0.5 ? value + 1 : decimalPortion > 0.25 ? value + 0.5 : value;
  }
</script>
