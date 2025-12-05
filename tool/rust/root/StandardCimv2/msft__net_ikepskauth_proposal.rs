// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetIKEPSKAuthProposal struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetIKEPSKAuthProposal {
    #[serde(flatten)]
    pub base: MSFT_NetIKEAuthProposal,

/// 
    #[serde(rename = "PreSharedKey")]
    pub pre_shared_key: Option<String>,
}

impl MSFT_NetIKEPSKAuthProposal {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetIKEAuthProposal::new(),
            pre_shared_key: None,
        }
    }


    /// Sets the value of PreSharedKey
    pub fn set_pre_shared_key(&mut self, value: String) {
        self.pre_shared_key = Some(value);
    }

    /// Gets the value of PreSharedKey
    pub fn get_pre_shared_key(&self) -> Option<&String> {
        self.pre_shared_key.as_ref()
    }
}

