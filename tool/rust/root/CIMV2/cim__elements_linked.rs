// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ElementsLinked struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ElementsLinked {
    #[serde(flatten)]
    pub base: CIM_Dependency,
}

impl CIM_ElementsLinked {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Dependency::new(),
        }
    }

}

