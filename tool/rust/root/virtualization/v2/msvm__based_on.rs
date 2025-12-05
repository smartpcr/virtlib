// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_BasedOn struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_BasedOn {
    #[serde(flatten)]
    pub base: CIM_BasedOn,
}

impl Msvm_BasedOn {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_BasedOn::new(),
        }
    }

}

