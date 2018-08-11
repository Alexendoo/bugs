const path = require("path");

module.exports = {
  target: "webworker",

  entry: {
    "worker-a": path.join(__dirname, "src/worker-a.js"),
    "worker-b": path.join(__dirname, "src/worker-b.js")
  },

  mode: "none",

  optimization: {
    splitChunks: {
      chunks: "initial",
      minSize: 0
    }
  }
};
