module.exports = {
  presets: [
    [
      // ES features necessary for user's Node version
      require("@babel/preset-env").default,
      {
        targets: {
          node: "current",
        },
      },
    ],
    [require("@babel/preset-typescript").default],
  ],
  plugins: ["@babel/plugin-transform-runtime"],
};
