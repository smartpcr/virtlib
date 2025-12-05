// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetIKEQMCryptoProposal struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetIKEQMCryptoProposal {
    #[serde(flatten)]
    pub base: MSFT_NetIKECryptoProposal,

/// 
    #[serde(rename = "Encapsulation")]
    pub encapsulation: Option<u16>,

/// 
    #[serde(rename = "HashAlgorithmAH")]
    pub hash_algorithm_ah: Option<u16>,

/// 
    #[serde(rename = "HashAlgorithmESP")]
    pub hash_algorithm_esp: Option<u16>,

/// 
    #[serde(rename = "MaxLifetimeMinutes")]
    pub max_lifetime_minutes: Option<u32>,
}

impl MSFT_NetIKEQMCryptoProposal {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetIKECryptoProposal::new(),
            encapsulation: None,
            hash_algorithm_ah: None,
            hash_algorithm_esp: None,
            max_lifetime_minutes: None,
        }
    }


    /// Sets the value of Encapsulation
    pub fn set_encapsulation(&mut self, value: u16) {
        self.encapsulation = Some(value);
    }

    /// Gets the value of Encapsulation
    pub fn get_encapsulation(&self) -> Option<&u16> {
        self.encapsulation.as_ref()
    }

    /// Sets the value of HashAlgorithmAH
    pub fn set_hash_algorithm_ah(&mut self, value: u16) {
        self.hash_algorithm_ah = Some(value);
    }

    /// Gets the value of HashAlgorithmAH
    pub fn get_hash_algorithm_ah(&self) -> Option<&u16> {
        self.hash_algorithm_ah.as_ref()
    }

    /// Sets the value of HashAlgorithmESP
    pub fn set_hash_algorithm_esp(&mut self, value: u16) {
        self.hash_algorithm_esp = Some(value);
    }

    /// Gets the value of HashAlgorithmESP
    pub fn get_hash_algorithm_esp(&self) -> Option<&u16> {
        self.hash_algorithm_esp.as_ref()
    }

    /// Sets the value of MaxLifetimeMinutes
    pub fn set_max_lifetime_minutes(&mut self, value: u32) {
        self.max_lifetime_minutes = Some(value);
    }

    /// Gets the value of MaxLifetimeMinutes
    pub fn get_max_lifetime_minutes(&self) -> Option<&u32> {
        self.max_lifetime_minutes.as_ref()
    }
}

