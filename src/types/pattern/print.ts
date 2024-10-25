export interface PrintSettings {
  font: Font;
  header: string;
  footer: string;
  margins: PageMargins;
  showPageNumbers: boolean;
  showAdjacentPageNumbers: boolean;
  centerChartOnPages: boolean;
}

export interface Font {
  name: string;
  size: number;
  weight: number;
  italic: boolean;
}

export interface PageMargins {
  left: number;
  right: number;
  top: number;
  bottom: number;
  header: number;
  footer: number;
}
