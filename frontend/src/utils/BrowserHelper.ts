export const isOnBrowser = () => typeof window !== 'undefined';

export const isTextSelected = () => {
  return isOnBrowser() && window.getSelection()?.toString() !== '';
};

export const getInitialWindowSize = () => {
  return isOnBrowser()
    ? {
        width: window.innerWidth,
        height: window.innerHeight,
      }
    : {
        width: 0,
        height: 0,
      };
};

export const addEventListener = (eventName: string, handler: EventListener) => {
  if (isOnBrowser()) {
    window.addEventListener(eventName, handler);
  }
  return () => window.removeEventListener(eventName, handler);
};
