// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_Perf struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_Perf {
    #[serde(flatten)]
    pub base: CIM_StatisticalInformation,

/// 
    #[serde(rename = "Frequency_Object")]
    pub frequency__object: Option<u64>,

/// 
    #[serde(rename = "Frequency_PerfTime")]
    pub frequency__perf_time: Option<u64>,

/// 
    #[serde(rename = "Frequency_Sys100NS")]
    pub frequency__sys100_ns: Option<u64>,

/// 
    #[serde(rename = "Timestamp_Object")]
    pub timestamp__object: Option<u64>,

/// 
    #[serde(rename = "Timestamp_PerfTime")]
    pub timestamp__perf_time: Option<u64>,

/// 
    #[serde(rename = "Timestamp_Sys100NS")]
    pub timestamp__sys100_ns: Option<u64>,
}

impl Win32_Perf {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_StatisticalInformation::new(),
            frequency__object: None,
            frequency__perf_time: None,
            frequency__sys100_ns: None,
            timestamp__object: None,
            timestamp__perf_time: None,
            timestamp__sys100_ns: None,
        }
    }


    /// Sets the value of Frequency_Object
    pub fn set_frequency__object(&mut self, value: u64) {
        self.frequency__object = Some(value);
    }

    /// Gets the value of Frequency_Object
    pub fn get_frequency__object(&self) -> Option<&u64> {
        self.frequency__object.as_ref()
    }

    /// Sets the value of Frequency_PerfTime
    pub fn set_frequency__perf_time(&mut self, value: u64) {
        self.frequency__perf_time = Some(value);
    }

    /// Gets the value of Frequency_PerfTime
    pub fn get_frequency__perf_time(&self) -> Option<&u64> {
        self.frequency__perf_time.as_ref()
    }

    /// Sets the value of Frequency_Sys100NS
    pub fn set_frequency__sys100_ns(&mut self, value: u64) {
        self.frequency__sys100_ns = Some(value);
    }

    /// Gets the value of Frequency_Sys100NS
    pub fn get_frequency__sys100_ns(&self) -> Option<&u64> {
        self.frequency__sys100_ns.as_ref()
    }

    /// Sets the value of Timestamp_Object
    pub fn set_timestamp__object(&mut self, value: u64) {
        self.timestamp__object = Some(value);
    }

    /// Gets the value of Timestamp_Object
    pub fn get_timestamp__object(&self) -> Option<&u64> {
        self.timestamp__object.as_ref()
    }

    /// Sets the value of Timestamp_PerfTime
    pub fn set_timestamp__perf_time(&mut self, value: u64) {
        self.timestamp__perf_time = Some(value);
    }

    /// Gets the value of Timestamp_PerfTime
    pub fn get_timestamp__perf_time(&self) -> Option<&u64> {
        self.timestamp__perf_time.as_ref()
    }

    /// Sets the value of Timestamp_Sys100NS
    pub fn set_timestamp__sys100_ns(&mut self, value: u64) {
        self.timestamp__sys100_ns = Some(value);
    }

    /// Gets the value of Timestamp_Sys100NS
    pub fn get_timestamp__sys100_ns(&self) -> Option<&u64> {
        self.timestamp__sys100_ns.as_ref()
    }
}

