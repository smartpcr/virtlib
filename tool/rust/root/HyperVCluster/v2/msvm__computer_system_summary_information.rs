// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_ComputerSystemSummaryInformation struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_ComputerSystemSummaryInformation {
    #[serde(flatten)]
    pub base: CIM_ElementView,
}

impl Msvm_ComputerSystemSummaryInformation {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ElementView::new(),
        }
    }

}

