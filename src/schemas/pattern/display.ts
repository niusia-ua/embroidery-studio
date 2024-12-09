import { field, fixedArray, option, vec } from "@dao-xyz/borsh";

export class Symbols {
  @field({ type: option("u16") })
  full?: number;

  @field({ type: option("u16") })
  petite?: number;

  @field({ type: option("u16") })
  half?: number;

  @field({ type: option("u16") })
  quarter?: number;

  @field({ type: option("u16") })
  frenchKnot?: number;

  @field({ type: option("u16") })
  bead?: number;

  constructor(data: Symbols) {
    this.full = data.full;
    this.petite = data.petite;
    this.half = data.half;
    this.quarter = data.quarter;
    this.frenchKnot = data.frenchKnot;
    this.bead = data.bead;
  }
}

export class SymbolSettings {
  @field({ type: fixedArray("u16", 2) })
  screenSpacing: [number, number];

  @field({ type: fixedArray("u16", 2) })
  printerSpacing: [number, number];

  @field({ type: "bool" })
  scaleUsingMaximumFontWidth: boolean;

  @field({ type: "bool" })
  scaleUsingFontHeight: boolean;

  @field({ type: "u16" })
  stitchSize: number;

  @field({ type: "u16" })
  smallStitchSize: number;

  @field({ type: "bool" })
  drawSymbolsOverBackstitches: boolean;

  @field({ type: "bool" })
  showStitchColor: boolean;

  @field({ type: "bool" })
  useLargeHalfStitchSymbol: boolean;

  @field({ type: "bool" })
  useTrianglesBehindQuarterStitches: boolean;

  constructor(data: SymbolSettings) {
    this.screenSpacing = data.screenSpacing;
    this.printerSpacing = data.printerSpacing;
    this.scaleUsingMaximumFontWidth = data.scaleUsingMaximumFontWidth;
    this.scaleUsingFontHeight = data.scaleUsingFontHeight;
    this.stitchSize = data.stitchSize;
    this.smallStitchSize = data.smallStitchSize;
    this.drawSymbolsOverBackstitches = data.drawSymbolsOverBackstitches;
    this.showStitchColor = data.showStitchColor;
    this.useLargeHalfStitchSymbol = data.useLargeHalfStitchSymbol;
    this.useTrianglesBehindQuarterStitches = data.useTrianglesBehindQuarterStitches;
  }
}

export class SymbolFormat {
  @field({ type: "bool" })
  useAltBgColor: boolean;

  @field({ type: "string" })
  bgColor: string;

  @field({ type: "string" })
  fgColor: string;

  constructor(data: SymbolFormat) {
    this.useAltBgColor = data.useAltBgColor;
    this.bgColor = data.bgColor;
    this.fgColor = data.fgColor;
  }
}

export class LineStitchFormat {
  @field({ type: "bool" })
  useAltColor: boolean;

  @field({ type: "string" })
  color: string;

  @field({ type: "u8" })
  style: LineStitchStyle;

  @field({ type: "f32" })
  thickness: number;

  constructor(data: LineStitchFormat) {
    this.useAltColor = data.useAltColor;
    this.color = data.color;
    this.style = data.style;
    this.thickness = data.thickness;
  }
}

export class NodeStitchFormat {
  @field({ type: "bool" })
  useDotStyle: boolean;

  @field({ type: "bool" })
  useAltColor: boolean;

  @field({ type: "string" })
  color: string;

  @field({ type: "f32" })
  diameter: number;

  constructor(data: NodeStitchFormat) {
    this.useDotStyle = data.useDotStyle;
    this.useAltColor = data.useAltColor;
    this.color = data.color;
    this.diameter = data.diameter;
  }
}

export class FontFormat {
  @field({ type: option("string") })
  fontName?: string;

  @field({ type: "bool" })
  bold: boolean;

  @field({ type: "bool" })
  italic: boolean;

  @field({ type: "u16" })
  stitchSize: number;

  @field({ type: "u16" })
  smallStitchSize: number;

  constructor(data: FontFormat) {
    this.fontName = data.fontName;
    this.bold = data.bold;
    this.italic = data.italic;
    this.stitchSize = data.stitchSize;
    this.smallStitchSize = data.smallStitchSize;
  }
}

export class Formats {
  @field({ type: SymbolFormat })
  symbol: SymbolFormat;

  @field({ type: LineStitchFormat })
  backstitch: LineStitchFormat;

  @field({ type: LineStitchFormat })
  straightstitch: LineStitchFormat;

  @field({ type: NodeStitchFormat })
  frenchKnot: NodeStitchFormat;

  @field({ type: NodeStitchFormat })
  bead: NodeStitchFormat;

  @field({ type: LineStitchFormat })
  specialstitch: LineStitchFormat;

  @field({ type: FontFormat })
  font: FontFormat;

