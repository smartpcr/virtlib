// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// LookupSids_End struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LookupSids_End {
    #[serde(flatten)]
    pub base: LookupSids,
}

impl LookupSids_End {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: LookupSids::new(),
        }
    }

}

