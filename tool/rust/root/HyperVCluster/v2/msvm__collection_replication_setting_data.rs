// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_CollectionReplicationSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_CollectionReplicationSettingData {
    #[serde(flatten)]
    pub base: Msvm_CollectionSettingData,

/// 
    #[serde(rename = "ApplicationConsistentSnapshotInterval")]
    pub application_consistent_snapshot_interval: Option<u16>,

/// 
    #[serde(rename = "AuthenticationType")]
    pub authentication_type: Option<u16>,

/// 
    #[serde(rename = "AutoResynchronizeEnabled")]
    pub auto_resynchronize_enabled: Option<bool>,

/// 
    #[serde(rename = "AutoResynchronizeIntervalEnd")]
    pub auto_resynchronize_interval_end: Option<String>,

/// 
    #[serde(rename = "AutoResynchronizeIntervalStart")]
    pub auto_resynchronize_interval_start: Option<String>,

/// 
    #[serde(rename = "BypassProxyServer")]
    pub bypass_proxy_server: Option<bool>,

/// 
    #[serde(rename = "CertificateThumbPrint")]
    pub certificate_thumb_print: Option<String>,

/// 
    #[serde(rename = "CompressionEnabled")]
    pub compression_enabled: Option<bool>,

/// 
    #[serde(rename = "IncludedDisks")]
    pub included_disks: Vec<String>,

/// 
    #[serde(rename = "PrimaryConnectionPoint")]
    pub primary_connection_point: Option<String>,

/// 
    #[serde(rename = "PrimaryReplicationEntityIDs")]
    pub primary_replication_entity_ids: Vec<String>,

/// 
    #[serde(rename = "RecoveryConnectionPoint")]
    pub recovery_connection_point: Option<String>,

/// 
    #[serde(rename = "RecoveryHistory")]
    pub recovery_history: Option<u16>,

/// 
    #[serde(rename = "RecoveryServerHosts")]
    pub recovery_server_hosts: Vec<String>,

/// 
    #[serde(rename = "RecoveryServerPortNumber")]
    pub recovery_server_port_number: Option<u16>,

/// 
    #[serde(rename = "ReplicateHostKvpItems")]
    pub replicate_host_kvp_items: Option<bool>,

/// 
    #[serde(rename = "ReplicationInterval")]
    pub replication_interval: Option<u16>,

/// 
    #[serde(rename = "RootCertificateThumbPrint")]
    pub root_certificate_thumb_print: Option<String>,
}

