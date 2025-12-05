// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.MSCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSCluster_MetricRecordReal64 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSCluster_MetricRecordReal64 {
    #[serde(flatten)]
    pub base: MSCluster_MetricRecord,

/// 
    #[serde(rename = "Value")]
    pub value: Option<f64>,
}

impl MSCluster_MetricRecordReal64 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSCluster_MetricRecord::new(),
            value: None,
        }
    }


    /// Sets the value of Value
    pub fn set_value(&mut self, value: f64) {
        self.value = Some(value);
    }

    /// Gets the value of Value
    pub fn get_value(&self) -> Option<&f64> {
        self.value.as_ref()
    }
}

