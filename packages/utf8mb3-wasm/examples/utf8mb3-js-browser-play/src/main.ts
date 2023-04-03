import "./style.css";
import { encode, decode } from "utf8mb3";

document.querySelector<HTMLDivElement>("#app")!.innerHTML = `
  <div>
    <h1>Encode 1000000 "😊"</h1>
    <p id="content" class="read-the-docs"></p>
  </div>
`;

let str = "😊".repeat(1000000);
const ts = performance.now();
decode(encode(str));
const te = performance.now();
document.querySelector("#content")!.innerHTML = `encode time: ${te - ts}ms`;
