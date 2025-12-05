// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SampledProfileInterval_V2 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SampledProfileInterval_V2 {
    #[serde(flatten)]
    pub base: PerfInfo_V2,

/// 
    #[serde(rename = "NewInterval")]
    pub new_interval: Option<u32>,

/// 
    #[serde(rename = "OldInterval")]
    pub old_interval: Option<u32>,

/// 
    #[serde(rename = "Source")]
    pub source: Option<u32>,
}

impl SampledProfileInterval_V2 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: PerfInfo_V2::new(),
            new_interval: None,
            old_interval: None,
            source: None,
        }
    }


    /// Sets the value of NewInterval
    pub fn set_new_interval(&mut self, value: u32) {
        self.new_interval = Some(value);
    }

    /// Gets the value of NewInterval
    pub fn get_new_interval(&self) -> Option<&u32> {
        self.new_interval.as_ref()
    }

    /// Sets the value of OldInterval
    pub fn set_old_interval(&mut self, value: u32) {
        self.old_interval = Some(value);
    }

    /// Gets the value of OldInterval
    pub fn get_old_interval(&self) -> Option<&u32> {
        self.old_interval.as_ref()
    }

    /// Sets the value of Source
    pub fn set_source(&mut self, value: u32) {
        self.source = Some(value);
    }

    /// Gets the value of Source
    pub fn get_source(&self) -> Option<&u32> {
        self.source.as_ref()
    }
}

