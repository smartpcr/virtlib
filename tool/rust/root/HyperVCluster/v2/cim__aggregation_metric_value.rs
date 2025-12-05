// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_AggregationMetricValue struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_AggregationMetricValue {
    #[serde(flatten)]
    pub base: CIM_BaseMetricValue,

/// Property that represents the time duration over which the aggregation was computed. The start of a monitoring interval over which the aggregation function is applied is determined by subtracting the AggregationDuration from the AggregationTimestamp.
    #[serde(rename = "AggregationDuration")]
    pub aggregation_duration: Option<String>,

/// Identifies the time when the aggregation function was applied to determine the value of the metric instance. Note that this is different from the time when the instance is created. For a given CIM_AggregationMetricValue instance, the AggregationTimeStamp changes whenever the aggregation function is applied to calculate the value.
    #[serde(rename = "AggregationTimeStamp")]
    pub aggregation_time_stamp: Option<String>,
}

impl CIM_AggregationMetricValue {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_BaseMetricValue::new(),
            aggregation_duration: None,
            aggregation_time_stamp: None,
        }
    }


    /// Sets the value of AggregationDuration
    pub fn set_aggregation_duration(&mut self, value: String) {
        self.aggregation_duration = Some(value);
    }

    /// Gets the value of AggregationDuration
    pub fn get_aggregation_duration(&self) -> Option<&String> {
        self.aggregation_duration.as_ref()
    }

    /// Sets the value of AggregationTimeStamp
    pub fn set_aggregation_time_stamp(&mut self, value: String) {
        self.aggregation_time_stamp = Some(value);
    }

    /// Gets the value of AggregationTimeStamp
    pub fn get_aggregation_time_stamp(&self) -> Option<&String> {
        self.aggregation_time_stamp.as_ref()
    }
}

