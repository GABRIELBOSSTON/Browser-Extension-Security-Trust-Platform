import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import {
  Loader2, ShieldAlert, X, AlertTriangle, FileCode, Sparkles,
  ChevronDown, ChevronUp, ShieldCheck, Zap, BookOpen, Bug, Activity
} from 'lucide-react';

export interface ASTFinding {
  filename: string;
  line: number;
  column: number;
  severity: string;
  reason: string;
  node_type: string;
}

export interface IOCFinding {
  id: string;
  category: string;
  severity: string;
  title: string;
  description: string;
  matched_pattern: string;
  file: string;
  line: number;
  column: number;
}

export interface VirusTotalReport {
  sha256: string;
  detection_ratio: string;
  malicious: number;
  suspicious: number;
  harmless: number;
  undetected: number;
  timeout: number;
  reputation: number;
  community_score: number;
  first_submission: number;
  last_analysis: number;
  permalink: string;
}

export interface ExtensionAnalysisResponse {
  id: string;
  browser: string;
  name: string;
  version: string;
  risk_score: number;
  risk_level: string;
  reasons: string[];
  permissions: string[];
  host_permissions: string[];
  content_scripts: any[];
  background: any;
  csp: any;
  ast_findings?: ASTFinding[];
  ioc_findings?: IOCFinding[];
  vt_reports?: VirusTotalReport[];
  trusted: boolean;
}

interface Evidence {
  category: string;
  detail: string;
  severity: string;
}

interface Recommendation {
  action: string;
  description: string;
  priority: number;
}

interface SecurityExplanation {
  extension_id: string;
  extension_name: string;
  risk_score: number;
  risk_level: string;
  summary: string;
  evidence: Evidence[];
  potential_impact: string;
  recommendations: Recommendation[];
}

