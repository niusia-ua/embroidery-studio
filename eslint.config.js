import js from "@eslint/js";
import vue from "eslint-plugin-vue";
import vuePrettierEslintConfig from "@vue/eslint-config-prettier/skip-formatting";
import vueTypescriptEslintConfig from "@vue/eslint-config-typescript";

export default [
  js.configs.recommended,
  {
    files: ["src/**/*.ts", "src/**/*.vue"],
    languageOptions: { ecmaVersion: "latest" },
  },
  ...vue.configs["flat/recommended"],
  vuePrettierEslintConfig,
  ...vueTypescriptEslintConfig(),
];
