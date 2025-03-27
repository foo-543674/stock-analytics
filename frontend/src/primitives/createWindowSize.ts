import { addEventListener, getInitialWindowSize } from '@/utils/BrowserHelper';
import { createEffect, createMemo, createSignal } from 'solid-js';

export enum DeviceType {
  Desktop = 'desktop',
  Tablet = 'tablet',
  LargeMobile = 'largeMobile',
  Mobile = 'mobile',
}

export const createWindowSize = () => {
  const [size, setSize] = createSignal(getInitialWindowSize());

  createEffect(() => {
    const handleResize = () => setSize(getInitialWindowSize());
    return addEventListener('resize', handleResize);
  });

  const deviceType = createMemo(() => {
    const { width } = size();
    if (width >= 1024) {
      return DeviceType.Desktop;
    } else if (width >= 768) {
      return DeviceType.Tablet;
    } else if (width >= 640) {
      return DeviceType.LargeMobile;
    } else {
      return DeviceType.Mobile;
    }
  });

  return [size, deviceType] as const;
};
