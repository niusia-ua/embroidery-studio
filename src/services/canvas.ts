import { Application, Container, Graphics, GraphicsContext, Point } from "pixi.js";
import type { FederatedMouseEvent, ColorSource } from "pixi.js";
import { Viewport } from "pixi-viewport";
import { SpatialHash as Culler } from "pixi-cull";
import type { PatternProject } from "#/types/pattern/project";
import type { Grid } from "#/types/pattern/display";
import type {
  FullStitch,
  Line,
  Node,
  PartStitch,
  PatternProperties,
  SpecialStitch,
  SpecialStitchModel,
} from "#/types/pattern/pattern";
import { FullStitchKind, NodeKind, PartStitchDirection, PartStitchKind } from "#/types/pattern/pattern";

export interface CanvasSize {
  width: number;
  height: number;
}

const FULL_STITCH_CONTEXT = {
  [FullStitchKind.Full]: new GraphicsContext().rect(0, 0, 1, 1).fill("FFFFFF"),
  [FullStitchKind.Petite]: new GraphicsContext()
    .rect(0, 0, 0.5, 0.5)
    .stroke({ width: 0.01, alignment: 0, color: "000000" })
    .fill("FFFFFF"),
};

const PART_STITCH_CONTEXT = {
  [PartStitchKind.Half]: {
    [PartStitchDirection.Forward]: new GraphicsContext()
      .poly([1, 0, 1, 0.25, 0.25, 1, 0, 1, 0, 0.75, 0.75, 0])
      .stroke({ width: 0.01, alignment: 0, color: "000000" })
      .fill("FFFFFF"),
    [PartStitchDirection.Backward]: new GraphicsContext()
      .poly([0, 0, 0.25, 0, 1, 0.75, 1, 1, 0.75, 1, 0, 0.25])
      .stroke({ width: 0.01, alignment: 0, color: "000000" })
      .fill("FFFFFF"),
  },
  [PartStitchKind.Quarter]: {
    [PartStitchDirection.Forward]: new GraphicsContext()
      .poly([0.5, 0, 0.5, 0.25, 0.25, 0.5, 0, 0.5, 0, 0.25, 0.25, 0])
      .stroke({ width: 0.01, alignment: 0, color: "000000" })
      .fill("FFFFFF"),
    [PartStitchDirection.Backward]: new GraphicsContext()
      .poly([0, 0, 0.25, 0, 0.5, 0.25, 0.5, 0.5, 0.25, 0.5, 0, 0.25])
      .stroke({ width: 0.01, alignment: 0, color: "000000" })
      .fill("FFFFFF"),
  },
};

const NODE_CONTEXT = {
  [NodeKind.FrenchKnot]: new GraphicsContext()
    .circle(0, 0, 5)
    .stroke({ width: 0.1, alignment: 0, color: "000000" })
    .fill("FFFFFF"),
  [NodeKind.Bead]: new GraphicsContext()
    // Set negative coordinates to rotate elements around their center.
    .roundRect(-3.75, -5, 7.5, 10, 10)
    .stroke({ width: 0.1, alignment: 0, color: "000000" })
    .fill("FFFFFF"),
};

