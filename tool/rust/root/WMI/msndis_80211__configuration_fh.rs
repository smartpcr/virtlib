// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_80211_ConfigurationFH struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_80211_ConfigurationFH {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "DwellTime")]
    pub dwell_time: Option<u32>,

/// 
    #[serde(rename = "FHLength")]
    pub fhlength: Option<u32>,

/// 
    #[serde(rename = "HopPattern")]
    pub hop_pattern: Option<u32>,

/// 
    #[serde(rename = "HopSet")]
    pub hop_set: Option<u32>,
}

impl MSNdis_80211_ConfigurationFH {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            dwell_time: None,
            fhlength: None,
            hop_pattern: None,
            hop_set: None,
        }
    }


    /// Sets the value of DwellTime
    pub fn set_dwell_time(&mut self, value: u32) {
        self.dwell_time = Some(value);
    }

    /// Gets the value of DwellTime
    pub fn get_dwell_time(&self) -> Option<&u32> {
        self.dwell_time.as_ref()
    }

    /// Sets the value of FHLength
    pub fn set_fhlength(&mut self, value: u32) {
        self.fhlength = Some(value);
    }

    /// Gets the value of FHLength
    pub fn get_fhlength(&self) -> Option<&u32> {
        self.fhlength.as_ref()
    }

    /// Sets the value of HopPattern
    pub fn set_hop_pattern(&mut self, value: u32) {
        self.hop_pattern = Some(value);
    }

    /// Gets the value of HopPattern
    pub fn get_hop_pattern(&self) -> Option<&u32> {
        self.hop_pattern.as_ref()
    }

    /// Sets the value of HopSet
    pub fn set_hop_set(&mut self, value: u32) {
        self.hop_set = Some(value);
    }

    /// Gets the value of HopSet
    pub fn get_hop_set(&self) -> Option<&u32> {
        self.hop_set.as_ref()
    }
}

