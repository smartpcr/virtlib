// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSiSCSI_SecurityCapabilities struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSiSCSI_SecurityCapabilities {

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// TRUE if adapter supports certificates
    #[serde(rename = "CertificatesSupported")]
    pub certificates_supported: Option<bool>,

/// **typedef** Array of encryption types. This field is a variable length array.
    #[serde(rename = "EncryptionAvailable")]
    pub encryption_available: Vec<SecurityCapabilities_EncryptionAvailable>,

/// Number of encryption types available.
    #[serde(rename = "EncryptionAvailableCount")]
    pub encryption_available_count: Option<u32>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// TRUE if the adapter can use IPSEC to protect iSCSI traffic.
    #[serde(rename = "ProtectiScsiTraffic")]
    pub protecti_scsi_traffic: Option<bool>,

/// TRUE if the adapter can use IPSEC to protect iSNS traffic.
    #[serde(rename = "ProtectiSNSTraffic")]
    pub protecti_snstraffic: Option<bool>,
}

impl MSiSCSI_SecurityCapabilities {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            active: None,
            certificates_supported: None,
            encryption_available: Vec::new(),
            encryption_available_count: None,
            instance_name: None,
            protecti_scsi_traffic: None,
            protecti_snstraffic: None,
        }
    }


    /// Sets the value of Active
    pub fn set_active(&mut self, value: bool) {
        self.active = Some(value);
    }

    /// Gets the value of Active
    pub fn get_active(&self) -> Option<&bool> {
        self.active.as_ref()
    }

    /// Sets the value of CertificatesSupported
    pub fn set_certificates_supported(&mut self, value: bool) {
        self.certificates_supported = Some(value);
    }

    /// Gets the value of CertificatesSupported
    pub fn get_certificates_supported(&self) -> Option<&bool> {
        self.certificates_supported.as_ref()
    }

    /// Sets the value of EncryptionAvailable
    pub fn set_encryption_available(&mut self, value: Vec<SecurityCapabilities_EncryptionAvailable>) {
        self.encryption_available = value;
    }

    /// Gets the value of EncryptionAvailable
    pub fn get_encryption_available(&self) -> &Vec<SecurityCapabilities_EncryptionAvailable> {
        &self.encryption_available
    }

    /// Sets the value of EncryptionAvailableCount
    pub fn set_encryption_available_count(&mut self, value: u32) {
        self.encryption_available_count = Some(value);
    }

    /// Gets the value of EncryptionAvailableCount
    pub fn get_encryption_available_count(&self) -> Option<&u32> {
        self.encryption_available_count.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of ProtectiScsiTraffic
    pub fn set_protecti_scsi_traffic(&mut self, value: bool) {
        self.protecti_scsi_traffic = Some(value);
    }

    /// Gets the value of ProtectiScsiTraffic
    pub fn get_protecti_scsi_traffic(&self) -> Option<&bool> {
        self.protecti_scsi_traffic.as_ref()
    }

    /// Sets the value of ProtectiSNSTraffic
    pub fn set_protecti_snstraffic(&mut self, value: bool) {
        self.protecti_snstraffic = Some(value);
    }

    /// Gets the value of ProtectiSNSTraffic
    pub fn get_protecti_snstraffic(&self) -> Option<&bool> {
        self.protecti_snstraffic.as_ref()
    }
}

