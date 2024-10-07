export function contrastColor(hex: string) {
  const r = parseInt(hex.substring(0, 2), 16);
  const g = parseInt(hex.substring(2, 4), 16);
  const b = parseInt(hex.substring(4, 6), 16);
  const brightness = r * 0.299 + g * 0.587 + b * 0.114;
  return brightness > 128 ? "black" : "white";
}
