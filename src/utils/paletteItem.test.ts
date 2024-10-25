import { describe, expect, test } from "vitest";
import { blendTitle, paletteItemTitle, type PaletteItemDisplayOptions } from "./paletteItem";
import type { Blend, PaletteItem } from "#/types/pattern/pattern";

const BLENDS: Blend[] = [
  {
    brand: "Anchor",
    number: "9159",
    strands: 1,
  },
  {
    brand: "Madeira",
    number: "0705",
    strands: 1,
  },
];
const PALETTE: PaletteItem[] = [
  {
    brand: "DMC",
    number: "310",
    name: "Black",
    color: "2C3225",
    strands: {},
  },
  {
    brand: "Anchor",
    number: "9159",
    name: "Glacier Blue",
    color: "B2D8E5",
    strands: {},
  },
  {
    brand: "Madeira",
    number: "0705",
    name: "Plum-DK",
    color: "901b6b",
    strands: {},
  },
  {
    brand: "Blends",
    number: "",
    name: "",
    color: "A382AE",
    blends: BLENDS,
    strands: {},
  },
];

describe("paletteItemTitle", () => {
  test("empty", () => {
    const options: PaletteItemDisplayOptions = {
      showBrand: false,
      showNumber: false,
      showName: false,
    };
    for (const pi of PALETTE) expect(paletteItemTitle(pi, options)).toBe("");
  });

  test("brand only", () => {
    const options: PaletteItemDisplayOptions = {
      showBrand: true,
      showNumber: false,
      showName: false,
    };
    expect(paletteItemTitle(PALETTE[0], options)).toBe("DMC");
    expect(paletteItemTitle(PALETTE[1], options)).toBe("Anchor");
    expect(paletteItemTitle(PALETTE[2], options)).toBe("Madeira");
    expect(paletteItemTitle(PALETTE[3], options)).toBe("Blends: Anchor, Madeira");
  });

  test("number only", () => {
    const options: PaletteItemDisplayOptions = {
      showBrand: false,
      showNumber: true,
      showName: false,
    };
    expect(paletteItemTitle(PALETTE[0], options)).toBe("310");
    expect(paletteItemTitle(PALETTE[1], options)).toBe("9159");
    expect(paletteItemTitle(PALETTE[2], options)).toBe("0705");
    expect(paletteItemTitle(PALETTE[3], options)).toBe("9159, 0705");
  });

  test("name only", () => {
    const options: PaletteItemDisplayOptions = {
      showBrand: false,
      showNumber: false,
      showName: true,
    };
    expect(paletteItemTitle(PALETTE[0], options)).toBe("Black");
    expect(paletteItemTitle(PALETTE[1], options)).toBe("Glacier Blue");
    expect(paletteItemTitle(PALETTE[2], options)).toBe("Plum-DK");
    expect(paletteItemTitle(PALETTE[3], options)).toBe("");
  });

  test("brand and number", () => {
    const options: PaletteItemDisplayOptions = {
      showBrand: true,
      showNumber: true,
      showName: false,
    };
    expect(paletteItemTitle(PALETTE[0], options)).toBe("DMC 310");
    expect(paletteItemTitle(PALETTE[1], options)).toBe("Anchor 9159");
    expect(paletteItemTitle(PALETTE[2], options)).toBe("Madeira 0705");
    expect(paletteItemTitle(PALETTE[3], options)).toBe("Blends: Anchor 9159, Madeira 0705");
  });

  test("brand and name", () => {
    const options: PaletteItemDisplayOptions = {
      showBrand: true,
      showNumber: false,
      showName: true,
    };
    expect(paletteItemTitle(PALETTE[0], options)).toBe("DMC, Black");
    expect(paletteItemTitle(PALETTE[1], options)).toBe("Anchor, Glacier Blue");
    expect(paletteItemTitle(PALETTE[2], options)).toBe("Madeira, Plum-DK");
    expect(paletteItemTitle(PALETTE[3], options)).toBe("Blends: Anchor, Madeira");
  });

  test("number and name", () => {
    const options: PaletteItemDisplayOptions = {
      showBrand: false,
      showNumber: true,
      showName: true,
    };
    expect(paletteItemTitle(PALETTE[0], options)).toBe("310, Black");
    expect(paletteItemTitle(PALETTE[1], options)).toBe("9159, Glacier Blue");
    expect(paletteItemTitle(PALETTE[2], options)).toBe("0705, Plum-DK");
    expect(paletteItemTitle(PALETTE[3], options)).toBe("9159, 0705");
  });
});

describe("blendTitle", () => {
  test("empty", () => {
    const options: PaletteItemDisplayOptions = {
      showBrand: false,
      showNumber: false,
      showName: false,
    };
    for (const blend of BLENDS) expect(blendTitle(blend, options)).toBe("");
  });

  test("brand only", () => {
    const options: PaletteItemDisplayOptions = {
      showBrand: true,
      showNumber: false,
      showName: false,
    };
    expect(blendTitle(BLENDS[0], options)).toBe("Anchor");
    expect(blendTitle(BLENDS[1], options)).toBe("Madeira");
  });

  test("number only", () => {
    const options: PaletteItemDisplayOptions = {
      showBrand: false,
      showNumber: true,
      showName: false,
    };
    expect(blendTitle(BLENDS[0], options)).toBe("9159");
    expect(blendTitle(BLENDS[1], options)).toBe("0705");
  });

  test("brand and number", () => {
    const options: PaletteItemDisplayOptions = {
      showBrand: true,
      showNumber: true,
      showName: false,
    };
    expect(blendTitle(BLENDS[0], options)).toBe("Anchor 9159");
    expect(blendTitle(BLENDS[1], options)).toBe("Madeira 0705");
  });
});
