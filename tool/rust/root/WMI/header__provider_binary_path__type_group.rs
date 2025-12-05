// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Header_ProviderBinaryPath_TypeGroup struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Header_ProviderBinaryPath_TypeGroup {
    #[serde(flatten)]
    pub base: EventTraceEvent,

/// 
    #[serde(rename = "BinaryPath")]
    pub binary_path: Option<String>,

/// 
    #[serde(rename = "Guid")]
    pub guid: Vec<serde_json::Value>,

/// 
    #[serde(rename = "GuidCount")]
    pub guid_count: Option<u32>,
}

impl Header_ProviderBinaryPath_TypeGroup {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: EventTraceEvent::new(),
            binary_path: None,
            guid: Vec::new(),
            guid_count: None,
        }
    }


    /// Sets the value of BinaryPath
    pub fn set_binary_path(&mut self, value: String) {
        self.binary_path = Some(value);
    }

    /// Gets the value of BinaryPath
    pub fn get_binary_path(&self) -> Option<&String> {
        self.binary_path.as_ref()
    }

    /// Sets the value of Guid
    pub fn set_guid(&mut self, value: Vec<serde_json::Value>) {
        self.guid = value;
    }

    /// Gets the value of Guid
    pub fn get_guid(&self) -> &Vec<serde_json::Value> {
        &self.guid
    }

    /// Sets the value of GuidCount
    pub fn set_guid_count(&mut self, value: u32) {
        self.guid_count = Some(value);
    }

    /// Gets the value of GuidCount
    pub fn get_guid_count(&self) -> Option<&u32> {
        self.guid_count.as_ref()
    }
}

