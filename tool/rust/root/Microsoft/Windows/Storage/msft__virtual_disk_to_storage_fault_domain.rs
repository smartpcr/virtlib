// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_VirtualDiskToStorageFaultDomain struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_VirtualDiskToStorageFaultDomain {

/// 
    #[serde(rename = "StorageFaultDomain")]
    pub storage_fault_domain: Option<MSFT_StorageFaultDomain>,

/// 
    #[serde(rename = "VirtualDisk")]
    pub virtual_disk: Option<MSFT_VirtualDisk>,
}

impl MSFT_VirtualDiskToStorageFaultDomain {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            storage_fault_domain: None,
            virtual_disk: None,
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

    /// Sets the value of VirtualDisk
    pub fn set_virtual_disk(&mut self, value: MSFT_VirtualDisk) {
        self.virtual_disk = Some(value);
    }

    /// Gets the value of VirtualDisk
    pub fn get_virtual_disk(&self) -> Option<&MSFT_VirtualDisk> {
        self.virtual_disk.as_ref()
    }
}

