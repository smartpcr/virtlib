// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_DeviceRegistrationInfo struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_DeviceRegistrationInfo {

/// 
    #[serde(rename = "CertificateThumbprint")]
    pub certificate_thumbprint: Option<String>,

/// 
    #[serde(rename = "DeviceId")]
    pub device_id: Option<String>,

/// 
    #[serde(rename = "UPN")]
    pub upn: Option<String>,
}

impl MDM_DeviceRegistrationInfo {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            certificate_thumbprint: None,
            device_id: None,
            upn: None,
        }
    }


    /// Sets the value of CertificateThumbprint
    pub fn set_certificate_thumbprint(&mut self, value: String) {
        self.certificate_thumbprint = Some(value);
    }

    /// Gets the value of CertificateThumbprint
    pub fn get_certificate_thumbprint(&self) -> Option<&String> {
        self.certificate_thumbprint.as_ref()
    }

    /// Sets the value of DeviceId
    pub fn set_device_id(&mut self, value: String) {
        self.device_id = Some(value);
    }

    /// Gets the value of DeviceId
    pub fn get_device_id(&self) -> Option<&String> {
        self.device_id.as_ref()
    }

    /// Sets the value of UPN
    pub fn set_upn(&mut self, value: String) {
        self.upn = Some(value);
    }

    /// Gets the value of UPN
    pub fn get_upn(&self) -> Option<&String> {
        self.upn.as_ref()
    }
}

