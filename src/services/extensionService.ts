import { tauriInvoke } from './ipc';
import type { ExtensionSummaryResponse } from '@/types/ipc';

/**
 * Service untuk mendapatkan daftar ekstensi yang terinstall secara lokal.
 */
export const ExtensionService = {
  async getInstalledExtensions(): Promise<ExtensionSummaryResponse[]> {
    try {
      return await tauriInvoke<ExtensionSummaryResponse[]>('get_installed_extensions');
    } catch (error) {
      console.error('Failed to get installed extensions:', error);
      throw new Error(error as string);
    }
  },
};
