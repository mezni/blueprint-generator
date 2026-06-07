/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./App.js', './src/**/*.{js,ts,jsx,tsx}'],
  theme: {
    extend: {
      colors: {
        ev: {
          bg: '#F8FAF6',
          surface: '#FFFFFF',
          green: '#007943',
          glow: '#00E676',
          mapBg: '#EAF0E6',
          muted: '#6B7280',
          border: '#E5E7EB',
          textMain: '#111827',
        },
      },
      fontFamily: {
        sans: ['Plus Jakarta Sans', 'Inter', 'sans-serif'],
      },
    },
  },
  plugins: [],
}
