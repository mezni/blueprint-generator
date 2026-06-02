import { Routes, Route } from 'react-router-dom'

function Admin() {
  return (
    <div>
      <h1>Admin Dashboard</h1>
      <nav>
        <a href="/">Home</a>
      </nav>
      <p>Platform administration will be here</p>
    </div>
  )
}

function App() {
  return (
    <Routes>
      <Route path="/" element={<Admin />} />
    </Routes>
  )
}

export default App