export class CanvasService extends EventTarget {
  #pixi = new Application();
  // @ts-expect-error The viewport is initialized in the `init` method.
  #viewport: Viewport;
  #culler = new Culler();
  #stages = {
    fabric: new Graphics(),
    fullstitches: new Container(),
    partstitches: new Container(),
    grid: new Graphics(),
    specialstitches: new Container(),
    lines: new Container(),
    nodes: new Container(),
  };

  #specialStitchModelContext: GraphicsContext[] = [];

  #startPoint: Point | undefined = undefined;

  constructor() {
    super();
  }

  async init() {
    await this.#pixi.init({ antialias: true, backgroundAlpha: 0 });
    this.#viewport = new Viewport({ events: this.#pixi.renderer.events });

    // Configure the viewport.
    this.#viewport.scale.set(10);
    this.#viewport
      .drag({ keyToPress: ["ShiftLeft"], factor: 2 })
      .wheel()
      .clampZoom({
        minScale: 1,
        maxScale: 100,
      });

    // Add stages to the viewport.
    for (const stage of Object.values(this.#stages)) {
      stage.interactiveChildren = false;
      stage.eventMode = "none";
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
    this.#viewport.on("rightup", this.#onRightUp, this);
  }

  get view() {
    return this.#pixi.canvas;
  }

  resize({ width, height }: CanvasSize) {
    this.#pixi.renderer.resize(width, height);
    this.#viewport.resize(width, height);
  }

  clearPattern() {
    this.#specialStitchModelContext = [];
    for (const elem of Object.values(this.#stages)) {
      if (elem instanceof Graphics) elem.clear();
      else elem.removeChildren();
    }
  }

  drawPattern({ pattern, displaySettings }: PatternProject) {
    this.clearPattern();

    this.#viewport.moveCenter(pattern.properties.width / 2, pattern.properties.height / 2);
    this.drawFabric(pattern.properties, pattern.fabric.color);
    this.drawGrid(pattern.properties, displaySettings.grid);

    // prettier-ignore
    for (const fullstitch of pattern.fullstitches) this.drawFullStitch(fullstitch, pattern.palette[fullstitch.palindex]!.color);
    // prettier-ignore
    for (const partstitch of pattern.partstitches) this.drawPartStitch(partstitch, pattern.palette[partstitch.palindex]!.color);
    for (const line of pattern.lines) this.drawLine(line, pattern.palette[line.palindex]!.color);
    for (const node of pattern.nodes) this.drawNode(node, pattern.palette[node.palindex]!.color);

    for (const spsModel of pattern.specialStitchModels) this.#prepareSpecialStitchModel(spsModel);
    for (const sps of pattern.specialstitches) this.drawSpecialStitch(sps, pattern.palette[sps.palindex]!.color);
  }

  drawFabric({ width, height }: PatternProperties, color: ColorSource) {
    this.#stages.fabric.rect(0, 0, width, height).fill(color);
  }

  drawGrid({ width, height }: PatternProperties, grid: Grid) {
    const graphics = this.#stages.grid;
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

  drawFullStitch(fullstitch: FullStitch, color: ColorSource) {
    const { x, y, kind } = fullstitch;
    const graphics = new Graphics(FULL_STITCH_CONTEXT[kind]);
    graphics.label = this.#fullStitchKey(fullstitch);
    graphics.tint = color;
    graphics.position.set(x, y);
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

  drawPartStitch(partstitch: PartStitch, color: ColorSource) {
    const { x, y, direction, kind } = partstitch;
    const graphics = new Graphics(PART_STITCH_CONTEXT[kind][direction]);
    graphics.label = this.#partStitchKey(partstitch);
    graphics.position.set(x, y);
    graphics.tint = color;
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

  drawLine(line: Line, color: ColorSource) {
    const { x, y } = line;
    const start = { x: x[0], y: y[0] };
    const end = { x: x[1], y: y[1] };
    const graphics = new Graphics()
      // Draw a line with a larger width to make it look like a border.
      .moveTo(start.x, start.y)
      .lineTo(end.x, end.y)
      .stroke({ width: 0.225, color: "000000", cap: "round" })
      // Draw a line with a smaller width to make it look like a fill.
      .moveTo(start.x, start.y)
      .lineTo(end.x, end.y)
      .stroke({ width: 0.2, color, cap: "round" });
    graphics.label = this.#lineKey(line);
    this.#stages.lines.addChild(graphics);
  }

  removeLine(line: Line) {
    const key = this.#lineKey(line);
    const graphics = this.#stages.lines.getChildByName(key);
    if (graphics) this.#stages.lines.removeChild(graphics);
  }

  #lineKey({ x, y }: Line) {
    return [x, y].toString();
  }

  drawNode(node: Node, color: ColorSource) {
    const { x, y, kind, rotated } = node;
    const graphics = new Graphics(NODE_CONTEXT[kind]);
    graphics.label = this.#nodeKey(node);
    // Actually, we create node graphics in a larger size so that they have more points.
    // We need to divide the size by 10 to display them in the correct size.
    // This is a workaround to display the graphics in the good quality.
    graphics.height /= 10;
    graphics.width /= 10;
    graphics.tint = color;
    graphics.position.set(x, y);
    if (rotated) graphics.angle = 90;
    this.#stages.nodes.addChild(graphics);
  }

  removeNode(node: Node) {
    const key = this.#nodeKey(node);
    const graphics = this.#stages.nodes.getChildByName(key);
    if (graphics) this.#stages.nodes.removeChild(graphics);
  }

  #nodeKey({ x, y }: Node) {
    return [x, y].toString();
  }

  #prepareSpecialStitchModel(specialStitchModel: SpecialStitchModel) {
    const context = new GraphicsContext();

    for (const { points } of specialStitchModel.curves) {
      // Draw a polyline with a larger width to make it look like a border.
      context.poly(points.flat(), false).stroke({ width: 0.225, color: "000000", cap: "round", join: "round" });
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
        .stroke({ width: 0.225, color: "000000", cap: "round" })
        // Draw a line with a smaller width to make it look like a fill.
        .moveTo(start.x, start.y)
        .lineTo(end.x, end.y)
        .stroke({ width: 0.2, cap: "round" });
    }

    // Decrease the scale factor to draw the nodes with more points.
    context.scale(0.1);
    for (const { x, y } of specialStitchModel.nodes) {
      // All nodes are french knotes.
      context
        .circle(x * 10, y * 10, 5)
        .stroke({ width: 0.01, alignment: 0, color: "000000" })
        .fill("FFFFFF");
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

  #onMouseDown(e: FederatedMouseEvent) {
    const point = this.#viewport.toWorld(e.global);
    this.#startPoint = this.#pointIsOutside(point) ? undefined : point;
  }

  #onMouseUp(e: FederatedMouseEvent) {
    // If the start point is not set or the shift key is pressed, do nothing.
    // Shift key is used to pan the viewport.
    if (!this.#startPoint || e.shiftKey) return;

    const point = this.#viewport.toWorld(e.global);
    if (this.#pointIsOutside(point)) return;

    const [start, end] = this.#orderPoints(this.#startPoint, point);

    this.dispatchEvent(new CustomEvent("draw", { detail: { start, end, modifier: e.ctrlKey } }));
    this.#startPoint = undefined;
  }

  #onRightUp(e: FederatedMouseEvent) {
    const point = this.#viewport.toWorld(e.global);
    if (this.#pointIsOutside(point)) return;

    this.dispatchEvent(new CustomEvent("remove", { detail: { point } }));
  }

  #pointIsOutside({ x, y }: Point) {
    const { width, height } = this.#stages.fabric.getLocalBounds();
    return x <= 0 || y <= 0 || x >= width || y >= height;
  }

  // Order points so that is no way to draw two lines with the same coordinates.
  #orderPoints(start: Point, end: Point): [Point, Point] {
    const x1 = Math.trunc(start.x);
    const y1 = Math.trunc(start.y);
    const x2 = Math.trunc(end.x);
    const y2 = Math.trunc(end.y);

    if (y1 === y2) return x1 < x2 ? [start, end] : [end, start];
    else return y1 < y2 ? [start, end] : [end, start];
  }
}
