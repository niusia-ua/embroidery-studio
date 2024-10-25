import { BorshSchema } from "borsher";

const PatternPropertiesSchema = BorshSchema.Struct({
  width: BorshSchema.u16,
  height: BorshSchema.u16,
});

const PatternInfoSchema = BorshSchema.Struct({
  title: BorshSchema.String,
  author: BorshSchema.String,
  company: BorshSchema.String,
  copyright: BorshSchema.String,
  description: BorshSchema.String,
});

const StitchStrandsSchema = BorshSchema.Struct({
  full: BorshSchema.Option(BorshSchema.u16),
  petite: BorshSchema.Option(BorshSchema.u16),
  half: BorshSchema.Option(BorshSchema.u16),
  quarter: BorshSchema.Option(BorshSchema.u16),
  back: BorshSchema.Option(BorshSchema.u16),
  straight: BorshSchema.Option(BorshSchema.u16),
  frenchKnot: BorshSchema.Option(BorshSchema.u16),
  special: BorshSchema.Option(BorshSchema.u16),
});

const BlendSchema = BorshSchema.Struct({
  brand: BorshSchema.String,
  number: BorshSchema.String,
  strands: BorshSchema.u8,
});

const BeadSchema = BorshSchema.Struct({
  lenght: BorshSchema.f32,
  diameter: BorshSchema.f32,
});

const PaletteItemSchema = BorshSchema.Struct({
  brand: BorshSchema.String,
  number: BorshSchema.String,
  name: BorshSchema.String,
  color: BorshSchema.String,
  blends: BorshSchema.Option(BorshSchema.Vec(BlendSchema)),
  bead: BorshSchema.Option(BeadSchema),
  strands: StitchStrandsSchema,
});

const FabricSchema = BorshSchema.Struct({
  spi: BorshSchema.Array(BorshSchema.u16, 2),
  kind: BorshSchema.String,
  name: BorshSchema.String,
  color: BorshSchema.String,
});

const FullStitchSchema = BorshSchema.Struct({
  x: BorshSchema.f32,
  y: BorshSchema.f32,
  palindex: BorshSchema.u8,
  kind: BorshSchema.u8,
});

const PartStitchSchema = BorshSchema.Struct({
  x: BorshSchema.f32,
  y: BorshSchema.f32,
  palindex: BorshSchema.u8,
  direction: BorshSchema.u8,
  kind: BorshSchema.u8,
});

const NodeSchema = BorshSchema.Struct({
  x: BorshSchema.f32,
  y: BorshSchema.f32,
  rotated: BorshSchema.bool,
  palindex: BorshSchema.u8,
  kind: BorshSchema.u8,
});

const LineSchema = BorshSchema.Struct({
  x: BorshSchema.Array(BorshSchema.f32, 2),
  y: BorshSchema.Array(BorshSchema.f32, 2),
  palindex: BorshSchema.u8,
  kind: BorshSchema.u8,
});

const CurveSchema = BorshSchema.Struct({
  points: BorshSchema.Vec(BorshSchema.Array(BorshSchema.f32, 2)),
  palindex: BorshSchema.u8,
});

const SpecialStitchSchema = BorshSchema.Struct({
  x: BorshSchema.f32,
  y: BorshSchema.f32,
  palindex: BorshSchema.u8,
  modindex: BorshSchema.u16,
});

const SpecialStitchModelSchema = BorshSchema.Struct({
  uniqueName: BorshSchema.String,
  name: BorshSchema.String,
  width: BorshSchema.u16,
  height: BorshSchema.u16,
  nodes: BorshSchema.Vec(NodeSchema),
  lines: BorshSchema.Vec(LineSchema),
  curves: BorshSchema.Vec(CurveSchema),
});

export const PatternSchema = BorshSchema.Struct({
  properties: PatternPropertiesSchema,
  info: PatternInfoSchema,
  palette: BorshSchema.Vec(PaletteItemSchema),
  fabric: FabricSchema,
  fullstitches: BorshSchema.Vec(FullStitchSchema),
  partstitches: BorshSchema.Vec(PartStitchSchema),
  nodes: BorshSchema.Vec(NodeSchema),
  lines: BorshSchema.Vec(LineSchema),
  specialstitches: BorshSchema.Vec(SpecialStitchSchema),
  specialStitchModels: BorshSchema.Vec(SpecialStitchModelSchema),
});
