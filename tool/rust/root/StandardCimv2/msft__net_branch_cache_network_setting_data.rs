// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetBranchCacheNetworkSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetBranchCacheNetworkSettingData {
    #[serde(flatten)]
    pub base: MSFT_NetBranchCacheSettingData,

/// 
    #[serde(rename = "ContentDownloadConnectPort")]
    pub content_download_connect_port: Option<u16>,

/// 
    #[serde(rename = "ContentDownloadListenPort")]
    pub content_download_listen_port: Option<u16>,

/// 
    #[serde(rename = "ContentRetrievalFirewallRulesEnabled")]
    pub content_retrieval_firewall_rules_enabled: Option<bool>,

/// 
    #[serde(rename = "ContentRetrievalUrlReservationEnabled")]
    pub content_retrieval_url_reservation_enabled: Option<bool>,

/// 
    #[serde(rename = "HostedCacheClientFirewallRulesEnabled")]
    pub hosted_cache_client_firewall_rules_enabled: Option<bool>,

/// 
    #[serde(rename = "HostedCacheHttpConnectPort")]
    pub hosted_cache_http_connect_port: Option<u16>,

/// 
    #[serde(rename = "HostedCacheHttpListenPort")]
    pub hosted_cache_http_listen_port: Option<u16>,

/// 
    #[serde(rename = "HostedCacheHttpsConnectPort")]
    pub hosted_cache_https_connect_port: Option<u16>,

/// 
    #[serde(rename = "HostedCacheHttpsListenPort")]
    pub hosted_cache_https_listen_port: Option<u16>,

/// 
    #[serde(rename = "HostedCacheHttpsUrlReservationEnabled")]
    pub hosted_cache_https_url_reservation_enabled: Option<bool>,

/// 
    #[serde(rename = "HostedCacheHttpUrlReservationEnabled")]
    pub hosted_cache_http_url_reservation_enabled: Option<bool>,

/// 
    #[serde(rename = "HostedCacheServerFirewallRulesEnabled")]
    pub hosted_cache_server_firewall_rules_enabled: Option<bool>,

/// 
    #[serde(rename = "PeerDiscoveryFirewallRulesEnabled")]
    pub peer_discovery_firewall_rules_enabled: Option<bool>,
}

impl MSFT_NetBranchCacheNetworkSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetBranchCacheSettingData::new(),
            content_download_connect_port: None,
            content_download_listen_port: None,
            content_retrieval_firewall_rules_enabled: None,
            content_retrieval_url_reservation_enabled: None,
            hosted_cache_client_firewall_rules_enabled: None,
            hosted_cache_http_connect_port: None,
            hosted_cache_http_listen_port: None,
            hosted_cache_https_connect_port: None,
            hosted_cache_https_listen_port: None,
            hosted_cache_https_url_reservation_enabled: None,
            hosted_cache_http_url_reservation_enabled: None,
            hosted_cache_server_firewall_rules_enabled: None,
            peer_discovery_firewall_rules_enabled: None,
        }
    }


    /// Sets the value of ContentDownloadConnectPort
    pub fn set_content_download_connect_port(&mut self, value: u16) {
        self.content_download_connect_port = Some(value);
    }

    /// Gets the value of ContentDownloadConnectPort
    pub fn get_content_download_connect_port(&self) -> Option<&u16> {
        self.content_download_connect_port.as_ref()
    }

    /// Sets the value of ContentDownloadListenPort
    pub fn set_content_download_listen_port(&mut self, value: u16) {
        self.content_download_listen_port = Some(value);
    }

    /// Gets the value of ContentDownloadListenPort
    pub fn get_content_download_listen_port(&self) -> Option<&u16> {
        self.content_download_listen_port.as_ref()
    }

    /// Sets the value of ContentRetrievalFirewallRulesEnabled
    pub fn set_content_retrieval_firewall_rules_enabled(&mut self, value: bool) {
        self.content_retrieval_firewall_rules_enabled = Some(value);
    }

    /// Gets the value of ContentRetrievalFirewallRulesEnabled
    pub fn get_content_retrieval_firewall_rules_enabled(&self) -> Option<&bool> {
        self.content_retrieval_firewall_rules_enabled.as_ref()
    }

    /// Sets the value of ContentRetrievalUrlReservationEnabled
    pub fn set_content_retrieval_url_reservation_enabled(&mut self, value: bool) {
        self.content_retrieval_url_reservation_enabled = Some(value);
    }

    /// Gets the value of ContentRetrievalUrlReservationEnabled
    pub fn get_content_retrieval_url_reservation_enabled(&self) -> Option<&bool> {
        self.content_retrieval_url_reservation_enabled.as_ref()
    }

    /// Sets the value of HostedCacheClientFirewallRulesEnabled
    pub fn set_hosted_cache_client_firewall_rules_enabled(&mut self, value: bool) {
        self.hosted_cache_client_firewall_rules_enabled = Some(value);
    }

    /// Gets the value of HostedCacheClientFirewallRulesEnabled
    pub fn get_hosted_cache_client_firewall_rules_enabled(&self) -> Option<&bool> {
        self.hosted_cache_client_firewall_rules_enabled.as_ref()
    }

    /// Sets the value of HostedCacheHttpConnectPort
    pub fn set_hosted_cache_http_connect_port(&mut self, value: u16) {
        self.hosted_cache_http_connect_port = Some(value);
    }

    /// Gets the value of HostedCacheHttpConnectPort
    pub fn get_hosted_cache_http_connect_port(&self) -> Option<&u16> {
        self.hosted_cache_http_connect_port.as_ref()
    }

    /// Sets the value of HostedCacheHttpListenPort
    pub fn set_hosted_cache_http_listen_port(&mut self, value: u16) {
        self.hosted_cache_http_listen_port = Some(value);
    }

    /// Gets the value of HostedCacheHttpListenPort
    pub fn get_hosted_cache_http_listen_port(&self) -> Option<&u16> {
        self.hosted_cache_http_listen_port.as_ref()
    }

    /// Sets the value of HostedCacheHttpsConnectPort
    pub fn set_hosted_cache_https_connect_port(&mut self, value: u16) {
        self.hosted_cache_https_connect_port = Some(value);
    }

    /// Gets the value of HostedCacheHttpsConnectPort
    pub fn get_hosted_cache_https_connect_port(&self) -> Option<&u16> {
        self.hosted_cache_https_connect_port.as_ref()
    }

    /// Sets the value of HostedCacheHttpsListenPort
    pub fn set_hosted_cache_https_listen_port(&mut self, value: u16) {
        self.hosted_cache_https_listen_port = Some(value);
    }

    /// Gets the value of HostedCacheHttpsListenPort
    pub fn get_hosted_cache_https_listen_port(&self) -> Option<&u16> {
        self.hosted_cache_https_listen_port.as_ref()
    }

    /// Sets the value of HostedCacheHttpsUrlReservationEnabled
    pub fn set_hosted_cache_https_url_reservation_enabled(&mut self, value: bool) {
        self.hosted_cache_https_url_reservation_enabled = Some(value);
    }

    /// Gets the value of HostedCacheHttpsUrlReservationEnabled
    pub fn get_hosted_cache_https_url_reservation_enabled(&self) -> Option<&bool> {
        self.hosted_cache_https_url_reservation_enabled.as_ref()
    }

    /// Sets the value of HostedCacheHttpUrlReservationEnabled
    pub fn set_hosted_cache_http_url_reservation_enabled(&mut self, value: bool) {
        self.hosted_cache_http_url_reservation_enabled = Some(value);
    }

    /// Gets the value of HostedCacheHttpUrlReservationEnabled
    pub fn get_hosted_cache_http_url_reservation_enabled(&self) -> Option<&bool> {
        self.hosted_cache_http_url_reservation_enabled.as_ref()
    }

    /// Sets the value of HostedCacheServerFirewallRulesEnabled
    pub fn set_hosted_cache_server_firewall_rules_enabled(&mut self, value: bool) {
        self.hosted_cache_server_firewall_rules_enabled = Some(value);
    }

    /// Gets the value of HostedCacheServerFirewallRulesEnabled
    pub fn get_hosted_cache_server_firewall_rules_enabled(&self) -> Option<&bool> {
        self.hosted_cache_server_firewall_rules_enabled.as_ref()
    }

    /// Sets the value of PeerDiscoveryFirewallRulesEnabled
    pub fn set_peer_discovery_firewall_rules_enabled(&mut self, value: bool) {
        self.peer_discovery_firewall_rules_enabled = Some(value);
    }

    /// Gets the value of PeerDiscoveryFirewallRulesEnabled
    pub fn get_peer_discovery_firewall_rules_enabled(&self) -> Option<&bool> {
        self.peer_discovery_firewall_rules_enabled.as_ref()
    }
}

