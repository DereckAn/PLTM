<script lang="ts">
  import SettingsPanel from "$lib/components/settings/SettingsPanel.svelte";
  import { TauriCommands } from "$lib/services/tauri-commands";
  import { hasPermissions, isNavigationActive } from "$lib/stores/app-state";
  import { onMount } from "svelte";

  let showPermissionModal = false;
  let dialogRef: HTMLDialogElement;

  onMount(async () => {
    // Verificar permisos al cargar la app
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
    // Guard: verificar permisos antes de activar navegación
    if (!$hasPermissions) {
      showPermissionModal = true;
      return;
    }

    try {
      isNavigationActive.set(true);
      const elements = await TauriCommands.scanElements();
      console.log("Scanned elements:", elements.length);
      // TODO: Mostrar overlay con hints
    } catch (e) {
      console.error("Failed to activate navigation:", e);
      isNavigationActive.set(false);
    }
  }

  function goToPermissions() {
    showPermissionModal = false;
    // Abrir preferencias del sistema en macOS
    window.open(
      "x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility"
    );
  }
</script>

<main class="min-h-screen bg-gray-50 p-6">
  <div class="max-w-md mx-auto bg-white rounded-xl shadow-md p-6">
    <SettingsPanel />

    <div class="mt-6 pt-4 border-t">
      <button
        class="w-full px-4 py-3 rounded-lg bg-indigo-600 text-white font-medium
               disabled:bg-gray-400 disabled:cursor-not-allowed
               hover:bg-indigo-700 transition-colors"
        on:click={activateNavigation}
        disabled={!$hasPermissions || $isNavigationActive}
      >
        {#if $isNavigationActive}
          Navegación activa...
        {:else if !$hasPermissions}
          Requiere permisos
        {:else}
          Activar navegación (Cmd+J)
        {/if}
      </button>
    </div>
  </div>

  <!-- Modal de permisos -->
  {#if showPermissionModal}
    <div
      class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
      on:click={() => (showPermissionModal = false)}
      on:keydown={(e) => e.key === "Escape" && (showPermissionModal = false)}
      role="dialog"
      aria-modal="true"
      tabindex="-1"
      aria-labelledby="permission-modal-title"
    >
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
