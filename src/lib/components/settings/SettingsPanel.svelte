<script lang="ts">
  import { TauriCommands } from "$lib/services/tauri-commands";
  import { hasPermissions } from "$lib/stores/app-state";
  import { onMount } from "svelte";

  let activate = "Cmd+J";
  let scanDepth = 8;
  let maxHints = 400;

  onMount(async () => {
    hasPermissions.set(await TauriCommands.checkPermissions());
  });

  async function requestAccess() {
    await TauriCommands.requestPermissions();
    hasPermissions.set(await TauriCommands.checkPermissions());
  }

  async function save() {
    await TauriCommands.registerHotkey(activate);
    // TODO: Persist config via Tauri command
  }
</script>

<section class="space-y-4">
  <header class="flex items-center gap-3">
    <h1 class="text-xl font-semibold">PLTM</h1>
    {#if $hasPermissions}
      <span
        class="px-2 py-1 rounded bg-green-100 text-green-700 text-xs"
      >
        Permisos OK
      </span>
    {:else}
      <button
        class="px-2 py-1 text-xs rounded bg-amber-100 text-amber-800"
        on:click={requestAccess}
      >
        Otorgar permisos
      </button>
    {/if}
  </header>

  <label class="flex items-center gap-2">
    <span class="w-36 text-sm font-medium">Hotkey principal</span>
    <input bind:value={activate} class="border px-2 py-1 rounded w-32" />
  </label>

  <label class="flex items-center gap-2">
    <span class="w-36 text-sm font-medium">Profundidad scan</span>
    <input
      type="number"
      min="1"
      max="15"
      bind:value={scanDepth}
      class="border px-2 py-1 rounded w-20"
    />
  </label>

  <label class="flex items-center gap-2">
    <span class="w-36 text-sm font-medium">Máx. hints</span>
    <input
      type="number"
      min="50"
      max="1000"
      bind:value={maxHints}
      class="border px-2 py-1 rounded w-24"
    />
  </label>

  <button class="px-3 py-2 rounded bg-blue-600 text-white" on:click={save}>
    Guardar
  </button>
</section>
