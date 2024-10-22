import { BorshSchema } from "borsher";

const SymbolsSchema = BorshSchema.Struct({
  full: BorshSchema.Option(BorshSchema.u16),
  petite: BorshSchema.Option(BorshSchema.u16),
  half: BorshSchema.Option(BorshSchema.u16),
  quarter: BorshSchema.Option(BorshSchema.u16),
  french_knot: BorshSchema.Option(BorshSchema.u16),
  bead: BorshSchema.Option(BorshSchema.u16),
});

const SymbolSettingsSchema = BorshSchema.Struct({
  screenSpacing: BorshSchema.Array(BorshSchema.u16, 2),
  printerSpacing: BorshSchema.Array(BorshSchema.u16, 2),
  scaleUsingMaximumFontWidth: BorshSchema.bool,
  scaleUsingFontHeight: BorshSchema.bool,
  stitchSize: BorshSchema.u16,
  smallStitchSize: BorshSchema.u16,
  drawSymbolsOverBackstitches: BorshSchema.bool,
  showStitchColor: BorshSchema.bool,
  useLargeHalfStitchSymbol: BorshSchema.bool,
  useTrianglesBehindQuarterStitches: BorshSchema.bool,
});

const SymbolFormatSchema = BorshSchema.Struct({
  useAltBgColor: BorshSchema.bool,
  bgColor: BorshSchema.String,
  fgColor: BorshSchema.String,
});

const LineFormatSchema = BorshSchema.Struct({
  useAltColor: BorshSchema.bool,
  color: BorshSchema.String,
  style: BorshSchema.u8,
  thickness: BorshSchema.f32,
});

const NodeFormatSchema = BorshSchema.Struct({
  useDotStyle: BorshSchema.bool,
  useAltColor: BorshSchema.bool,
  color: BorshSchema.String,
  diameter: BorshSchema.f32,
});

const FontFormatSchema = BorshSchema.Struct({
  fontName: BorshSchema.Option(BorshSchema.String),
  bold: BorshSchema.bool,
  italic: BorshSchema.bool,
  stitchSize: BorshSchema.u16,
  smallStitchSize: BorshSchema.u16,
});

const FormatsSchema = BorshSchema.Struct({
  symbol: SymbolFormatSchema,
  back: LineFormatSchema,
  straight: LineFormatSchema,
  french: NodeFormatSchema,
  bead: NodeFormatSchema,
  special: LineFormatSchema,
  font: FontFormatSchema,
});

const GridLineStyleSchema = BorshSchema.Struct({
  color: BorshSchema.String,
  thickness: BorshSchema.f32,
});

const GridSchema = BorshSchema.Struct({
  majorLineEveryStitches: BorshSchema.u16,
  minorScreenLines: GridLineStyleSchema,
  majorScreenLines: GridLineStyleSchema,
  minorPrinterLines: GridLineStyleSchema,
  majorPrinterLines: GridLineStyleSchema,
});

const StitchOutlineSchema = BorshSchema.Struct({
  color: BorshSchema.Option(BorshSchema.String),
  colorPercentage: BorshSchema.u16,
  thickness: BorshSchema.f32,
});

const DefaultStitchStrandsSchema = BorshSchema.Struct({
  full: BorshSchema.u16,
  petite: BorshSchema.u16,
  half: BorshSchema.u16,
  quarter: BorshSchema.u16,
  back: BorshSchema.u16,
  straight: BorshSchema.u16,
  special: BorshSchema.u16,
});

const StitchSettingsSchema = BorshSchema.Struct({
  defaultStrands: DefaultStitchStrandsSchema,
  displayThickness: BorshSchema.Array(BorshSchema.f32, 13),
});

export const DisplaySettingsSchema = BorshSchema.Struct({
  defaultStitchFont: BorshSchema.String,
  symbols: BorshSchema.Vec(SymbolsSchema),
  symbolSettings: SymbolSettingsSchema,
  formats: BorshSchema.Vec(FormatsSchema),
  grid: GridSchema,
  view: BorshSchema.u8,
  zoom: BorshSchema.u16,
  showGrid: BorshSchema.bool,
  showRulers: BorshSchema.bool,
  showCenteringMarks: BorshSchema.bool,
  showFabricColorsWithSymbols: BorshSchema.bool,
  gapsBetweenStitches: BorshSchema.bool,
  outlinedStitches: BorshSchema.bool,
  stitchOutline: StitchOutlineSchema,
  stitchSettings: StitchSettingsSchema,
});
