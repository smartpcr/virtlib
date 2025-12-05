// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.DeliveryOptimization
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_DeliveryOptimizationFile struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_DeliveryOptimizationFile {

/// 
    #[serde(rename = "BytesFromCacheServer")]
    pub bytes_from_cache_server: Option<u64>,

/// 
    #[serde(rename = "BytesFromGroupPeers")]
    pub bytes_from_group_peers: Option<u64>,

/// 
    #[serde(rename = "BytesFromHttp")]
    pub bytes_from_http: Option<u64>,

/// 
    #[serde(rename = "BytesFromInternetPeers")]
    pub bytes_from_internet_peers: Option<u64>,

/// 
    #[serde(rename = "BytesFromLanPeers")]
    pub bytes_from_lan_peers: Option<u64>,

/// 
    #[serde(rename = "BytesFromLinkLocalPeers")]
    pub bytes_from_link_local_peers: Option<u64>,

/// 
    #[serde(rename = "BytesToGroupPeers")]
    pub bytes_to_group_peers: Option<u64>,

/// 
    #[serde(rename = "BytesToInternetPeers")]
    pub bytes_to_internet_peers: Option<u64>,

/// 
    #[serde(rename = "BytesToLanPeers")]
    pub bytes_to_lan_peers: Option<u64>,

/// 
    #[serde(rename = "BytesToLinkLocalPeers")]
    pub bytes_to_link_local_peers: Option<u64>,

/// 
    #[serde(rename = "CacheHost")]
    pub cache_host: Option<String>,

/// 
    #[serde(rename = "CacheServerConnectionCount")]
    pub cache_server_connection_count: Option<u32>,

/// 
    #[serde(rename = "DownloadDurationMsecs")]
    pub download_duration_msecs: Option<u64>,

/// 
    #[serde(rename = "DownloadMode")]
    pub download_mode: Option<DeliveryOptimizationFile_DownloadMode>,

/// 
    #[serde(rename = "ExpireOn")]
    pub expire_on: Option<String>,

/// 
    #[serde(rename = "FileId")]
    pub file_id: Option<String>,

/// 
    #[serde(rename = "FileSize")]
    pub file_size: Option<u64>,

/// 
    #[serde(rename = "FileSizeInCache")]
    pub file_size_in_cache: Option<u64>,

/// 
    #[serde(rename = "GroupConnectionCount")]
    pub group_connection_count: Option<u32>,

/// 
    #[serde(rename = "HttpConnectionCount")]
    pub http_connection_count: Option<u32>,

/// 
    #[serde(rename = "InternetConnectionCount")]
    pub internet_connection_count: Option<u32>,

/// 
    #[serde(rename = "IsBackground")]
    pub is_background: Option<bool>,

/// 
    #[serde(rename = "IsPinned")]
    pub is_pinned: Option<bool>,

/// 
    #[serde(rename = "LanConnectionCount")]
    pub lan_connection_count: Option<u32>,

/// 
    #[serde(rename = "LinkLocalConnectionCount")]
    pub link_local_connection_count: Option<u32>,

/// 
    #[serde(rename = "PeerCount")]
    pub peer_count: Option<u32>,

/// 
    #[serde(rename = "PredefinedCallerApplication")]
    pub predefined_caller_application: Option<String>,

/// 
    #[serde(rename = "SourceURL")]
    pub source_url: Option<String>,

/// 
    #[serde(rename = "Status")]
    pub status: Option<DeliveryOptimizationFile_Status>,

/// 
    #[serde(rename = "TotalBytesDownloaded")]
    pub total_bytes_downloaded: Option<u64>,
}