impl Msvm_CollectionReplicationSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Msvm_CollectionSettingData::new(),
            application_consistent_snapshot_interval: None,
            authentication_type: None,
            auto_resynchronize_enabled: None,
            auto_resynchronize_interval_end: None,
            auto_resynchronize_interval_start: None,
            bypass_proxy_server: None,
            certificate_thumb_print: None,
            compression_enabled: None,
            included_disks: Vec::new(),
            primary_connection_point: None,
            primary_replication_entity_ids: Vec::new(),
            recovery_connection_point: None,
            recovery_history: None,
            recovery_server_hosts: Vec::new(),
            recovery_server_port_number: None,
            replicate_host_kvp_items: None,
            replication_interval: None,
            root_certificate_thumb_print: None,
        }
    }


    /// Sets the value of ApplicationConsistentSnapshotInterval
    pub fn set_application_consistent_snapshot_interval(&mut self, value: u16) {
        self.application_consistent_snapshot_interval = Some(value);
    }

    /// Gets the value of ApplicationConsistentSnapshotInterval
    pub fn get_application_consistent_snapshot_interval(&self) -> Option<&u16> {
        self.application_consistent_snapshot_interval.as_ref()
    }

    /// Sets the value of AuthenticationType
    pub fn set_authentication_type(&mut self, value: u16) {
        self.authentication_type = Some(value);
    }

    /// Gets the value of AuthenticationType
    pub fn get_authentication_type(&self) -> Option<&u16> {
        self.authentication_type.as_ref()
    }

    /// Sets the value of AutoResynchronizeEnabled
    pub fn set_auto_resynchronize_enabled(&mut self, value: bool) {
        self.auto_resynchronize_enabled = Some(value);
    }

    /// Gets the value of AutoResynchronizeEnabled
    pub fn get_auto_resynchronize_enabled(&self) -> Option<&bool> {
        self.auto_resynchronize_enabled.as_ref()
    }

    /// Sets the value of AutoResynchronizeIntervalEnd
    pub fn set_auto_resynchronize_interval_end(&mut self, value: String) {
        self.auto_resynchronize_interval_end = Some(value);
    }

    /// Gets the value of AutoResynchronizeIntervalEnd
    pub fn get_auto_resynchronize_interval_end(&self) -> Option<&String> {
        self.auto_resynchronize_interval_end.as_ref()
    }

    /// Sets the value of AutoResynchronizeIntervalStart
    pub fn set_auto_resynchronize_interval_start(&mut self, value: String) {
        self.auto_resynchronize_interval_start = Some(value);
    }

    /// Gets the value of AutoResynchronizeIntervalStart
    pub fn get_auto_resynchronize_interval_start(&self) -> Option<&String> {
        self.auto_resynchronize_interval_start.as_ref()
    }

    /// Sets the value of BypassProxyServer
    pub fn set_bypass_proxy_server(&mut self, value: bool) {
        self.bypass_proxy_server = Some(value);
    }

    /// Gets the value of BypassProxyServer
    pub fn get_bypass_proxy_server(&self) -> Option<&bool> {
        self.bypass_proxy_server.as_ref()
    }

    /// Sets the value of CertificateThumbPrint
    pub fn set_certificate_thumb_print(&mut self, value: String) {
        self.certificate_thumb_print = Some(value);
    }

    /// Gets the value of CertificateThumbPrint
    pub fn get_certificate_thumb_print(&self) -> Option<&String> {
        self.certificate_thumb_print.as_ref()
    }

    /// Sets the value of CompressionEnabled
    pub fn set_compression_enabled(&mut self, value: bool) {
        self.compression_enabled = Some(value);
    }

    /// Gets the value of CompressionEnabled
    pub fn get_compression_enabled(&self) -> Option<&bool> {
        self.compression_enabled.as_ref()
    }

    /// Sets the value of IncludedDisks
    pub fn set_included_disks(&mut self, value: Vec<String>) {
        self.included_disks = value;
    }

    /// Gets the value of IncludedDisks
    pub fn get_included_disks(&self) -> &Vec<String> {
        &self.included_disks
    }

    /// Sets the value of PrimaryConnectionPoint
    pub fn set_primary_connection_point(&mut self, value: String) {
        self.primary_connection_point = Some(value);
    }

    /// Gets the value of PrimaryConnectionPoint
    pub fn get_primary_connection_point(&self) -> Option<&String> {
        self.primary_connection_point.as_ref()
    }

    /// Sets the value of PrimaryReplicationEntityIDs
    pub fn set_primary_replication_entity_ids(&mut self, value: Vec<String>) {
        self.primary_replication_entity_ids = value;
    }

    /// Gets the value of PrimaryReplicationEntityIDs
    pub fn get_primary_replication_entity_ids(&self) -> &Vec<String> {
        &self.primary_replication_entity_ids
    }

    /// Sets the value of RecoveryConnectionPoint
    pub fn set_recovery_connection_point(&mut self, value: String) {
        self.recovery_connection_point = Some(value);
    }

    /// Gets the value of RecoveryConnectionPoint
    pub fn get_recovery_connection_point(&self) -> Option<&String> {
        self.recovery_connection_point.as_ref()
    }

    /// Sets the value of RecoveryHistory
    pub fn set_recovery_history(&mut self, value: u16) {
        self.recovery_history = Some(value);
    }

    /// Gets the value of RecoveryHistory
    pub fn get_recovery_history(&self) -> Option<&u16> {
        self.recovery_history.as_ref()
    }

    /// Sets the value of RecoveryServerHosts
    pub fn set_recovery_server_hosts(&mut self, value: Vec<String>) {
        self.recovery_server_hosts = value;
    }

    /// Gets the value of RecoveryServerHosts
    pub fn get_recovery_server_hosts(&self) -> &Vec<String> {
        &self.recovery_server_hosts
    }

    /// Sets the value of RecoveryServerPortNumber
    pub fn set_recovery_server_port_number(&mut self, value: u16) {
        self.recovery_server_port_number = Some(value);
    }

    /// Gets the value of RecoveryServerPortNumber
    pub fn get_recovery_server_port_number(&self) -> Option<&u16> {
        self.recovery_server_port_number.as_ref()
    }

    /// Sets the value of ReplicateHostKvpItems
    pub fn set_replicate_host_kvp_items(&mut self, value: bool) {
        self.replicate_host_kvp_items = Some(value);
    }

    /// Gets the value of ReplicateHostKvpItems
    pub fn get_replicate_host_kvp_items(&self) -> Option<&bool> {
        self.replicate_host_kvp_items.as_ref()
    }

    /// Sets the value of ReplicationInterval
    pub fn set_replication_interval(&mut self, value: u16) {
        self.replication_interval = Some(value);
    }

    /// Gets the value of ReplicationInterval
    pub fn get_replication_interval(&self) -> Option<&u16> {
        self.replication_interval.as_ref()
    }

    /// Sets the value of RootCertificateThumbPrint
    pub fn set_root_certificate_thumb_print(&mut self, value: String) {
        self.root_certificate_thumb_print = Some(value);
    }

    /// Gets the value of RootCertificateThumbPrint
    pub fn get_root_certificate_thumb_print(&self) -> Option<&String> {
        self.root_certificate_thumb_print.as_ref()
    }
}

