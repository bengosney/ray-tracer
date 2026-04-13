import { wasmVec3, wasmRGB } from "./conversions";
import { vec3 } from "./math";
import { rgb } from "./colour";

describe("wasmVec3", () => {
  it("passes the correct components to the WASM constructor", () => {
    const result = wasmVec3(vec3(1, 2, 3));
    expect(result).toMatchObject({ x: 1, y: 2, z: 3 });
  });
});

describe("wasmRGB", () => {
  it("passes the correct components to the WASM constructor", () => {
    const result = wasmRGB(rgb(255, 128, 0));
    expect(result).toMatchObject({ r: 255, g: 128, b: 0 });
  });
});
