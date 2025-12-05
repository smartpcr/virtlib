// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SystemConfig_Boot struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemConfig_Boot {
    #[serde(flatten)]
    pub base: SystemConfig_V2,

/// 
    #[serde(rename = "BootFlags")]
    pub boot_flags: Option<u64>,

/// 
    #[serde(rename = "FirmwareType")]
    pub firmware_type: Option<u32>,

/// 
    #[serde(rename = "Reserved1")]
    pub reserved1: Option<u8>,

/// 
    #[serde(rename = "Reserved2")]
    pub reserved2: Option<u8>,

/// 
    #[serde(rename = "SecureBootCapable")]
    pub secure_boot_capable: Option<u8>,

/// 
    #[serde(rename = "SecureBootEnabled")]
    pub secure_boot_enabled: Option<u8>,
}

impl SystemConfig_Boot {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: SystemConfig_V2::new(),
            boot_flags: None,
            firmware_type: None,
            reserved1: None,
            reserved2: None,
            secure_boot_capable: None,
            secure_boot_enabled: None,
        }
    }


    /// Sets the value of BootFlags
    pub fn set_boot_flags(&mut self, value: u64) {
        self.boot_flags = Some(value);
    }

    /// Gets the value of BootFlags
    pub fn get_boot_flags(&self) -> Option<&u64> {
        self.boot_flags.as_ref()
    }

    /// Sets the value of FirmwareType
    pub fn set_firmware_type(&mut self, value: u32) {
        self.firmware_type = Some(value);
    }

    /// Gets the value of FirmwareType
    pub fn get_firmware_type(&self) -> Option<&u32> {
        self.firmware_type.as_ref()
    }

    /// Sets the value of Reserved1
    pub fn set_reserved1(&mut self, value: u8) {
        self.reserved1 = Some(value);
    }

    /// Gets the value of Reserved1
    pub fn get_reserved1(&self) -> Option<&u8> {
        self.reserved1.as_ref()
    }

    /// Sets the value of Reserved2
    pub fn set_reserved2(&mut self, value: u8) {
        self.reserved2 = Some(value);
    }

    /// Gets the value of Reserved2
    pub fn get_reserved2(&self) -> Option<&u8> {
        self.reserved2.as_ref()
    }

    /// Sets the value of SecureBootCapable
    pub fn set_secure_boot_capable(&mut self, value: u8) {
        self.secure_boot_capable = Some(value);
    }

    /// Gets the value of SecureBootCapable
    pub fn get_secure_boot_capable(&self) -> Option<&u8> {
        self.secure_boot_capable.as_ref()
    }

    /// Sets the value of SecureBootEnabled
    pub fn set_secure_boot_enabled(&mut self, value: u8) {
        self.secure_boot_enabled = Some(value);
    }

    /// Gets the value of SecureBootEnabled
    pub fn get_secure_boot_enabled(&self) -> Option<&u8> {
        self.secure_boot_enabled.as_ref()
    }
}

