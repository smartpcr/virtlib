// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// IdleStateBucketEx struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IdleStateBucketEx {

/// 
    #[serde(rename = "Count")]
    pub count: Option<u32>,

/// 
    #[serde(rename = "MaxTimeUs")]
    pub max_time_us: Option<u32>,

/// 
    #[serde(rename = "MinTimeUs")]
    pub min_time_us: Option<u32>,

/// 
    #[serde(rename = "TotalTimeUs")]
    pub total_time_us: Option<u64>,
}

impl IdleStateBucketEx {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            count: None,
            max_time_us: None,
            min_time_us: None,
            total_time_us: None,
        }
    }


    /// Sets the value of Count
    pub fn set_count(&mut self, value: u32) {
        self.count = Some(value);
    }

    /// Gets the value of Count
    pub fn get_count(&self) -> Option<&u32> {
        self.count.as_ref()
    }

    /// Sets the value of MaxTimeUs
    pub fn set_max_time_us(&mut self, value: u32) {
        self.max_time_us = Some(value);
    }

    /// Gets the value of MaxTimeUs
    pub fn get_max_time_us(&self) -> Option<&u32> {
        self.max_time_us.as_ref()
    }

    /// Sets the value of MinTimeUs
    pub fn set_min_time_us(&mut self, value: u32) {
        self.min_time_us = Some(value);
    }

    /// Gets the value of MinTimeUs
    pub fn get_min_time_us(&self) -> Option<&u32> {
        self.min_time_us.as_ref()
    }

    /// Sets the value of TotalTimeUs
    pub fn set_total_time_us(&mut self, value: u64) {
        self.total_time_us = Some(value);
    }

    /// Gets the value of TotalTimeUs
    pub fn get_total_time_us(&self) -> Option<&u64> {
        self.total_time_us.as_ref()
    }
}

