<script lang="ts">
  import PortTable from "./lib/PortTable.svelte";
</script>

<main>
  <header>
    <h1>Killswitch</h1>
    <p class="subtitle">View, search, and kill processes by port</p>
  </header>
  <PortTable />
</main>

<style>
  main {
    max-width: 1200px;
    margin: 0 auto;
    padding: 20px;
  }

  header {
    margin-bottom: 24px;
  }

  h1 {
    margin: 0;
    font-size: 28px;
    font-weight: 600;
    color: #fff;
  }

  .subtitle {
    margin: 4px 0 0 0;
    font-size: 14px;
    color: #888;
  }
</style>
