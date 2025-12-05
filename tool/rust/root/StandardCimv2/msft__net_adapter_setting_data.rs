// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetAdapterSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetAdapterSettingData {
    #[serde(flatten)]
    pub base: MSFT_NetSettingData,

/// 
    #[serde(rename = "InterfaceDescription")]
    pub interface_description: Option<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "Source")]
    pub source: Option<u32>,

/// 
    #[serde(rename = "SystemName")]
    pub system_name: Option<String>,
}

impl MSFT_NetAdapterSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetSettingData::new(),
            interface_description: None,
            name: None,
            source: None,
            system_name: None,
        }
    }


    /// Sets the value of InterfaceDescription
    pub fn set_interface_description(&mut self, value: String) {
        self.interface_description = Some(value);
    }

    /// Gets the value of InterfaceDescription
    pub fn get_interface_description(&self) -> Option<&String> {
        self.interface_description.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of Source
    pub fn set_source(&mut self, value: u32) {
        self.source = Some(value);
    }

    /// Gets the value of Source
    pub fn get_source(&self) -> Option<&u32> {
        self.source.as_ref()
    }

    /// Sets the value of SystemName
    pub fn set_system_name(&mut self, value: String) {
        self.system_name = Some(value);
    }

    /// Gets the value of SystemName
    pub fn get_system_name(&self) -> Option<&String> {
        self.system_name.as_ref()
    }
}

