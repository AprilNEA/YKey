/**
 * Copyright 2025 AprilNEA LLC
 * SPDX-License-Identifier: MIT
 */

import "./index.css";
import { Route, Router } from "wouter";

import LandingPage from "./pages/index";
import XKeyManager from "./pages/manager";

function App() {
  return (
    <Router>
      <Route path="/" component={LandingPage} />
      <Route path="/manager" component={XKeyManager} />
      <Route path="/manager/:section" component={XKeyManager} />
    </Router>
  );
}

export default App;
