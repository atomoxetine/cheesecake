module.exports = {
  purge: {
    mode: "all",
    content: [
      "./src/**/*.rs",
      "./templates/index.html",
      "./templates/**/*.html",
      "./templates/**/*.css",
    ],
  },
  theme: {},
  variants: {},
  plugins: [],
};
