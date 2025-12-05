// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.MSCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSCluster_NativeUpdateManager struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSCluster_NativeUpdateManager {
}

impl MSCluster_NativeUpdateManager {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// 

    /// * `return_value` -  (u32)
    pub fn enable_cluster_native_update(&self) -> Result<(), WmiError> {
        self.invoke_method("EnableClusterNativeUpdate", &[])

    }


/// 

    /// * `return_value` -  (u32)
    pub fn disable_cluster_native_update(&self) -> Result<(), WmiError> {
        self.invoke_method("DisableClusterNativeUpdate", &[])

    }

}

