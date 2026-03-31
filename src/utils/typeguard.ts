export const exhaustiveMatchGuard = (message: string): never => {
  throw new Error(message);
};
