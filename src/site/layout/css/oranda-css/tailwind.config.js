const defaultColors = require("tailwindcss/colors");
const defaultTheme = require("tailwindcss/defaultTheme");

const extend = {
  fontFamily: {
    sans: ["Comfortaa", "Comfortaa override", ...defaultTheme.fontFamily.sans],
  },
};

const theme = {
  colors: {
    "axo-pink": "hsla(326, 100%, 73%, 1)",
    "axo-pink-dark": "hsla(326, 52%, 58%, 1)",
    "axo-orange": "hsla(0, 87%, 70%, 1)",
    "axo-orange-dark": "hsla(356, 75%, 64%, 1)",
    "axo-highlighter": "hsla(51, 100%, 50%, 1)",
    "axo-black": "hsla(0, 0%, 5%, 1)",
    slate: defaultColors.slate,
    violet: defaultColors.violet,
    gray: defaultColors.gray,
    transparent: defaultColors.transparent,
  },
  extend,
};

const extractColorVars = (themeColors, colorGroup = "") =>
  Object.keys(themeColors).reduce((currentVars, colorKey) => {
    const value = themeColors[colorKey];
    const newVars =
      typeof value === "string"
        ? { [`--color${colorGroup}-${colorKey}`]: value }
        : extractColorVars(value, `-${colorKey}`);

    return { ...currentVars, ...newVars };
  }, {});

const tailwindColorsToCSSVariables = ({ addBase, theme }) =>
  addBase({
    ":root": extractColorVars(theme("colors")),
  });

module.exports = {
  darkMode: "class",
  theme: theme,
  plugins: [
    require("@tailwindcss/typography"),
    require("@tailwindcss/forms"),
    require("@tailwindcss/line-clamp"),
    tailwindColorsToCSSVariables,
  ],
};
