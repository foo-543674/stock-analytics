export const waitMockResolved = async () => {
  await new Promise(resolve => setTimeout(resolve, 0));
};
