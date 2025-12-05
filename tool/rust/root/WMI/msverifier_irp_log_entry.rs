// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSVerifierIrpLogEntry struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSVerifierIrpLogEntry {

/// Arg1
    #[serde(rename = "Arg1")]
    pub arg1: Option<u64>,

/// Arg2
    #[serde(rename = "Arg2")]
    pub arg2: Option<u64>,

/// Arg3
    #[serde(rename = "Arg3")]
    pub arg3: Option<u64>,

/// Arg4
    #[serde(rename = "Arg4")]
    pub arg4: Option<u64>,

/// Control
    #[serde(rename = "Control")]
    pub control: Option<u8>,

/// Count
    #[serde(rename = "Count")]
    pub count: Option<u32>,

/// Flags
    #[serde(rename = "Flags")]
    pub flags: Option<u8>,

/// Major Function
    #[serde(rename = "Major")]
    pub major: Option<u8>,

/// Minor Function
    #[serde(rename = "Minor")]
    pub minor: Option<u8>,
}

impl MSVerifierIrpLogEntry {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            arg1: None,
            arg2: None,
            arg3: None,
            arg4: None,
            control: None,
            count: None,
            flags: None,
            major: None,
            minor: None,
        }
    }


    /// Sets the value of Arg1
    pub fn set_arg1(&mut self, value: u64) {
        self.arg1 = Some(value);
    }

    /// Gets the value of Arg1
    pub fn get_arg1(&self) -> Option<&u64> {
        self.arg1.as_ref()
    }

    /// Sets the value of Arg2
    pub fn set_arg2(&mut self, value: u64) {
        self.arg2 = Some(value);
    }

    /// Gets the value of Arg2
    pub fn get_arg2(&self) -> Option<&u64> {
        self.arg2.as_ref()
    }

    /// Sets the value of Arg3
    pub fn set_arg3(&mut self, value: u64) {
        self.arg3 = Some(value);
    }

    /// Gets the value of Arg3
    pub fn get_arg3(&self) -> Option<&u64> {
        self.arg3.as_ref()
    }

    /// Sets the value of Arg4
    pub fn set_arg4(&mut self, value: u64) {
        self.arg4 = Some(value);
    }

    /// Gets the value of Arg4
    pub fn get_arg4(&self) -> Option<&u64> {
        self.arg4.as_ref()
    }

    /// Sets the value of Control
    pub fn set_control(&mut self, value: u8) {
        self.control = Some(value);
    }

    /// Gets the value of Control
    pub fn get_control(&self) -> Option<&u8> {
        self.control.as_ref()
    }

    /// Sets the value of Count
    pub fn set_count(&mut self, value: u32) {
        self.count = Some(value);
    }

    /// Gets the value of Count
    pub fn get_count(&self) -> Option<&u32> {
        self.count.as_ref()
    }

    /// Sets the value of Flags
    pub fn set_flags(&mut self, value: u8) {
        self.flags = Some(value);
    }

    /// Gets the value of Flags
    pub fn get_flags(&self) -> Option<&u8> {
        self.flags.as_ref()
    }

    /// Sets the value of Major
    pub fn set_major(&mut self, value: u8) {
        self.major = Some(value);
    }

    /// Gets the value of Major
    pub fn get_major(&self) -> Option<&u8> {
        self.major.as_ref()
    }

    /// Sets the value of Minor
    pub fn set_minor(&mut self, value: u8) {
        self.minor = Some(value);
    }

    /// Gets the value of Minor
    pub fn get_minor(&self) -> Option<&u8> {
        self.minor.as_ref()
    }
}

