import js from '@eslint/js'
import globals from 'globals'
import prettierPlugin from "eslint-plugin-prettier"
import solidPlugin from "eslint-plugin-solid";
import tseslint from 'typescript-eslint'

export default tseslint.config(
  { ignores: ['dist'] },
  {
    "plugins": ["solid"],
    extends: [js.configs.recommended, ...tseslint.configs.recommended],
    files: ['**/*.{ts,tsx}'],
    languageOptions: {
      ecmaVersion: 2020,
      globals: globals.browser,
    },
    plugins: {
      prettier: prettierPlugin,
      solid: solidPlugin,
    },
    rules: {
      ...prettierPlugin.configs.recommended.rules,
      ...solidPlugin.configs.recommended.rules,
    },
  },
)
