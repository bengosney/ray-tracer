import { Vec3 as WasmVec3, Rgb as WasmRGB } from "wasm-lib";
import type { Vec3 } from "./math";
import type { RGB } from "./colour";

export const wasmVec3 = (v: Vec3): WasmVec3 => new WasmVec3(v.x, v.y, v.z);
export const wasmRGB = (c: RGB): WasmRGB => new WasmRGB(c.r, c.g, c.b);
