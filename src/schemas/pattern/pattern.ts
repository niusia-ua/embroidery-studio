import { field, fixedArray, option, vec } from "@dao-xyz/borsh";

export class PatternProperties {
  @field({ type: "u16" })
  width: number;

  @field({ type: "u16" })
  height: number;

  constructor(data: PatternProperties) {
    this.width = data.width;
    this.height = data.height;
  }
}

export class PatternInfo {
  @field({ type: "string" })
  title: string;

  @field({ type: "string" })
  author: string;

  @field({ type: "string" })
  company: string;

  @field({ type: "string" })
  copyright: string;

  @field({ type: "string" })
  description: string;

  constructor(data: PatternInfo) {
    this.title = data.title;
    this.author = data.author;
    this.company = data.company;
    this.copyright = data.copyright;
    this.description = data.description;
  }
}

export class PaletteItemStitchStrands {
  @field({ type: option("u8") })
  full?: number;

  @field({ type: option("u8") })
  petite?: number;

  @field({ type: option("u8") })
  half?: number;

  @field({ type: option("u8") })
  quarter?: number;

  @field({ type: option("u8") })
  back?: number;

  @field({ type: option("u8") })
  straight?: number;

  @field({ type: option("u8") })
  frenchKnot?: number;

  @field({ type: option("u8") })
  special?: number;

  constructor(data: PaletteItemStitchStrands) {
    this.full = data.full;
    this.petite = data.petite;
    this.half = data.half;
    this.quarter = data.quarter;
    this.back = data.back;
    this.straight = data.straight;
    this.frenchKnot = data.frenchKnot;
    this.special = data.special;
  }
}

export class Blend {
  @field({ type: "string" })
  brand: string;

  @field({ type: "string" })
  number: string;

  @field({ type: "u8" })
  strands: number;

  constructor(data: Blend) {
    this.brand = data.brand;
    this.number = data.number;
    this.strands = data.strands;
  }
}

export class Bead {
  @field({ type: "f32" })
  length: number;

  @field({ type: "f32" })
  diameter: number;

  constructor(data: Bead) {
    this.length = data.length;
    this.diameter = data.diameter;
  }
}

export class PaletteItem {
  @field({ type: "string" })
  brand: string;

  @field({ type: "string" })
  number: string;

  @field({ type: "string" })
  name: string;

  @field({ type: "string" })
  color: string;

  @field({ type: option(vec(Blend)) })
  blends?: Blend[];

  @field({ type: option(Bead) })
  bead?: Bead;

  @field({ type: option(PaletteItemStitchStrands) })
  strands?: PaletteItemStitchStrands;

  constructor(data: PaletteItem) {
    this.brand = data.brand;
    this.number = data.number;
    this.name = data.name;
    this.color = data.color;
    this.blends = data.blends;
    this.bead = data.bead;
    this.strands = data.strands;
  }
}

export type StitchesPerInch = [number, number];
export class Fabric {
  @field({ type: fixedArray("u16", 2) })
  spi: StitchesPerInch;

  @field({ type: "string" })
  kind: string;

  @field({ type: "string" })
  name: string;

  @field({ type: "string" })
  color: string;

  constructor(data: Fabric) {
    this.spi = data.spi;
    this.kind = data.kind;
    this.name = data.name;
    this.color = data.color;
  }
}

export class FullStitch {
  @field({ type: "f32" })
  x: number;

  @field({ type: "f32" })
  y: number;

  @field({ type: "u8" })
  palindex: number;

  @field({
    serialize: (kind, writer) => (kind === FullStitchKind.Full ? writer.u8(0) : writer.u8(1)),
    deserialize: (reader) => (reader.u8() === 0 ? FullStitchKind.Full : FullStitchKind.Petite),
  })
  kind: FullStitchKind;

  constructor(data: FullStitch) {
    this.x = data.x;
    this.y = data.y;
    this.palindex = data.palindex;
    this.kind = data.kind;
  }
}

export const enum FullStitchKind {
  Full = "Full",
  Petite = "Petite",
}

export class PartStitch {
  @field({ type: "f32" })
  x: number;

  @field({ type: "f32" })
  y: number;

  @field({ type: "u8" })
  palindex: number;

  @field({
    serialize: (direction, writer) => (direction === PartStitchDirection.Forward ? writer.u8(1) : writer.u8(2)),
    deserialize: (reader) => (reader.u8() === 1 ? PartStitchDirection.Forward : PartStitchDirection.Backward),
  })
  direction: PartStitchDirection;

  @field({
    serialize: (kind, writer) => (kind === PartStitchKind.Half ? writer.u8(0) : writer.u8(1)),
    deserialize: (reader) => (reader.u8() === 0 ? PartStitchKind.Half : PartStitchKind.Quarter),
  })
  kind: PartStitchKind;

