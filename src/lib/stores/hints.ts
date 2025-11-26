import type { Hint } from "$lib/types";
import { derived, writable } from "svelte/store";

export const hints = writable<Hint[]>([]);
export const activeHint = writable<string>("");
export const inputSequence = writable<string>("");

export const filteredHints = derived(
  [hints, inputSequence],
  ([$hints, $inputSequence]) => {
    if (!$inputSequence) return $hints;
    return $hints.filter((hint) =>
      hint.label.startsWith($inputSequence.toLowerCase())
    );
  }
);
