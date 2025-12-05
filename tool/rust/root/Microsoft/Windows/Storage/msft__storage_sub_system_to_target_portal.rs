// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageSubSystemToTargetPortal struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageSubSystemToTargetPortal {

/// 
    #[serde(rename = "StorageSubSystem")]
    pub storage_sub_system: Option<MSFT_StorageSubSystem>,

/// 
    #[serde(rename = "TargetPortal")]
    pub target_portal: Option<MSFT_TargetPortal>,
}

impl MSFT_StorageSubSystemToTargetPortal {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            storage_sub_system: None,
            target_portal: None,
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

    /// Sets the value of TargetPortal
    pub fn set_target_portal(&mut self, value: MSFT_TargetPortal) {
        self.target_portal = Some(value);
    }

    /// Gets the value of TargetPortal
    pub fn get_target_portal(&self) -> Option<&MSFT_TargetPortal> {
        self.target_portal.as_ref()
    }
}

