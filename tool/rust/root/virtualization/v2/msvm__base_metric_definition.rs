// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_BaseMetricDefinition struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_BaseMetricDefinition {
    #[serde(flatten)]
    pub base: CIM_BaseMetricDefinition,
}

impl Msvm_BaseMetricDefinition {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_BaseMetricDefinition::new(),
        }
    }

}

impl Msvm_BaseMetricDefinition {
    /// Gets the related Msvm_BaseMetricDefinition object(s)
    pub fn get_related__base_metric_definition(&self) -> Result<Msvm_BaseMetricDefinition, WmiError> {
        self.get_related("Msvm_BaseMetricDefinition")
    }

    /// Gets the related Msvm_MetricService object(s)
    pub fn get_related__metric_service(&self) -> Result<Msvm_MetricService, WmiError> {
        self.get_related("Msvm_MetricService")
    }

}

