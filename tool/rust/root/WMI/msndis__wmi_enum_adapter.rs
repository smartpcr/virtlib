// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_WmiEnumAdapter struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_WmiEnumAdapter {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "DeviceName")]
    pub device_name: Option<String>,

/// 
    #[serde(rename = "Header")]
    pub header: Option<MSNdis_ObjectHeader>,

/// 
    #[serde(rename = "IfIndex")]
    pub if_index: Option<u32>,

/// 
    #[serde(rename = "NetLuid")]
    pub net_luid: Option<u64>,
}

impl MSNdis_WmiEnumAdapter {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            device_name: None,
            header: None,
            if_index: None,
            net_luid: None,
        }
    }


    /// Sets the value of DeviceName
    pub fn set_device_name(&mut self, value: String) {
        self.device_name = Some(value);
    }

    /// Gets the value of DeviceName
    pub fn get_device_name(&self) -> Option<&String> {
        self.device_name.as_ref()
    }

    /// Sets the value of Header
    pub fn set_header(&mut self, value: MSNdis_ObjectHeader) {
        self.header = Some(value);
    }

    /// Gets the value of Header
    pub fn get_header(&self) -> Option<&MSNdis_ObjectHeader> {
        self.header.as_ref()
    }

    /// Sets the value of IfIndex
    pub fn set_if_index(&mut self, value: u32) {
        self.if_index = Some(value);
    }

    /// Gets the value of IfIndex
    pub fn get_if_index(&self) -> Option<&u32> {
        self.if_index.as_ref()
    }

    /// Sets the value of NetLuid
    pub fn set_net_luid(&mut self, value: u64) {
        self.net_luid = Some(value);
    }

    /// Gets the value of NetLuid
    pub fn get_net_luid(&self) -> Option<&u64> {
        self.net_luid.as_ref()
    }
}

