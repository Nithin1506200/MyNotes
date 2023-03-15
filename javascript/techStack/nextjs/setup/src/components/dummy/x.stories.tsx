import React from "react";
import { ComponentStory, ComponentMeta } from "@storybook/react";

import Button from "../../components/Button";

// More on default export: https://storybook.js.org/docs/react/writing-stories/introduction#default-export
export default {
  title: "Example2/Button",
  component: Button,
  // More on argTypes: https://storybook.js.org/docs/react/api/argtypes
} as ComponentMeta<typeof Button>;

// More on component templates: https://storybook.js.org/docs/react/writing-stories/introduction#using-args
const Template: ComponentStory<typeof Button> = (args) => <Button {...args} />;
/**
 * this is to
 */
export const Primary = Template.bind({});
// More on args: https://storybook.js.org/docs/react/writing-stories/args

Primary.args = {
  message: "nithin",
};

export const Secondary = Template.bind({});
Secondary.args = {
  message: "nithin 1",
};

export const Large = Template.bind({});
Large.args = {
  message: "ashees",
};

export const Small = Template.bind({});
Small.args = {
  message: "nithin",
};
const skeleton: ComponentStory<typeof Button> = (args) => <Button.skeleton />;
export const skeletonOfButton = skeleton.bind({});
