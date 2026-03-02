import { useState } from 'react'
import './App.css'

function App() {
  // 1. Form State Variables
  const [routeName, setRouteName] = useState('')
  const [time, setTime] = useState('')
  const [cost, setCost] = useState('')
  const [opportunities, setOpportunities] = useState('')
  const [defects, setDefects] = useState('')

  // 2. Results State
  const [results, setResults] = useState([])
  const [isEvaluating, setIsEvaluating] = useState(false)

  // 3. The Submit Function
  const handleEvaluate = async (e) => {
    e.preventDefault() 
    setIsEvaluating(true)

    // Package the input exactly how your Rust engine expects it
    const payload = [{
      route_name: routeName,
      estimated_time_hours: parseFloat(time),
      component_cost: parseFloat(cost),
      total_opportunities: parseFloat(opportunities),
      estimated_defects: parseFloat(defects)
    }]

    try {
      // ⚠️ UPDATE THIS LINE to use your live Render URL
      const response = await fetch('https://project-a-l-i-s.onrender.com/api/evaluate', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload)
      })

      const data = await response.json()
      setResults(data)
    } catch (error) {
      console.error("Engine connection failed:", error)
      alert("Failed to connect to the A.L.I.S. backend. Make sure the Rust server is running on port 3000!")
    }
    
    setIsEvaluating(false)
  }

  return (
    <div className="App" style={{ fontFamily: 'system-ui, sans-serif', maxWidth: '800px', margin: '0 auto', padding: '20px' }}>
      <h1>Project A.L.I.S. Dashboard 🧠⚙️</h1>
      <p>Lean Six Sigma Technical Route Optimizer</p>

      <div style={{ display: 'flex', gap: '40px', marginTop: '30px', flexWrap: 'wrap' }}>
        
        {/* LEFT SIDE: The Input Form */}
        <div style={{ flex: '1', minWidth: '300px', textAlign: 'left', background: '#f8f9fa', padding: '20px', borderRadius: '8px', color: '#333' }}>
          <h3>Configure Route</h3>
          <form onSubmit={handleEvaluate} style={{ display: 'flex', flexDirection: 'column', gap: '15px' }}>
            
            <label>Route Name / Strategy
              <input type="text" required value={routeName} onChange={(e) => setRouteName(e.target.value)} style={{ width: '100%', padding: '8px', marginTop: '5px' }} />
            </label>
            
            <label>Estimated Time (Hours)
              <input type="number" step="0.1" required value={time} onChange={(e) => setTime(e.target.value)} style={{ width: '100%', padding: '8px', marginTop: '5px' }} />
            </label>

            <label>Component/Server Cost ($)
              <input type="number" step="0.1" required value={cost} onChange={(e) => setCost(e.target.value)} style={{ width: '100%', padding: '8px', marginTop: '5px' }} />
            </label>

            <label>Total Opportunities (Lines of code, API calls)
              <input type="number" required value={opportunities} onChange={(e) => setOpportunities(e.target.value)} style={{ width: '100%', padding: '8px', marginTop: '5px' }} />
            </label>

            <label>Estimated Defects (Bugs, failures)
              <input type="number" required value={defects} onChange={(e) => setDefects(e.target.value)} style={{ width: '100%', padding: '8px', marginTop: '5px' }} />
            </label>

            <button type="submit" disabled={isEvaluating} style={{ padding: '12px', background: '#007bff', color: 'white', border: 'none', borderRadius: '4px', cursor: 'pointer', fontWeight: 'bold' }}>
              {isEvaluating ? 'Evaluating...' : 'Run Engine'}
            </button>
          </form>
        </div>

        {/* RIGHT SIDE: The Results */}
        <div style={{ flex: '1', minWidth: '300px', textAlign: 'left' }}>
          <h3>Evaluation Results</h3>
          {results.length === 0 ? (
            <p style={{ color: '#666' }}>Awaiting data input...</p>
          ) : (
            results.map((route, index) => (
              <div key={index} style={{ border: '2px solid #28a745', padding: '20px', borderRadius: '8px', background: '#e9f7ef', color: '#155724' }}>
                <h2 style={{ margin: '0 0 10px 0' }}>🛠️ {route.route_name}</h2>
                <h1 style={{ margin: '0 0 10px 0', fontSize: '2.5rem' }}>{route.process_yield.toFixed(2)}% Yield</h1>
                <p><strong>Lean Score:</strong> {route.lean_score.toFixed(2)}</p>
                <p><strong>DPMO:</strong> {route.dpmo.toFixed(2)}</p>
              </div>
            ))
          )}
        </div>

      </div>
    </div>
  )
}

export default App