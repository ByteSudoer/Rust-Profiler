import { h, render } from "https://unpkg.com/preact?module";
import htm from "https://unpkg.com/htm?module";

const html = htm.bind(h);





function App(props) {
  return html`
    <div>
      <div class="core-container">
        <label class="core">Memory: </label>
        <div class="bar">
          <div class="bar-inner" style="width: ${props.mem}%"></div>
          <label>${props.mem.toFixed(2)}%</label>
        </div>
      </div>
      <div>
        ${props.cpus.map((cpu, index) => {
    return html`
            <div class="core-container">
              <label class="core">Core ${index + 1}: </label>
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



async function getFullSystemData() {
  try {
    const response = await fetch("http://127.0.0.1:3000/full_system");

    if (!response.ok) {
      throw new Error("Network response was not OK");
    }

    const full_system = await response.json();
    console.log("Full System Data:", full_system);
    return full_system;
  } catch (error) {
    console.error("Error fetching data:", error);
    return null; // Return a default or error state if needed
  }
}


async function renderFullSystem() {
  const full_system = await getFullSystemData();
  if (!full_system) {
    return; // Handle error state if needed
  }

  const dataContainer = document.getElementById("data");

  const cpuInfo = `
    <h2>CPU Information</h2>
    <table>
      <tr><td><strong>Vendor:</strong></td><td>${full_system.cpu.vendor}</td></tr>
      <tr><td><strong>Model:</strong></td><td>${full_system.cpu.model}</td></tr>
      <tr><td><strong>Number of Cores:</strong></td><td>${full_system.cpu.num_cores}</td></tr>
      <tr><td><strong>Base Frequency:</strong></td><td>${full_system.cpu.frequency.base} MHz</td></tr>
      <tr><td><strong>Max Frequency:</strong></td><td>${full_system.cpu.frequency.max} MHz</td></tr>
    </table>
  `;

  const disksInfo = `
    <h2>Disks Information</h2>
    <table>
      ${full_system.disks.disks.map(disk => `
        <tr>
          <td><strong>Device Name:</strong></td><td>${disk.device_name}</td>
          <td><strong>Disk Type:</strong></td><td>${disk.disk_type}</td>
          <td><strong>File System:</strong></td><td>${disk.file_system}</td>
          <td><strong>Mount Point:</strong></td><td>${disk.mount_point}</td>
          <td><strong>Total Space:</strong></td><td>${disk.total_space}</td>
          <td><strong>Available Space:</strong></td><td>${disk.available_space}</td>
        </tr>
      `).join('')}
    </table>
  `;

  const usersInfo = `
    <h2>Users Information</h2>
    <table>
      ${full_system.users.users.map(user => `
        <tr>
          <td><strong>Name:</strong></td><td>${user.name}</td>
          <td><strong>UID:</strong></td><td>${user.uid}</td>
          <td><strong>Groups:</strong></td><td>${user.groups.join(', ')}</td>
        </tr>
      `).join('')}
    </table>
  `;

  const memoryInfo = `
    <h2>Memory Information</h2>
    <table>
      <tr><td><strong>RAM Size:</strong></td><td>${full_system.memory.ram_size}</td></tr>
      <tr><td><strong>Swap Size:</strong></td><td>${full_system.memory.swap_size}</td></tr>
    </table>
  `;

  dataContainer.innerHTML = cpuInfo + memoryInfo + disksInfo + usersInfo  ;
}


renderFullSystem();
initApp();

