export const exhaustiveMatchGuard = (value: never): never => {
  throw new Error(`Unexpected value: ${JSON.stringify(value)}`);
};
