const HtmlWebpackPlugin = require('html-webpack-plugin');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin')
const path = require('path');

module.exports = {
  entry: {
    index: "./index.ts",
    worker: "./worker.ts"
  },
  experiments: {
    asyncWebAssembly: true,
  },
  output: {
    path: path.resolve(__dirname, "../docs"),
    filename: "[name].js"
  },
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: 'ts-loader',
        exclude: /node_modules/,
      },
      {
        test: /\.css$/i,
        use: ["style-loader", "css-loader"],
      },
    ],
  },
  devServer: {
    historyApiFallback: true,
  },
  resolve: {
    extensions: [".ts", ".js"],
  },
  mode: "development",
  plugins: [
    new WasmPackPlugin({ crateDirectory: path.resolve(__dirname, '..'), outDir: path.resolve(__dirname, 'pkg') }),
    new HtmlWebpackPlugin({ title: 'AOC 2021 Rust WebAssembly', chunks: ['index'] }),
    new HtmlWebpackPlugin({ title: 'AOC 2021 Rust WebAssembly', chunks: ['index'], filename: '404.html' }),
  ],
};