impl MSFT_DeliveryOptimizationFile {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            bytes_from_cache_server: None,
            bytes_from_group_peers: None,
            bytes_from_http: None,
            bytes_from_internet_peers: None,
            bytes_from_lan_peers: None,
            bytes_from_link_local_peers: None,
            bytes_to_group_peers: None,
            bytes_to_internet_peers: None,
            bytes_to_lan_peers: None,
            bytes_to_link_local_peers: None,
            cache_host: None,
            cache_server_connection_count: None,
            download_duration_msecs: None,
            download_mode: None,
            expire_on: None,
            file_id: None,
            file_size: None,
            file_size_in_cache: None,
            group_connection_count: None,
            http_connection_count: None,
            internet_connection_count: None,
            is_background: None,
            is_pinned: None,
            lan_connection_count: None,
            link_local_connection_count: None,
            peer_count: None,
            predefined_caller_application: None,
            source_url: None,
            status: None,
            total_bytes_downloaded: None,
        }
    }


    /// Sets the value of BytesFromCacheServer
    pub fn set_bytes_from_cache_server(&mut self, value: u64) {
        self.bytes_from_cache_server = Some(value);
    }

    /// Gets the value of BytesFromCacheServer
    pub fn get_bytes_from_cache_server(&self) -> Option<&u64> {
        self.bytes_from_cache_server.as_ref()
    }

    /// Sets the value of BytesFromGroupPeers
    pub fn set_bytes_from_group_peers(&mut self, value: u64) {
        self.bytes_from_group_peers = Some(value);
    }

    /// Gets the value of BytesFromGroupPeers
    pub fn get_bytes_from_group_peers(&self) -> Option<&u64> {
        self.bytes_from_group_peers.as_ref()
    }

    /// Sets the value of BytesFromHttp
    pub fn set_bytes_from_http(&mut self, value: u64) {
        self.bytes_from_http = Some(value);
    }

    /// Gets the value of BytesFromHttp
    pub fn get_bytes_from_http(&self) -> Option<&u64> {
        self.bytes_from_http.as_ref()
    }

    /// Sets the value of BytesFromInternetPeers
    pub fn set_bytes_from_internet_peers(&mut self, value: u64) {
        self.bytes_from_internet_peers = Some(value);
    }

    /// Gets the value of BytesFromInternetPeers
    pub fn get_bytes_from_internet_peers(&self) -> Option<&u64> {
        self.bytes_from_internet_peers.as_ref()
    }

    /// Sets the value of BytesFromLanPeers
    pub fn set_bytes_from_lan_peers(&mut self, value: u64) {
        self.bytes_from_lan_peers = Some(value);
    }

    /// Gets the value of BytesFromLanPeers
    pub fn get_bytes_from_lan_peers(&self) -> Option<&u64> {
        self.bytes_from_lan_peers.as_ref()
    }

    /// Sets the value of BytesFromLinkLocalPeers
    pub fn set_bytes_from_link_local_peers(&mut self, value: u64) {
        self.bytes_from_link_local_peers = Some(value);
    }

    /// Gets the value of BytesFromLinkLocalPeers
    pub fn get_bytes_from_link_local_peers(&self) -> Option<&u64> {
        self.bytes_from_link_local_peers.as_ref()
    }

    /// Sets the value of BytesToGroupPeers
    pub fn set_bytes_to_group_peers(&mut self, value: u64) {
        self.bytes_to_group_peers = Some(value);
    }

    /// Gets the value of BytesToGroupPeers
    pub fn get_bytes_to_group_peers(&self) -> Option<&u64> {
        self.bytes_to_group_peers.as_ref()
    }

    /// Sets the value of BytesToInternetPeers
    pub fn set_bytes_to_internet_peers(&mut self, value: u64) {
        self.bytes_to_internet_peers = Some(value);
    }

    /// Gets the value of BytesToInternetPeers
    pub fn get_bytes_to_internet_peers(&self) -> Option<&u64> {
        self.bytes_to_internet_peers.as_ref()
    }

    /// Sets the value of BytesToLanPeers
    pub fn set_bytes_to_lan_peers(&mut self, value: u64) {
        self.bytes_to_lan_peers = Some(value);
    }

    /// Gets the value of BytesToLanPeers
    pub fn get_bytes_to_lan_peers(&self) -> Option<&u64> {
        self.bytes_to_lan_peers.as_ref()
    }

    /// Sets the value of BytesToLinkLocalPeers
    pub fn set_bytes_to_link_local_peers(&mut self, value: u64) {
        self.bytes_to_link_local_peers = Some(value);
    }

    /// Gets the value of BytesToLinkLocalPeers
    pub fn get_bytes_to_link_local_peers(&self) -> Option<&u64> {
        self.bytes_to_link_local_peers.as_ref()
    }

    /// Sets the value of CacheHost
    pub fn set_cache_host(&mut self, value: String) {
        self.cache_host = Some(value);
    }

    /// Gets the value of CacheHost
    pub fn get_cache_host(&self) -> Option<&String> {
        self.cache_host.as_ref()
    }

    /// Sets the value of CacheServerConnectionCount
    pub fn set_cache_server_connection_count(&mut self, value: u32) {
        self.cache_server_connection_count = Some(value);
    }

    /// Gets the value of CacheServerConnectionCount
    pub fn get_cache_server_connection_count(&self) -> Option<&u32> {
        self.cache_server_connection_count.as_ref()
    }

    /// Sets the value of DownloadDurationMsecs
    pub fn set_download_duration_msecs(&mut self, value: u64) {
        self.download_duration_msecs = Some(value);
    }

    /// Gets the value of DownloadDurationMsecs
    pub fn get_download_duration_msecs(&self) -> Option<&u64> {
        self.download_duration_msecs.as_ref()
    }

    /// Sets the value of DownloadMode
    pub fn set_download_mode(&mut self, value: DeliveryOptimizationFile_DownloadMode) {
        self.download_mode = Some(value);
    }

    /// Gets the value of DownloadMode
    pub fn get_download_mode(&self) -> Option<&DeliveryOptimizationFile_DownloadMode> {
        self.download_mode.as_ref()
    }

    /// Sets the value of ExpireOn
    pub fn set_expire_on(&mut self, value: String) {
        self.expire_on = Some(value);
    }

    /// Gets the value of ExpireOn
    pub fn get_expire_on(&self) -> Option<&String> {
        self.expire_on.as_ref()
    }

    /// Sets the value of FileId
    pub fn set_file_id(&mut self, value: String) {
        self.file_id = Some(value);
    }

    /// Gets the value of FileId
    pub fn get_file_id(&self) -> Option<&String> {
        self.file_id.as_ref()
    }

    /// Sets the value of FileSize
    pub fn set_file_size(&mut self, value: u64) {
        self.file_size = Some(value);
    }

    /// Gets the value of FileSize
    pub fn get_file_size(&self) -> Option<&u64> {
        self.file_size.as_ref()
    }

    /// Sets the value of FileSizeInCache
    pub fn set_file_size_in_cache(&mut self, value: u64) {
        self.file_size_in_cache = Some(value);
    }

    /// Gets the value of FileSizeInCache
    pub fn get_file_size_in_cache(&self) -> Option<&u64> {
        self.file_size_in_cache.as_ref()
    }

    /// Sets the value of GroupConnectionCount
    pub fn set_group_connection_count(&mut self, value: u32) {
        self.group_connection_count = Some(value);
    }

    /// Gets the value of GroupConnectionCount
    pub fn get_group_connection_count(&self) -> Option<&u32> {
        self.group_connection_count.as_ref()
    }

    /// Sets the value of HttpConnectionCount
    pub fn set_http_connection_count(&mut self, value: u32) {
        self.http_connection_count = Some(value);
    }

    /// Gets the value of HttpConnectionCount
    pub fn get_http_connection_count(&self) -> Option<&u32> {
        self.http_connection_count.as_ref()
    }

    /// Sets the value of InternetConnectionCount
    pub fn set_internet_connection_count(&mut self, value: u32) {
        self.internet_connection_count = Some(value);
    }

    /// Gets the value of InternetConnectionCount
    pub fn get_internet_connection_count(&self) -> Option<&u32> {
        self.internet_connection_count.as_ref()
    }

    /// Sets the value of IsBackground
    pub fn set_is_background(&mut self, value: bool) {
        self.is_background = Some(value);
    }

    /// Gets the value of IsBackground
    pub fn get_is_background(&self) -> Option<&bool> {
        self.is_background.as_ref()
    }

    /// Sets the value of IsPinned
    pub fn set_is_pinned(&mut self, value: bool) {
        self.is_pinned = Some(value);
    }

    /// Gets the value of IsPinned
    pub fn get_is_pinned(&self) -> Option<&bool> {
        self.is_pinned.as_ref()
    }

    /// Sets the value of LanConnectionCount
    pub fn set_lan_connection_count(&mut self, value: u32) {
        self.lan_connection_count = Some(value);
    }

    /// Gets the value of LanConnectionCount
    pub fn get_lan_connection_count(&self) -> Option<&u32> {
        self.lan_connection_count.as_ref()
    }

    /// Sets the value of LinkLocalConnectionCount
    pub fn set_link_local_connection_count(&mut self, value: u32) {
        self.link_local_connection_count = Some(value);
    }

    /// Gets the value of LinkLocalConnectionCount
    pub fn get_link_local_connection_count(&self) -> Option<&u32> {
        self.link_local_connection_count.as_ref()
    }

    /// Sets the value of PeerCount
    pub fn set_peer_count(&mut self, value: u32) {
        self.peer_count = Some(value);
    }

    /// Gets the value of PeerCount
    pub fn get_peer_count(&self) -> Option<&u32> {
        self.peer_count.as_ref()
    }

    /// Sets the value of PredefinedCallerApplication
    pub fn set_predefined_caller_application(&mut self, value: String) {
        self.predefined_caller_application = Some(value);
    }

    /// Gets the value of PredefinedCallerApplication
    pub fn get_predefined_caller_application(&self) -> Option<&String> {
        self.predefined_caller_application.as_ref()
    }

    /// Sets the value of SourceURL
    pub fn set_source_url(&mut self, value: String) {
        self.source_url = Some(value);
    }

    /// Gets the value of SourceURL
    pub fn get_source_url(&self) -> Option<&String> {
        self.source_url.as_ref()
    }

    /// Sets the value of Status
    pub fn set_status(&mut self, value: DeliveryOptimizationFile_Status) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&DeliveryOptimizationFile_Status> {
        self.status.as_ref()
    }

    /// Sets the value of TotalBytesDownloaded
    pub fn set_total_bytes_downloaded(&mut self, value: u64) {
        self.total_bytes_downloaded = Some(value);
    }

    /// Gets the value of TotalBytesDownloaded
    pub fn get_total_bytes_downloaded(&self) -> Option<&u64> {
        self.total_bytes_downloaded.as_ref()
    }

/// 11

    /// * `delete_pinned` -  (bool)
    /// * `file_id` -  (String)

    /// * `return_value` -  (u32)
    pub fn delete(&self, file_id: &String, delete_pinned: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "fileId".to_string(), value: file_id.into() });
        args.push(MethodParameter { name: "deletePinned".to_string(), value: delete_pinned.into() });
        self.invoke_method("Delete", &args)

    }


/// 12

    /// * `file_id` -  (String)
    /// * `pinned` -  (bool)

    /// * `return_value` -  (u32)
    pub fn set_pinned(&self, file_id: &String, pinned: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "fileId".to_string(), value: file_id.into() });
        args.push(MethodParameter { name: "pinned".to_string(), value: pinned.into() });
        self.invoke_method("SetPinned", &args)

    }


/// 13

    /// * `expiration` -  (String)
    /// * `file_id` -  (String)

    /// * `return_value` -  (u32)
    pub fn set_expiration(&self, file_id: &String, expiration: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "fileId".to_string(), value: file_id.into() });
        args.push(MethodParameter { name: "expiration".to_string(), value: expiration.into() });
        self.invoke_method("SetExpiration", &args)

    }

}

