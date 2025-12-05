// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_LocalSessionManager_TerminalServices struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_LocalSessionManager_TerminalServices {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "ActiveSessions")]
    pub active_sessions: Option<u32>,

/// 
    #[serde(rename = "InactiveSessions")]
    pub inactive_sessions: Option<u32>,

/// 
    #[serde(rename = "TotalSessions")]
    pub total_sessions: Option<u32>,
}

impl Win32_PerfRawData_LocalSessionManager_TerminalServices {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            active_sessions: None,
            inactive_sessions: None,
            total_sessions: None,
        }
    }


    /// Sets the value of ActiveSessions
    pub fn set_active_sessions(&mut self, value: u32) {
        self.active_sessions = Some(value);
    }

    /// Gets the value of ActiveSessions
    pub fn get_active_sessions(&self) -> Option<&u32> {
        self.active_sessions.as_ref()
    }

    /// Sets the value of InactiveSessions
    pub fn set_inactive_sessions(&mut self, value: u32) {
        self.inactive_sessions = Some(value);
    }

    /// Gets the value of InactiveSessions
    pub fn get_inactive_sessions(&self) -> Option<&u32> {
        self.inactive_sessions.as_ref()
    }

    /// Sets the value of TotalSessions
    pub fn set_total_sessions(&mut self, value: u32) {
        self.total_sessions = Some(value);
    }

    /// Gets the value of TotalSessions
    pub fn get_total_sessions(&self) -> Option<&u32> {
        self.total_sessions.as_ref()
    }
}

