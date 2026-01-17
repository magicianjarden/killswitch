<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  interface PortInfo {
    pid: number;
    process_name: string;
    port: number;
    protocol: string;
    state: string;
    local_address: string;
  }

  let ports: PortInfo[] = $state([]);
  let filteredPorts: PortInfo[] = $state([]);
  let searchQuery = $state("");
  let loading = $state(false);
  let error = $state("");
  let autoRefresh = $state(false);
  let refreshInterval: number | null = null;
  let killConfirm: { pid: number; name: string } | null = $state(null);
  let feedback = $state({ message: "", type: "" });

  $effect(() => {
    const query = searchQuery.toLowerCase();
    filteredPorts = ports.filter(
      (p) =>
        p.port.toString().includes(query) ||
        p.process_name.toLowerCase().includes(query) ||
        p.pid.toString().includes(query)
    );
  });

  $effect(() => {
    if (autoRefresh) {
      refreshInterval = setInterval(fetchPorts, 5000) as unknown as number;
    } else if (refreshInterval) {
      clearInterval(refreshInterval);
      refreshInterval = null;
    }
    return () => {
      if (refreshInterval) clearInterval(refreshInterval);
    };
  });

  async function fetchPorts() {
    loading = true;
    error = "";
    try {
      ports = await invoke<PortInfo[]>("get_ports");
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  async function killProcess(pid: number) {
    try {
      const result = await invoke<string>("kill_process", { pid });
      feedback = { message: result, type: "success" };
      killConfirm = null;
      await fetchPorts();
    } catch (e) {
      feedback = { message: String(e), type: "error" };
    }
    setTimeout(() => {
      feedback = { message: "", type: "" };
    }, 3000);
  }

  function confirmKill(pid: number, name: string) {
    killConfirm = { pid, name };
  }

  function cancelKill() {
    killConfirm = null;
  }

  // Initial fetch
  fetchPorts();
</script>

<div class="controls">
  <input
    type="text"
    placeholder="Search by port, process name, or PID..."
    bind:value={searchQuery}
    class="search-input"
  />
  <div class="control-buttons">
    <label class="auto-refresh">
      <input type="checkbox" bind:checked={autoRefresh} />
      Auto-refresh
    </label>
    <button onclick={fetchPorts} disabled={loading} class="refresh-btn">
      {loading ? "Loading..." : "Refresh"}
    </button>
  </div>
</div>

{#if feedback.message}
  <div class="feedback {feedback.type}">{feedback.message}</div>
{/if}

{#if error}
  <div class="error">{error}</div>
{/if}

{#if killConfirm}
  <div class="modal-overlay">
    <div class="modal">
      <h3>Confirm Kill</h3>
      <p>
        Kill process <strong>{killConfirm.name}</strong> (PID: {killConfirm.pid})?
      </p>
      <div class="modal-buttons">
        <button onclick={cancelKill} class="cancel-btn">Cancel</button>
        <button onclick={() => killProcess(killConfirm!.pid)} class="kill-btn">
          Kill
        </button>
      </div>
    </div>
  </div>
{/if}

<div class="table-container">
  <table>
    <thead>
      <tr>
        <th>Port</th>
        <th>Protocol</th>
        <th>PID</th>
        <th>Process</th>
        <th>Address</th>
        <th>State</th>
        <th>Action</th>
      </tr>
    </thead>
    <tbody>
      {#if filteredPorts.length === 0 && !loading}
        <tr>
          <td colspan="7" class="empty">
            {searchQuery ? "No matching ports found" : "No ports found"}
          </td>
        </tr>
      {:else}
        {#each filteredPorts as port}
          <tr>
            <td class="port">{port.port}</td>
            <td class="protocol">{port.protocol}</td>
            <td class="pid">{port.pid}</td>
            <td class="process">{port.process_name}</td>
            <td class="address">{port.local_address}</td>
            <td class="state">{port.state}</td>
            <td>
              <button
                class="kill-btn-small"
                onclick={() => confirmKill(port.pid, port.process_name)}
              >
                Kill
              </button>
            </td>
          </tr>
        {/each}
      {/if}
    </tbody>
  </table>
</div>

<div class="status-bar">
  Showing {filteredPorts.length} of {ports.length} ports
</div>

<style>
  .controls {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 16px;
    margin-bottom: 16px;
  }

  .search-input {
    flex: 1;
    padding: 10px 14px;
    font-size: 14px;
    background: #2a2a2a;
    border: 1px solid #3a3a3a;
    border-radius: 6px;
    color: #fff;
    outline: none;
    font-family: inherit;
  }

  .search-input:focus {
    border-color: #555;
  }

  .search-input::placeholder {
    color: #666;
  }

  .control-buttons {
    display: flex;
    align-items: center;
    gap: 16px;
  }

  .auto-refresh {
    display: flex;
    align-items: center;
    gap: 6px;
    color: #888;
    font-size: 13px;
    cursor: pointer;
  }

  .auto-refresh input {
    cursor: pointer;
  }

  .refresh-btn {
    padding: 10px 20px;
    font-size: 14px;
    background: #2a2a2a;
    border: 1px solid #3a3a3a;
    border-radius: 6px;
    color: #fff;
    cursor: pointer;
    transition: background 0.2s;
  }

  .refresh-btn:hover:not(:disabled) {
    background: #333;
  }

  .refresh-btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .feedback {
    padding: 10px 14px;
    margin-bottom: 16px;
    border-radius: 6px;
    font-size: 13px;
  }

  .feedback.success {
    background: #1a3a1a;
    border: 1px solid #2a5a2a;
    color: #6a6;
  }

  .feedback.error {
    background: #3a1a1a;
    border: 1px solid #5a2a2a;
    color: #a66;
  }

  .error {
    padding: 10px 14px;
    margin-bottom: 16px;
    background: #3a1a1a;
    border: 1px solid #5a2a2a;
    border-radius: 6px;
    color: #a66;
    font-size: 13px;
  }

  .table-container {
    overflow-x: auto;
    border: 1px solid #2a2a2a;
    border-radius: 8px;
  }

  table {
    width: 100%;
    border-collapse: collapse;
    font-size: 13px;
  }

  th {
    text-align: left;
    padding: 12px 14px;
    background: #222;
    color: #888;
    font-weight: 500;
    text-transform: uppercase;
    font-size: 11px;
    letter-spacing: 0.5px;
    border-bottom: 1px solid #2a2a2a;
  }

  td {
    padding: 10px 14px;
    border-bottom: 1px solid #222;
    color: #ccc;
  }

  tr:hover td {
    background: #1f1f1f;
  }

  .port {
    font-family: "SF Mono", Monaco, "Consolas", monospace;
    font-weight: 600;
    color: #6cf;
  }

  .protocol {
    font-family: "SF Mono", Monaco, "Consolas", monospace;
    color: #999;
  }

  .pid {
    font-family: "SF Mono", Monaco, "Consolas", monospace;
    color: #888;
  }

  .process {
    font-weight: 500;
    color: #fff;
  }

  .address {
    font-family: "SF Mono", Monaco, "Consolas", monospace;
    color: #888;
    font-size: 12px;
  }

  .state {
    font-size: 11px;
    text-transform: uppercase;
    color: #6a6;
  }

  .empty {
    text-align: center;
    color: #666;
    padding: 40px !important;
  }

  .kill-btn-small {
    padding: 4px 10px;
    font-size: 11px;
    background: transparent;
    border: 1px solid #633;
    border-radius: 4px;
    color: #c66;
    cursor: pointer;
    transition: all 0.2s;
  }

  .kill-btn-small:hover {
    background: #3a1a1a;
    border-color: #944;
  }

  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.7);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 100;
  }

  .modal {
    background: #222;
    border: 1px solid #333;
    border-radius: 10px;
    padding: 24px;
    min-width: 320px;
  }

  .modal h3 {
    margin: 0 0 12px 0;
    color: #fff;
    font-size: 18px;
  }

  .modal p {
    margin: 0 0 20px 0;
    color: #aaa;
    font-size: 14px;
  }

  .modal strong {
    color: #fff;
  }

  .modal-buttons {
    display: flex;
    gap: 10px;
    justify-content: flex-end;
  }

  .cancel-btn {
    padding: 8px 16px;
    font-size: 13px;
    background: #333;
    border: 1px solid #444;
    border-radius: 6px;
    color: #ccc;
    cursor: pointer;
  }

  .cancel-btn:hover {
    background: #3a3a3a;
  }

  .kill-btn {
    padding: 8px 16px;
    font-size: 13px;
    background: #8b2222;
    border: 1px solid #a33;
    border-radius: 6px;
    color: #fff;
    cursor: pointer;
  }

  .kill-btn:hover {
    background: #a33;
  }

  .status-bar {
    margin-top: 12px;
    text-align: right;
    font-size: 12px;
    color: #666;
  }
</style>
