module.exports = {
  testEnvironment: "jsdom",
  moduleNameMapper: {
    "\\.(css)$": "<rootDir>/src/__mocks__/styleMock.js",
    "^wasm-lib$": "<rootDir>/src/__mocks__/wasm-lib.js",
  },
};
