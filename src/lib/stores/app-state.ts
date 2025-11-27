import { writable } from "svelte/store";

export type AppMode = "idle" | "navigation" | "settings";

export const appMode = writable<AppMode>("idle");
export const isNavigationActive = writable<boolean>(false);
export const hasPermissions = writable<boolean>(false);
export const isLoading = writable<boolean>(false);