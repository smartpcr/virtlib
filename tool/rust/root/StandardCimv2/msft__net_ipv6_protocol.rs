// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetIPv6Protocol struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetIPv6Protocol {
    #[serde(flatten)]
    pub base: MSFT_NetBaseIPProtocol,

/// 
    #[serde(rename = "MaxDadAttempts")]
    pub max_dad_attempts: Option<u32>,

/// 
    #[serde(rename = "MaxPreferredLifetime")]
    pub max_preferred_lifetime: Option<String>,

/// 
    #[serde(rename = "MaxRandomTime")]
    pub max_random_time: Option<String>,

/// 
    #[serde(rename = "MaxValidLifetime")]
    pub max_valid_lifetime: Option<String>,

/// 
    #[serde(rename = "RegenerateTime")]
    pub regenerate_time: Option<String>,

/// 
    #[serde(rename = "UseTemporaryAddresses")]
    pub use_temporary_addresses: Option<u32>,
}

impl MSFT_NetIPv6Protocol {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetBaseIPProtocol::new(),
            max_dad_attempts: None,
            max_preferred_lifetime: None,
            max_random_time: None,
            max_valid_lifetime: None,
            regenerate_time: None,
            use_temporary_addresses: None,
        }
    }


    /// Sets the value of MaxDadAttempts
    pub fn set_max_dad_attempts(&mut self, value: u32) {
        self.max_dad_attempts = Some(value);
    }

    /// Gets the value of MaxDadAttempts
    pub fn get_max_dad_attempts(&self) -> Option<&u32> {
        self.max_dad_attempts.as_ref()
    }

    /// Sets the value of MaxPreferredLifetime
    pub fn set_max_preferred_lifetime(&mut self, value: String) {
        self.max_preferred_lifetime = Some(value);
    }

    /// Gets the value of MaxPreferredLifetime
    pub fn get_max_preferred_lifetime(&self) -> Option<&String> {
        self.max_preferred_lifetime.as_ref()
    }

    /// Sets the value of MaxRandomTime
    pub fn set_max_random_time(&mut self, value: String) {
        self.max_random_time = Some(value);
    }

    /// Gets the value of MaxRandomTime
    pub fn get_max_random_time(&self) -> Option<&String> {
        self.max_random_time.as_ref()
    }

    /// Sets the value of MaxValidLifetime
    pub fn set_max_valid_lifetime(&mut self, value: String) {
        self.max_valid_lifetime = Some(value);
    }

    /// Gets the value of MaxValidLifetime
    pub fn get_max_valid_lifetime(&self) -> Option<&String> {
        self.max_valid_lifetime.as_ref()
    }

    /// Sets the value of RegenerateTime
    pub fn set_regenerate_time(&mut self, value: String) {
        self.regenerate_time = Some(value);
    }

    /// Gets the value of RegenerateTime
    pub fn get_regenerate_time(&self) -> Option<&String> {
        self.regenerate_time.as_ref()
    }

    /// Sets the value of UseTemporaryAddresses
    pub fn set_use_temporary_addresses(&mut self, value: u32) {
        self.use_temporary_addresses = Some(value);
    }

    /// Gets the value of UseTemporaryAddresses
    pub fn get_use_temporary_addresses(&self) -> Option<&u32> {
        self.use_temporary_addresses.as_ref()
    }
}

