// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetIKECryptoProposal struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetIKECryptoProposal {
    #[serde(flatten)]
    pub base: CIM_IKEProposal,
}

impl MSFT_NetIKECryptoProposal {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_IKEProposal::new(),
        }
    }

}

