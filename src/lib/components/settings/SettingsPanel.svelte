<script lang="ts">
  import { TauriCommands } from "$lib/services/tauri-commands";
  import { hasPermissions } from "$lib/stores/app-state";
  import { onMount } from "svelte";

  let activate = "Cmd+J";
  let scanDepth = 8;
  let maxHints = 400;
  let isChecking = false;
  let isSaving = false;
  let isOpeningPrefs = false;

  onMount(async () => {
    // hasPermissions.set(await TauriCommands.checkPermissions());
    await checkPermissionsStatus();
  });

  async function checkPermissionsStatus() {
    isChecking = true;
    try {
      hasPermissions.set(await TauriCommands.checkPermissions());
    } catch (e) {
      console.error("Error checking permissions:", e);
      hasPermissions.set(false);
    } finally {
      isChecking = false;
    }
  }

  async function requestAccess() {
    try {
      await TauriCommands.requestPermissions();
      await new Promise((resolve) => setTimeout(resolve, 1000)); // wait a second for the OS to process
      await checkPermissionsStatus();
    } catch (e) {
      console.error("Error requesting permissions:", e);
    }
  }

  async function openPreferences() {
    if (isOpeningPrefs) return;
    isOpeningPrefs = true;
    try {
      await TauriCommands.openAccessibilitySettings();
      const poll = async (attempts = 0) => {
        const ok = await TauriCommands.checkPermissions();
        hasPermissions.set(ok);
        if (!ok && attempts < 10) {
          await new Promise((r) => setTimeout(r, 1000));
          return poll(attempts + 1);
        }
      };
      await poll();
    } catch (e) {
      console.error("Error opening accessibility settings:", e);
    } finally {
      isOpeningPrefs = false;
    }
  }

  async function save() {
    if (!$hasPermissions) return;

    isSaving = true;
    try {
      await TauriCommands.registerHotkey(activate);
      // TODO: Persist config via Tauri command
    } catch (e) {
      console.error("Error saving settings:", e);
    } finally {
      isSaving = false;
    }
  }
</script>

<section class="space-y-4">
  <header class="flex items-center gap-3">
    <h1 class="text-xl font-semibold">PLTM</h1>
    {#if isChecking}
      <span class="px-2 py-1 rounded bg-gray-100 text-gray-600 text-xs">
        Verificando...
      </span>
    {:else if $hasPermissions}
      <span class="px-2 py-1 rounded bg-green-100 text-green-700 text-xs">
        Permisos OK
      </span>
    {:else}
      <button
        class="px-2 py-1 text-xs rounded bg-amber-100 text-amber-800 hover:bg-amber-200"
        on:click={requestAccess}
      >
        Otorgar permisos
      </button>
    {/if}
  </header>

  <!-- Banner de advertencia cuando no hay permisos -->
  {#if !$hasPermissions && !isChecking}
    <div
      class="flex items-center justify-between p-3 rounded-lg bg-amber-50 border border-amber-200"
    >
      <div class="flex items-center gap-2">
        <svg
          class="w-5 h-5 text-amber-600"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
          />
        </svg>
        <span class="text-sm text-amber-800">
          Se requieren permisos de accesibilidad para usar PLTM.
        </span>
      </div>
      <button
        class="px-2 py-1 text-xs rounded bg-amber-200 text-amber-800 hover:bg-amber-300"
        on:click={checkPermissionsStatus}
        disabled={isChecking}
      >
        Volver a comprobar
      </button>
      <button
        class="px-2 py-1 text-xs rounded bg-amber-100 text-amber-800 hover:bg-amber-200 disabled:opacity-50"
        on:click={openPreferences}
        disabled={isOpeningPrefs}
      >
        {isOpeningPrefs ? "Abriendo..." : "Ver permisos"}
      </button>
    </div>
  {/if}

  <label class="flex items-center gap-2">
    <span class="w-36 text-sm font-medium">Hotkey principal</span>
    <input
      bind:value={activate}
      class="border px-2 py-1 rounded w-32 disabled:bg-gray-100 disabled:text-gray-500"
      disabled={!$hasPermissions}
    />
  </label>

  <label class="flex items-center gap-2">
    <span class="w-36 text-sm font-medium">Profundidad scan</span>
    <input
      type="number"
      min="1"
      max="15"
      bind:value={scanDepth}
      class="border px-2 py-1 rounded w-20 disabled:bg-gray-100 disabled:text-gray-500"
      disabled={!$hasPermissions}
    />
  </label>

  <label class="flex items-center gap-2">
    <span class="w-36 text-sm font-medium">Máx. hints</span>
    <input
      type="number"
      min="50"
      max="1000"
      bind:value={maxHints}
      class="border px-2 py-1 rounded w-24 disabled:bg-gray-100 disabled:text-gray-500"
      disabled={!$hasPermissions}
    />
  </label>

  <button
    class="px-3 py-2 rounded bg-blue-600 text-white disabled:bg-gray-400 disabled:cursor-not-allowed"
    on:click={save}
    disabled={!$hasPermissions || isSaving}
  >
    {#if isSaving}
      Guardando...
    {:else}
      Guardar
    {/if}
  </button>
</section>
