// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_NetFtPerfProvider_ClusterNetFtHeartbeats struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_NetFtPerfProvider_ClusterNetFtHeartbeats {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "Missingheartbeats")]
    pub missingheartbeats: Option<u32>,

/// 
    #[serde(rename = "Missingheartbeatslimit")]
    pub missingheartbeatslimit: Option<u32>,
}

impl Win32_PerfFormattedData_NetFtPerfProvider_ClusterNetFtHeartbeats {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            missingheartbeats: None,
            missingheartbeatslimit: None,
        }
    }


    /// Sets the value of Missingheartbeats
    pub fn set_missingheartbeats(&mut self, value: u32) {
        self.missingheartbeats = Some(value);
    }

    /// Gets the value of Missingheartbeats
    pub fn get_missingheartbeats(&self) -> Option<&u32> {
        self.missingheartbeats.as_ref()
    }

    /// Sets the value of Missingheartbeatslimit
    pub fn set_missingheartbeatslimit(&mut self, value: u32) {
        self.missingheartbeatslimit = Some(value);
    }

    /// Gets the value of Missingheartbeatslimit
    pub fn get_missingheartbeatslimit(&self) -> Option<&u32> {
        self.missingheartbeatslimit.as_ref()
    }
}

