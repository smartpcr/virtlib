// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_ReplicationServiceSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_ReplicationServiceSettingData {
    #[serde(flatten)]
    pub base: CIM_SettingData,

/// 
    #[serde(rename = "AllowedAuthenticationType")]
    pub allowed_authentication_type: Option<u16>,

/// 
    #[serde(rename = "CertificateThumbPrint")]
    pub certificate_thumb_print: Option<String>,

/// 
    #[serde(rename = "HttpPort")]
    pub http_port: Option<u16>,

/// 
    #[serde(rename = "HttpsPort")]
    pub https_port: Option<u16>,

/// 
    #[serde(rename = "MonitoringInterval")]
    pub monitoring_interval: Option<u32>,

/// 
    #[serde(rename = "MonitoringStartTime")]
    pub monitoring_start_time: Option<String>,

/// 
    #[serde(rename = "RecoveryServerEnabled")]
    pub recovery_server_enabled: Option<bool>,
}

impl Msvm_ReplicationServiceSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SettingData::new(),
            allowed_authentication_type: None,
            certificate_thumb_print: None,
            http_port: None,
            https_port: None,
            monitoring_interval: None,
            monitoring_start_time: None,
            recovery_server_enabled: None,
        }
    }


    /// Sets the value of AllowedAuthenticationType
    pub fn set_allowed_authentication_type(&mut self, value: u16) {
        self.allowed_authentication_type = Some(value);
    }

    /// Gets the value of AllowedAuthenticationType
    pub fn get_allowed_authentication_type(&self) -> Option<&u16> {
        self.allowed_authentication_type.as_ref()
    }

    /// Sets the value of CertificateThumbPrint
    pub fn set_certificate_thumb_print(&mut self, value: String) {
        self.certificate_thumb_print = Some(value);
    }

    /// Gets the value of CertificateThumbPrint
    pub fn get_certificate_thumb_print(&self) -> Option<&String> {
        self.certificate_thumb_print.as_ref()
    }

    /// Sets the value of HttpPort
    pub fn set_http_port(&mut self, value: u16) {
        self.http_port = Some(value);
    }

    /// Gets the value of HttpPort
    pub fn get_http_port(&self) -> Option<&u16> {
        self.http_port.as_ref()
    }

    /// Sets the value of HttpsPort
    pub fn set_https_port(&mut self, value: u16) {
        self.https_port = Some(value);
    }

    /// Gets the value of HttpsPort
    pub fn get_https_port(&self) -> Option<&u16> {
        self.https_port.as_ref()
    }

    /// Sets the value of MonitoringInterval
    pub fn set_monitoring_interval(&mut self, value: u32) {
        self.monitoring_interval = Some(value);
    }

    /// Gets the value of MonitoringInterval
    pub fn get_monitoring_interval(&self) -> Option<&u32> {
        self.monitoring_interval.as_ref()
    }

    /// Sets the value of MonitoringStartTime
    pub fn set_monitoring_start_time(&mut self, value: String) {
        self.monitoring_start_time = Some(value);
    }

    /// Gets the value of MonitoringStartTime
    pub fn get_monitoring_start_time(&self) -> Option<&String> {
        self.monitoring_start_time.as_ref()
    }

    /// Sets the value of RecoveryServerEnabled
    pub fn set_recovery_server_enabled(&mut self, value: bool) {
        self.recovery_server_enabled = Some(value);
    }

    /// Gets the value of RecoveryServerEnabled
    pub fn get_recovery_server_enabled(&self) -> Option<&bool> {
        self.recovery_server_enabled.as_ref()
    }
}

impl Msvm_ReplicationServiceSettingData {
    /// Gets the related Msvm_ReplicationService object(s)
    pub fn get_related__replication_service(&self) -> Result<Msvm_ReplicationService, WmiError> {
        self.get_related("Msvm_ReplicationService")
    }

}

