module.exports = {
  extends: "eslint:recommended",
  parser: "babel-eslint",
  rules: {
    "no-unused-vars": ["warn"],

    "no-console": "off",
    "no-debugger": "off",
  },
  env: {
    "browser": true,
    "node": true,
  },
};
