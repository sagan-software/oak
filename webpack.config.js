const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const CleanWebpackPlugin = require("clean-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
// const chokidar = require("chokidar");
// const { spawn } = require("child_process");

const examples = ["counter_ecs"];
// const exampleTarget = e => `target/wasm-bindgen/${e}_bg.wasm`;
// const exampleTargets = examples.map(exampleTarget);

// function watchRustFiles() {
//   chokidar
//     .watch(["crates/**/*.rs", "examples/**/*.rs"])
//     .on("change", filePath => {
//       console.log(filePath);
//       if (filePath.startsWith("crates")) {
//         // Libraries changed so rebuild all examples
//         console.log(`Crate file "${filePath} changed, rebuilding examples...`);
//         run("make", exampleTargets);
//       }
//       if (filePath.startsWith("examples")) {
//         const pkg = filePath.split(path.sep)[1];
//         console.log(`Example file "${pkg}" changed, rebuilding example...`);
//         run("make", [exampleTarget(pkg)]);
//       }
//     });
// }

// function run(command, options) {
//   const child = spawn(command, options);
//   child.stdout.pipe(process.stdout);
//   child.stderr.pipe(process.stderr);
// }

module.exports = env => {
  // if (env && env.watch) {
  //   watchRustFiles();
  // }

  return examples.map(pkg => {
    // run("make", [exampleTarget(pkg)]);
    const crateDirectory = path.resolve(__dirname, "examples", pkg);
    const outPath = path.resolve(__dirname, "gh-pages", pkg);
    return {
      entry: path.resolve(crateDirectory, "static", "index.js"),
      output: {
        path: outPath,
        filename: "index.js",
        publicPath: `/${pkg}/`
      },
      devServer: {
        contentBase: path.resolve(__dirname, "gh-pages")
      },
      plugins: [
        new CleanWebpackPlugin([outPath, path.resolve(crateDirectory, "pkg")]),
        new WasmPackPlugin({
          crateDirectory,
          watchDirectories: [path.resolve(__dirname, "crates", "oak")]
        }),
        new HtmlWebpackPlugin({
          template: path.resolve(crateDirectory, "static", "index.html"),
          minify: true
        })
      ],
      // resolve: {
      //   alias: {
      //     "wasm-bindgen": path.resolve(__dirname, "target", "wasm-bindgen")
      //   }
      // },
      mode: "production"
    };
  });
};
