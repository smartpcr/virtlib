// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetAdapter_Group_Affinity struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetAdapter_Group_Affinity {

/// 
    #[serde(rename = "ProcessorAffinityMask")]
    pub processor_affinity_mask: Option<u64>,

/// 
    #[serde(rename = "ProcessorGroup")]
    pub processor_group: Option<u16>,
}

impl MSFT_NetAdapter_Group_Affinity {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            processor_affinity_mask: None,
            processor_group: None,
        }
    }


    /// Sets the value of ProcessorAffinityMask
    pub fn set_processor_affinity_mask(&mut self, value: u64) {
        self.processor_affinity_mask = Some(value);
    }

    /// Gets the value of ProcessorAffinityMask
    pub fn get_processor_affinity_mask(&self) -> Option<&u64> {
        self.processor_affinity_mask.as_ref()
    }

    /// Sets the value of ProcessorGroup
    pub fn set_processor_group(&mut self, value: u16) {
        self.processor_group = Some(value);
    }

    /// Gets the value of ProcessorGroup
    pub fn get_processor_group(&self) -> Option<&u16> {
        self.processor_group.as_ref()
    }
}

