/* eslint-disable no-undef */
/* eslint-disable @typescript-eslint/no-var-requires */
/** @format */
/* eslint @typescript-eslint/no-var-requires: "off" */

const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const InterpolateHtmlPlugin = require("interpolate-html-plugin");
const Dotenv = require("dotenv-webpack");
const BundleAnalyzerPlugin =
  require("webpack-bundle-analyzer").BundleAnalyzerPlugin;
const webpack = require("webpack");
const CopyPlugin = require("copy-webpack-plugin");
const { URLSearchParams } = require("url");
//const ImageMinimizerPlugin = require("image-minimizer-webpack-plugin");
module.exports = {
  entry: {
    bundle: path.resolve(__dirname, "../src", "index.tsx"),
  },
  plugins: [
    new CopyPlugin({
      patterns: [
        {
          from: "public",
          to: "static",
          globOptions: {
            dot: true,
            gitignore: true,
            ignore: ["**/index.html"],
          },
        },
      ],
    }),
    new MiniCssExtractPlugin(),
    new HtmlWebpackPlugin({
      filename: "index.html",
      inject: true,
      template: path.resolve(__dirname, "../public", "index.html"),
    }),
    new InterpolateHtmlPlugin({
      PUBLIC_URL: "static", // can modify `static` to another name or get it from `process`
    }),
    new webpack.ProvidePlugin({
      process: "process/browser",
    }),
    new BundleAnalyzerPlugin(),
    new Dotenv(),
  ],
  module: {
    rules: [
      {
        test: /\.js$|\.jsx$|\.tsx$/,
        exclude: /node_modules/,
        loader: "babel-loader",
      },
      {
        test: /\.tsx?$/,
        use: "ts-loader",
      },
      {
        test: /\.scss$/,

        use: [MiniCssExtractPlugin.loader, "css-loader", "sass-loader"],
      },
      {
        test: /\.css$/,

        use: ["style-loader", "css-loader"],
      },
      // {
      //   test: /\.svg$/,
      //   use: [
      //     {
      //       loader: "svg-url-loader",
      //       options: {
      //         limit: 10000,
      //       },
      //     },
      //   ],
      // },
      {
        test: /\.svg/,

        type: "asset/resource",
        generator: {
          filename: "assets/svg/[name].[hash][ext][query]",
        },
      },
      {
        test: /\.(png|jpg|gif|csv|jpeg|docx|pdf|mp4)$/i,
        type: "asset/resource",

        generator: {
          //filename: "assets/[ext]/[name].[hash][ext][query]",
          filename: (pathData) => {
            const info = pathData.module.resourceResolveData;
            const params = new URLSearchParams(info.query);
            const format =
              params.get("format") || path.extname(info.path).split(".").pop();
            let fileName = "assets/myext/[name].[hash][ext][query]";
            format && (fileName = fileName.replace("myext", format));
            return fileName;
          },
        },
      },
      // image compression

      {
        test: /\.m?js$/,
        resolve: {
          fullySpecified: false,
        },
      },
    ],
    noParse: [require.resolve("typescript/lib/typescript.js")],
  },
  resolve: {
    extensions: [".tsx", ".ts", ".js", ".jsx"],
  },

  optimization: {
    // minimizer: [
    //   "...",
    //   new ImageMinimizerPlugin({
    //     minimizer: {
    //       implementation: ImageMinimizerPlugin.imageminMinify,
    //       options: {
    //         // Lossless optimization with custom option
    //         // Feel free to experiment with options for better result for you
    //         plugins: [
    //           ["gifsicle", { interlaced: true }],
    //           ["jpegtran", { progressive: true }],
    //           ["optipng", { optimizationLevel: 5 }],
    //           // Svgo configuration here https://github.com/svg/svgo#configuration
    //           [
    //             "svgo",
    //             {
    //               plugins: [
    //                 {
    //                   name: "preset-default",
    //                   params: {
    //                     overrides: {
    //                       removeViewBox: false,
    //                       addAttributesToSVGElement: {
    //                         params: {
    //                           attributes: [
    //                             { xmlns: "http://www.w3.org/2000/svg" },
    //                           ],
    //                         },
    //                       },
    //                     },
    //                   },
    //                 },
    //               ],
    //             },
    //           ],
    //         ],
    //       },
    //     },
    //   }),
    // ],
    splitChunks: {
      chunks: "all",
      minSize: 4000,
      minRemainingSize: 0,
      minChunks: 7,
      maxAsyncRequests: 30,
      maxInitialRequests: 30,
      enforceSizeThreshold: 10000,
      cacheGroups: {
        defaultVendors: {
          test: /node_modules/,
          priority: -40,
          name: "node",
          reuseExistingChunk: true,
        },
        default: {
          minChunks: 2,
          priority: -40,
          reuseExistingChunk: true,
        },
      },
    },
  },
  output: {
    // filename: "build/[name].js",
    // path: path.resolve(__dirname,"../", "build"),
    // chunkFilename: "[id].[hash:8].js",
    path: path.resolve(__dirname, "../", "build"),
    filename: "js/[name].[contenthash].js",

    chunkFilename: "js/chunks/chunk.[id].[hash:8].js",
    clean: true,
  },
};
