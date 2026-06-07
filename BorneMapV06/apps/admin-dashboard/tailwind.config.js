module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        admin: {
          bg: '#F8FAF6',          // Base grid canvas background
          sidebar: '#FFFFFF',     // Clean white fixed navigation sidebar
          card: '#FFFFFF',        // Elevated panel background
          emerald: '#007943',     // Primary brand identity / High-contrast green
          sageLight: '#EAF0E6',   // Selected state accents & map regions
          textMain: '#111827',    // Crisp charcoal gray for readable data text
          textMuted: '#6B7280',   // Subdued ash gray for column labels and metadata
          border: '#E5E7EB',      // Grid dividers
          
          // Operational statuses
          statusGreen: '#10B981',  // Available / Healthy
          statusOrange: '#F59E0B', // In Use
          statusRed: '#EF4444',    // Needs Maintenance
        }
      },
      fontFamily: {
        sans: ['Plus Jakarta Sans', 'Inter', 'sans-serif'],
      }
    }
  },
  plugins: [],
}
