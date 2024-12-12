import type { Bead, Blend, PaletteItemStitchStrands } from "#/schemas/pattern/pattern";

// TODO: remove these types
/** These are properties that uniquely indentify the palette item. */
export interface PaletteItemUnique {
  brand: string;
  number: string;
}

/**
 * These are properties that are common for all palette items.
 * Generally, these are the properties that are displayed in the palette panel and used in the palette catalog files.
 */
export interface PaletteItemBase extends PaletteItemUnique {
  name: string;
  color: string;
}

/**
 * These are all the properties that can be defined for a palette item.
 */
export interface PaletteItem extends PaletteItemBase {
  blends?: Blend[];
  bead?: Bead;
  strands?: PaletteItemStitchStrands;
}
// TODO:
