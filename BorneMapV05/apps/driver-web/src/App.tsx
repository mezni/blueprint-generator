import Header from './components/Header'
import MapView from './components/MapView'
import ErrorBoundary from './components/ErrorBoundary'
import { useState } from 'react'

function App() {
  const [searchOpen, setSearchOpen] = useState(false)

  return (
    <ErrorBoundary>
      <div className="flex h-svh w-full flex-col overflow-hidden">
        <Header onSearchToggle={() => setSearchOpen((p) => !p)} />
        <main className="flex flex-1 overflow-hidden">
          <MapView searchOpen={searchOpen} onSearchClose={() => setSearchOpen(false)} />
        </main>
      </div>
    </ErrorBoundary>
  )
}

export default App
