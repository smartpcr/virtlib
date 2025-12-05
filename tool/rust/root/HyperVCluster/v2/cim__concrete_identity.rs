// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ConcreteIdentity struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ConcreteIdentity {
    #[serde(flatten)]
    pub base: CIM_LogicalIdentity,
}

impl CIM_ConcreteIdentity {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalIdentity::new(),
        }
    }

}

