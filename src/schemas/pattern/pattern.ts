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

export class StitchStrands {
  @field({ type: option("u16") })
  full?: number;

  @field({ type: option("u16") })
  petite?: number;

  @field({ type: option("u16") })
  half?: number;

  @field({ type: option("u16") })
  quarter?: number;

  @field({ type: option("u16") })
  back?: number;

  @field({ type: option("u16") })
  straight?: number;

  @field({ type: option("u16") })
  frenchKnot?: number;

  @field({ type: option("u16") })
  special?: number;

  constructor(data: StitchStrands) {
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

  @field({ type: option(StitchStrands) })
  strands?: StitchStrands;

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

export class Fabric {
  @field({ type: fixedArray("u16", 2) })
  spi: [number, number];

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

  @field({ type: "u8" })
  kind: FullStitchKind;

  constructor(data: FullStitch) {
    this.x = data.x;
    this.y = data.y;
    this.palindex = data.palindex;
    this.kind = data.kind;
  }
}

export const enum FullStitchKind {
  Full = 0,
  Petite = 1,
}

export class PartStitch {
  @field({ type: "f32" })
  x: number;

  @field({ type: "f32" })
  y: number;

  @field({ type: "u8" })
  palindex: number;

  @field({ type: "u8" })
  direction: PartStitchDirection;

  @field({ type: "u8" })
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
  Forward = 1,
  Backward = 2,
}

export const enum PartStitchKind {
  Half = 0,
  Quarter = 1,
}

export class LineStitch {
  @field({ type: fixedArray("f32", 2) })
  x: [number, number];

  @field({ type: fixedArray("f32", 2) })
  y: [number, number];

  @field({ type: "u8" })
  palindex: number;

  @field({ type: "u8" })
  kind: LineStitchKind;

  constructor(data: LineStitch) {
    this.x = data.x;
    this.y = data.y;
    this.palindex = data.palindex;
    this.kind = data.kind;
  }
}

export const enum LineStitchKind {
  Back = 0,
  Straight = 1,
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

  @field({ type: "u8" })
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
  FrenchKnot = 0,
  Bead = 1,
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

  @field({ type: "u16" })
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

export const enum StitchKind {
  Full = 0,
  Petite = 1,
  Half = 2,
  Quarter = 3,
  Back = 4,
  Straight = 5,
  FrenchKnot = 6,
  Bead = 7,
}

export type Stitch = { full: FullStitch } | { part: PartStitch } | { node: NodeStitch } | { line: LineStitch };
