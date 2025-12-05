// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_SystemResource struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_SystemResource {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,
}

impl CIM_SystemResource {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
        }
    }

}

