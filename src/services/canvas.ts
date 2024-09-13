import { Application, Container, Graphics, LINE_CAP, Point, Polygon, type ColorSource } from "pixi.js";
import { Viewport } from "pixi-viewport";
import { Simple as SimpleCulling } from "pixi-cull";
import { FullStitchKind, PartStitchDirection, PartStitchKind } from "#/types/pattern";
import type { FullStitch, Line, Node, PartStitch, Pattern, PatternProperties } from "#/types/pattern";
import type { GridSettings } from "#/types/view";

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

const FULL_STITCH_GEOMETRIES = {
  Full: new Graphics().beginFill("FFFFFF").drawRect(0, 0, 1, 1).endFill().geometry,
  Petite: new Graphics().lineStyle({ width: 0.01, alignment: 0 }).beginFill("FFFFFF").drawRect(0, 0, 0.5, 0.5).endFill()
    .geometry,
};

const PART_STITCH_GEOMETRIES = {
  Half: {
    Forward: new Graphics()
      .lineStyle({ width: 0.01, alignment: 0 })
      .beginFill("FFFFFF")
      .drawPolygon(new Polygon([1, 0, 1, 0.25, 0.25, 1, 0, 1, 0, 0.75, 0.75, 0]))
      .endFill().geometry,
    Backward: new Graphics()
      .lineStyle({ width: 0.01, alignment: 0 })
      .beginFill("FFFFFF")
      .drawPolygon(new Polygon([0, 0, 0.25, 0, 1, 0.75, 1, 1, 0.75, 1, 0, 0.25]))
      .endFill().geometry,
  },
  Quarter: {
    Forward: new Graphics()
      .lineStyle({ width: 0.01, alignment: 0 })
      .beginFill("FFFFFF")
      .drawPolygon(new Polygon([0.5, 0, 0.5, 0.25, 0.25, 0.5, 0, 0.5, 0, 0.25, 0.25, 0]))
      .endFill().geometry,
    Backward: new Graphics()
      .lineStyle({ width: 0.01, alignment: 0 })
      .beginFill("FFFFFF")
      .drawPolygon(new Polygon([0, 0, 0.25, 0, 0.5, 0.25, 0.5, 0.5, 0.25, 0.5, 0, 0.25]))
      .endFill().geometry,
  },
};

const NODE_GEOMETRIES = {
  FrenchKnot: new Graphics()
    .beginFill("FFFFFF")
    .lineStyle({ width: 0.1, color: 0x000000, alignment: 0 })
    .drawCircle(0, 0, 5)
    .endFill().geometry,
  Bead: new Graphics()
    .beginFill("FFFFFF")
    .lineStyle({ width: 0.1, color: 0x000000, alignment: 0 })
    // Set negative coordinates to rotate elements around their center.
    .drawRoundedRect(-3.75, -5, 7.5, 10, 10)
    .endFill().geometry,
};

