# Project A.L.I.S. 🧠⚙️
**Artificial Linguistic Intelligence System | Technical Route Optimizer**

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Methodology](https://img.shields.io/badge/Methodology-Lean_Six_Sigma-blue.svg)](#methodology)

---

## 🌐 Live Dashboard
**Access the live production application here:** [https://alis-dashboard.onrender.com](https://alis-dashboard.onrender.com)

---

## 📖 Overview

Project A.L.I.S. is a high-performance, open-source command-line engine built in Rust. It serves as an automated Technical Lead, guiding engineers toward the most efficient, scalable, and lean development routes. 

Instead of relying on guesswork, A.L.I.S. applies **Lean Six Sigma methodologies** to software architecture. It calculates technical debt, processing waste, and failure probabilities to recommend the optimal engineering path, saving teams hours of trial and error.

---

## 📐 The Methodology: Six Sigma for Code
A.L.I.S. treats lines of code, API calls, and memory usage as points of potential failure. It evaluates proposed technical routes using standard process optimization metrics:
* **DPMO (Defects Per Million Opportunities):** Calculates the probable failure rate of a specific coding approach.
* **Process Yield:** Determines the efficiency and stability of the route.
* **DMAIC Framework:** Analyzes inputs to Define constraints, Measure baselines, Analyze waste, Improve the route, and Control the output.

---

## ✨ Features
* **Full-Stack Architecture:** Lightning-fast Rust (`axum`) REST API backend paired with a dynamic React (`vite`) frontend.
* **Actionable Intelligence:** Provides real-time, human-readable analysis of technical debt and risk levels based on Process Yield.
* **Cloud Integration:** Automatically syncs evaluation metrics to a centralized Render PostgreSQL database.
* **Versatile CLI:** Can be run locally as a terminal tool for file parsing or spawned as a local web server.

---

## 🗂️ Project Structure

```
project_alis/
├── .github/workflows/     # CI/CD Automation scripts
├── frontend/              # React (Vite) Web Dashboard
├── src/
│   ├── main.rs            # Async CLI & Server entry point
│   ├── api.rs             # Axum REST API routes
│   ├── database.rs        # PostgreSQL Render integration
│   ├── engine.rs          # Decision-matrix and CSV export logic
│   ├── six_sigma.rs       # Mathematical modules (DPMO, Yield)
│   └── ai.rs              # AI-powered route generation
└── data/                  # Sample JSON lab metrics
```

---

## 🚀 Getting Started

### Prerequisites

* Rust and Cargo installed on your system.
* Node.js and npm installed (for the frontend dashboard).

---

1. Installation
   
Clone the repository:

```
git clone [https://github.com/EngineerMapatac/Project-ALIS.git](https://github.com/EngineerMapatac/Project-ALIS.git)
```

---

2. Local Web Dashboard (Full-Stack)

To run the visual dashboard locally, you will need two terminals.

Terminal 1 (Start the Rust Backend):

```
cargo run -- serve
```

Terminal 2 (Start the React Frontend):

```
cd frontend
npm install
npm run dev
```
---

3. CLI Usage

If you prefer to use A.L.I.S. purely as a command-line tool:

Evaluate a dataset, export to CSV, and save to the cloud database:

```
cargo run -- evaluate --data data/lab_results.json --csv report.csv --save
```

Ask A.L.I.S. for an AI-suggested route:

```
cargo run -- suggest --prompt "implementing a real-time notification system"
```

---

### 🤝 Contributing
Contributions are highly encouraged! Whether it is adding new Lean metrics, optimizing the Rust concurrency models, or improving the documentation, your pull requests are welcome.

---

📄 License
This project is licensed under the *Apache License 2.0* - see the LICENSE file for details.
