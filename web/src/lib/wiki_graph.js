import { writable } from "svelte/store";
export const result = writable([]);

/**
 * @param {any} arg
 */
export function display_result(arg) {
  result.set(JSON.parse(arg));
}
