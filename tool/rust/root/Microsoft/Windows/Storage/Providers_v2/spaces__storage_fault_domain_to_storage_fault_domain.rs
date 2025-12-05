// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage.Providers_v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SPACES_StorageFaultDomainToStorageFaultDomain struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SPACES_StorageFaultDomainToStorageFaultDomain {
    #[serde(flatten)]
    pub base: MSFT_StorageFaultDomainToStorageFaultDomain,
}

impl SPACES_StorageFaultDomainToStorageFaultDomain {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_StorageFaultDomainToStorageFaultDomain::new(),
        }
    }

}

