// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_MetricServiceCapabilities struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_MetricServiceCapabilities {
    #[serde(flatten)]
    pub base: CIM_MetricServiceCapabilities,
}

impl Msvm_MetricServiceCapabilities {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_MetricServiceCapabilities::new(),
        }
    }

}

impl Msvm_MetricServiceCapabilities {
    /// Gets the related Msvm_MetricService object(s)
    pub fn get_related__metric_service(&self) -> Result<Msvm_MetricService, WmiError> {
        self.get_related("Msvm_MetricService")
    }

}

