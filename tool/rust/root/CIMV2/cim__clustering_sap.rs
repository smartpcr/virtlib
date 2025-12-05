// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ClusteringSAP struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ClusteringSAP {
    #[serde(flatten)]
    pub base: CIM_ServiceAccessPoint,
}

impl CIM_ClusteringSAP {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ServiceAccessPoint::new(),
        }
    }

}

