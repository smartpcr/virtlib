// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_ClussvcPerfProvider_ClusterDatabase struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_ClussvcPerfProvider_ClusterDatabase {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "Flushes")]
    pub flushes: Option<u64>,

/// 
    #[serde(rename = "FlushesPersec")]
    pub flushes_persec: Option<u64>,
}

impl Win32_PerfRawData_ClussvcPerfProvider_ClusterDatabase {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            flushes: None,
            flushes_persec: None,
        }
    }


    /// Sets the value of Flushes
    pub fn set_flushes(&mut self, value: u64) {
        self.flushes = Some(value);
    }

    /// Gets the value of Flushes
    pub fn get_flushes(&self) -> Option<&u64> {
        self.flushes.as_ref()
    }

    /// Sets the value of FlushesPersec
    pub fn set_flushes_persec(&mut self, value: u64) {
        self.flushes_persec = Some(value);
    }

    /// Gets the value of FlushesPersec
    pub fn get_flushes_persec(&self) -> Option<&u64> {
        self.flushes_persec.as_ref()
    }
}

