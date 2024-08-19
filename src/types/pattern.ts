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

export interface PatternProperties {
  width: number;
  height: number;
}

export interface PatternInfo {
  title: string;
  author: string;
  copyright: string;
  description: string;
}

export interface PaletteItem {
  vendorId: number;
  number: string;
  name: string;
  color: string;
  blends: Blend[];
}

export interface Blend {
  vendorId: number;
  number: string;
  strands: number;
}

export interface Bead {
  length: number;
  diameter: number;
}

export interface Fabric {
  stitchesPerInch: [number, number];
  kind: string;
  name: string;
  color: string;
}

interface StitchBase<K, C = number> {
  x: C;
  y: C;
  palindex: number;
  kind: K;
}

export type FullStitch = StitchBase<FullStitchKind>;
export type FullStitchKind = keyof typeof FullStitchKind;
export const FullStitchKind = { Full: "Full", Petite: "Petite" } as const;

export type PartStitch = StitchBase<PartStitchKind> & { direction: PartStitchDirection };
export type PartStitchKind = keyof typeof PartStitchKind;
export const PartStitchKind = { Half: "Half", Quarter: "Quarter" } as const;
export type PartStitchDirection = keyof typeof PartStitchDirection;
export const PartStitchDirection = { Forward: "Forward", Backward: "Backward" } as const;

export type Node = StitchBase<NodeKind> & { rotation: boolean };
export type NodeKind = keyof typeof NodeKind;
export const NodeKind = { FrenchKnot: "FrenchKnot", Bead: "Bead" } as const;

export type Line = StitchBase<LineKind, [number, number]>;
export type LineKind = keyof typeof LineKind;
export const LineKind = { Back: "Back", Straight: "Straight" } as const;
