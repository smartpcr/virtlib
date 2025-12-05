// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_ClussvcPerfProvider_ClusterResourceControlManager struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_ClussvcPerfProvider_ClusterResourceControlManager {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "GroupsOnline")]
    pub groups_online: Option<u64>,

/// 
    #[serde(rename = "RHSProcesses")]
    pub rhsprocesses: Option<u64>,

/// 
    #[serde(rename = "RHSRestarts")]
    pub rhsrestarts: Option<u64>,
}

impl Win32_PerfRawData_ClussvcPerfProvider_ClusterResourceControlManager {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            groups_online: None,
            rhsprocesses: None,
            rhsrestarts: None,
        }
    }


    /// Sets the value of GroupsOnline
    pub fn set_groups_online(&mut self, value: u64) {
        self.groups_online = Some(value);
    }

    /// Gets the value of GroupsOnline
    pub fn get_groups_online(&self) -> Option<&u64> {
        self.groups_online.as_ref()
    }

    /// Sets the value of RHSProcesses
    pub fn set_rhsprocesses(&mut self, value: u64) {
        self.rhsprocesses = Some(value);
    }

    /// Gets the value of RHSProcesses
    pub fn get_rhsprocesses(&self) -> Option<&u64> {
        self.rhsprocesses.as_ref()
    }

    /// Sets the value of RHSRestarts
    pub fn set_rhsrestarts(&mut self, value: u64) {
        self.rhsrestarts = Some(value);
    }

    /// Gets the value of RHSRestarts
    pub fn get_rhsrestarts(&self) -> Option<&u64> {
        self.rhsrestarts.as_ref()
    }
}

