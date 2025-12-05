// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.SecurityClient
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_ProviderEx struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_ProviderEx {
    #[serde(flatten)]
    pub base: __Win32Provider,
}

impl Win32_ProviderEx {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __Win32Provider::new(),
        }
    }

}

