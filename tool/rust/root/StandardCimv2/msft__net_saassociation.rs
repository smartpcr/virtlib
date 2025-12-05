// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetSAAssociation struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetSAAssociation {
    #[serde(flatten)]
    pub base: CIM_Phase1SAUsedForPhase2,
}

impl MSFT_NetSAAssociation {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Phase1SAUsedForPhase2::new(),
        }
    }

}

