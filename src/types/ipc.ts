export interface ExtensionSummaryResponse {
  extension_id: string;
  name: string;
  version: string;
  browser_family: string;
  install_path: string;
}

export interface ScanExtensionRequest {
  extension_id: string;
  browser_family: string;
  install_path: string;
}

export interface ScanExtensionResponse {
  pipeline_id: string;
  status: string;
  risk_score: number;
  severity: string;
  elapsed_ms: number;
}
