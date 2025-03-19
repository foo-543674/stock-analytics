export const waitMockResolved = async () => {
  await new Promise(resolve => setTimeout(resolve, 0));
};

export const delay = async (ms: number) => {
  await new Promise(resolve => setTimeout(resolve, ms));
};
