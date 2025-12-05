// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_ProcessTrace struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_ProcessTrace {
    #[serde(flatten)]
    pub base: Win32_SystemTrace,

/// 
    #[serde(rename = "ParentProcessID")]
    pub parent_process_id: Option<u32>,

/// 
    #[serde(rename = "ProcessID")]
    pub process_id: Option<u32>,

/// 
    #[serde(rename = "ProcessName")]
    pub process_name: Option<String>,

/// 
    #[serde(rename = "SessionID")]
    pub session_id: Option<u32>,

/// 
    #[serde(rename = "Sid")]
    pub sid: Vec<u8>,
}

impl Win32_ProcessTrace {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_SystemTrace::new(),
            parent_process_id: None,
            process_id: None,
            process_name: None,
            session_id: None,
            sid: Vec::new(),
        }
    }


    /// Sets the value of ParentProcessID
    pub fn set_parent_process_id(&mut self, value: u32) {
        self.parent_process_id = Some(value);
    }

    /// Gets the value of ParentProcessID
    pub fn get_parent_process_id(&self) -> Option<&u32> {
        self.parent_process_id.as_ref()
    }

    /// Sets the value of ProcessID
    pub fn set_process_id(&mut self, value: u32) {
        self.process_id = Some(value);
    }

    /// Gets the value of ProcessID
    pub fn get_process_id(&self) -> Option<&u32> {
        self.process_id.as_ref()
    }

    /// Sets the value of ProcessName
    pub fn set_process_name(&mut self, value: String) {
        self.process_name = Some(value);
    }

    /// Gets the value of ProcessName
    pub fn get_process_name(&self) -> Option<&String> {
        self.process_name.as_ref()
    }

    /// Sets the value of SessionID
    pub fn set_session_id(&mut self, value: u32) {
        self.session_id = Some(value);
    }

    /// Gets the value of SessionID
    pub fn get_session_id(&self) -> Option<&u32> {
        self.session_id.as_ref()
    }

    /// Sets the value of Sid
    pub fn set_sid(&mut self, value: Vec<u8>) {
        self.sid = value;
    }

    /// Gets the value of Sid
    pub fn get_sid(&self) -> &Vec<u8> {
        &self.sid
    }
}

