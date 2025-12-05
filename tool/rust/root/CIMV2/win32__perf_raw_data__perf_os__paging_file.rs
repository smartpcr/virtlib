// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_PerfOS_PagingFile struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_PerfOS_PagingFile {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "PercentUsage")]
    pub percent_usage: Option<u32>,

/// 
    #[serde(rename = "PercentUsage_Base")]
    pub percent_usage__base: Option<u32>,

/// 
    #[serde(rename = "PercentUsagePeak")]
    pub percent_usage_peak: Option<u32>,

/// 
    #[serde(rename = "PercentUsagePeak_Base")]
    pub percent_usage_peak__base: Option<u32>,
}

impl Win32_PerfRawData_PerfOS_PagingFile {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            percent_usage: None,
            percent_usage__base: None,
            percent_usage_peak: None,
            percent_usage_peak__base: None,
        }
    }


    /// Sets the value of PercentUsage
    pub fn set_percent_usage(&mut self, value: u32) {
        self.percent_usage = Some(value);
    }

    /// Gets the value of PercentUsage
    pub fn get_percent_usage(&self) -> Option<&u32> {
        self.percent_usage.as_ref()
    }

    /// Sets the value of PercentUsage_Base
    pub fn set_percent_usage__base(&mut self, value: u32) {
        self.percent_usage__base = Some(value);
    }

    /// Gets the value of PercentUsage_Base
    pub fn get_percent_usage__base(&self) -> Option<&u32> {
        self.percent_usage__base.as_ref()
    }

    /// Sets the value of PercentUsagePeak
    pub fn set_percent_usage_peak(&mut self, value: u32) {
        self.percent_usage_peak = Some(value);
    }

    /// Gets the value of PercentUsagePeak
    pub fn get_percent_usage_peak(&self) -> Option<&u32> {
        self.percent_usage_peak.as_ref()
    }

    /// Sets the value of PercentUsagePeak_Base
    pub fn set_percent_usage_peak__base(&mut self, value: u32) {
        self.percent_usage_peak__base = Some(value);
    }

    /// Gets the value of PercentUsagePeak_Base
    pub fn get_percent_usage_peak__base(&self) -> Option<&u32> {
        self.percent_usage_peak__base.as_ref()
    }
}

