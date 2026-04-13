import { humanDuration } from "./time";

describe("humanDuration", () => {
  it("returns NaN for NaN input", () => {
    expect(humanDuration(NaN)).toBe("NaN");
  });

  it("returns 0ms for zero", () => {
    expect(humanDuration(0)).toBe("0ms");
  });

  it("returns 0ms for negative values", () => {
    expect(humanDuration(-100)).toBe("0ms");
  });

  it("returns milliseconds for sub-second durations", () => {
    expect(humanDuration(500)).toBe("500ms");
    expect(humanDuration(1)).toBe("1ms");
    expect(humanDuration(999)).toBe("999ms");
  });

  it("returns seconds with one decimal place for single-digit seconds", () => {
    expect(humanDuration(1000)).toBe("1.0s");
    expect(humanDuration(9900)).toBe("9.9s");
  });

  it("returns whole seconds for double-digit seconds", () => {
    expect(humanDuration(10000)).toBe("10s");
    expect(humanDuration(59999)).toBe("59s");
  });

  it("returns minutes and seconds", () => {
    expect(humanDuration(60000)).toBe("1m");
    expect(humanDuration(61000)).toBe("1m 1s");
    expect(humanDuration(90000)).toBe("1m 30s");
  });

  it("returns hours, minutes, and seconds", () => {
    expect(humanDuration(3600000)).toBe("1h");
    expect(humanDuration(3660000)).toBe("1h 1m");
    expect(humanDuration(3661000)).toBe("1h 1m 1s");
    expect(humanDuration(3690000)).toBe("1h 1m 30s");
  });

  it("handles hours without minutes or seconds", () => {
    expect(humanDuration(7200000)).toBe("2h");
  });
});
