// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_NTLogEventComputer struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_NTLogEventComputer {

/// 
    #[serde(rename = "Computer")]
    pub computer: Option<Win32_ComputerSystem>,

/// 
    #[serde(rename = "Record")]
    pub record: Option<Win32_NTLogEvent>,
}

impl Win32_NTLogEventComputer {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            computer: None,
            record: None,
        }
    }


    /// Sets the value of Computer
    pub fn set_computer(&mut self, value: Win32_ComputerSystem) {
        self.computer = Some(value);
    }

    /// Gets the value of Computer
    pub fn get_computer(&self) -> Option<&Win32_ComputerSystem> {
        self.computer.as_ref()
    }

    /// Sets the value of Record
    pub fn set_record(&mut self, value: Win32_NTLogEvent) {
        self.record = Some(value);
    }

    /// Gets the value of Record
    pub fn get_record(&self) -> Option<&Win32_NTLogEvent> {
        self.record.as_ref()
    }
}

