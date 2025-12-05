// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_NETFramework_NETCLRSecurity struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_NETFramework_NETCLRSecurity {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "NumberLinkTimeChecks")]
    pub number_link_time_checks: Option<u32>,

/// 
    #[serde(rename = "PercentTimeinRTchecks")]
    pub percent_timein_rtchecks: Option<u32>,

/// 
    #[serde(rename = "PercentTimeSigAuthenticating")]
    pub percent_time_sig_authenticating: Option<u64>,

/// 
    #[serde(rename = "StackWalkDepth")]
    pub stack_walk_depth: Option<u32>,

/// 
    #[serde(rename = "TotalRuntimeChecks")]
    pub total_runtime_checks: Option<u32>,
}

impl Win32_PerfFormattedData_NETFramework_NETCLRSecurity {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            number_link_time_checks: None,
            percent_timein_rtchecks: None,
            percent_time_sig_authenticating: None,
            stack_walk_depth: None,
            total_runtime_checks: None,
        }
    }


    /// Sets the value of NumberLinkTimeChecks
    pub fn set_number_link_time_checks(&mut self, value: u32) {
        self.number_link_time_checks = Some(value);
    }

    /// Gets the value of NumberLinkTimeChecks
    pub fn get_number_link_time_checks(&self) -> Option<&u32> {
        self.number_link_time_checks.as_ref()
    }

    /// Sets the value of PercentTimeinRTchecks
    pub fn set_percent_timein_rtchecks(&mut self, value: u32) {
        self.percent_timein_rtchecks = Some(value);
    }

    /// Gets the value of PercentTimeinRTchecks
    pub fn get_percent_timein_rtchecks(&self) -> Option<&u32> {
        self.percent_timein_rtchecks.as_ref()
    }

    /// Sets the value of PercentTimeSigAuthenticating
    pub fn set_percent_time_sig_authenticating(&mut self, value: u64) {
        self.percent_time_sig_authenticating = Some(value);
    }

    /// Gets the value of PercentTimeSigAuthenticating
    pub fn get_percent_time_sig_authenticating(&self) -> Option<&u64> {
        self.percent_time_sig_authenticating.as_ref()
    }

    /// Sets the value of StackWalkDepth
    pub fn set_stack_walk_depth(&mut self, value: u32) {
        self.stack_walk_depth = Some(value);
    }

    /// Gets the value of StackWalkDepth
    pub fn get_stack_walk_depth(&self) -> Option<&u32> {
        self.stack_walk_depth.as_ref()
    }

    /// Sets the value of TotalRuntimeChecks
    pub fn set_total_runtime_checks(&mut self, value: u32) {
        self.total_runtime_checks = Some(value);
    }

    /// Gets the value of TotalRuntimeChecks
    pub fn get_total_runtime_checks(&self) -> Option<&u32> {
        self.total_runtime_checks.as_ref()
    }
}

