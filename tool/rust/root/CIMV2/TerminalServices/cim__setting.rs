// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.TerminalServices
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_Setting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_Setting {
    #[serde(flatten)]
    pub base: CIM_ManagedSystemElement,
}

impl CIM_Setting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ManagedSystemElement::new(),
        }
    }

}

