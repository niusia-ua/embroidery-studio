import { field } from "@dao-xyz/borsh";
export class Font {
  @field({ type: "string" })
  name: string;

  @field({ type: "u16" })
  size: number;

  @field({ type: "u16" })
  weight: number;

  @field({ type: "bool" })
  italic: boolean;

  constructor(data: Font) {
    this.name = data.name;
    this.size = data.size;
    this.weight = data.weight;
    this.italic = data.italic;
  }
}

export class PageMargins {
  @field({ type: "f32" })
  left: number;

  @field({ type: "f32" })
  right: number;

  @field({ type: "f32" })
  top: number;

  @field({ type: "f32" })
  bottom: number;

  @field({ type: "f32" })
  header: number;

  @field({ type: "f32" })
  footer: number;

  constructor(data: PageMargins) {
    this.left = data.left;
    this.right = data.right;
    this.top = data.top;
    this.bottom = data.bottom;
    this.header = data.header;
    this.footer = data.footer;
  }
}

export class PrintSettings {
  @field({ type: Font })
  font: Font;

  @field({ type: "string" })
  header: string;

  @field({ type: "string" })
  footer: string;

  @field({ type: PageMargins })
  margins: PageMargins;

  @field({ type: "bool" })
  showPageNumbers: boolean;

  @field({ type: "bool" })
  showAdjacentPageNumbers: boolean;

  @field({ type: "bool" })
  centerChartOnPages: boolean;

  constructor(data: PrintSettings) {
    this.font = data.font;
    this.header = data.header;
    this.footer = data.footer;
    this.margins = data.margins;
    this.showPageNumbers = data.showPageNumbers;
    this.showAdjacentPageNumbers = data.showAdjacentPageNumbers;
    this.centerChartOnPages = data.centerChartOnPages;
  }
}
