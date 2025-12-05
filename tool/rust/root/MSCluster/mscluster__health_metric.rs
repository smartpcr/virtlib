// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.MSCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSCluster_HealthMetric struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSCluster_HealthMetric {

/// 
    #[serde(rename = "MetricId")]
    pub metric_id: Option<String>,

/// 
    #[serde(rename = "Records")]
    pub records: Vec<MSCluster_MetricRecord>,
}

impl MSCluster_HealthMetric {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            metric_id: None,
            records: Vec::new(),
        }
    }


    /// Sets the value of MetricId
    pub fn set_metric_id(&mut self, value: String) {
        self.metric_id = Some(value);
    }

    /// Gets the value of MetricId
    pub fn get_metric_id(&self) -> Option<&String> {
        self.metric_id.as_ref()
    }

    /// Sets the value of Records
    pub fn set_records(&mut self, value: Vec<MSCluster_MetricRecord>) {
        self.records = value;
    }

    /// Gets the value of Records
    pub fn get_records(&self) -> &Vec<MSCluster_MetricRecord> {
        &self.records
    }
}