export function ScanDashboard() {
  const [loading, setLoading] = useState(false);
  const [results, setResults] = useState<ExtensionAnalysisResponse[]>([]);
  const [selectedExt, setSelectedExt] = useState<ExtensionAnalysisResponse | null>(null);
  const [activeTab, setActiveTab] = useState<'manifest' | 'ast' | 'ioc' | 'virustotal' | 'explain'>('manifest');
  const [severityFilter, setSeverityFilter] = useState<string>('All');
  const [iocCategoryFilter, setIocCategoryFilter] = useState<string>('All');
  const [explanation, setExplanation] = useState<SecurityExplanation | null>(null);
  const [explainLoading, setExplainLoading] = useState(false);
  const [expandedRec, setExpandedRec] = useState<number | null>(null);

  const handleScan = async () => {
    setLoading(true);
    setResults([]);
    setSelectedExt(null);
    setExplanation(null);
    try {
      const res = await invoke<ExtensionAnalysisResponse[]>('scan_extensions');
      setResults(res);
    } catch (error) {
      console.error('Failed to scan extensions:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleExplain = async (ext: ExtensionAnalysisResponse) => {
    setExplainLoading(true);
    setExplanation(null);
    try {
      const result = await invoke<SecurityExplanation>('explain_extension', {
        request: {
          extension_id: ext.id,
          extension_name: ext.name,
          risk_score: ext.risk_score,
          risk_level: ext.risk_level,
          reasons: ext.reasons,
          permissions: ext.permissions,
          host_permissions: ext.host_permissions,
          ast_findings: ext.ast_findings ?? [],
          ioc_findings: ext.ioc_findings ?? [],
          vt_reports: ext.vt_reports ?? [],
          trusted: ext.trusted ?? false,
        },
      });
      setExplanation(result);
    } catch (err) {
      console.error('Explain failed:', err);
    } finally {
      setExplainLoading(false);
    }
  };

  const getRiskColor = (level: string) => {
    switch (level.toLowerCase()) {
      case 'safe': return 'text-green-500 font-semibold';
      case 'low': return 'text-blue-500 font-semibold';
      case 'medium': return 'text-yellow-500 font-semibold';
      case 'high': return 'text-orange-500 font-bold';
      case 'critical': return 'text-red-500 font-bold';
      default: return 'text-slate-400';
    }
  };

  const getSeverityColor = (severity: string) => {
    switch (severity.toLowerCase()) {
      case 'critical': return 'bg-red-500/10 text-red-400 border-red-500/20';
      case 'high': return 'bg-orange-500/10 text-orange-400 border-orange-500/20';
      case 'medium': return 'bg-yellow-500/10 text-yellow-400 border-yellow-500/20';
      case 'low': return 'bg-blue-500/10 text-blue-400 border-blue-500/20';
      default: return 'bg-slate-500/10 text-slate-400 border-slate-500/20';
    }
  };

  const getSeverityBadge = (severity: string) => {
    switch (severity.toLowerCase()) {
      case 'critical': return 'bg-red-500/20 text-red-300 border border-red-500/30';
      case 'high': return 'bg-orange-500/20 text-orange-300 border border-orange-500/30';
      case 'medium': return 'bg-yellow-500/20 text-yellow-300 border border-yellow-500/30';
      case 'low': return 'bg-blue-500/20 text-blue-300 border border-blue-500/30';
      default: return 'bg-slate-500/20 text-slate-300 border border-slate-500/30';
    }
  };

  const filteredAstFindings = selectedExt?.ast_findings?.filter(f =>
    severityFilter === 'All' || f.severity === severityFilter
  ) || [];

  return (
    <div className="flex flex-col gap-6 relative w-full">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold tracking-tight text-white sm:text-4xl">
            Risk Analysis Dashboard
          </h1>
          <p className="mt-2 text-slate-400">Discover and analyze installed browser extensions.</p>
        </div>
        <button
          onClick={handleScan}
          disabled={loading}
          className="bg-indigo-600 hover:bg-indigo-500 text-white font-medium py-2.5 px-5 rounded-lg shadow-lg flex items-center gap-2 transition disabled:opacity-50 disabled:cursor-not-allowed"
        >
          {loading ? <Loader2 className="animate-spin w-5 h-5" /> : <ShieldAlert className="w-5 h-5" />}
          {loading ? 'Analyzing...' : 'Scan Extensions'}
        </button>
      </div>

      {loading && (
        <div className="flex flex-col items-center justify-center p-12 gap-4 text-indigo-400 bg-slate-900/50 rounded-xl border border-slate-800">
          <Loader2 className="animate-spin w-10 h-10" />
          <span className="text-lg">Analyzing browser extensions, manifests, and AST...</span>
        </div>
      )}

      {!loading && results.length > 0 && (
        <div className="overflow-x-auto rounded-xl border border-slate-800 bg-slate-900/80 shadow-2xl backdrop-blur-sm">
          <table className="w-full text-left text-sm text-slate-300">
            <thead className="bg-slate-950/80 text-xs uppercase text-slate-400 border-b border-slate-800">
              <tr>
                <th className="px-6 py-5 font-semibold">Browser</th>
                <th className="px-6 py-5 font-semibold">Name</th>
                <th className="px-6 py-5 font-semibold">Version</th>
                <th className="px-6 py-5 font-semibold">Risk Score</th>
                <th className="px-6 py-5 font-semibold">Risk Level</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-slate-800/60">
              {results.map((ext) => (
                <tr
                  key={`${ext.browser}-${ext.id}`}
                  onClick={() => {
                    setSelectedExt(ext);
                    setActiveTab('manifest');
                    setExplanation(null);
                    setExpandedRec(null);
                    setIocCategoryFilter('All');
                  }}
                  className="hover:bg-slate-800/70 cursor-pointer transition-colors"
                >
                  <td className="px-6 py-4">{ext.browser}</td>
                  <td className="px-6 py-4 font-medium text-slate-200">{ext.name}</td>
                  <td className="px-6 py-4">{ext.version}</td>
                  <td className="px-6 py-4">{ext.risk_score}</td>
                  <td className={`px-6 py-4 flex items-center gap-2 ${getRiskColor(ext.risk_level)}`}>
                    {ext.risk_level}
                    {ext.trusted && (
                      <span className="flex items-center gap-1 text-[10px] px-1.5 py-0.5 bg-green-500/20 text-green-400 border border-green-500/30 rounded uppercase tracking-wider" title="Trusted Publisher">
                        <ShieldCheck className="w-3 h-3" />
                        Trusted
                      </span>
                    )}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}

      {!loading && results.length === 0 && (
        <div className="flex flex-col items-center justify-center p-12 text-slate-500 bg-slate-900/30 rounded-xl border border-slate-800 border-dashed">
          <p>No extensions scanned yet. Click the button above to start.</p>
        </div>
      )}

      {/* Side Sheet Detail Panel */}
      {selectedExt && (
        <>
          <div
            className="fixed inset-0 bg-black/40 backdrop-blur-sm z-40"
            onClick={() => setSelectedExt(null)}
          />
          <div className="fixed inset-y-0 right-0 w-[540px] bg-slate-900 border-l border-slate-700 shadow-2xl flex flex-col z-50">
            {/* Header */}
            <div className="p-6 border-b border-slate-800 shrink-0">
              <div className="flex justify-between items-start mb-4">
                <div>
                  <h2 className="text-2xl font-bold text-white mb-1">{selectedExt.name}</h2>
                  <p className="text-xs font-mono text-slate-400">{selectedExt.id}</p>
                  <div className="flex items-center gap-2 mt-3">
                    <span className={`px-2 py-1 text-xs font-bold rounded uppercase tracking-wider bg-slate-950 border border-slate-800 ${getRiskColor(selectedExt.risk_level)}`}>
                      {selectedExt.risk_level} ({selectedExt.risk_score})
                    </span>
                    {selectedExt.trusted && (
                      <span className="flex items-center gap-1 text-xs font-bold rounded uppercase tracking-wider px-2 py-1 bg-green-500/20 text-green-400 border border-green-500/30">
                        <ShieldCheck className="w-4 h-4" />
                        Trusted Publisher
                      </span>
                    )}
                  </div>
                </div>
                <button
                  onClick={() => setSelectedExt(null)}
                  className="text-slate-400 hover:text-white p-2 hover:bg-slate-800 rounded-full transition"
                >
                  <X className="w-5 h-5" />
                </button>
              </div>

              {/* Tabs */}
              <div className="flex gap-1 mt-6 border-b border-slate-800">
                <button
                  onClick={() => setActiveTab('manifest')}
                  className={`pb-2 px-3 text-sm font-medium transition-colors flex items-center gap-1.5 ${activeTab === 'manifest' ? 'text-indigo-400 border-b-2 border-indigo-400' : 'text-slate-400 hover:text-slate-200'}`}
                >
                  <AlertTriangle className="w-3.5 h-3.5" />
                  Manifest
                </button>
                <button
                  onClick={() => setActiveTab('ast')}
                  className={`pb-2 px-3 text-sm font-medium transition-colors flex items-center gap-1.5 ${activeTab === 'ast' ? 'text-indigo-400 border-b-2 border-indigo-400' : 'text-slate-400 hover:text-slate-200'}`}
                >
                  <FileCode className="w-3.5 h-3.5" />
                  AST
                  {selectedExt.ast_findings && selectedExt.ast_findings.length > 0 && (
                    <span className="bg-slate-800 text-xs px-1.5 py-0.5 rounded-full">{selectedExt.ast_findings.length}</span>
                  )}
                </button>
                <button
                  onClick={() => setActiveTab('ioc')}
                  className={`pb-2 px-3 text-sm font-medium transition-colors flex items-center gap-1.5 ${activeTab === 'ioc' ? 'text-indigo-400 border-b-2 border-indigo-400' : 'text-slate-400 hover:text-slate-200'}`}
                >
                  <Bug className="w-3.5 h-3.5" />
                  IOCs
                  {selectedExt.ioc_findings && selectedExt.ioc_findings.length > 0 && (
                    <span className="bg-slate-800 text-xs px-1.5 py-0.5 rounded-full">{selectedExt.ioc_findings.length}</span>
                  )}
                </button>
                <button
                  onClick={() => setActiveTab('virustotal')}
                  className={`pb-2 px-3 text-sm font-medium transition-colors flex items-center gap-1.5 ${activeTab === 'virustotal' ? 'text-indigo-400 border-b-2 border-indigo-400' : 'text-slate-400 hover:text-slate-200'}`}
                >
                  <Activity className="w-3.5 h-3.5" />
                  VirusTotal
                  {selectedExt.vt_reports && selectedExt.vt_reports.length > 0 && (
                    <span className="bg-slate-800 text-xs px-1.5 py-0.5 rounded-full">{selectedExt.vt_reports.filter(r => r.malicious > 0).length}</span>
                  )}
                </button>
                <button
                  onClick={() => {
                    setActiveTab('explain');
                    if (!explanation && !explainLoading) {
                      handleExplain(selectedExt);
                    }
                  }}
                  className={`pb-2 px-3 text-sm font-medium transition-colors flex items-center gap-1.5 ${activeTab === 'explain' ? 'text-violet-400 border-b-2 border-violet-400' : 'text-slate-400 hover:text-slate-200'}`}
                >
                  <Sparkles className="w-3.5 h-3.5" />
                  AI Explain
                </button>
              </div>
            </div>

            {/* Content Area */}
            <div className="flex-1 overflow-y-auto p-6">
              {/* ── Manifest Tab ─────────────────────────────── */}
              {activeTab === 'manifest' && (
                <div className="space-y-6">
                  <div>
                    <h3 className="text-xs font-bold text-slate-500 uppercase tracking-widest mb-3">Penalty Reasons</h3>
                    {selectedExt.reasons.length > 0 ? (
                      <ul className="list-disc list-inside text-red-400 text-sm space-y-1 bg-red-950/20 border border-red-900/30 rounded-lg p-4">
                        {selectedExt.reasons.map((r, i) => <li key={i}>{r}</li>)}
                      </ul>
                    ) : (
                      <span className="text-green-400 text-sm px-4 py-2 bg-green-950/20 border border-green-900/30 rounded-lg inline-block">No risks detected</span>
                    )}
                  </div>
                  <div>
                    <h3 className="text-xs font-bold text-slate-500 uppercase tracking-widest mb-3">Permissions</h3>
                    {selectedExt.permissions.length > 0 ? (
                      <div className="flex flex-wrap gap-2">
                        {selectedExt.permissions.map((p, i) => (
                          <span key={i} className="px-2.5 py-1 text-xs font-medium rounded-md bg-slate-800 text-slate-300 border border-slate-700">{p}</span>
                        ))}
                      </div>
                    ) : <span className="text-slate-500 text-sm">None</span>}
                  </div>
                  <div>
                    <h3 className="text-xs font-bold text-slate-500 uppercase tracking-widest mb-3">Host Permissions</h3>
                    {selectedExt.host_permissions.length > 0 ? (
                      <div className="flex flex-wrap gap-2">
                        {selectedExt.host_permissions.map((p, i) => (
                          <span key={i} className="px-2.5 py-1 text-xs font-medium rounded-md bg-indigo-950 text-indigo-300 border border-indigo-900/50">{p}</span>
                        ))}
                      </div>
                    ) : <span className="text-slate-500 text-sm">None</span>}
                  </div>
                  <div>
                    <h3 className="text-xs font-bold text-slate-500 uppercase tracking-widest mb-3">Background</h3>
                    <pre className="text-xs font-mono bg-[#0d1117] p-4 rounded-lg overflow-x-auto text-slate-300 border border-slate-800 shadow-inner">
                      {JSON.stringify(selectedExt.background, null, 2)}
                    </pre>
                  </div>
                  <div>
                    <h3 className="text-xs font-bold text-slate-500 uppercase tracking-widest mb-3">Content Scripts</h3>
                    <pre className="text-xs font-mono bg-[#0d1117] p-4 rounded-lg overflow-x-auto text-slate-300 border border-slate-800 shadow-inner">
                      {JSON.stringify(selectedExt.content_scripts, null, 2)}
                    </pre>
                  </div>
                  <div>
                    <h3 className="text-xs font-bold text-slate-500 uppercase tracking-widest mb-3">CSP</h3>
                    <pre className="text-xs font-mono bg-[#0d1117] p-4 rounded-lg overflow-x-auto text-slate-300 border border-slate-800 shadow-inner">
                      {JSON.stringify(selectedExt.csp, null, 2)}
                    </pre>
                  </div>
                </div>
              )}

              {/* ── AST Tab ───────────────────────────────────── */}
              {activeTab === 'ast' && (
                <div className="space-y-4">
                  <div className="flex gap-2 pb-2">
                    {['All', 'Critical', 'High', 'Medium', 'Low'].map(sev => (
                      <button
                        key={sev}
                        onClick={() => setSeverityFilter(sev)}
                        className={`px-3 py-1 text-xs font-medium rounded-md transition ${severityFilter === sev ? 'bg-indigo-600 text-white' : 'bg-slate-800 text-slate-400 hover:bg-slate-700'}`}
                      >
                        {sev}
                      </button>
                    ))}
                  </div>
                  {filteredAstFindings.length > 0 ? (
                    <div className="space-y-3">
                      {filteredAstFindings.map((finding, idx) => (
                        <div key={idx} className={`p-4 rounded-lg border ${getSeverityColor(finding.severity)}`}>
                          <div className="flex items-start justify-between mb-2">
                            <span className="font-bold text-sm tracking-wide uppercase">{finding.severity}</span>
                            <span className="text-xs font-mono opacity-80">{finding.node_type}</span>
                          </div>
                          <p className="text-sm font-medium mb-3">{finding.reason}</p>
                          <div className="flex justify-between items-end text-xs opacity-75 font-mono">
                            <span className="truncate max-w-[280px]" title={finding.filename}>{finding.filename}</span>
                            <span>Ln {finding.line}, Col {finding.column}</span>
                          </div>
                        </div>
                      ))}
                    </div>
                  ) : (
                    <div className="py-12 text-center text-slate-500 border border-dashed border-slate-700 rounded-lg">
                      <p>No AST findings match the selected filter.</p>
                    </div>
                  )}
                </div>
              )}

              {/* ── IOC Tab ───────────────────────────────────── */}
              {activeTab === 'ioc' && (
                <div className="space-y-4">
                  <div className="flex flex-wrap gap-2 pb-2 border-b border-slate-800">
                    {['All', 'Network', 'Secret', 'Obfuscation', 'Crypto', 'WebAssembly', 'EncodedPayload'].map(cat => (
                      <button
                        key={cat}
                        onClick={() => setIocCategoryFilter(cat)}
                        className={`px-3 py-1 text-xs font-medium rounded-md transition ${iocCategoryFilter === cat ? 'bg-indigo-600 text-white' : 'bg-slate-800 text-slate-400 hover:bg-slate-700'}`}
                      >
                        {cat}
                      </button>
                    ))}
                  </div>
                  
                  {(() => {
                    const filteredIocs = selectedExt.ioc_findings?.filter(f => iocCategoryFilter === 'All' || f.category === iocCategoryFilter) || [];
                    
                    if (filteredIocs.length === 0) {
                      return (
                        <div className="py-12 text-center text-slate-500 border border-dashed border-slate-700 rounded-lg">
                          <p>No IOC findings match the selected filter.</p>
                        </div>
                      );
                    }

                    return (
                      <div className="space-y-3">
                        {filteredIocs.map((finding) => (
                          <div key={finding.id} className={`p-4 rounded-lg border ${getSeverityColor(finding.severity)}`}>
                            <div className="flex items-start justify-between mb-2">
                              <div className="flex items-center gap-2">
                                <span className={`text-[10px] font-bold px-1.5 py-0.5 rounded uppercase ${getSeverityBadge(finding.severity)}`}>
                                  {finding.severity}
                                </span>
                                <span className="text-xs font-mono bg-slate-900/50 px-1.5 py-0.5 rounded uppercase border border-slate-700/50">
                                  {finding.category}
                                </span>
                              </div>
                              <span className="text-xs font-mono opacity-50">{finding.id}</span>
                            </div>
                            
                            <h4 className="text-sm font-bold mb-1">{finding.title}</h4>
                            <p className="text-sm opacity-90 mb-3">{finding.description}</p>
                            
                            <div className="bg-slate-950/50 rounded p-2 mb-3 border border-slate-800">
                              <span className="text-xs font-mono opacity-50 block mb-1">Matched Pattern:</span>
                              <code className="text-xs font-mono text-red-300 break-all">{finding.matched_pattern}</code>
                            </div>

                            <div className="flex justify-between items-end text-xs opacity-75 font-mono pt-2 border-t border-slate-800/50">
                              <span className="truncate max-w-[280px]" title={finding.file}>{finding.file}</span>
                              <span>Ln {finding.line}, Col {finding.column}</span>
                            </div>
                          </div>
                        ))}
                      </div>
                    );
                  })()}
                </div>
              )}

              {/* ── VirusTotal Tab ────────────────────────────── */}
              {activeTab === 'virustotal' && (
                <div className="space-y-4">
                  {!selectedExt.vt_reports || selectedExt.vt_reports.length === 0 ? (
                    <div className="py-12 text-center text-slate-500 border border-dashed border-slate-700 rounded-lg">
                      <Activity className="w-12 h-12 mx-auto mb-3 opacity-20" />
                      <p>No VirusTotal reports available.</p>
                      <p className="text-xs opacity-75 mt-1">Make sure VT_API_KEY environment variable is configured.</p>
                    </div>
                  ) : (
                    <div className="space-y-4">
                      {selectedExt.vt_reports.map((report, idx) => (
                        <div key={idx} className={`p-4 rounded-lg border ${report.malicious > 0 ? 'bg-red-950/20 border-red-900/50' : 'bg-slate-800/30 border-slate-700/50'}`}>
                          <div className="flex justify-between items-start mb-3">
                            <div>
                              <div className="flex items-center gap-2 mb-1">
                                {report.malicious > 0 ? (
                                  <span className="px-2 py-0.5 rounded text-[10px] font-bold uppercase bg-red-500/20 text-red-400 border border-red-500/30">
                                    Malicious
                                  </span>
                                ) : (
                                  <span className="px-2 py-0.5 rounded text-[10px] font-bold uppercase bg-green-500/20 text-green-400 border border-green-500/30">
                                    Clean
                                  </span>
                                )}
                                <span className="text-sm font-bold opacity-90">Detection: {report.detection_ratio}</span>
                              </div>
                              <div className="text-xs font-mono opacity-50 break-all">{report.sha256}</div>
                            </div>
                            <a
                              href={report.permalink}
                              target="_blank"
                              rel="noreferrer"
                              className="text-xs font-medium text-indigo-400 hover:text-indigo-300 bg-indigo-500/10 hover:bg-indigo-500/20 px-3 py-1.5 rounded transition"
                            >
                              View on VirusTotal
                            </a>
                          </div>
                          
                          <div className="grid grid-cols-2 md:grid-cols-6 gap-2 text-xs">
                            <div className="bg-slate-900/50 p-2 rounded border border-slate-800">
                              <span className="block opacity-50 mb-0.5">Malicious</span>
                              <span className={`font-bold ${report.malicious > 0 ? 'text-red-400' : 'text-slate-300'}`}>{report.malicious}</span>
                            </div>
                            <div className="bg-slate-900/50 p-2 rounded border border-slate-800">
                              <span className="block opacity-50 mb-0.5">Suspicious</span>
                              <span className={`font-bold ${report.suspicious > 0 ? 'text-orange-400' : 'text-slate-300'}`}>{report.suspicious}</span>
                            </div>
                            <div className="bg-slate-900/50 p-2 rounded border border-slate-800">
                              <span className="block opacity-50 mb-0.5">Harmless</span>
                              <span className="font-bold text-slate-300">{report.harmless}</span>
                            </div>
                            <div className="bg-slate-900/50 p-2 rounded border border-slate-800">
                              <span className="block opacity-50 mb-0.5">Undetected</span>
                              <span className="font-bold text-slate-300">{report.undetected}</span>
                            </div>
                            <div className="bg-slate-900/50 p-2 rounded border border-slate-800">
                              <span className="block opacity-50 mb-0.5">Reputation</span>
                              <span className={`font-bold ${report.community_score < 0 ? 'text-red-400' : report.community_score > 0 ? 'text-green-400' : 'text-slate-300'}`}>{report.community_score}</span>
                            </div>
                            <div className="bg-slate-900/50 p-2 rounded border border-slate-800">
                              <span className="block opacity-50 mb-0.5">Last Analysis</span>
                              <span className="font-bold text-slate-300">{report.last_analysis ? new Date(report.last_analysis * 1000).toLocaleDateString() : 'N/A'}</span>
                            </div>
                          </div>
                        </div>
                      ))}
                    </div>
                  )}
                </div>
              )}

              {/* ── AI Explain Tab ────────────────────────────── */}
              {activeTab === 'explain' && (
                <div className="space-y-5">
                  {explainLoading && (
                    <div className="flex flex-col items-center justify-center py-16 gap-3 text-violet-400">
                      <Loader2 className="animate-spin w-8 h-8" />
                      <p className="text-sm">Generating security report…</p>
                    </div>
                  )}

                  {!explainLoading && !explanation && (
                    <div className="flex flex-col items-center justify-center py-16 gap-4 text-slate-500 border border-dashed border-slate-700 rounded-lg">
                      <Sparkles className="w-8 h-8" />
                      <p className="text-sm">Click "AI Explain" tab to generate report</p>
                      <button
                        onClick={() => handleExplain(selectedExt)}
                        className="mt-1 px-4 py-2 rounded-lg bg-violet-600 hover:bg-violet-500 text-white text-sm font-medium transition"
                      >
                        Generate Report
                      </button>
                    </div>
                  )}

                  {!explainLoading && explanation && (
                    <>
                      {/* Summary Card */}
                      <div className="rounded-xl bg-gradient-to-br from-slate-800 to-slate-900 border border-slate-700 p-5">
                        <div className="flex items-center gap-2 mb-3">
                          <BookOpen className="w-4 h-4 text-violet-400" />
                          <h3 className="text-xs font-bold text-violet-400 uppercase tracking-widest">Summary</h3>
                        </div>
                        <p className="text-sm text-slate-200 leading-relaxed">{explanation.summary}</p>
                      </div>

                      {/* Evidence */}
                      {explanation.evidence.length > 0 && (
                        <div>
                          <div className="flex items-center gap-2 mb-3">
                            <AlertTriangle className="w-4 h-4 text-slate-400" />
                            <h3 className="text-xs font-bold text-slate-400 uppercase tracking-widest">Evidence ({explanation.evidence.length})</h3>
                          </div>
                          <div className="space-y-2">
                            {explanation.evidence.map((ev, i) => (
                              <div key={i} className={`flex items-start gap-3 p-3 rounded-lg border ${getSeverityColor(ev.severity)}`}>
                                <span className={`shrink-0 text-[10px] font-bold px-1.5 py-0.5 rounded uppercase ${getSeverityBadge(ev.severity)}`}>
                                  {ev.severity}
                                </span>
                                <div className="min-w-0">
                                  <p className="text-[10px] font-semibold uppercase tracking-wider opacity-60 mb-0.5">{ev.category}</p>
                                  <p className="text-xs leading-relaxed break-words">{ev.detail}</p>
                                </div>
                              </div>
                            ))}
                          </div>
                        </div>
                      )}

                      {/* Potential Impact */}
                      <div className="rounded-xl bg-red-950/20 border border-red-900/30 p-5">
                        <div className="flex items-center gap-2 mb-3">
                          <Zap className="w-4 h-4 text-red-400" />
                          <h3 className="text-xs font-bold text-red-400 uppercase tracking-widest">Potential Impact</h3>
                        </div>
                        <div className="space-y-2">
                          {explanation.potential_impact.split('\n\n').map((para, i) => (
                            <p key={i} className="text-sm text-red-200/80 leading-relaxed">{para}</p>
                          ))}
                        </div>
                      </div>

                      {/* Recommendations */}
                      <div>
                        <div className="flex items-center gap-2 mb-3">
                          <ShieldCheck className="w-4 h-4 text-green-400" />
                          <h3 className="text-xs font-bold text-green-400 uppercase tracking-widest">Recommendations</h3>
                        </div>
                        <div className="space-y-2">
                          {explanation.recommendations.map((rec, i) => (
                            <div key={i} className="rounded-lg bg-slate-800 border border-slate-700 overflow-hidden">
                              <button
                                onClick={() => setExpandedRec(expandedRec === i ? null : i)}
                                className="w-full flex items-center justify-between p-4 text-left hover:bg-slate-700/50 transition"
                              >
                                <div className="flex items-center gap-3">
                                  <span className="shrink-0 w-5 h-5 rounded-full bg-green-500/20 text-green-400 text-xs flex items-center justify-center font-bold">
                                    {rec.priority}
                                  </span>
                                  <span className="text-sm font-medium text-slate-200">{rec.action}</span>
                                </div>
                                {expandedRec === i
                                  ? <ChevronUp className="w-4 h-4 text-slate-400 shrink-0" />
                                  : <ChevronDown className="w-4 h-4 text-slate-400 shrink-0" />}
                              </button>
                              {expandedRec === i && (
                                <div className="px-4 pb-4 pt-0 border-t border-slate-700">
                                  <p className="text-sm text-slate-300 leading-relaxed mt-3">{rec.description}</p>
                                </div>
                              )}
                            </div>
                          ))}
                        </div>
                      </div>

                      {/* Re-generate button */}
                      <button
                        onClick={() => handleExplain(selectedExt)}
                        className="w-full mt-2 py-2 rounded-lg border border-violet-500/30 text-violet-400 text-sm hover:bg-violet-500/10 transition"
                      >
                        <span className="flex items-center justify-center gap-2">
                          <Sparkles className="w-4 h-4" />
                          Regenerate Report
                        </span>
                      </button>
                    </>
                  )}
                </div>
              )}
            </div>
          </div>
        </>
      )}
    </div>
  );
}
