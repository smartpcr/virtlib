// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageSubSystemToTargetPort struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageSubSystemToTargetPort {

/// 
    #[serde(rename = "StorageSubSystem")]
    pub storage_sub_system: Option<MSFT_StorageSubSystem>,

/// 
    #[serde(rename = "TargetPort")]
    pub target_port: Option<MSFT_TargetPort>,
}

impl MSFT_StorageSubSystemToTargetPort {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            storage_sub_system: None,
            target_port: None,
        }
    }


    /// Sets the value of StorageSubSystem
    pub fn set_storage_sub_system(&mut self, value: MSFT_StorageSubSystem) {
        self.storage_sub_system = Some(value);
    }

    /// Gets the value of StorageSubSystem
    pub fn get_storage_sub_system(&self) -> Option<&MSFT_StorageSubSystem> {
        self.storage_sub_system.as_ref()
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

