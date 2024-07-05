/** @type {import("tailwindcss").Config} */
export default {
    content: [
        "./index.html", "./src/**/*.{js,ts,jsx,tsx}",
    ],
    theme: {
        extend: {
            colors: {
                "header": "rgba(25, 25, 25, 0.7)",
                "item-odd": "rgb(48, 48, 48, 0.7)",
                "item": "rgba(40, 40, 40, 0.7)",
                "usage": "#6fad00",
            },
        },
    },
    plugins: [
        require("@tailwindcss/typography"),
    ],
};
