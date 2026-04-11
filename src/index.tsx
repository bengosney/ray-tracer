import React from "react";
import ReactDOM from "react-dom/client";
import "./index.css";
import App from "./App";

// Ensure static assets are copied to build folder (webpack asset modules)
if (process.env.NODE_ENV === "production") {
  // @ts-expect-error: file not in src
  import("../public/coi-serviceworker.js");
  // @ts-expect-error: file not in src
  import("../public/favicon.ico");
  // @ts-expect-error: file not in src
  import("../public/logo192.png");
  // @ts-expect-error: file not in src
  import("../public/logo512.png");
  // @ts-expect-error: file not in src
  import("../public/manifest.json");
  // @ts-expect-error: file not in src
  import("../public/robots.txt");
}

const root = ReactDOM.createRoot(document.getElementById("root") as HTMLElement);
root.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
