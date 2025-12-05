// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_Synthetic3DDisplayController struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_Synthetic3DDisplayController {
    #[serde(flatten)]
    pub base: CIM_DisplayController,

/// 
    #[serde(rename = "AllocatedGPU")]
    pub allocated_gpu: Option<String>,
}

impl Msvm_Synthetic3DDisplayController {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_DisplayController::new(),
            allocated_gpu: None,
        }
    }


    /// Sets the value of AllocatedGPU
    pub fn set_allocated_gpu(&mut self, value: String) {
        self.allocated_gpu = Some(value);
    }

    /// Gets the value of AllocatedGPU
    pub fn get_allocated_gpu(&self) -> Option<&String> {
        self.allocated_gpu.as_ref()
    }
}

