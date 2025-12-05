// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_AggregationMetricDefinition struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_AggregationMetricDefinition {
    #[serde(flatten)]
    pub base: CIM_AggregationMetricDefinition,
}

impl Msvm_AggregationMetricDefinition {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_AggregationMetricDefinition::new(),
        }
    }

}

impl Msvm_AggregationMetricDefinition {
    /// Gets the related Msvm_MemorySettingData object(s)
    pub fn get_related__memory_setting_data(&self) -> Result<Vec<Msvm_MemorySettingData>, WmiError> {
        self.get_all_related("Msvm_MemorySettingData")
    }

    /// Gets the related Msvm_AggregationMetricDefinition object(s)
    pub fn get_related__aggregation_metric_definition(&self) -> Result<Msvm_AggregationMetricDefinition, WmiError> {
        self.get_related("Msvm_AggregationMetricDefinition")
    }

    /// Gets the related Msvm_MetricService object(s)
    pub fn get_related__metric_service(&self) -> Result<Msvm_MetricService, WmiError> {
        self.get_related("Msvm_MetricService")
    }

}

