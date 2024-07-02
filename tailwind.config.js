/** @type {import("tailwindcss").Config} */
export default {
    content: [
        "./index.html", "./src/**/*.{js,ts,jsx,tsx}",
    ],
    theme: {
        extend: {
            colors: {
                "main": "rgba(40, 40, 40, 0.7)",
                "header": "rgba(20, 20, 20, 0.7)",
            },
        },
    },
    plugins: [
        require("@tailwindcss/typography"),
    ],
};
