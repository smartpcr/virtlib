// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_Counters_StorageManagementWSPSpacesRuntime struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_Counters_StorageManagementWSPSpacesRuntime {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "RuntimeCount16ms")]
    pub runtime_count16ms: Option<u32>,

/// 
    #[serde(rename = "RuntimeCount16s")]
    pub runtime_count16s: Option<u32>,

/// 
    #[serde(rename = "RuntimeCount1min")]
    pub runtime_count1min: Option<u32>,

/// 
    #[serde(rename = "RuntimeCount1s")]
    pub runtime_count1s: Option<u32>,

/// 
    #[serde(rename = "RuntimeCount256ms")]
    pub runtime_count256ms: Option<u32>,

/// 
    #[serde(rename = "RuntimeCount4ms")]
    pub runtime_count4ms: Option<u32>,

/// 
    #[serde(rename = "RuntimeCount4s")]
    pub runtime_count4s: Option<u32>,

/// 
    #[serde(rename = "RuntimeCount64ms")]
    pub runtime_count64ms: Option<u32>,

/// 
    #[serde(rename = "RuntimeCountInfinite")]
    pub runtime_count_infinite: Option<u32>,
}

impl Win32_PerfRawData_Counters_StorageManagementWSPSpacesRuntime {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            runtime_count16ms: None,
            runtime_count16s: None,
            runtime_count1min: None,
            runtime_count1s: None,
            runtime_count256ms: None,
            runtime_count4ms: None,
            runtime_count4s: None,
            runtime_count64ms: None,
            runtime_count_infinite: None,
        }
    }


    /// Sets the value of RuntimeCount16ms
    pub fn set_runtime_count16ms(&mut self, value: u32) {
        self.runtime_count16ms = Some(value);
    }

    /// Gets the value of RuntimeCount16ms
    pub fn get_runtime_count16ms(&self) -> Option<&u32> {
        self.runtime_count16ms.as_ref()
    }

    /// Sets the value of RuntimeCount16s
    pub fn set_runtime_count16s(&mut self, value: u32) {
        self.runtime_count16s = Some(value);
    }

    /// Gets the value of RuntimeCount16s
    pub fn get_runtime_count16s(&self) -> Option<&u32> {
        self.runtime_count16s.as_ref()
    }

    /// Sets the value of RuntimeCount1min
    pub fn set_runtime_count1min(&mut self, value: u32) {
        self.runtime_count1min = Some(value);
    }

    /// Gets the value of RuntimeCount1min
    pub fn get_runtime_count1min(&self) -> Option<&u32> {
        self.runtime_count1min.as_ref()
    }

    /// Sets the value of RuntimeCount1s
    pub fn set_runtime_count1s(&mut self, value: u32) {
        self.runtime_count1s = Some(value);
    }

    /// Gets the value of RuntimeCount1s
    pub fn get_runtime_count1s(&self) -> Option<&u32> {
        self.runtime_count1s.as_ref()
    }

    /// Sets the value of RuntimeCount256ms
    pub fn set_runtime_count256ms(&mut self, value: u32) {
        self.runtime_count256ms = Some(value);
    }

    /// Gets the value of RuntimeCount256ms
    pub fn get_runtime_count256ms(&self) -> Option<&u32> {
        self.runtime_count256ms.as_ref()
    }

    /// Sets the value of RuntimeCount4ms
    pub fn set_runtime_count4ms(&mut self, value: u32) {
        self.runtime_count4ms = Some(value);
    }

    /// Gets the value of RuntimeCount4ms
    pub fn get_runtime_count4ms(&self) -> Option<&u32> {
        self.runtime_count4ms.as_ref()
    }

    /// Sets the value of RuntimeCount4s
    pub fn set_runtime_count4s(&mut self, value: u32) {
        self.runtime_count4s = Some(value);
    }

    /// Gets the value of RuntimeCount4s
    pub fn get_runtime_count4s(&self) -> Option<&u32> {
        self.runtime_count4s.as_ref()
    }

    /// Sets the value of RuntimeCount64ms
    pub fn set_runtime_count64ms(&mut self, value: u32) {
        self.runtime_count64ms = Some(value);
    }

    /// Gets the value of RuntimeCount64ms
    pub fn get_runtime_count64ms(&self) -> Option<&u32> {
        self.runtime_count64ms.as_ref()
    }

    /// Sets the value of RuntimeCountInfinite
    pub fn set_runtime_count_infinite(&mut self, value: u32) {
        self.runtime_count_infinite = Some(value);
    }

    /// Gets the value of RuntimeCountInfinite
    pub fn get_runtime_count_infinite(&self) -> Option<&u32> {
        self.runtime_count_infinite.as_ref()
    }
}

