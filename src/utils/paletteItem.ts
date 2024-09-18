import type { Blend, PaletteItem } from "#/schemas/pattern";

/** Interface representing display options for a palette item's title. */
export interface PaletteItemDisplayOptions {
  /** If true, the vendor/brand will be displayed. */
  showVendor: boolean;
  /** If true, the item number will be displayed. */
  showNumber: boolean;
  /** If true, the item name will be displayed. */
  showName: boolean;
}

/**
 * Composes a title for a palette item based on the provided display options.
 *
 * @param pi The palette item for which the title is composed.
 * @param options The display options to customize the title.
 * @returns The composed title for the palette item.
 */
export function paletteItemTitle(pi: PaletteItem, options: PaletteItemDisplayOptions): string {
  const components = [];
  if (options.showVendor) components.push(pi.brand);
  if (pi.blends?.length) {
    components.push(
      pi.blends
        .map((blend) => blendTitle(blend, options))
        // Remove empty strings.
        .filter((v) => v.length)
        .join(", "),
    );
    return components.join(": ");
  }
  if (options.showNumber) components.push(pi.number);
  // The name can be an empty string. For example, if the palette item is blend.
  if (options.showName && pi.name.length) {
    if (!components.length) return pi.name;
    return [components.join(" "), pi.name].join(", ");
  }
  return components.join(" ");
}

/**
 * Composes a title for a blend based on the provided display options.
 *
 * @param blend - The blend for which the title is composed.
 * @param options - The display options to customize the title.
 * @returns The composed title for the blend.
 */
export function blendTitle({ brand, number }: Blend, options: PaletteItemDisplayOptions): string {
  const components = [];
  if (options.showVendor) components.push(brand);
  if (options.showNumber) components.push(number);
  return components.join(" ");
}