import { Application, Container, Graphics, GraphicsContext, Point } from "pixi.js";
import type { FederatedMouseEvent, ColorSource, ApplicationOptions, StrokeInput } from "pixi.js";
import { Viewport } from "pixi-viewport";
import { SpatialHash as Culler } from "pixi-cull";
import { mm2px } from "#/utils/measurement";
import { AddStitchEventStage, EventType } from "./events.types";
import type { AddStitchData, RemoveStitchData } from "./events.types";
import type { PatternProject } from "#/schemas/pattern/project";
import type { Grid } from "#/schemas/pattern/display";
import type {
  FullStitch,
  LineStitch,
  NodeStitch,
  PaletteItem,
  PartStitch,
  PatternProperties,
  SpecialStitch,
  SpecialStitchModel,
} from "#/schemas/pattern/pattern";
import { FullStitchKind, NodeStitchKind, PartStitchDirection, PartStitchKind } from "#/schemas/pattern/pattern";

const SCALE_FACTOR = 10;
const STITCH_STROKE: StrokeInput = { pixelLine: true, alignment: 1, color: 0x000000 };
const FULL_STITCH_CONTEXT = {
  [FullStitchKind.Full]: new GraphicsContext().rect(0, 0, 1, 1).fill(0xffffff),
  [FullStitchKind.Petite]: new GraphicsContext().rect(0, 0, 0.5, 0.5).stroke(STITCH_STROKE).fill(0xffffff),
};
const PART_STITCH_CONTEXT = {
  [PartStitchKind.Half]: {
    [PartStitchDirection.Forward]: new GraphicsContext()
      .poly([1, 0, 1, 0.25, 0.25, 1, 0, 1, 0, 0.75, 0.75, 0])
      .stroke(STITCH_STROKE)
      .fill(0xffffff),
    [PartStitchDirection.Backward]: new GraphicsContext()
      .poly([0, 0, 0.25, 0, 1, 0.75, 1, 1, 0.75, 1, 0, 0.25])
      .stroke(STITCH_STROKE)
      .fill(0xffffff),
  },
  [PartStitchKind.Quarter]: {
    [PartStitchDirection.Forward]: new GraphicsContext()
      .poly([0.5, 0, 0.5, 0.25, 0.25, 0.5, 0, 0.5, 0, 0.25, 0.25, 0])
      .stroke(STITCH_STROKE)
      .fill(0xffffff),
    [PartStitchDirection.Backward]: new GraphicsContext()
      .poly([0, 0, 0.25, 0, 0.5, 0.25, 0.5, 0.5, 0.25, 0.5, 0, 0.25])
      .stroke(STITCH_STROKE)
      .fill(0xffffff),
  },
};

const DEFAULT_INIT_OPTIONS: Partial<ApplicationOptions> = {
  eventMode: "static",
  eventFeatures: { globalMove: false },
  antialias: true,
  backgroundAlpha: 0,
};

export class CanvasService extends EventTarget {
  #pixi = new Application();
  // @ts-expect-error The viewport is initialized in the `init` method.
  #viewport: Viewport;
  #culler = new Culler();
  #stages = {
    // lowest
    fabric: new Graphics(),
    fullstitches: new Container(),
    partstitches: new Container(),
    grid: new Graphics(),
    specialstitches: new Container(),
    lines: new Container(),
    nodes: new Container(),
    hint: new Graphics(),
    // highest
  };

  #specialStitchModelContext: GraphicsContext[] = [];

  #startPoint: Point | undefined = undefined;

  constructor() {
    super();

    // Configure the stages.
    this.#stages.fabric.eventMode = "none";
    this.#stages.grid.eventMode = "none";
    this.#stages.hint.eventMode = "none";
    this.#stages.hint.alpha = 0.5;
  }

  async init(options?: Partial<ApplicationOptions>) {
    await this.#pixi.init(Object.assign({}, DEFAULT_INIT_OPTIONS, options));
    this.#viewport = new Viewport({ events: this.#pixi.renderer.events });

    // Configure the viewport.
    this.#viewport.scale.set(SCALE_FACTOR);
    this.#viewport
      .drag({ keyToPress: ["ShiftLeft"], factor: 2 })
      .wheel()
      .clampZoom({ minScale: 1, maxScale: 100 });

    // Add stages to the viewport.
    for (const stage of Object.values(this.#stages)) {
      this.#viewport.addChild(stage);
      if (stage instanceof Container) this.#culler.addContainer(stage, true);
    }
    this.#pixi.stage.addChild(this.#viewport);

    // Initialize the culler.
    this.#pixi.ticker.add(() => {
      if (this.#viewport.dirty) {
        this.#culler.cull(this.#viewport.getVisibleBounds());
        this.#viewport.dirty = false;
      }
    });

    // Set up event listeners.
    this.#viewport.on("mousedown", this.#onMouseDown, this);
    this.#viewport.on("mouseup", this.#onMouseUp, this);
    this.#viewport.on("mousemove", this.#onMouseMove, this);
  }

  resize({ width, height }: CanvasSize) {
    this.#pixi.renderer.resize(width, height);
    this.#viewport.resize(width, height);
  }

  clearPattern() {
    this.#specialStitchModelContext = [];
    for (const elem of Object.values(this.#stages)) {
      if (elem instanceof Container) elem.removeChildren();
      if (elem instanceof Graphics) elem.clear();
    }
  }

  drawPattern({ pattern, displaySettings }: PatternProject) {
    this.clearPattern();

    this.#viewport.moveCenter(pattern.properties.width / 2, pattern.properties.height / 2);
    this.drawFabric(pattern.properties, pattern.fabric.color);
    this.drawGrid(pattern.properties, displaySettings.grid);

    for (const full of pattern.fullstitches) this.drawFullStitch(full, pattern.palette[full.palindex]!);
    for (const part of pattern.partstitches) this.drawPartStitch(part, pattern.palette[part.palindex]!);
    for (const line of pattern.lines) this.drawLine(line, pattern.palette[line.palindex]!);
    for (const node of pattern.nodes) this.drawNode(node, pattern.palette[node.palindex]!);

    for (const spsModel of pattern.specialStitchModels) this.#prepareSpecialStitchModel(spsModel);
    for (const sps of pattern.specialstitches) this.drawSpecialStitch(sps, pattern.palette[sps.palindex]!.color);
  }

  drawFabric({ width, height }: PatternProperties, color: ColorSource) {
    this.#stages.fabric.rect(0, 0, width, height).fill(color);
    this.#stages.fabric.eventMode = "none";
  }

  drawGrid({ width, height }: PatternProperties, grid: Grid) {
    const graphics = this.#stages.grid;
    graphics.eventMode = "none";
    {
      // Draw horizontal lines.
      for (let i = 1; i < width; i++) {
        graphics.moveTo(i, 0);
        graphics.lineTo(i, height);
      }

      // Draw vertical lines.
      for (let i = 1; i < height; i++) {
        graphics.moveTo(0, i);
        graphics.lineTo(width, i);
      }

      const { thickness, color } = grid.minorScreenLines;
      graphics.stroke({ width: thickness, color: color as ColorSource });
    }
    {
      const interval = grid.majorLineEveryStitches;

      // Draw horizontal lines.
      for (let i = 0; i <= Math.ceil(height / interval); i++) {
        const point = Math.min(i * interval, height);
        graphics.moveTo(0, point);
        graphics.lineTo(width, point);
      }

      // Draw vertical lines.
      for (let i = 0; i <= Math.ceil(width / interval); i++) {
        const point = Math.min(i * interval, width);
        graphics.moveTo(point, 0);
        graphics.lineTo(point, height);
      }

      const { thickness, color } = grid.majorScreenLines;
      graphics.stroke({ width: thickness, color: color as ColorSource });
    }
  }

  drawFullStitch(fullstitch: FullStitch, palitem: PaletteItem) {
    const { x, y, kind } = fullstitch;
    const graphics = new Graphics(FULL_STITCH_CONTEXT[kind]);
    graphics.label = this.#fullStitchKey(fullstitch);
    graphics.tint = palitem.color;
    graphics.position.set(x, y);
    graphics.on("rightup", () => {
      const detail: RemoveStitchData = { full: fullstitch };
      this.dispatchEvent(new CustomEvent(EventType.RemoveStitch, { detail }));
    });
    this.#stages.fullstitches.addChild(graphics);
  }

  removeFullStitches(fullstitches: FullStitch[]) {
    for (const fullstitch of fullstitches) this.removeFullStitch(fullstitch);
  }

  removeFullStitch(fullstitch: FullStitch) {
    const key = this.#fullStitchKey(fullstitch);
    const graphics = this.#stages.fullstitches.getChildByName(key);
    if (graphics) this.#stages.fullstitches.removeChild(graphics);
  }

  #fullStitchKey({ x, y, kind }: FullStitch) {
    const k = kind === FullStitchKind.Full ? 0 : 1;
    return [x, y, k].toString();
  }

  drawPartStitch(partstitch: PartStitch, palitem: PaletteItem) {
    const { x, y, direction, kind } = partstitch;
    const graphics = new Graphics(PART_STITCH_CONTEXT[kind][direction]);
    graphics.label = this.#partStitchKey(partstitch);
    graphics.position.set(x, y);
    graphics.tint = palitem.color;
    graphics.on("rightup", () => {
      const detail: RemoveStitchData = { part: partstitch };
      this.dispatchEvent(new CustomEvent(EventType.RemoveStitch, { detail }));
    });
    this.#stages.partstitches.addChild(graphics);
  }

  removePartStitches(partstitches: PartStitch[]) {
    for (const partstitch of partstitches) this.removePartStitch(partstitch);
  }

  removePartStitch(partstitch: PartStitch) {
    const key = this.#partStitchKey(partstitch);
    const graphics = this.#stages.partstitches.getChildByName(key);
    if (graphics) this.#stages.partstitches.removeChild(graphics);
  }

  #partStitchKey({ x, y, direction, kind }: PartStitch) {
    const d = direction === PartStitchDirection.Forward ? 0 : 1;
    const k = kind === PartStitchKind.Half ? 0 : 1;
    return [x, y, d, k].toString();
  }

  drawLine(line: LineStitch, palitem: PaletteItem, hint = false) {
    const { x, y } = line;
    const start = { x: x[0], y: y[0] };
    const end = { x: x[1], y: y[1] };
    const graphics = hint ? this.#clearHint() : new Graphics();
    graphics
      .moveTo(start.x, start.y)
      .lineTo(end.x, end.y)
      // Draw a line with a larger width to make it look like a border.
      .stroke({ width: 0.225, color: 0x000000, cap: "round" })
      .moveTo(start.x, start.y)
      .lineTo(end.x, end.y)
      // Draw a line with a smaller width to make it look like a fill.
      .stroke({ width: 0.2, color: palitem.color, cap: "round" });
    if (!hint) {
      graphics.label = this.#lineKey(line);
      graphics.on("rightup", () => {
        const detail: RemoveStitchData = { line };
        this.dispatchEvent(new CustomEvent(EventType.RemoveStitch, { detail }));
      });
      this.#stages.lines.addChild(graphics);
    }
  }

  removeLine(line: LineStitch) {
    const key = this.#lineKey(line);
    const graphics = this.#stages.lines.getChildByName(key);
    if (graphics) this.#stages.lines.removeChild(graphics);
  }

  #lineKey({ x, y }: LineStitch) {
    return [x, y].toString();
  }

  drawNode(node: NodeStitch, palitem: PaletteItem, hint = false) {
    const { x, y, kind, rotated } = node;
    const graphics = hint ? this.#clearHint() : new Graphics();
    if (kind === NodeStitchKind.FrenchKnot) graphics.circle(0, 0, 3);
    else {
      const width = mm2px(palitem.bead?.length ?? 1.5);
      const height = mm2px(palitem.bead?.diameter ?? 2.5);
      graphics.roundRect(0, 0, width, height, 2);
      graphics.pivot.set(width / 2, height / 2);
    }
    graphics.stroke(STITCH_STROKE).fill(palitem.color);
    // Actually, we create node graphics in a larger size so that they have more points.
    // We need to divide the size by the `SCALE_FACTOR` to display them in the correct size.
    // This is a workaround to display the graphics in the good quality.
    graphics.scale.set(1 / SCALE_FACTOR);
    graphics.position.set(x, y);
    if (rotated) graphics.angle = 90;
    if (!hint) {
      graphics.label = this.#nodeKey(node);
      graphics.on("rightup", () => {
        const detail: RemoveStitchData = { node };
        this.dispatchEvent(new CustomEvent(EventType.RemoveStitch, { detail }));
      });
      this.#stages.nodes.addChild(graphics);
    }
  }

  removeNode(node: NodeStitch) {
    const key = this.#nodeKey(node);
    const graphics = this.#stages.nodes.getChildByName(key);
    if (graphics) this.#stages.nodes.removeChild(graphics);
  }

  #nodeKey({ x, y }: NodeStitch) {
    return [x, y].toString();
  }

  #prepareSpecialStitchModel(specialStitchModel: SpecialStitchModel) {
    const context = new GraphicsContext();

    for (const { points } of specialStitchModel.curves) {
      // Draw a polyline with a larger width to make it look like a border.
      context.poly(points.flat(), false).stroke({ width: 0.225, color: 0x000000, cap: "round", join: "round" });
      // Draw a polyline with a smaller width to make it look like a fill.
      context.poly(points.flat(), false).stroke({ width: 0.2, cap: "round", join: "round" });
    }

    for (const { x, y } of specialStitchModel.lines) {
      const start = { x: x[0], y: y[0] };
      const end = { x: x[1], y: y[1] };
      context
        // Draw a line with a larger width to make it look like a border.
        .moveTo(start.x, start.y)
        .lineTo(end.x, end.y)
        .stroke({ width: 0.225, color: 0x000000, cap: "round" })
        // Draw a line with a smaller width to make it look like a fill.
        .moveTo(start.x, start.y)
        .lineTo(end.x, end.y)
        .stroke({ width: 0.2, cap: "round" });
    }

    // Decrease the scale factor to draw the nodes with more points.
    context.scale(0.1);
    for (const { x, y } of specialStitchModel.nodes) {
      // All nodes are french knotes there.
      context
        .circle(x * 10, y * 10, 5)
        .stroke(STITCH_STROKE)
        .fill(0xffffff);
    }

    this.#specialStitchModelContext.push(context);
  }

  drawSpecialStitch(specialStitch: SpecialStitch, color: ColorSource) {
    const { x, y, rotation, flip, modindex } = specialStitch;
    const graphics = new Graphics(this.#specialStitchModelContext[modindex]);
    graphics.tint = color;
    graphics.position.set(x, y);
    graphics.angle = rotation;
    if (flip[0]) graphics.scale.x = -1;
    if (flip[1]) graphics.scale.y = -1;
    this.#stages.specialstitches.addChild(graphics);
  }

  #clearHint() {
    const hint = this.#stages.hint.clear().restore();
    hint.angle = 0;
    hint.pivot.set(0, 0);
    hint.scale.set(1, 1);
    hint.position.set(0, 0);
    return hint;
  }

  #fireAddStitchEvent(e: FederatedMouseEvent, stage: AddStitchEventStage) {
    const point = this.#viewport.toWorld(e.global);
    if (this.#pointIsOutside(point)) return;
    const detail: AddStitchData = {
      stage,
      start: this.#startPoint!,
      end: point,
      alt: e.ctrlKey,
      fixed: e.ctrlKey,
    };
    this.dispatchEvent(new CustomEvent(EventType.AddStitch, { detail }));
  }

  #onMouseDown(e: FederatedMouseEvent) {
    if (e.shiftKey) return;
    const point = this.#viewport.toWorld(e.global);
    this.#startPoint = this.#pointIsOutside(point) ? undefined : point;
    if (this.#startPoint === undefined) {
      this.#clearHint();
      return;
    }
    this.#fireAddStitchEvent(e, AddStitchEventStage.Start);
  }

  #onMouseUp(e: FederatedMouseEvent) {
    // If the start point is not set or the shift key is pressed, do nothing.
    // Shift key is used to pan the viewport.
    if (e.shiftKey || this.#startPoint === undefined) {
      this.#clearHint();
      return;
    }
    this.#fireAddStitchEvent(e, AddStitchEventStage.End);
    this.#startPoint = undefined;
    this.#clearHint();
  }

  #onMouseMove(e: FederatedMouseEvent) {
    if (e.shiftKey || this.#startPoint === undefined) {
      this.#clearHint();
      return;
    }
    this.#fireAddStitchEvent(e, AddStitchEventStage.Continue);
  }

  #pointIsOutside({ x, y }: Point) {
    const { width, height } = this.#stages.fabric.getLocalBounds();
    return x <= 0 || y <= 0 || x >= width || y >= height;
  }
}

export interface CanvasSize {
  width: number;
  height: number;
}
