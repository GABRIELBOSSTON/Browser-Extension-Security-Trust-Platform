import { tauriInvoke } from './ipc';
import type { ScanExtensionRequest, ScanExtensionResponse } from '@/types/ipc';

/**
 * Service untuk memicu dan mengelola siklus analisis (scan) extension.
 */
export const ScanService = {
  async scanExtension(request: ScanExtensionRequest): Promise<ScanExtensionResponse> {
    try {
      return await tauriInvoke<ScanExtensionResponse>('scan_extension', { request });
    } catch (error) {
      console.error('Failed to scan extension:', error);
      throw new Error(error as string);
    }
  },
};
