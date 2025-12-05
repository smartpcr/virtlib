// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetIKEKerbAuthProposal struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetIKEKerbAuthProposal {
    #[serde(flatten)]
    pub base: MSFT_NetIKEAuthProposal,

/// 
    #[serde(rename = "KerbProxy")]
    pub kerb_proxy: Option<String>,
}

impl MSFT_NetIKEKerbAuthProposal {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetIKEAuthProposal::new(),
            kerb_proxy: None,
        }
    }


    /// Sets the value of KerbProxy
    pub fn set_kerb_proxy(&mut self, value: String) {
        self.kerb_proxy = Some(value);
    }

    /// Gets the value of KerbProxy
    pub fn get_kerb_proxy(&self) -> Option<&String> {
        self.kerb_proxy.as_ref()
    }
}

