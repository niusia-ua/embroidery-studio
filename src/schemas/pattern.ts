import { BorshSchema } from "borsher";

export const PatternPropertiesSchema = BorshSchema.Struct({
  width: BorshSchema.u16,
  height: BorshSchema.u16,
});

export interface PatternProperties {
  width: number;
  height: number;
}

export const PatternInfoSchema = BorshSchema.Struct({
  title: BorshSchema.String,
  author: BorshSchema.String,
  copyright: BorshSchema.String,
  description: BorshSchema.String,
});

export interface PatternInfo {
  title: string;
  author: string;
  copyright: string;
  description: string;
}

export const BlendSchema = BorshSchema.Struct({
  brand: BorshSchema.String,
  number: BorshSchema.String,
  strands: BorshSchema.u8,
});

export interface Blend {
  brand: string;
  number: string;
  strands: number;
}

export const PaletteItemSchema = BorshSchema.Struct({
  brand: BorshSchema.String,
  number: BorshSchema.String,
  name: BorshSchema.String,
  color: BorshSchema.String,
  blends: BorshSchema.Option(BorshSchema.Vec(BlendSchema)),
});

export interface PaletteItem {
  brand: string;
  number: string;
  name: string;
  color: string;
  blends?: Blend[];
}

export const FabricSchema = BorshSchema.Struct({
  spi: BorshSchema.Array(BorshSchema.u16, 2),
  kind: BorshSchema.String,
  name: BorshSchema.String,
  color: BorshSchema.String,
});

export interface Fabric {
  spi: [number, number];
  kind: string;
  name: string;
  color: string;
}

export const FullStitchSchema = BorshSchema.Struct({
  x: BorshSchema.f32,
  y: BorshSchema.f32,
  palindex: BorshSchema.u8,
  kind: BorshSchema.u8,
});

export interface FullStitch {
  x: number;
  y: number;
  palindex: number;
  kind: FullStitchKind;
}
export const enum FullStitchKind {
  Full = 0,
  Petite = 1,
}

export const PartStitchSchema = BorshSchema.Struct({
  x: BorshSchema.f32,
  y: BorshSchema.f32,
  palindex: BorshSchema.u8,
  direction: BorshSchema.u8,
  kind: BorshSchema.u8,
});

export interface PartStitch {
  x: number;
  y: number;
  palindex: number;
  direction: PartStitchDirection;
  kind: PartStitchKind;
}
export const enum PartStitchDirection {
  Forward = 1,
  Backward = 2,
}
export const enum PartStitchKind {
  Half = 0,
  Quarter = 1,
}

export const NodeSchema = BorshSchema.Struct({
  x: BorshSchema.f32,
  y: BorshSchema.f32,
  rotated: BorshSchema.bool,
  palindex: BorshSchema.u8,
  kind: BorshSchema.u8,
});

export interface Node {
  x: number;
  y: number;
  rotated: boolean;
  palindex: number;
  kind: NodeKind;
}
export const enum NodeKind {
  FrenchKnot = 0,
  Bead = 1,
}

export const LineSchema = BorshSchema.Struct({
  x: BorshSchema.Array(BorshSchema.f32, 2),
  y: BorshSchema.Array(BorshSchema.f32, 2),
  palindex: BorshSchema.u8,
  kind: BorshSchema.u8,
});

export interface Line {
  x: [number, number];
  y: [number, number];
  palindex: number;
  kind: LineKind;
}
export const enum LineKind {
  Back = 0,
  Straight = 1,
}

export const PatternSchema = BorshSchema.Struct({
  properties: PatternPropertiesSchema,
  info: PatternInfoSchema,
  palette: BorshSchema.Vec(PaletteItemSchema),
  fabric: FabricSchema,
  fullstitches: BorshSchema.Vec(FullStitchSchema),
  partstitches: BorshSchema.Vec(PartStitchSchema),
  nodes: BorshSchema.Vec(NodeSchema),
  lines: BorshSchema.Vec(LineSchema),
});

export interface Pattern {
  properties: PatternProperties;
  info: PatternInfo;
  palette: PaletteItem[];
  fabric: Fabric;
  fullstitches: FullStitch[];
  partstitches: PartStitch[];
  nodes: Node[];
  lines: Line[];
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
