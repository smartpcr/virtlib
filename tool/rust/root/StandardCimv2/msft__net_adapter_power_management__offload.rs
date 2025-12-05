// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetAdapterPowerManagement_Offload struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetAdapterPowerManagement_Offload {

/// 
    #[serde(rename = "FriendlyName")]
    pub friendly_name: Option<String>,

/// 
    #[serde(rename = "ID")]
    pub id: Option<u32>,

/// 
    #[serde(rename = "OffloadType")]
    pub offload_type: Option<u32>,

/// 
    #[serde(rename = "Priority")]
    pub priority: Option<u32>,
}

impl MSFT_NetAdapterPowerManagement_Offload {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            friendly_name: None,
            id: None,
            offload_type: None,
            priority: None,
        }
    }


    /// Sets the value of FriendlyName
    pub fn set_friendly_name(&mut self, value: String) {
        self.friendly_name = Some(value);
    }

    /// Gets the value of FriendlyName
    pub fn get_friendly_name(&self) -> Option<&String> {
        self.friendly_name.as_ref()
    }

    /// Sets the value of ID
    pub fn set_id(&mut self, value: u32) {
        self.id = Some(value);
    }

    /// Gets the value of ID
    pub fn get_id(&self) -> Option<&u32> {
        self.id.as_ref()
    }

    /// Sets the value of OffloadType
    pub fn set_offload_type(&mut self, value: u32) {
        self.offload_type = Some(value);
    }

    /// Gets the value of OffloadType
    pub fn get_offload_type(&self) -> Option<&u32> {
        self.offload_type.as_ref()
    }

    /// Sets the value of Priority
    pub fn set_priority(&mut self, value: u32) {
        self.priority = Some(value);
    }

    /// Gets the value of Priority
    pub fn get_priority(&self) -> Option<&u32> {
        self.priority.as_ref()
    }
}

