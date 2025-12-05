// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_ClussvcPerfProvider_ClusterGoodEnoughMulticastMessages2 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_ClussvcPerfProvider_ClusterGoodEnoughMulticastMessages2 {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "UnacknowledgedMessageCount")]
    pub unacknowledged_message_count: Option<u64>,
}

impl Win32_PerfFormattedData_ClussvcPerfProvider_ClusterGoodEnoughMulticastMessages2 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            unacknowledged_message_count: None,
        }
    }


    /// Sets the value of UnacknowledgedMessageCount
    pub fn set_unacknowledged_message_count(&mut self, value: u64) {
        self.unacknowledged_message_count = Some(value);
    }

    /// Gets the value of UnacknowledgedMessageCount
    pub fn get_unacknowledged_message_count(&self) -> Option<&u64> {
        self.unacknowledged_message_count.as_ref()
    }
}

