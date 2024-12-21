import roundn from "@stdlib/math-base-special-roundn";

export function inches2mm(inches: number) {
  return roundn(inches * 25.4, 0);
}

export function mm2inches(mm: number) {
  return roundn(mm / 25.4, -2);
}

export function mm2px(mm: number) {
  return mm * 3.7795275591;
}

export function size2stitches(size: number, count: number) {
  return roundn(size * count, 0);
}

export function stitches2inches(stitches: number, count: number) {
  return roundn(stitches / count, -2);
}

export function stitches2mm(stitches: number, count: number) {
  return inches2mm(stitches2inches(stitches, count));
}
