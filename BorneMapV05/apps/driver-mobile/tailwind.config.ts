/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./src/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    extend: {},
    important: true,
  },
  plugins: [
    require("tailwindcss-plugin-rtl")({
      prependDir: "node_modules/tailwindcss-plugin-rtl/lib",
    }),
  ],
}
