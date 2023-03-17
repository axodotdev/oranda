const cssnano = require("cssnano");
const tailwindcss = require("tailwindcss");
const autoprefixer = require("autoprefixer");
const postcssImport = require("postcss-import");
const nesting = require("tailwindcss/nesting");

const plugins = [tailwindcss, autoprefixer, cssnano, postcssImport, nesting];

module.exports = { plugins };
