// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_WinNatCounters_WinNATICMP struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_WinNatCounters_WinNATICMP {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "NumberOfBindings")]
    pub number_of_bindings: Option<u32>,

/// 
    #[serde(rename = "NumberOfSessions")]
    pub number_of_sessions: Option<u32>,

/// 
    #[serde(rename = "NumExtToIntTranslations")]
    pub num_ext_to_int_translations: Option<u32>,

/// 
    #[serde(rename = "NumIntToExtTranslations")]
    pub num_int_to_ext_translations: Option<u32>,

/// 
    #[serde(rename = "NumPacketsDropped")]
    pub num_packets_dropped: Option<u32>,

/// 
    #[serde(rename = "NumSessionsTimedOut")]
    pub num_sessions_timed_out: Option<u32>,
}

impl Win32_PerfFormattedData_WinNatCounters_WinNATICMP {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            number_of_bindings: None,
            number_of_sessions: None,
            num_ext_to_int_translations: None,
            num_int_to_ext_translations: None,
            num_packets_dropped: None,
            num_sessions_timed_out: None,
        }
    }


    /// Sets the value of NumberOfBindings
    pub fn set_number_of_bindings(&mut self, value: u32) {
        self.number_of_bindings = Some(value);
    }

    /// Gets the value of NumberOfBindings
    pub fn get_number_of_bindings(&self) -> Option<&u32> {
        self.number_of_bindings.as_ref()
    }

    /// Sets the value of NumberOfSessions
    pub fn set_number_of_sessions(&mut self, value: u32) {
        self.number_of_sessions = Some(value);
    }

    /// Gets the value of NumberOfSessions
    pub fn get_number_of_sessions(&self) -> Option<&u32> {
        self.number_of_sessions.as_ref()
    }

    /// Sets the value of NumExtToIntTranslations
    pub fn set_num_ext_to_int_translations(&mut self, value: u32) {
        self.num_ext_to_int_translations = Some(value);
    }

    /// Gets the value of NumExtToIntTranslations
    pub fn get_num_ext_to_int_translations(&self) -> Option<&u32> {
        self.num_ext_to_int_translations.as_ref()
    }

    /// Sets the value of NumIntToExtTranslations
    pub fn set_num_int_to_ext_translations(&mut self, value: u32) {
        self.num_int_to_ext_translations = Some(value);
    }

    /// Gets the value of NumIntToExtTranslations
    pub fn get_num_int_to_ext_translations(&self) -> Option<&u32> {
        self.num_int_to_ext_translations.as_ref()
    }

    /// Sets the value of NumPacketsDropped
    pub fn set_num_packets_dropped(&mut self, value: u32) {
        self.num_packets_dropped = Some(value);
    }

    /// Gets the value of NumPacketsDropped
    pub fn get_num_packets_dropped(&self) -> Option<&u32> {
        self.num_packets_dropped.as_ref()
    }

    /// Sets the value of NumSessionsTimedOut
    pub fn set_num_sessions_timed_out(&mut self, value: u32) {
        self.num_sessions_timed_out = Some(value);
    }

    /// Gets the value of NumSessionsTimedOut
    pub fn get_num_sessions_timed_out(&self) -> Option<&u32> {
        self.num_sessions_timed_out.as_ref()
    }
}

