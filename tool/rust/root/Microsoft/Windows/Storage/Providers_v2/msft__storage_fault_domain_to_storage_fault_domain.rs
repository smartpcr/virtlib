// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage.Providers_v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageFaultDomainToStorageFaultDomain struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageFaultDomainToStorageFaultDomain {

/// 
    #[serde(rename = "SourceStorageFaultDomain")]
    pub source_storage_fault_domain: Option<MSFT_StorageFaultDomain>,

/// 
    #[serde(rename = "TargetStorageFaultDomain")]
    pub target_storage_fault_domain: Option<MSFT_StorageFaultDomain>,
}

impl MSFT_StorageFaultDomainToStorageFaultDomain {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            source_storage_fault_domain: None,
            target_storage_fault_domain: None,
        }
    }


    /// Sets the value of SourceStorageFaultDomain
    pub fn set_source_storage_fault_domain(&mut self, value: MSFT_StorageFaultDomain) {
        self.source_storage_fault_domain = Some(value);
    }

    /// Gets the value of SourceStorageFaultDomain
    pub fn get_source_storage_fault_domain(&self) -> Option<&MSFT_StorageFaultDomain> {
        self.source_storage_fault_domain.as_ref()
    }

    /// Sets the value of TargetStorageFaultDomain
    pub fn set_target_storage_fault_domain(&mut self, value: MSFT_StorageFaultDomain) {
        self.target_storage_fault_domain = Some(value);
    }

    /// Gets the value of TargetStorageFaultDomain
    pub fn get_target_storage_fault_domain(&self) -> Option<&MSFT_StorageFaultDomain> {
        self.target_storage_fault_domain.as_ref()
    }
}

