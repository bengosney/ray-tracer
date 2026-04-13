import { exhaustiveMatchGuard } from "./typeguard";

describe("exhaustiveMatchGuard", () => {
  it("throws an error with the unexpected value", () => {
    expect(() => exhaustiveMatchGuard("unexpected" as never)).toThrow("Unexpected value");
  });

  it("includes the value in the error message", () => {
    expect(() => exhaustiveMatchGuard({ foo: "bar" } as never)).toThrow(JSON.stringify({ foo: "bar" }));
  });
});
