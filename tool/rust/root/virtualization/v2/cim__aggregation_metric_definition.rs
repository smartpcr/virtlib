// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_AggregationMetricDefinition struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_AggregationMetricDefinition {
    #[serde(flatten)]
    pub base: CIM_BaseMetricDefinition,

/// The SimpleFunction property identifies the basic computation performed on an underlying metric to arrive at the value of this derived metric. This property shall be NULL when the ChangeType property has a value other than 5 "Simple Function". 
/// 2="Minimum" indicates that the metric reports the lowest value detected for the associated monitored entity. This is also known as a low watermark.
/// .3="Maximum" indicates that the metric reports the maximum value detected for the associated monitored entity. This is also known as a high watermark.
/// 4="Average" indicates the metric reports the average value of the underlying metric values.
/// 5="Median" indicates the metric reports the median value of the underlying metric values.
/// 6="Mode" indicates the metric reports the modal value of the underlying metric values.
    #[serde(rename = "SimpleFunction")]
    pub simple_function: Option<AggregationMetricDefinition_SimpleFunction>,
}

impl CIM_AggregationMetricDefinition {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_BaseMetricDefinition::new(),
            simple_function: None,
        }
    }


    /// Sets the value of SimpleFunction
    pub fn set_simple_function(&mut self, value: AggregationMetricDefinition_SimpleFunction) {
        self.simple_function = Some(value);
    }

    /// Gets the value of SimpleFunction
    pub fn get_simple_function(&self) -> Option<&AggregationMetricDefinition_SimpleFunction> {
        self.simple_function.as_ref()
    }
}

