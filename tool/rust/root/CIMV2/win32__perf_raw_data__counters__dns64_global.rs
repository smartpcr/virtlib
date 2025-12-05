// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_Counters_DNS64Global struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_Counters_DNS64Global {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "AAAAqueriesFailed")]
    pub aaaaqueries_failed: Option<u64>,

/// 
    #[serde(rename = "AAAAqueriesSuccessful")]
    pub aaaaqueries_successful: Option<u64>,

/// 
    #[serde(rename = "AAAASynthesizedrecords")]
    pub aaaasynthesizedrecords: Option<u64>,

/// 
    #[serde(rename = "IP6ARPAqueriesMatched")]
    pub ip6_arpaqueries_matched: Option<u64>,

/// 
    #[serde(rename = "OtherqueriesFailed")]
    pub otherqueries_failed: Option<u64>,

/// 
    #[serde(rename = "OtherqueriesSuccessful")]
    pub otherqueries_successful: Option<u64>,
}

impl Win32_PerfRawData_Counters_DNS64Global {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            aaaaqueries_failed: None,
            aaaaqueries_successful: None,
            aaaasynthesizedrecords: None,
            ip6_arpaqueries_matched: None,
            otherqueries_failed: None,
            otherqueries_successful: None,
        }
    }


    /// Sets the value of AAAAqueriesFailed
    pub fn set_aaaaqueries_failed(&mut self, value: u64) {
        self.aaaaqueries_failed = Some(value);
    }

    /// Gets the value of AAAAqueriesFailed
    pub fn get_aaaaqueries_failed(&self) -> Option<&u64> {
        self.aaaaqueries_failed.as_ref()
    }

    /// Sets the value of AAAAqueriesSuccessful
    pub fn set_aaaaqueries_successful(&mut self, value: u64) {
        self.aaaaqueries_successful = Some(value);
    }

    /// Gets the value of AAAAqueriesSuccessful
    pub fn get_aaaaqueries_successful(&self) -> Option<&u64> {
        self.aaaaqueries_successful.as_ref()
    }

    /// Sets the value of AAAASynthesizedrecords
    pub fn set_aaaasynthesizedrecords(&mut self, value: u64) {
        self.aaaasynthesizedrecords = Some(value);
    }

    /// Gets the value of AAAASynthesizedrecords
    pub fn get_aaaasynthesizedrecords(&self) -> Option<&u64> {
        self.aaaasynthesizedrecords.as_ref()
    }

    /// Sets the value of IP6ARPAqueriesMatched
    pub fn set_ip6_arpaqueries_matched(&mut self, value: u64) {
        self.ip6_arpaqueries_matched = Some(value);
    }

    /// Gets the value of IP6ARPAqueriesMatched
    pub fn get_ip6_arpaqueries_matched(&self) -> Option<&u64> {
        self.ip6_arpaqueries_matched.as_ref()
    }

    /// Sets the value of OtherqueriesFailed
    pub fn set_otherqueries_failed(&mut self, value: u64) {
        self.otherqueries_failed = Some(value);
    }

    /// Gets the value of OtherqueriesFailed
    pub fn get_otherqueries_failed(&self) -> Option<&u64> {
        self.otherqueries_failed.as_ref()
    }

    /// Sets the value of OtherqueriesSuccessful
    pub fn set_otherqueries_successful(&mut self, value: u64) {
        self.otherqueries_successful = Some(value);
    }

    /// Gets the value of OtherqueriesSuccessful
    pub fn get_otherqueries_successful(&self) -> Option<&u64> {
        self.otherqueries_successful.as_ref()
    }
}

