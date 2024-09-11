import { describe, expect, test } from "vitest";
import { contrastColor } from "./color";

describe("color utils", () => {
  test("returns the contrast color", () => {
    expect(contrastColor("000000")).toBe("white"); // Just black
    expect(contrastColor("2C3225")).toBe("white"); // DMC 310
    expect(contrastColor("7A5577")).toBe("white"); // DMC 327
    expect(contrastColor("973E3B")).toBe("white"); // DMC 816
    expect(contrastColor("50442B")).toBe("white"); // DMC 938
    expect(contrastColor("FFFFFF")).toBe("black"); // Just white
    expect(contrastColor("F6E311")).toBe("black"); // DMC 307
    expect(contrastColor("91B1DB")).toBe("black"); // DMC 809
    expect(contrastColor("ECEDC5")).toBe("black"); // DMC 3823
    expect(contrastColor("F5A7B6")).toBe("black"); // DMC 3708
  });
});
