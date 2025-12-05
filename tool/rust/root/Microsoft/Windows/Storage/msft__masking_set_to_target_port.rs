// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_MaskingSetToTargetPort struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_MaskingSetToTargetPort {

/// 
    #[serde(rename = "MaskingSet")]
    pub masking_set: Option<MSFT_MaskingSet>,

/// 
    #[serde(rename = "TargetPort")]
    pub target_port: Option<MSFT_TargetPort>,
}

impl MSFT_MaskingSetToTargetPort {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            masking_set: None,
            target_port: None,
        }
    }


    /// Sets the value of MaskingSet
    pub fn set_masking_set(&mut self, value: MSFT_MaskingSet) {
        self.masking_set = Some(value);
    }

    /// Gets the value of MaskingSet
    pub fn get_masking_set(&self) -> Option<&MSFT_MaskingSet> {
        self.masking_set.as_ref()
    }

    /// Sets the value of TargetPort
    pub fn set_target_port(&mut self, value: MSFT_TargetPort) {
        self.target_port = Some(value);
    }

    /// Gets the value of TargetPort
    pub fn get_target_port(&self) -> Option<&MSFT_TargetPort> {
        self.target_port.as_ref()
    }
}

