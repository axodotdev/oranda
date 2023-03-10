const defaultColors = require("tailwindcss/colors");
const defaultTheme = require("tailwindcss/defaultTheme");
const listStyleType = {
  circle: "circle",
  square: "square",
  "lower-roman": "lower-roman",
  "lower-alpha": "lower-alpha",
};

const maxWidth = {
  "prose-lg": "80ch",
};

const backgroundImage = {
  "github-logo": `url("data:image/svg+xml,%3Csvg role='img' viewBox='0 0 24 24' xmlns='http://www.w3.org/2000/svg'%3E%3Ctitle%3EGitHub%3C/title%3E%3Cpath fill='%23111827' d='M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.399 3-.405 1.02.006 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.42.36.81 1.096.81 2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12'/%3E%3C/svg%3E")`,
};
const extend = {
  fontFamily: {
    sans: ["Comfortaa", "Comfortaa override", ...defaultTheme.fontFamily.sans],
  },
  listStyleType,
  maxWidth,
  backgroundImage,
  colors: {
    "axo-pink": "hsla(326, 100%, 73%, 1)",
    "axo-pink-dark": "hsla(326, 52%, 58%, 1)",
    "axo-orange": "hsla(0, 87%, 70%, 1)",
    "axo-orange-dark": "hsla(356, 75%, 64%, 1)",
    "axo-highlighter": "hsla(51, 100%, 50%, 1)",
    "axo-black": "hsla(0, 0%, 5%, 1)",
  },
};

const theme = {
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
