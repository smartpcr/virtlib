// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_SCSIProtocolController struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_SCSIProtocolController {
    #[serde(flatten)]
    pub base: CIM_SCSIProtocolController,
}

impl Msvm_SCSIProtocolController {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SCSIProtocolController::new(),
        }
    }

}

