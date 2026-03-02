import { useState } from 'react'
import './App.css'

function App() {
  const [results, setResults] = useState([])

  const runEvaluation = async () => {
    // The raw engineering data we are sending to A.L.I.S.
    const testData = [{
      route_name: "React Dashboard Implementation",
      estimated_time_hours: 4.5,
      component_cost: 0.0,
      total_opportunities: 800.0,
      estimated_defects: 3.0
    }];

    try {
      const response = await fetch('http://localhost:3000/api/evaluate', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(testData)
      });

      const data = await response.json();
      setResults(data); // Save the calculated Lean metrics to the UI
    } catch (error) {
      console.error("Engine connection failed:", error);
    }
  }

  return (
    <div className="App">
      <h1>Project A.L.I.S. Engine 🧠⚙️</h1>
      <p>Technical Route Optimizer & Lean Six Sigma Calculator</p>
      
      <button onClick={runEvaluation} style={{ padding: '10px 20px', fontSize: '16px', cursor: 'pointer' }}>
        Run Route Evaluation
      </button>

      <div style={{ marginTop: '30px', textAlign: 'left', maxWidth: '400px', margin: '30px auto' }}>
        {results.map((route, index) => (
          <div key={index} style={{ border: '1px solid #ccc', padding: '20px', borderRadius: '8px' }}>
            <h2>🛠️ {route.route_name}</h2>
            <p><strong>Lean Score:</strong> {route.lean_score.toFixed(2)}</p>
            <p><strong>DPMO:</strong> {route.dpmo.toFixed(2)}</p>
            <p><strong>Process Yield:</strong> {route.process_yield.toFixed(2)}%</p>
          </div>
        ))}
      </div>
    </div>
  )
}

export default App