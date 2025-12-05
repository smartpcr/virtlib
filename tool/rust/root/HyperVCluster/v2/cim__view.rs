// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_View struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_View {
    #[serde(flatten)]
    pub base: CIM_ManagedElement,
}

impl CIM_View {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ManagedElement::new(),
        }
    }

}

