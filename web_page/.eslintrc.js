module.exports = {
  "env": {
    "browser": true,
    "es2021": true,
    "node": true
  },
  globals: {
    defineProps: "readonly",
    defineEmits: "readonly",
    defineExpose: "readonly"
  },
  parser: "vue-eslint-parser",
  extends: [
    "eslint:recommended",
    'plugin:vue/vue3-recommended', // 使用插件支持vue3
    'plugin:vue/vue3-essential',
    "plugin:@typescript-eslint/recommended",
    './.eslintrc-auto-import.json',
  ],
  parserOptions: {
    "ecmaVersion": "latest",
    "parser": {
      // Script parser for `<script>`
      "js": "espree",

      // Script parser for `<script lang="ts">`
      "ts": "@typescript-eslint/parser",

      // Script parser for vue directives (e.g. `v-if=` or `:attribute=`)
      // and vue interpolations (e.g. `{{variable}}`).
      // If not specified, the parser determined by `<script lang ="...">` is used.
      "<template>": "espree",
    },
    "sourceType": "module"
  },
  plugins: [
    "vue",
    "@typescript-eslint"
  ],
  rules: {
    "vue/multi-word-component-names": "off",
    "@typescript-eslint/no-empty-function": "off", // 关闭空方法
    "@typescript-eslint/no-explicit-any": "off", // 关闭any类型的警告
    "vue/no-v-model-argument": "off",
  }
}
