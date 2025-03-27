import { describe, expect, vi, beforeEach, afterEach } from 'vitest';
import { it, fc } from '@fast-check/vitest';
import { createWindowSize } from './createWindowSize';
import { getInitialWindowSize, addEventListener } from '@/utils/BrowserHelper';
import { renderHook } from '@solidjs/testing-library';
import { waitMockResolved } from '@tests/waitMockResolved';

vi.mock('@/utils/BrowserHelper');

describe('createWindowSize', () => {
  const mockAddEventListener = vi.fn();
  beforeEach(() => {
    vi.mocked(addEventListener).mockImplementation(mockAddEventListener);
  });
  afterEach(() => {
    vi.resetAllMocks();
  });

  it('should return window size', () => {
    vi.mocked(getInitialWindowSize).mockReturnValue({
      width: 1024,
      height: 768,
    });

    const {
      result: [size],
    } = renderHook(() => createWindowSize());
    expect(size()).toEqual({ width: 1024, height: 768 });
  });

  it.prop([fc.integer({ min: 1024 })])(
    'should be desktop when width is greater than or equal to 1024',
    width => {
      vi.mocked(getInitialWindowSize).mockReturnValue({
        width: width,
        height: 768,
      });

      const {
        result: [, deviceType],
      } = renderHook(() => createWindowSize());
      expect(deviceType()).toBe('desktop');
    },
  );

  it.prop([fc.integer({ min: 768, max: 1023 })])(
    'should be tablet when width is greater than or equal to 768 and less than 1024',
    width => {
      vi.mocked(getInitialWindowSize).mockReturnValue({
        width: width,
        height: 768,
      });

      const {
        result: [, deviceType],
      } = renderHook(() => createWindowSize());
      expect(deviceType()).toBe('tablet');
    },
  );

  it.prop([fc.integer({ min: 640, max: 767 })])(
    'should be large mobile when width is greater than or equal to 640 and less than 768',
    width => {
      vi.mocked(getInitialWindowSize).mockReturnValue({
        width: width,
        height: 768,
      });

      const {
        result: [, deviceType],
      } = renderHook(() => createWindowSize());
      expect(deviceType()).toBe('largeMobile');
    },
  );

  it.prop([fc.integer({ max: 639 })])(
    'should be mobile when width is less than 640',
    width => {
      vi.mocked(getInitialWindowSize).mockReturnValue({
        width: width,
        height: 768,
      });

      const {
        result: [, deviceType],
      } = renderHook(() => createWindowSize());
      expect(deviceType()).toBe('mobile');
    },
  );

  it('should add resize event listener', async () => {
    vi.mocked(getInitialWindowSize).mockReturnValue({
      width: 1024,
      height: 768,
    });
    renderHook(() => createWindowSize());
    await waitMockResolved();
    expect(mockAddEventListener).toHaveBeenCalledWith(
      'resize',
      expect.any(Function),
    );
  });
});
