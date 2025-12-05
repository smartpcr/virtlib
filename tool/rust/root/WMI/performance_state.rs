// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// PerformanceState struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PerformanceState {

/// 
    #[serde(rename = "Flags")]
    pub flags: Option<u32>,

/// 
    #[serde(rename = "Frequency")]
    pub frequency: Option<u32>,

/// 
    #[serde(rename = "PercentFrequency")]
    pub percent_frequency: Option<u32>,
}

impl PerformanceState {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            flags: None,
            frequency: None,
            percent_frequency: None,
        }
    }


    /// Sets the value of Flags
    pub fn set_flags(&mut self, value: u32) {
        self.flags = Some(value);
    }

    /// Gets the value of Flags
    pub fn get_flags(&self) -> Option<&u32> {
        self.flags.as_ref()
    }

    /// Sets the value of Frequency
    pub fn set_frequency(&mut self, value: u32) {
        self.frequency = Some(value);
    }

    /// Gets the value of Frequency
    pub fn get_frequency(&self) -> Option<&u32> {
        self.frequency.as_ref()
    }

    /// Sets the value of PercentFrequency
    pub fn set_percent_frequency(&mut self, value: u32) {
        self.percent_frequency = Some(value);
    }

    /// Gets the value of PercentFrequency
    pub fn get_percent_frequency(&self) -> Option<&u32> {
        self.percent_frequency.as_ref()
    }
}

