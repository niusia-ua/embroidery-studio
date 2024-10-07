export interface GridSettings {
  majorLinesEveryStitches: number;
  minorLines: GridLineStyle;
  majorLines: GridLineStyle;
}

export interface GridLineStyle {
  thickness: number;
  color: string;
}
