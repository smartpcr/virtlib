// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Cluster.Scaleout
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_ClusterSetNodeToVM struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_ClusterSetNodeToVM {
    #[serde(flatten)]
    pub base: CIM_Component,
}

impl MSFT_ClusterSetNodeToVM {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Component::new(),
        }
    }

}

