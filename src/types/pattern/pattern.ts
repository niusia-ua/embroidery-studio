export interface Pattern {
  properties: PatternProperties;
  info: PatternInfo;
  palette: PaletteItem[];
  fabric: Fabric;
  fullstitches: FullStitch[];
  partstitches: PartStitch[];
  nodes: Node[];
  lines: Line[];
  specialstitches: SpecialStitch[];
  special_stitch_models: SpecialModelStitch[];
}

export interface PatternProperties {
  width: number;
  height: number;
}

export interface PatternInfo {
  title: string;
  author: string;
  company: string;
  copyright: string;
  description: string;
}

export interface PaletteItem {
  brand: string;
  number: string;
  name: string;
  color: string;
  blends?: Blend[];
  bead?: Bead;
  strands: StitchStrands;
}

export interface Blend {
  brand: string;
  number: string;
  strands: number;
}

export interface Bead {
  length: number;
  diameter: number;
}

export interface StitchStrands {
  full?: number;
  petite?: number;
  half?: number;
  quarter?: number;
  back?: number;
  straight?: number;
  french_knot?: number;
  special?: number;
}

export interface Fabric {
  spi: [number, number];
  kind: string;
  name: string;
  color: string;
}

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

export interface SpecialStitch {
  x: number;
  y: number;
  palindex: number;
  modindex: number;
}

export interface SpecialModelStitch {
  uniqueName: string;
  name: string;
  width: number;
  height: number;
  nodes: Node[];
  lines: Line[];
  curves: Curve[];
}

export interface Curve {
  points: [number, number][];
  palindex: number;
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
