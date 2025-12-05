// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_Counters_AppVClientStreamedDataPercentage struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_Counters_AppVClientStreamedDataPercentage {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "PrimaryFeaturePercentStreamed")]
    pub primary_feature_percent_streamed: Option<u32>,
}

impl Win32_PerfRawData_Counters_AppVClientStreamedDataPercentage {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            primary_feature_percent_streamed: None,
        }
    }


    /// Sets the value of PrimaryFeaturePercentStreamed
    pub fn set_primary_feature_percent_streamed(&mut self, value: u32) {
        self.primary_feature_percent_streamed = Some(value);
    }

    /// Gets the value of PrimaryFeaturePercentStreamed
    pub fn get_primary_feature_percent_streamed(&self) -> Option<&u32> {
        self.primary_feature_percent_streamed.as_ref()
    }
}

