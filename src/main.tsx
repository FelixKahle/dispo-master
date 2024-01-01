// Copyright 2023 Felix Kahle. All rights reserved.

import React from "react";
import ReactDOM from "react-dom/client";
import App from "./app/App";
import { closeSplashScreen } from "./tauri-api/tauriApi";

// Close the splash screen when the DOM is loaded.
document.addEventListener("DOMContentLoaded", async () => {
  await setTimeout(() => {
    closeSplashScreen();
  }, 5000);
});

// Render the application.
// Like the main entry point.
ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
