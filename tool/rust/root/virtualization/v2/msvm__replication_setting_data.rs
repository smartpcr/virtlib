// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_ReplicationSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_ReplicationSettingData {
    #[serde(flatten)]
    pub base: CIM_VirtualSystemSettingData,

/// 
    #[serde(rename = "AdditionalSettings")]
    pub additional_settings: Option<String>,

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
    #[serde(rename = "EnableWriteOrderPreservationAcrossDisks")]
    pub enable_write_order_preservation_across_disks: Option<bool>,

/// The list of VHD attached to the ComputerSystem that will be replicated by the Failover Replication Engine. This is an array of strings each containing an the InstanceID of the resource allocation setting data (RASD) of the VHD.
    #[serde(rename = "IncludedDisks")]
    pub included_disks: Vec<String>,

/// 
    #[serde(rename = "PrimaryConnectionPoint")]
    pub primary_connection_point: Option<String>,

/// 
    #[serde(rename = "PrimaryHostSystem")]
    pub primary_host_system: Option<String>,

/// 
    #[serde(rename = "RecoveryConnectionPoint")]
    pub recovery_connection_point: Option<String>,

/// 
    #[serde(rename = "RecoveryHistory")]
    pub recovery_history: Option<u16>,

/// 
    #[serde(rename = "RecoveryHostSystem")]
    pub recovery_host_system: Option<String>,

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
    #[serde(rename = "ReplicationProvider")]
    pub replication_provider: Option<String>,

/// 
    #[serde(rename = "RootCertificateThumbPrint")]
    pub root_certificate_thumb_print: Option<String>,
}

impl Msvm_ReplicationSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_VirtualSystemSettingData::new(),
            additional_settings: None,
            application_consistent_snapshot_interval: None,
            authentication_type: None,
            auto_resynchronize_enabled: None,
            auto_resynchronize_interval_end: None,
            auto_resynchronize_interval_start: None,
            bypass_proxy_server: None,
            certificate_thumb_print: None,
            compression_enabled: None,
            enable_write_order_preservation_across_disks: None,
            included_disks: Vec::new(),
            primary_connection_point: None,
            primary_host_system: None,
            recovery_connection_point: None,
            recovery_history: None,
            recovery_host_system: None,
            recovery_server_port_number: None,
            replicate_host_kvp_items: None,
            replication_interval: None,
            replication_provider: None,
            root_certificate_thumb_print: None,
        }
    }


    /// Sets the value of AdditionalSettings
    pub fn set_additional_settings(&mut self, value: String) {
        self.additional_settings = Some(value);
    }

    /// Gets the value of AdditionalSettings
    pub fn get_additional_settings(&self) -> Option<&String> {
        self.additional_settings.as_ref()
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

    /// Sets the value of EnableWriteOrderPreservationAcrossDisks
    pub fn set_enable_write_order_preservation_across_disks(&mut self, value: bool) {
        self.enable_write_order_preservation_across_disks = Some(value);
    }

    /// Gets the value of EnableWriteOrderPreservationAcrossDisks
    pub fn get_enable_write_order_preservation_across_disks(&self) -> Option<&bool> {
        self.enable_write_order_preservation_across_disks.as_ref()
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

    /// Sets the value of PrimaryHostSystem
    pub fn set_primary_host_system(&mut self, value: String) {
        self.primary_host_system = Some(value);
    }

    /// Gets the value of PrimaryHostSystem
    pub fn get_primary_host_system(&self) -> Option<&String> {
        self.primary_host_system.as_ref()
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

    /// Sets the value of RecoveryHostSystem
    pub fn set_recovery_host_system(&mut self, value: String) {
        self.recovery_host_system = Some(value);
    }

    /// Gets the value of RecoveryHostSystem
    pub fn get_recovery_host_system(&self) -> Option<&String> {
        self.recovery_host_system.as_ref()
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

    /// Sets the value of ReplicationProvider
    pub fn set_replication_provider(&mut self, value: String) {
        self.replication_provider = Some(value);
    }

    /// Gets the value of ReplicationProvider
    pub fn get_replication_provider(&self) -> Option<&String> {
        self.replication_provider.as_ref()
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

impl Msvm_ReplicationSettingData {
    /// Gets the related Msvm_ReplicationRelationship object(s)
    pub fn get_related__replication_relationship(&self) -> Result<Msvm_ReplicationRelationship, WmiError> {
        self.get_related("Msvm_ReplicationRelationship")
    }

    /// Gets the related Msvm_ComputerSystem object(s)
    pub fn get_related__computer_system(&self) -> Result<Msvm_ComputerSystem, WmiError> {
        self.get_related("Msvm_ComputerSystem")
    }

}

