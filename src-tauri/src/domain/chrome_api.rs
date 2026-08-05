use std::collections::HashSet;
use serde::{Deserialize, Serialize};

use crate::domain::ast_detector::SourceLocation;
use crate::domain::call_graph::FunctionId;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChromeApiId(pub u32);

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChromeApiCategory {
    Tabs,
    Windows,
    Storage,
    Runtime,
    Extension,
    Action,
    Scripting,
    Cookies,
    History,
    Bookmarks,
    Downloads,
    Debugger,
    Proxy,
    Identity,
    Management,
    Permissions,
    WebNavigation,
    WebRequest,
    DeclarativeNetRequest,
    Offscreen,
    SidePanel,
    Notifications,
    ContextMenus,
    Commands,
    Unknown,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChromeApi {
    TabsQuery,
    TabsExecuteScript,
    StorageLocalGet,
    StorageSyncSet,
    RuntimeSendMessage,
    RuntimeConnect,
    ExtensionGetBackgroundPage,
    CookiesGetAll,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChromeApiCall {
    pub api: ChromeApi,
    pub api_id: ChromeApiId,
    pub category: ChromeApiCategory,
    pub raw_api_name: String,
    pub function_id: FunctionId,
    pub location: SourceLocation,
    pub call_depth: usize,
    pub argument_count: usize,
    pub is_await: bool,
    pub is_callback: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChromeApiInventory {
    pub calls: Vec<ChromeApiCall>,
    pub unique_apis_used: HashSet<ChromeApiId>,
    pub most_frequent_category: Option<ChromeApiCategory>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChromeApiStatistics {
    pub total_calls: usize,
    pub unique_calls: usize,
    pub unknown_calls: usize,
    pub await_calls: usize,
    pub callback_calls: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChromeApiResult {
    pub detector_id: String,
    pub inventory: ChromeApiInventory,
    pub statistics: ChromeApiStatistics,
    pub elapsed_ms: u64,
    pub diagnostics: Vec<String>,
}
