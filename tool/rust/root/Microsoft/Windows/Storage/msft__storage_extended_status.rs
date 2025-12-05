// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageExtendedStatus struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageExtendedStatus {
    #[serde(flatten)]
    pub base: CIM_Error,
}

impl MSFT_StorageExtendedStatus {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Error::new(),
        }
    }

}

