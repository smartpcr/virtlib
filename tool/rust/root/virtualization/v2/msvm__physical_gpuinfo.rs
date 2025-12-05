// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_PhysicalGPUInfo struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_PhysicalGPUInfo {
    #[serde(flatten)]
    pub base: CIM_ManagedElement,

/// 
    #[serde(rename = "AvailableVideoMemory")]
    pub available_video_memory: Option<u64>,

/// 
    #[serde(rename = "ID")]
    pub id: Option<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "TotalVideoMemory")]
    pub total_video_memory: Option<u64>,
}

impl Msvm_PhysicalGPUInfo {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ManagedElement::new(),
            available_video_memory: None,
            id: None,
            name: None,
            total_video_memory: None,
        }
    }


    /// Sets the value of AvailableVideoMemory
    pub fn set_available_video_memory(&mut self, value: u64) {
        self.available_video_memory = Some(value);
    }

    /// Gets the value of AvailableVideoMemory
    pub fn get_available_video_memory(&self) -> Option<&u64> {
        self.available_video_memory.as_ref()
    }

    /// Sets the value of ID
    pub fn set_id(&mut self, value: String) {
        self.id = Some(value);
    }

    /// Gets the value of ID
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of TotalVideoMemory
    pub fn set_total_video_memory(&mut self, value: u64) {
        self.total_video_memory = Some(value);
    }

    /// Gets the value of TotalVideoMemory
    pub fn get_total_video_memory(&self) -> Option<&u64> {
        self.total_video_memory.as_ref()
    }
}

