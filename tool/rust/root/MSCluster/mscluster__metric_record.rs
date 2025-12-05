// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.MSCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSCluster_MetricRecord struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSCluster_MetricRecord {

/// 
    #[serde(rename = "TimeStamp")]
    pub time_stamp: Option<String>,
}

impl MSCluster_MetricRecord {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            time_stamp: None,
        }
    }


    /// Sets the value of TimeStamp
    pub fn set_time_stamp(&mut self, value: String) {
        self.time_stamp = Some(value);
    }

    /// Gets the value of TimeStamp
    pub fn get_time_stamp(&self) -> Option<&String> {
        self.time_stamp.as_ref()
    }
}

