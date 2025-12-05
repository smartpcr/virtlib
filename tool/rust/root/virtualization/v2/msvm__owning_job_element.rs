// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_OwningJobElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_OwningJobElement {
    #[serde(flatten)]
    pub base: CIM_OwningJobElement,
}

impl Msvm_OwningJobElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_OwningJobElement::new(),
        }
    }

}

