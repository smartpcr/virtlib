// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageSubSystemToStorageFaultDomain struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageSubSystemToStorageFaultDomain {

/// 
    #[serde(rename = "StorageFaultDomain")]
    pub storage_fault_domain: Option<MSFT_StorageFaultDomain>,

/// 
    #[serde(rename = "StorageSubSystem")]
    pub storage_sub_system: Option<MSFT_StorageSubSystem>,
}

impl MSFT_StorageSubSystemToStorageFaultDomain {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            storage_fault_domain: None,
            storage_sub_system: None,
        }
    }


    /// Sets the value of StorageFaultDomain
    pub fn set_storage_fault_domain(&mut self, value: MSFT_StorageFaultDomain) {
        self.storage_fault_domain = Some(value);
    }

    /// Gets the value of StorageFaultDomain
    pub fn get_storage_fault_domain(&self) -> Option<&MSFT_StorageFaultDomain> {
        self.storage_fault_domain.as_ref()
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

