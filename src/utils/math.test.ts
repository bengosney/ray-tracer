import {
  vec2,
  vec3,
  add,
  sub,
  mag,
  mul,
  avg,
  mulParts,
  dot,
  normalize,
  reflect,
  degreeToRadians,
  vecToArray,
} from "./math";

describe("vec3", () => {
  it("creates a vector with the given components", () => {
    expect(vec3(1, 2, 3)).toEqual({ x: 1, y: 2, z: 3 });
  });
});

describe("vec2", () => {
  it("creates a vector with the given components", () => {
    expect(vec2(1, 2)).toEqual({ x: 1, y: 2 });
  });
});

describe("vecToArray", () => {
  it("converts a vec3 to an array", () => {
    expect(vecToArray(vec3(1, 2, 3))).toEqual([1, 2, 3]);
  });

  it("converts a vec2 to an array", () => {
    expect(vecToArray(vec2(1, 2))).toEqual([1, 2]);
  });
});

describe("degreeToRadians", () => {
  it("converts 0 degrees", () => {
    expect(degreeToRadians(0)).toBe(0);
  });

  it("converts 180 degrees to PI", () => {
    expect(degreeToRadians(180)).toBeCloseTo(Math.PI);
  });

  it("converts 90 degrees to PI/2", () => {
    expect(degreeToRadians(90)).toBeCloseTo(Math.PI / 2);
  });
});

describe("add", () => {
  it("adds two vectors", () => {
    expect(add(vec3(1, 2, 3), vec3(4, 5, 6))).toEqual(vec3(5, 7, 9));
  });

  it("handles negative components", () => {
    expect(add(vec3(1, 2, 3), vec3(-1, -2, -3))).toEqual(vec3(0, 0, 0));
  });
});

describe("sub", () => {
  it("subtracts two vectors", () => {
    expect(sub(vec3(4, 5, 6), vec3(1, 2, 3))).toEqual(vec3(3, 3, 3));
  });

  it("produces a zero vector when subtracting itself", () => {
    expect(sub(vec3(1, 2, 3), vec3(1, 2, 3))).toEqual(vec3(0, 0, 0));
  });
});

describe("mag", () => {
  it("returns 0 for a zero vector", () => {
    expect(mag(vec3(0, 0, 0))).toBe(0);
  });

  it("calculates the magnitude", () => {
    expect(mag(vec3(1, 0, 0))).toBe(1);
    expect(mag(vec3(3, 4, 0))).toBe(5);
    expect(mag(vec3(1, 1, 1))).toBeCloseTo(Math.sqrt(3));
  });
});

describe("mul", () => {
  it("scales a vector by a scalar", () => {
    expect(mul(vec3(1, 2, 3), 2)).toEqual(vec3(2, 4, 6));
  });

  it("scales to zero", () => {
    expect(mul(vec3(1, 2, 3), 0)).toEqual(vec3(0, 0, 0));
  });

  it("scales by a negative scalar", () => {
    expect(mul(vec3(1, 2, 3), -1)).toEqual(vec3(-1, -2, -3));
  });
});

describe("mulParts", () => {
  it("multiplies component-wise", () => {
    expect(mulParts(vec3(1, 2, 3), vec3(4, 5, 6))).toEqual(vec3(4, 10, 18));
  });
});

describe("dot", () => {
  it("returns 0 for perpendicular vectors", () => {
    expect(dot(vec3(1, 0, 0), vec3(0, 1, 0))).toBe(0);
  });

  it("returns the magnitude squared for a vector with itself", () => {
    expect(dot(vec3(2, 3, 4), vec3(2, 3, 4))).toBe(4 + 9 + 16);
  });

  it("calculates the dot product", () => {
    expect(dot(vec3(1, 2, 3), vec3(4, 5, 6))).toBe(32);
  });
});

describe("normalize", () => {
  it("produces a unit vector", () => {
    expect(mag(normalize(vec3(3, 4, 0)))).toBeCloseTo(1);
    expect(mag(normalize(vec3(1, 2, 3)))).toBeCloseTo(1);
  });

  it("preserves direction", () => {
    const v = vec3(2, 0, 0);
    expect(normalize(v)).toEqual(vec3(1, 0, 0));
  });
});

describe("avg", () => {
  it("averages a single vector", () => {
    expect(avg([vec3(2, 4, 6)])).toEqual(vec3(2, 4, 6));
  });

  it("averages multiple vectors", () => {
    expect(avg([vec3(0, 0, 0), vec3(2, 4, 6)])).toEqual(vec3(1, 2, 3));
  });
});

describe("reflect", () => {
  it("reflects off a flat surface", () => {
    const direction = vec3(1, -1, 0);
    const normal = vec3(0, 1, 0);
    const result = reflect(direction, normal);
    expect(result.x).toBeCloseTo(1);
    expect(result.y).toBeCloseTo(1);
    expect(result.z).toBeCloseTo(0);
  });

  it("reflects a vector pointing straight down off a floor", () => {
    const result = reflect(vec3(0, -1, 0), vec3(0, 1, 0));
    expect(result.x).toBeCloseTo(0);
    expect(result.y).toBeCloseTo(1);
    expect(result.z).toBeCloseTo(0);
  });
});
