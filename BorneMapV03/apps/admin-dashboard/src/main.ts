import "./style.css";
import { initAuth } from "./auth";

document.querySelector<HTMLDivElement>("#app")!.innerHTML = `
  <div style="display:flex;align-items:center;justify-content:center;min-height:100vh;">
    <p>Loading...</p>
  </div>
`;

initAuth();
