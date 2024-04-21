import { writable } from "svelte/store";
export const result = writable([]);

/**
 * @param {any} arg
 */
export function display_result(arg) {
  if (arg == "ERROR") {
    // @ts-ignore
    result.set(["ERROR"]);
    return;
  }
  result.set(JSON.parse(arg));
}
