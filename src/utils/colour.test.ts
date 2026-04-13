import { rgb, rgbToVec3, vec3ToRGB, RGBToHex } from "./colour";
import { vec3 } from "./math";

describe("rgb", () => {
  it("creates an RGB value with the given components", () => {
    expect(rgb(255, 128, 0)).toEqual({ r: 255, g: 128, b: 0 });
  });
});

describe("rgbToVec3", () => {
  it("converts RGB to a vec3", () => {
    expect(rgbToVec3(rgb(1, 2, 3))).toEqual(vec3(1, 2, 3));
  });
});

describe("vec3ToRGB", () => {
  it("converts a vec3 to RGB", () => {
    expect(vec3ToRGB(vec3(1, 2, 3))).toEqual(rgb(1, 2, 3));
  });
});

describe("RGBToHex", () => {
  it("converts black", () => {
    expect(RGBToHex(rgb(0, 0, 0))).toBe("#000000");
  });

  it("converts white", () => {
    expect(RGBToHex(rgb(255, 255, 255))).toBe("#ffffff");
  });

  it("converts red", () => {
    expect(RGBToHex(rgb(255, 0, 0))).toBe("#ff0000");
  });

  it("converts a mixed colour", () => {
    expect(RGBToHex(rgb(18, 52, 86))).toBe("#123456");
  });
});
