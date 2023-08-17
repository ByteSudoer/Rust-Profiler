import { h, render } from "https://unpkg.com/preact?module";
import htm from "https://unpkg.com/htm?module";

const html = htm.bind(h);

function App(props) {
  return html`
    <div>
      <div class="core-container">
        <label>Memory: </label>
        <div class="bar">
          <div class="bar-inner" style="width: ${props.mem}%"></div>
          <label>${props.mem.toFixed(2)}%</label>
        </div>
      </div>
      <div>
        ${props.cpus.map((cpu, index) => {
    return html`
            <div class="core-container">
              <label>Core ${index + 1}: </label>
              <div class="bar">
                <div class="bar-inner" style="width: ${cpu}%"></div>
                <label>${cpu.toFixed(2)}%</label>
              </div>
            </div>
          `;
  })}
      </div>
    </div>
  `;
}

// Use async function to await both WebSocket connections
async function initApp() {
  let url = new URL("/realtime/cpus", window.location.href);
  let mem_url = new URL("/realtime/mem", window.location.href);
  url.protocol = url.protocol.replace("http", "ws");
  mem_url.protocol = mem_url.protocol.replace("http", "ws");

  // Wait for both WebSocket connections to open
  const [ws, ws_mem] = await Promise.all([
    new Promise((resolve) => {
      const ws = new WebSocket(url.href);
      ws.onopen = () => resolve(ws);
    }),
    new Promise((resolve) => {
      const ws_mem = new WebSocket(mem_url.href);
      ws_mem.onopen = () => resolve(ws_mem);
    }),
  ]);

  // Store the received data from WebSocket connections
  let cpuData = [];
  let memData = 0;

  // Update UI when data from both connections have arrived
  function updateUI() {
    render(
      html`<${App} cpus=${cpuData} mem=${memData}></${App}>`,
      document.getElementById("graph"),
    );
  }

  ws.onmessage = (ev) => {
    let json = JSON.parse(ev.data);
    cpuData = json;
    updateUI();
  };

  ws_mem.onmessage = (ev) => {
    let json = JSON.parse(ev.data);
    memData = json;
    updateUI();
  };
}

initApp();