  constructor(data: Formats) {
    this.symbol = data.symbol;
    this.backstitch = data.backstitch;
    this.straightstitch = data.straightstitch;
    this.frenchKnot = data.frenchKnot;
    this.bead = data.bead;
    this.specialstitch = data.specialstitch;
    this.font = data.font;
  }
}

export class GridLineStyle {
  @field({ type: "string" })
  color: string;

  @field({ type: "f32" })
  thickness: number;

  constructor(data: GridLineStyle) {
    this.color = data.color;
    this.thickness = data.thickness;
  }
}

export class Grid {
  @field({ type: "u16" })
  majorLineEveryStitches: number;

  @field({ type: GridLineStyle })
  minorScreenLines: GridLineStyle;

  @field({ type: GridLineStyle })
  majorScreenLines: GridLineStyle;

  @field({ type: GridLineStyle })
  minorPrinterLines: GridLineStyle;

  @field({ type: GridLineStyle })
  majorPrinterLines: GridLineStyle;

  constructor(data: Grid) {
    this.majorLineEveryStitches = data.majorLineEveryStitches;
    this.minorScreenLines = data.minorScreenLines;
    this.majorScreenLines = data.majorScreenLines;
    this.minorPrinterLines = data.minorPrinterLines;
    this.majorPrinterLines = data.majorPrinterLines;
  }
}

export class StitchOutline {
  @field({ type: option("string") })
  color?: string;

  @field({ type: "u16" })
  colorPercentage: number;

  @field({ type: "f32" })
  thickness: number;

  constructor(data: StitchOutline) {
    this.color = data.color;
    this.colorPercentage = data.colorPercentage;
    this.thickness = data.thickness;
  }
}

export class DefaultStitchStrands {
  @field({ type: "u16" })
  full: number;

  @field({ type: "u16" })
  petite: number;

  @field({ type: "u16" })
  half: number;

  @field({ type: "u16" })
  quarter: number;

  @field({ type: "u16" })
  back: number;

  @field({ type: "u16" })
  straight: number;

  @field({ type: "u16" })
  frenchKnot: number;

  @field({ type: "u16" })
  special: number;

  constructor(data: DefaultStitchStrands) {
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

export class StitchSettings {
  @field({ type: DefaultStitchStrands })
  defaultStrands: DefaultStitchStrands;

  @field({ type: fixedArray("f32", 13) })
  displayThickness: number[];

  constructor(data: StitchSettings) {
    this.defaultStrands = data.defaultStrands;
    this.displayThickness = data.displayThickness;
  }
}

export class DisplaySettings {
  @field({ type: "string" })
  defaultStitchFont: string;

  @field({ type: vec(Symbols) })
  symbols: Symbols[];

  @field({ type: SymbolSettings })
  symbolSettings: SymbolSettings;

  @field({ type: vec(Formats) })
  formats: Formats[];

  @field({ type: Grid })
  grid: Grid;

  @field({ type: "u8" })
  view: number;

  @field({ type: "u16" })
  zoom: number;

  @field({ type: "bool" })
  showGrid: boolean;

  @field({ type: "bool" })
  showRulers: boolean;

  @field({ type: "bool" })
  showCenteringMarks: boolean;

  @field({ type: "bool" })
  showFabricColorsWithSymbols: boolean;

  @field({ type: "bool" })
  gapsBetweenStitches: boolean;

  @field({ type: "bool" })
  outlinedStitches: boolean;

  @field({ type: StitchOutline })
  stitchOutline: StitchOutline;

  @field({ type: StitchSettings })
  stitchSettings: StitchSettings;

  constructor(data: DisplaySettings) {
    this.defaultStitchFont = data.defaultStitchFont;
    this.symbols = data.symbols;
    this.symbolSettings = data.symbolSettings;
    this.formats = data.formats;
    this.grid = data.grid;
    this.view = data.view;
    this.zoom = data.zoom;
    this.showGrid = data.showGrid;
    this.showRulers = data.showRulers;
    this.showCenteringMarks = data.showCenteringMarks;
    this.showFabricColorsWithSymbols = data.showFabricColorsWithSymbols;
    this.gapsBetweenStitches = data.gapsBetweenStitches;
    this.outlinedStitches = data.outlinedStitches;
    this.stitchOutline = data.stitchOutline;
    this.stitchSettings = data.stitchSettings;
  }
}

export const enum LineStitchStyle {
  Solid = 0,
  Barred = 1,
  Dotted = 2,
  ChainDotted = 3,
  Dashed = 4,
  Outlined = 5,
  Zebra = 6,
  ZigZag = 7,
  Morse = 8,
}

export const enum View {
  Stitches = 0,
  Symbols = 1,
  Solid = 2,
  Information = 3,
  MachineEmbInfo = 4,
}
