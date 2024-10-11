import { BorshSchema } from "borsher";

export const PatternPropertiesSchema = BorshSchema.Struct({
  width: BorshSchema.u16,
  height: BorshSchema.u16,
});

export const PatternInfoSchema = BorshSchema.Struct({
  title: BorshSchema.String,
  author: BorshSchema.String,
  copyright: BorshSchema.String,
  description: BorshSchema.String,
});

export const BlendSchema = BorshSchema.Struct({
  brand: BorshSchema.String,
  number: BorshSchema.String,
  strands: BorshSchema.u8,
});

export const PaletteItemSchema = BorshSchema.Struct({
  brand: BorshSchema.String,
  number: BorshSchema.String,
  name: BorshSchema.String,
  color: BorshSchema.String,
  blends: BorshSchema.Option(BorshSchema.Vec(BlendSchema)),
});

export const FabricSchema = BorshSchema.Struct({
  spi: BorshSchema.Array(BorshSchema.u16, 2),
  kind: BorshSchema.String,
  name: BorshSchema.String,
  color: BorshSchema.String,
});

export const FullStitchSchema = BorshSchema.Struct({
  x: BorshSchema.f32,
  y: BorshSchema.f32,
  palindex: BorshSchema.u8,
  kind: BorshSchema.u8,
});

export const PartStitchSchema = BorshSchema.Struct({
  x: BorshSchema.f32,
  y: BorshSchema.f32,
  palindex: BorshSchema.u8,
  direction: BorshSchema.u8,
  kind: BorshSchema.u8,
});

export const NodeSchema = BorshSchema.Struct({
  x: BorshSchema.f32,
  y: BorshSchema.f32,
  rotated: BorshSchema.bool,
  palindex: BorshSchema.u8,
  kind: BorshSchema.u8,
});

export const LineSchema = BorshSchema.Struct({
  x: BorshSchema.Array(BorshSchema.f32, 2),
  y: BorshSchema.Array(BorshSchema.f32, 2),
  palindex: BorshSchema.u8,
  kind: BorshSchema.u8,
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
});
