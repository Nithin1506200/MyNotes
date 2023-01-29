/* eslint-disable no-undef */
/* eslint-disable @typescript-eslint/no-var-requires */
/** @format */
/* eslint @typescript-eslint/no-var-requires: "off" */

const { merge } = require("webpack-merge");
const common = require("./webpack.common.js");
const path = require("path");

module.exports = merge(common, {
  mode: "development",
  devtool: "inline-source-map",
  devServer: {
    static: path.resolve(__dirname, "../build"),
    port: 8000,
    open: true,
    hot: true,
    compress: true,
    historyApiFallback: true,
  },
});
