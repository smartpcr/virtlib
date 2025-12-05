// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_HostedBootSAP struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_HostedBootSAP {
    #[serde(flatten)]
    pub base: CIM_HostedAccessPoint,
}

impl CIM_HostedBootSAP {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_HostedAccessPoint::new(),
        }
    }

}

