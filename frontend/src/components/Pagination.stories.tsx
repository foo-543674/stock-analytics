import { Meta, StoryObj } from 'storybook-solidjs';
import { Pagination } from './Pagination';
import { expect, fn, userEvent, waitFor, within } from '@storybook/test';
import { delay } from '@tests/waitMockResolved';

const meta: Meta<typeof Pagination> = {
  component: Pagination,
};
export default meta;
type Story = StoryObj<typeof Pagination>;

export const Default: Story = {
  args: {
    page: 3,
    maxPage: 5,
    onChanged: fn(),
  },
};

export const HasOmmited: Story = {
  args: {
    page: 7,
    maxPage: 13,
    onChanged: fn(),
  },
  parameters: {
    viewport: {
      defaultViewport: 'dekstop',
    },
  },
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement);

    const omittedButtons = await canvas.findAllByTestId('ommitted-button');
    await waitFor(() => expect(omittedButtons).toHaveLength(2));
    omittedButtons.forEach(async omittedButton => {
      await waitFor(() => expect(omittedButton).toHaveClass('btn-disabled'));
    });
  },
};

export const LastPage: Story = {
  args: {
    page: 12,
    maxPage: 12,
    onChanged: fn(),
  },
  parameters: {
    viewport: {
      defaultViewport: 'dekstop',
    },
  },
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement);

    const toNextButton = await canvas.findByTestId('to-next-button');
    await waitFor(() => expect(toNextButton).toHaveClass('btn-disabled'));
    const toLastButton = await canvas.findByTestId('to-last-button');
    await waitFor(() => expect(toLastButton).toHaveClass('btn-disabled'));
  },
};

export const FistPage: Story = {
  args: {
    page: 1,
    maxPage: 12,
    onChanged: fn(),
  },
  parameters: {
    viewport: {
      defaultViewport: 'dekstop',
    },
  },
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement);

    const toPrevButton = await canvas.findByTestId('to-prev-button');
    await waitFor(() => expect(toPrevButton).toHaveClass('btn-disabled'));
    const toFirstButton = await canvas.findByTestId('to-first-button');
    await waitFor(() => expect(toFirstButton).toHaveClass('btn-disabled'));
  },
};

export const Mobile: Story = {
  args: {
    page: 3,
    maxPage: 5,
    onChanged: fn(),
  },
  parameters: {
    viewport: {
      defaultViewport: 'mobile1',
    },
  },
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await delay(500);

    await expect(canvas.queryByTestId('to-1-button')).not.toBeInTheDocument();
    await expect(canvas.queryByTestId('to-5-button')).not.toBeInTheDocument();
  },
};

export const Tablet: Story = {
  args: {
    page: 5,
    maxPage: 9,
    onChanged: fn(),
  },
  parameters: {
    viewport: {
      defaultViewport: 'tablet',
    },
  },
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await delay(500);

    await expect(canvas.queryByTestId('to-1-button')).not.toBeInTheDocument();
    await expect(canvas.queryByTestId('to-9-button')).not.toBeInTheDocument();
  },
};

export const ClickEachPages: Story = {
  args: {
    page: 3,
    maxPage: 5,
    onChanged: fn(),
  },
  parameters: {
    viewport: {
      defaultViewport: 'dekstop',
    },
  },
  play: async ({ args, canvasElement }) => {
    const canvas = within(canvasElement);

    const toThreeButton = await canvas.findByTestId('to-3-button');
    await waitFor(() => expect(toThreeButton).toHaveClass('btn-active'));
    await waitFor(() => expect(toThreeButton).toHaveClass('btn-disabled'));

    await Promise.all(
      Array.from({ length: 5 }, (_, i) => i + 1)
        .filter(p => p !== 3)
        .map(async p => {
          const numButton = await canvas.findByTestId(`to-${p}-button`);
          await userEvent.click(numButton);
          await waitFor(() => expect(args.onChanged).toHaveBeenCalledWith(p));
        }),
    );
  },
};

export const ClickEachArrows: Story = {
  args: {
    page: 3,
    maxPage: 5,
    onChanged: fn(),
  },
  parameters: {
    viewport: {
      defaultViewport: 'dekstop',
    },
  },
  play: async ({ args, canvasElement }) => {
    const canvas = within(canvasElement);

    const toPrevButton = await canvas.findByTestId('to-prev-button');
    await userEvent.click(toPrevButton);
    await waitFor(() => expect(args.onChanged).toHaveBeenCalledWith(2));
    const toNextButton = await canvas.findByTestId('to-next-button');
    await userEvent.click(toNextButton);
    await waitFor(() => expect(args.onChanged).toHaveBeenCalledWith(4));
    const toFirstButton = await canvas.findByTestId('to-first-button');
    await userEvent.click(toFirstButton);
    await waitFor(() => expect(args.onChanged).toHaveBeenCalledWith(1));
    const toLastButton = await canvas.findByTestId('to-last-button');
    await userEvent.click(toLastButton);
    await waitFor(() => expect(args.onChanged).toHaveBeenCalledWith(5));
  },
};
