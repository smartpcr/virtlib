// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.MSCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSCluster_ResourceGroupToResource struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSCluster_ResourceGroupToResource {
    #[serde(flatten)]
    pub base: CIM_Component,
}

impl MSCluster_ResourceGroupToResource {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Component::new(),
        }
    }

}

