<script lang="ts">
  import SettingsPanel from "$lib/components/settings/SettingsPanel.svelte";
  import { TauriCommands } from "$lib/services/tauri-commands";
  import { hasPermissions, isNavigationActive } from "$lib/stores/app-state";
  import type { Hint } from "$lib/types";
  import { onMount } from "svelte";

  let showPermissionModal = false;
  let lastHints: Hint[] = [];
  let errorMessage = "";
  let dialogRef: HTMLDialogElement | null = null;

  onMount(async () => {
    try {
      hasPermissions.set(await TauriCommands.checkPermissions());
    } catch (e) {
      console.error("Failed to check permissions on mount:", e);
      hasPermissions.set(false);
    }
  });

  function openModal() {
    showPermissionModal = true;
    dialogRef?.showModal();
  }

  function closeModal() {
    showPermissionModal = false;
    dialogRef?.close();
  }

  async function activateNavigation() {
    if (!$hasPermissions) {
      showPermissionModal = true;
      return;
    }

    errorMessage = "";

    try {
      isNavigationActive.set(true);
      const hints = await TauriCommands.activateNavigation();
      lastHints = hints;
      console.log("Navigation activated with", hints.length, "hints");
    } catch (e) {
      console.error("Failed to activate navigation:", e);
      errorMessage = e instanceof Error ? e.message : String(e);
      isNavigationActive.set(false);
    }
  }

  async function deactivateNavigation() {
    try {
      await TauriCommands.deactivateNavigation();
      isNavigationActive.set(false);
      lastHints = [];
      console.log("Navigation deactivated");
    } catch (e) {
      console.error("Failed to deactivate navigation:", e);
    }
  }

  function goToPermissions() {
    showPermissionModal = false;
    window.open(
      "x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility"
    );
  }
</script>

<main class="min-h-screen bg-gray-50 p-6">
  <div class="max-w-md mx-auto bg-white rounded-xl shadow-md p-6">
    <SettingsPanel />

    <div class="mt-6 pt-4 border-t space-y-3">
      {#if errorMessage}
        <div
          class="p-3 rounded-lg bg-red-50 border border-red-200 text-red-700 text-sm"
        >
          {errorMessage}
        </div>
      {/if}

      {#if $isNavigationActive}
        <button
          class="w-full px-4 py-3 rounded-lg bg-red-600 text-white font-medium hover:bg-red-700 transition-colors"
          on:click={deactivateNavigation}
        >
          Desactivar navegación
        </button>

        {#if lastHints.length > 0}
          <div class="p-3 rounded-lg bg-green-50 border border-green-200">
            <p class="text-sm text-green-800 font-medium">
              {lastHints.length} hints generados
            </p>
            <div class="mt-2 max-h-32 overflow-y-auto">
              {#each lastHints.slice(0, 10) as hint}
                <span
                  class="inline-block px-2 py-1 m-0.5 bg-yellow-200 text-yellow-800 text-xs rounded font-mono"
                >
                  {hint.label}
                </span>
              {/each}
              {#if lastHints.length > 10}
                <span class="text-xs text-gray-500"
                  >+{lastHints.length - 10} más</span
                >
              {/if}
            </div>
          </div>
        {/if}
      {:else}
        <button
          class="w-full px-4 py-3 rounded-lg bg-indigo-600 text-white font-medium
                 disabled:bg-gray-400 disabled:cursor-not-allowed
                 hover:bg-indigo-700 transition-colors"
          on:click={activateNavigation}
          disabled={!$hasPermissions}
        >
          {#if !$hasPermissions}
            Requiere permisos
          {:else}
            Activar navegación (Cmd+J)
          {/if}
        </button>
      {/if}
    </div>
  </div>

  <!-- Modal de permisos -->
  {#if showPermissionModal}
    <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
    <div
      class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
      on:click={() => (showPermissionModal = false)}
      on:keydown={(e) => e.key === "Escape" && (showPermissionModal = false)}
      role="dialog"
      aria-modal="true"
      aria-labelledby="permission-modal-title"
      tabindex="-1"
    >
      <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
      <dialog
        class="bg-white rounded-xl p-6 max-w-sm mx-4 shadow-2xl"
        bind:this={dialogRef}
        on:close={() => (showPermissionModal = false)}
      >
        <h2 id="permission-modal-title" class="text-lg font-semibold mb-3">
          Permisos requeridos
        </h2>
        <p class="text-gray-600 text-sm mb-4">
          PLTM necesita permisos de accesibilidad para funcionar. Por favor,
          habilítalos en Preferencias del Sistema.
        </p>
        <div class="flex gap-3">
          <button
            class="flex-1 px-4 py-2 rounded bg-gray-200 text-gray-800 hover:bg-gray-300"
            on:click={() => (showPermissionModal = false)}
          >
            Cancelar
          </button>
          <button
            class="flex-1 px-4 py-2 rounded bg-indigo-600 text-white hover:bg-indigo-700"
            on:click={goToPermissions}
          >
            Abrir Preferencias
          </button>
        </div>
      </dialog>
    </div>
  {/if}
</main>
