/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    extend: {
      colors: {
        borne: {
          green: "#22c55e",
          red: "#ef4444",
        },
      },
    },
  },
  plugins: [],
};
