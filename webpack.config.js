const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");

module.exports = (env, argv) => {
  const isProd = argv.mode === "production";

  return {
    entry: "./src/index.tsx",
    output: {
      path: path.resolve(__dirname, "build"),
      filename: "static/js/[name].[contenthash:8].js",
      chunkFilename: "static/js/[name].[contenthash:8].chunk.js",
      assetModuleFilename: "static/media/[name].[hash][ext]",
      clean: true,
      publicPath: isProd ? "/ray-tracer/" : "/",
    },
    resolve: {
      extensions: [".tsx", ".ts", ".js"],
      alias: {
        "wasm-lib": path.resolve(__dirname, "wasm-lib/pkg"),
      },
    },
    module: {
      rules: [
        {
          oneOf: [
            // App source: babel with TypeScript + React
            {
              test: /\.(js|mjs|jsx|ts|tsx)$/,
              include: path.resolve(__dirname, "src"),
              loader: "babel-loader",
              options: {
                presets: [
                  ["@babel/preset-env", { targets: { browsers: "last 2 versions" } }],
                  ["@babel/preset-react", { runtime: "automatic" }],
                  "@babel/preset-typescript",
                ],
                cacheDirectory: true,
                cacheCompression: false,
              },
            },
            // node_modules JS: babel with CJS modules so dynamic imports are bundled inline
            {
              test: /\.(js|mjs)$/,
              exclude: [/@babel(?:\/|\\{1,2})runtime/, /wasm-lib/],
              loader: "babel-loader",
              resolve: { fullySpecified: false },
              options: {
                presets: [["@babel/preset-env", { targets: { browsers: "last 2 versions" }, modules: "commonjs" }]],
                cacheDirectory: true,
                cacheCompression: false,
                compact: false,
              },
            },
            // wasm-lib JS: skip babel but disable fullySpecified
            {
              test: /\.(js|mjs)$/,
              include: /wasm-lib/,
              resolve: { fullySpecified: false },
            },
            // CSS
            {
              test: /\.css$/,
              use: ["style-loader", "css-loader"],
            },
            // Catch-all: everything else (including .wasm) as asset/resource
            {
              exclude: [/^$/, /\.(js|mjs|jsx|ts|tsx)$/, /\.html$/, /\.json$/],
              type: "asset/resource",
            },
          ],
        },
      ],
    },
    plugins: [
      new HtmlWebpackPlugin({
        template: "./public/index.html",
      }),
    ],
    devServer: {
      port: 3000,
      headers: {
        "Cross-Origin-Opener-Policy": "same-origin",
        "Cross-Origin-Embedder-Policy": "require-corp",
      },
    },
  };
};
