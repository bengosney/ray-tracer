import { Vec3, vec3 } from "./math";

export interface RGB {
  r: number;
  g: number;
  b: number;
}

export const rgb = (r: number, g: number, b: number): RGB => ({ r, g, b });

export const rgbToVec3 = ({ r, g, b }: RGB): Vec3 => vec3(r, g, b);
export const vec3ToRGB = ({ x, y, z }: Vec3): RGB => rgb(x, y, z);

export const RGBToHex = ({ r, g, b }: RGB): string => "#" + ((1 << 24) | (r << 16) | (g << 8) | b).toString(16).slice(1);
