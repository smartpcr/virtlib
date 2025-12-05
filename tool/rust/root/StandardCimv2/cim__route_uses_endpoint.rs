// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_RouteUsesEndpoint struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_RouteUsesEndpoint {
    #[serde(flatten)]
    pub base: CIM_Dependency,
}

impl CIM_RouteUsesEndpoint {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Dependency::new(),
        }
    }

}

