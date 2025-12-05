// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CounterCorruptionStatus struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CounterCorruptionStatus {

/// 
    #[serde(rename = "LastKnownGoodTimestamp")]
    pub last_known_good_timestamp: Option<u64>,

/// 
    #[serde(rename = "ProfileSource")]
    pub profile_source: Option<u32>,
}

impl CounterCorruptionStatus {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            last_known_good_timestamp: None,
            profile_source: None,
        }
    }


    /// Sets the value of LastKnownGoodTimestamp
    pub fn set_last_known_good_timestamp(&mut self, value: u64) {
        self.last_known_good_timestamp = Some(value);
    }

    /// Gets the value of LastKnownGoodTimestamp
    pub fn get_last_known_good_timestamp(&self) -> Option<&u64> {
        self.last_known_good_timestamp.as_ref()
    }

    /// Sets the value of ProfileSource
    pub fn set_profile_source(&mut self, value: u32) {
        self.profile_source = Some(value);
    }

    /// Gets the value of ProfileSource
    pub fn get_profile_source(&self) -> Option<&u32> {
        self.profile_source.as_ref()
    }
}

