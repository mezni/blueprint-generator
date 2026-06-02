import { Routes, Route } from 'react-router-dom'

function Dashboard() {
  return (
    <div>
      <h1>Partner Dashboard</h1>
      <nav>
        <a href="/">Dashboard</a>
      </nav>
      <p>Station management will be here</p>
    </div>
  )
}

function App() {
  return (
    <Routes>
      <Route path="/" element={<Dashboard />} />
    </Routes>
  )
}

export default App