  constructor(data: PartStitch) {
    this.x = data.x;
    this.y = data.y;
    this.palindex = data.palindex;
    this.direction = data.direction;
    this.kind = data.kind;
  }
}

export const enum PartStitchDirection {
  Forward = "Forward",
  Backward = "Backward",
}

export const enum PartStitchKind {
  Half = "Half",
  Quarter = "Quarter",
}

export class LineStitch {
  @field({ type: fixedArray("f32", 2) })
  x: [number, number];

  @field({ type: fixedArray("f32", 2) })
  y: [number, number];

  @field({ type: "u8" })
  palindex: number;

  @field({
    serialize: (kind, writer) => (kind === LineStitchKind.Back ? writer.u8(0) : writer.u8(1)),
    deserialize: (reader) => (reader.u8() === 0 ? LineStitchKind.Straight : PartStitchKind.Quarter),
  })
  kind: LineStitchKind;

  constructor(data: LineStitch) {
    this.x = data.x;
    this.y = data.y;
    this.palindex = data.palindex;
    this.kind = data.kind;
  }
}

export const enum LineStitchKind {
  Back = "Back",
  Straight = "Straight",
}

export class NodeStitch {
  @field({ type: "f32" })
  x: number;

  @field({ type: "f32" })
  y: number;

  @field({ type: "bool" })
  rotated: boolean;

  @field({ type: "u8" })
  palindex: number;

  @field({
    serialize: (kind, writer) => (kind === NodeStitchKind.FrenchKnot ? writer.u8(0) : writer.u8(1)),
    deserialize: (reader) => (reader.u8() === 0 ? NodeStitchKind.FrenchKnot : NodeStitchKind.Bead),
  })
  kind: NodeStitchKind;

  constructor(data: NodeStitch) {
    this.x = data.x;
    this.y = data.y;
    this.palindex = data.palindex;
    this.rotated = data.rotated;
    this.kind = data.kind;
  }
}

export const enum NodeStitchKind {
  FrenchKnot = "FrenchKnot",
  Bead = "Bead",
}

export class CurvedStitch {
  @field({ type: vec(fixedArray("f32", 2)) })
  points: [number, number][];

  constructor(data: CurvedStitch) {
    this.points = data.points;
  }
}

export class SpecialStitch {
  @field({ type: "f32" })
  x: number;

  @field({ type: "f32" })
  y: number;

  @field({ type: "u16" })
  rotation: number;

  @field({ type: fixedArray("bool", 2) })
  flip: [boolean, boolean];

  @field({ type: "u8" })
  palindex: number;

  @field({ type: "u8" })
  modindex: number;

  constructor(data: SpecialStitch) {
    this.x = data.x;
    this.y = data.y;
    this.palindex = data.palindex;
    this.modindex = data.modindex;
    this.rotation = data.rotation;
    this.flip = data.flip;
  }
}

export class SpecialStitchModel {
  @field({ type: "string" })
  uniqueName: string;

  @field({ type: "string" })
  name: string;

  @field({ type: vec(NodeStitch) })
  nodes: NodeStitch[];

  @field({ type: vec(LineStitch) })
  lines: LineStitch[];

  @field({ type: vec(CurvedStitch) })
  curves: CurvedStitch[];

  constructor(data: SpecialStitchModel) {
    this.uniqueName = data.uniqueName;
    this.name = data.name;
    this.nodes = data.nodes;
    this.lines = data.lines;
    this.curves = data.curves;
  }
}

export class Pattern {
  @field({ type: PatternProperties })
  properties: PatternProperties;

  @field({ type: PatternInfo })
  info: PatternInfo;

  @field({ type: vec(PaletteItem) })
  palette: PaletteItem[];

  @field({ type: Fabric })
  fabric: Fabric;

  @field({ type: vec(FullStitch) })
  fullstitches: FullStitch[];

  @field({ type: vec(PartStitch) })
  partstitches: PartStitch[];

  @field({ type: vec(NodeStitch) })
  nodes: NodeStitch[];

  @field({ type: vec(LineStitch) })
  lines: LineStitch[];

  @field({ type: vec(SpecialStitch) })
  specialstitches: SpecialStitch[];

  @field({ type: vec(SpecialStitchModel) })
  specialStitchModels: SpecialStitchModel[];

  constructor(data: Pattern) {
    this.properties = data.properties;
    this.info = data.info;
    this.palette = data.palette;
    this.fabric = data.fabric;
    this.fullstitches = data.fullstitches;
    this.partstitches = data.partstitches;
    this.nodes = data.nodes;
    this.lines = data.lines;
    this.specialstitches = data.specialstitches;
    this.specialStitchModels = data.specialStitchModels;
  }
}

export type Stitch = { full: FullStitch } | { part: PartStitch } | { node: NodeStitch } | { line: LineStitch };
export type StitchKind = FullStitchKind | PartStitchKind | NodeStitchKind | LineStitchKind;
