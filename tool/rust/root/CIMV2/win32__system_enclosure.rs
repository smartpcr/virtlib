// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_SystemEnclosure struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_SystemEnclosure {
    #[serde(flatten)]
    pub base: CIM_Chassis,

/// 
    #[serde(rename = "SecurityStatus")]
    pub security_status: Option<u16>,

/// 
    #[serde(rename = "SMBIOSAssetTag")]
    pub smbiosasset_tag: Option<String>,
}

impl Win32_SystemEnclosure {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Chassis::new(),
            security_status: None,
            smbiosasset_tag: None,
        }
    }


    /// Sets the value of SecurityStatus
    pub fn set_security_status(&mut self, value: u16) {
        self.security_status = Some(value);
    }

    /// Gets the value of SecurityStatus
    pub fn get_security_status(&self) -> Option<&u16> {
        self.security_status.as_ref()
    }

    /// Sets the value of SMBIOSAssetTag
    pub fn set_smbiosasset_tag(&mut self, value: String) {
        self.smbiosasset_tag = Some(value);
    }

    /// Gets the value of SMBIOSAssetTag
    pub fn get_smbiosasset_tag(&self) -> Option<&String> {
        self.smbiosasset_tag.as_ref()
    }
}

