export interface RGB {
  r: number;
  g: number;
  b: number;
}

export const rgb = (r: number, g: number, b: number): RGB => ({ r, g, b });

export const RGBToHex = ({ r, g, b }: RGB): string => "#" + ((1 << 24) | (r << 16) | (g << 8) | b).toString(16).slice(1);
