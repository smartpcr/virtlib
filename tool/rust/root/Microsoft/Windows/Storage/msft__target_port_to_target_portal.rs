// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_TargetPortToTargetPortal struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_TargetPortToTargetPortal {

/// 
    #[serde(rename = "TargetPort")]
    pub target_port: Option<MSFT_TargetPort>,

/// 
    #[serde(rename = "TargetPortal")]
    pub target_portal: Option<MSFT_TargetPortal>,
}

impl MSFT_TargetPortToTargetPortal {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            target_port: None,
            target_portal: None,
        }
    }


    /// Sets the value of TargetPort
    pub fn set_target_port(&mut self, value: MSFT_TargetPort) {
        self.target_port = Some(value);
    }

    /// Gets the value of TargetPort
    pub fn get_target_port(&self) -> Option<&MSFT_TargetPort> {
        self.target_port.as_ref()
    }

    /// Sets the value of TargetPortal
    pub fn set_target_portal(&mut self, value: MSFT_TargetPortal) {
        self.target_portal = Some(value);
    }

    /// Gets the value of TargetPortal
    pub fn get_target_portal(&self) -> Option<&MSFT_TargetPortal> {
        self.target_portal.as_ref()
    }
}

