// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PCMCIAController struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PCMCIAController {
    #[serde(flatten)]
    pub base: CIM_PCMCIAController,
}

impl Win32_PCMCIAController {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_PCMCIAController::new(),
        }
    }

}

