// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage.Providers_v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageSubSystemToInitiatorId struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageSubSystemToInitiatorId {

/// 
    #[serde(rename = "InitiatorId")]
    pub initiator_id: Option<MSFT_InitiatorId>,

/// 
    #[serde(rename = "StorageSubSystem")]
    pub storage_sub_system: Option<MSFT_StorageSubSystem>,
}

impl MSFT_StorageSubSystemToInitiatorId {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            initiator_id: None,
            storage_sub_system: None,
        }
    }


    /// Sets the value of InitiatorId
    pub fn set_initiator_id(&mut self, value: MSFT_InitiatorId) {
        self.initiator_id = Some(value);
    }

    /// Gets the value of InitiatorId
    pub fn get_initiator_id(&self) -> Option<&MSFT_InitiatorId> {
        self.initiator_id.as_ref()
    }

    /// Sets the value of StorageSubSystem
    pub fn set_storage_sub_system(&mut self, value: MSFT_StorageSubSystem) {
        self.storage_sub_system = Some(value);
    }

    /// Gets the value of StorageSubSystem
    pub fn get_storage_sub_system(&self) -> Option<&MSFT_StorageSubSystem> {
        self.storage_sub_system.as_ref()
    }
}

