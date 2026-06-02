import { Routes, Route } from 'react-router-dom'

function Home() {
  return (
    <div>
      <h1>Driver Web</h1>
      <nav>
        <a href="/">Home</a>
      </nav>
      <p>Station discovery map will be here</p>
    </div>
  )
}

function App() {
  return (
    <Routes>
      <Route path="/" element={<Home />} />
    </Routes>
  )
}

export default App
