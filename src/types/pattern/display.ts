export interface DisplaySettings {
  defaultStitchFont: String;
  symbols: Symbols[];
  symbolSettings: SymbolSettings;
  formats: Formats[];
  grid: Grid;
  view: View;
  zoom: number;
  showGrid: boolean;
  showRulers: boolean;
  showCenteringMarks: boolean;
  showFabricColorsWithSymbols: boolean;
  gapsBetweenStitches: boolean;
  outlinedStitches: boolean;
  stitchOutline: StitchOutline;
  stitchSettings: StitchSettings;
}

export interface Symbols {
  full?: number;
  petite?: number;
  half?: number;
  quarter?: number;
  french_knot?: number;
  bead?: number;
}

export interface SymbolSettings {
  screenSpacing: [number, number];
  printerSpacing: [number, number];
  scaleUsingMaximumFontWidth: boolean;
  scaleUsingFontHeight: boolean;
  stitchSize: number;
  smallStitchSize: number;
  drawSymbolsOverBackstitches: boolean;
  showStitchColor: boolean;
  useLargeHalfStitchSymbol: boolean;
  useTrianglesBehindQuarterStitches: boolean;
}

export interface Formats {
  symbol: SymbolFormat;
  back: LineFormat;
  straight: LineFormat;
  french: NodeFormat;
  bead: NodeFormat;
  special: LineFormat;
  font: FontFormat;
}

export interface SymbolFormat {
  useAltBgColor: boolean;
  bgColor: String;
  fgColor: String;
}

export interface LineFormat {
  useAltColor: boolean;
  color: String;
  style: LineStyle;
  thickness: number;
}

export const enum LineStyle {
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

export interface NodeFormat {
  useDotStyle: boolean;
  useAltColor: boolean;
  color: String;
  diameter: number;
}

export interface FontFormat {
  fontName?: String;
  bold: boolean;
  italic: boolean;
  stitctSize: number;
  smallStitchSize: number;
}

export interface Grid {
  majorLineEveryStitches: number;
  minorScreenLines: GridLineStyle;
  majorScreenLines: GridLineStyle;
  minorPrinterLines: GridLineStyle;
  majorPrinterLines: GridLineStyle;
}

export interface GridLineStyle {
  color: String;
  thickness: number;
}

export const enum View {
  Stitches = 0,
  Symbols = 1,
  Solid = 2,
  Information = 3,
  MachineEmbInfo = 4,
}

export interface StitchOutline {
  color?: String;
  colorPercentage: number;
  thickness: number;
}

export interface StitchSettings {
  defaultStrands: DefaultStitchStrands;

  /**
   * 1..=12 - strands, 13 - french knot.
   */
  displayThickness: [
    number,
    number,
    number,
    number,
    number,
    number,
    number,
    number,
    number,
    number,
    number,
    number,
    number,
  ];
}

export interface DefaultStitchStrands {
  full: number;
  petite: number;
  half: number;
  quarter: number;
  back: number;
  straight: number;
  special: number;
}
