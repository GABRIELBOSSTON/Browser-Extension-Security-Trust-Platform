import { invoke } from '@tauri-apps/api/core';

/**
 * Murni wrapper memanggil invoke dari Tauri API.
 * Menyediakan safe boundaries agar file lain tidak langsung dependen pada tauri-apps/api.
 */
export async function tauriInvoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  return await invoke<T>(cmd, args);
}
