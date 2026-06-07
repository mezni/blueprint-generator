import React from 'react'

function App() {
  return (
    <div className="min-h-screen bg-ev-bg flex flex-col items-center justify-center p-4">
      <div className="w-12 h-12 rounded-xl bg-ev-green flex items-center justify-center mb-4">
        <svg className="w-6 h-6 text-white" fill="currentColor" viewBox="0 0 24 24">
          <path d="M13 10V3L4 14h7v7l9-11h-7z" />
        </svg>
      </div>
      <h1 className="text-2xl font-black text-ev-green tracking-tight">
        BorneMap
      </h1>
      <p className="text-ev-muted text-sm mt-2">
        Driver Web App
      </p>
      <p className="text-ev-muted text-xs mt-8">
        Sprint 0 — Foundation
      </p>
    </div>
  )
}

export default App
