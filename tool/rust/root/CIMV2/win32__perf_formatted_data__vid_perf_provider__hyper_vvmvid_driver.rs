// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_VidPerfProvider_HyperVVMVidDriver struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_VidPerfProvider_HyperVVMVidDriver {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "VidPartitions")]
    pub vid_partitions: Option<u64>,
}

impl Win32_PerfFormattedData_VidPerfProvider_HyperVVMVidDriver {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            vid_partitions: None,
        }
    }


    /// Sets the value of VidPartitions
    pub fn set_vid_partitions(&mut self, value: u64) {
        self.vid_partitions = Some(value);
    }

    /// Gets the value of VidPartitions
    pub fn get_vid_partitions(&self) -> Option<&u64> {
        self.vid_partitions.as_ref()
    }
}

