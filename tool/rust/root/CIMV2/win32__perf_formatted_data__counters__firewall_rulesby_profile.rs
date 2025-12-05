// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_Counters_FirewallRulesbyProfile struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_Counters_FirewallRulesbyProfile {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "DomainProfile")]
    pub domain_profile: Option<u64>,

/// 
    #[serde(rename = "PrivateProfile")]
    pub private_profile: Option<u64>,

/// 
    #[serde(rename = "PublicProfile")]
    pub public_profile: Option<u64>,
}

impl Win32_PerfFormattedData_Counters_FirewallRulesbyProfile {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            domain_profile: None,
            private_profile: None,
            public_profile: None,
        }
    }


    /// Sets the value of DomainProfile
    pub fn set_domain_profile(&mut self, value: u64) {
        self.domain_profile = Some(value);
    }

    /// Gets the value of DomainProfile
    pub fn get_domain_profile(&self) -> Option<&u64> {
        self.domain_profile.as_ref()
    }

    /// Sets the value of PrivateProfile
    pub fn set_private_profile(&mut self, value: u64) {
        self.private_profile = Some(value);
    }

    /// Gets the value of PrivateProfile
    pub fn get_private_profile(&self) -> Option<&u64> {
        self.private_profile.as_ref()
    }

    /// Sets the value of PublicProfile
    pub fn set_public_profile(&mut self, value: u64) {
        self.public_profile = Some(value);
    }

    /// Gets the value of PublicProfile
    pub fn get_public_profile(&self) -> Option<&u64> {
        self.public_profile.as_ref()
    }
}

