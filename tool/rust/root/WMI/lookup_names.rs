// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// LookupNames struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LookupNames {
    #[serde(flatten)]
    pub base: MSLSATrace,
}

impl LookupNames {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSLSATrace::new(),
        }
    }

}

