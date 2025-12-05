// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_BaseMetricValue struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_BaseMetricValue {
    #[serde(flatten)]
    pub base: CIM_BaseMetricValue,
}

impl Msvm_BaseMetricValue {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_BaseMetricValue::new(),
        }
    }

}