export class CanvasService {
  #pixi = new Application({ backgroundAlpha: 0 });
  #viewport = new Viewport({ events: this.#pixi.renderer.events });
  #culler = new SimpleCulling();
  #stages = {
    fabric: new Graphics(),
    fullstitches: new Container(),
    partstitches: new Container(),
    lines: new Container(),
    nodes: new Container(),
    grid: new Graphics(),
  };

  #startPoint: Point | undefined = undefined;

  constructor() {
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
    for (const elem of Object.values(this.#stages)) {
      this.#viewport.addChild(elem);
      if (elem instanceof Graphics) this.#culler.add(elem);
      else this.#culler.addList(elem.children);
    }
    this.#pixi.stage.addChild(this.#viewport);

    // Initialize the culler.
    this.#culler.cull(this.#viewport.getVisibleBounds());
    this.#pixi.ticker.add(() => {
      if (this.#viewport.dirty) {
        this.#culler.cull(this.#viewport.getVisibleBounds());
        this.#viewport.dirty = false;
      }
    });
  }

  get view() {
    return this.#pixi.view;
  }

  resize({ width, height }: DOMRect) {
    this.#pixi.renderer.resize(width, height);
    this.#viewport.resize(width, height);
  }

  clearPattern() {
    for (const elem of Object.values(this.#stages)) {
      if (elem instanceof Graphics) elem.clear();
      else elem.removeChildren();
    }
  }

  drawPattern(pattern: Pattern) {
    this.clearPattern();
    this.#viewport.moveCenter(pattern.properties.width / 2, pattern.properties.height / 2);
    this.drawFabric(pattern.properties, pattern.fabric.color);
    this.drawGrid(pattern.properties, GRID_SETTINGS);
    // prettier-ignore
    for (const fullstitch of pattern.fullstitches) this.drawFullStitch(fullstitch, pattern.palette[fullstitch.palindex]!.color);
    // prettier-ignore
    for (const partstitch of pattern.partstitches) this.drawPartStitch(partstitch, pattern.palette[partstitch.palindex]!.color);
    for (const line of pattern.lines) this.drawLine(line, pattern.palette[line.palindex]!.color);
    for (const node of pattern.nodes) this.drawNode(node, pattern.palette[node.palindex]!.color);
  }

  drawFabric({ width, height }: PatternProperties, color: ColorSource) {
    this.#stages.fabric.beginFill(color).drawRect(0, 0, width, height).endFill();
  }

  drawGrid({ width, height }: PatternProperties, gridSettings: GridSettings) {
    const graphics = this.#stages.grid;
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

  drawFullStitch(fullstitch: FullStitch, color: ColorSource) {
    const { x, y, kind } = fullstitch;
    const graphics = new Graphics(FULL_STITCH_GEOMETRIES[kind]);
    graphics.name = this.#fullStitchKey(fullstitch);
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
    const graphics = new Graphics(PART_STITCH_GEOMETRIES[kind][direction]);
    graphics.name = this.#partStitchKey(partstitch);
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
    const cap = LINE_CAP.ROUND;
    const graphics = new Graphics()
      // Draw a line with a larger width to make it look like a border.
      .moveTo(start.x, start.y)
      .lineStyle({ width: 0.225, cap })
      .lineTo(end.x, end.y)
      // Draw a line with a smaller width to make it look like a fill.
      .lineStyle({ width: 0.2, color, cap })
      .lineTo(start.x, start.y);
    graphics.name = this.#lineKey(line);
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
    const graphics = new Graphics(NODE_GEOMETRIES[kind]);
    graphics.name = this.#nodeKey(node);
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

  onDraw(callback: (start: Point, end: Point, ctrl: boolean) => void) {
    this.#viewport.addEventListener("mousedown", (e) => {
      const point = this.#viewport.toWorld(e.global);
      this.#startPoint = this.#pointIsOutside(point) ? undefined : point;
    });

    this.#viewport.addEventListener("mouseup", (e) => {
      // If the start point is not set or the shift key is pressed, do nothing.
      // Shift key is used to pan the viewport.
      if (!this.#startPoint || e.shiftKey) return;

      const point = this.#viewport.toWorld(e.global);
      if (this.#pointIsOutside(point)) return;

      const [start, end] = this.#orderPoints(this.#startPoint, point);

      // TODO: Improve the way to detect the control key.
      // Control key is used to change the rotation of the node.
      callback(start!, end!, e.ctrlKey);
      this.#startPoint = undefined;
    });
  }

  #pointIsOutside({ x, y }: Point) {
    const { width, height } = this.#stages.fabric.getLocalBounds();
    return x <= 0 || y <= 0 || x >= width || y >= height;
  }

  // Order points so that is no way to draw two lines with the same coordinates.
  #orderPoints(start: Point, end: Point) {
    const x1 = Math.trunc(start.x);
    const y1 = Math.trunc(start.y);
    const x2 = Math.trunc(end.x);
    const y2 = Math.trunc(end.y);

    if (y1 === y2) return x1 < x2 ? [start, end] : [end, start];
    else return y1 < y2 ? [start, end] : [end, start];
  }
}
