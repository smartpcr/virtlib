// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_ServiceModel4000_ServiceModelOperation4000 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_ServiceModel4000_ServiceModelOperation4000 {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "CallFailedPerSecond")]
    pub call_failed_per_second: Option<u32>,

/// 
    #[serde(rename = "Calls")]
    pub calls: Option<u32>,

/// 
    #[serde(rename = "CallsDuration")]
    pub calls_duration: Option<u32>,

/// 
    #[serde(rename = "CallsFailed")]
    pub calls_failed: Option<u32>,

/// 
    #[serde(rename = "CallsFaulted")]
    pub calls_faulted: Option<u32>,

/// 
    #[serde(rename = "CallsFaultedPerSecond")]
    pub calls_faulted_per_second: Option<u32>,

/// 
    #[serde(rename = "CallsOutstanding")]
    pub calls_outstanding: Option<u32>,

/// 
    #[serde(rename = "CallsPerSecond")]
    pub calls_per_second: Option<u32>,

/// 
    #[serde(rename = "SecurityCallsNotAuthorized")]
    pub security_calls_not_authorized: Option<u32>,

/// 
    #[serde(rename = "SecurityCallsNotAuthorizedPerSecond")]
    pub security_calls_not_authorized_per_second: Option<u32>,

/// 
    #[serde(rename = "SecurityValidationandAuthenticationFailures")]
    pub security_validationand_authentication_failures: Option<u32>,

/// 
    #[serde(rename = "SecurityValidationandAuthenticationFailuresPerSecond")]
    pub security_validationand_authentication_failures_per_second: Option<u32>,

/// 
    #[serde(rename = "TransactionsFlowed")]
    pub transactions_flowed: Option<u32>,

/// 
    #[serde(rename = "TransactionsFlowedPerSecond")]
    pub transactions_flowed_per_second: Option<u32>,
}

impl Win32_PerfFormattedData_ServiceModel4000_ServiceModelOperation4000 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            call_failed_per_second: None,
            calls: None,
            calls_duration: None,
            calls_failed: None,
            calls_faulted: None,
            calls_faulted_per_second: None,
            calls_outstanding: None,
            calls_per_second: None,
            security_calls_not_authorized: None,
            security_calls_not_authorized_per_second: None,
            security_validationand_authentication_failures: None,
            security_validationand_authentication_failures_per_second: None,
            transactions_flowed: None,
            transactions_flowed_per_second: None,
        }
    }


    /// Sets the value of CallFailedPerSecond
    pub fn set_call_failed_per_second(&mut self, value: u32) {
        self.call_failed_per_second = Some(value);
    }

    /// Gets the value of CallFailedPerSecond
    pub fn get_call_failed_per_second(&self) -> Option<&u32> {
        self.call_failed_per_second.as_ref()
    }

    /// Sets the value of Calls
    pub fn set_calls(&mut self, value: u32) {
        self.calls = Some(value);
    }

    /// Gets the value of Calls
    pub fn get_calls(&self) -> Option<&u32> {
        self.calls.as_ref()
    }

    /// Sets the value of CallsDuration
    pub fn set_calls_duration(&mut self, value: u32) {
        self.calls_duration = Some(value);
    }

    /// Gets the value of CallsDuration
    pub fn get_calls_duration(&self) -> Option<&u32> {
        self.calls_duration.as_ref()
    }

    /// Sets the value of CallsFailed
    pub fn set_calls_failed(&mut self, value: u32) {
        self.calls_failed = Some(value);
    }

    /// Gets the value of CallsFailed
    pub fn get_calls_failed(&self) -> Option<&u32> {
        self.calls_failed.as_ref()
    }

    /// Sets the value of CallsFaulted
    pub fn set_calls_faulted(&mut self, value: u32) {
        self.calls_faulted = Some(value);
    }

    /// Gets the value of CallsFaulted
    pub fn get_calls_faulted(&self) -> Option<&u32> {
        self.calls_faulted.as_ref()
    }

    /// Sets the value of CallsFaultedPerSecond
    pub fn set_calls_faulted_per_second(&mut self, value: u32) {
        self.calls_faulted_per_second = Some(value);
    }

    /// Gets the value of CallsFaultedPerSecond
    pub fn get_calls_faulted_per_second(&self) -> Option<&u32> {
        self.calls_faulted_per_second.as_ref()
    }

    /// Sets the value of CallsOutstanding
    pub fn set_calls_outstanding(&mut self, value: u32) {
        self.calls_outstanding = Some(value);
    }

    /// Gets the value of CallsOutstanding
    pub fn get_calls_outstanding(&self) -> Option<&u32> {
        self.calls_outstanding.as_ref()
    }

    /// Sets the value of CallsPerSecond
    pub fn set_calls_per_second(&mut self, value: u32) {
        self.calls_per_second = Some(value);
    }

    /// Gets the value of CallsPerSecond
    pub fn get_calls_per_second(&self) -> Option<&u32> {
        self.calls_per_second.as_ref()
    }

    /// Sets the value of SecurityCallsNotAuthorized
    pub fn set_security_calls_not_authorized(&mut self, value: u32) {
        self.security_calls_not_authorized = Some(value);
    }

    /// Gets the value of SecurityCallsNotAuthorized
    pub fn get_security_calls_not_authorized(&self) -> Option<&u32> {
        self.security_calls_not_authorized.as_ref()
    }

    /// Sets the value of SecurityCallsNotAuthorizedPerSecond
    pub fn set_security_calls_not_authorized_per_second(&mut self, value: u32) {
        self.security_calls_not_authorized_per_second = Some(value);
    }

    /// Gets the value of SecurityCallsNotAuthorizedPerSecond
    pub fn get_security_calls_not_authorized_per_second(&self) -> Option<&u32> {
        self.security_calls_not_authorized_per_second.as_ref()
    }

    /// Sets the value of SecurityValidationandAuthenticationFailures
    pub fn set_security_validationand_authentication_failures(&mut self, value: u32) {
        self.security_validationand_authentication_failures = Some(value);
    }

    /// Gets the value of SecurityValidationandAuthenticationFailures
    pub fn get_security_validationand_authentication_failures(&self) -> Option<&u32> {
        self.security_validationand_authentication_failures.as_ref()
    }

    /// Sets the value of SecurityValidationandAuthenticationFailuresPerSecond
    pub fn set_security_validationand_authentication_failures_per_second(&mut self, value: u32) {
        self.security_validationand_authentication_failures_per_second = Some(value);
    }

    /// Gets the value of SecurityValidationandAuthenticationFailuresPerSecond
    pub fn get_security_validationand_authentication_failures_per_second(&self) -> Option<&u32> {
        self.security_validationand_authentication_failures_per_second.as_ref()
    }

    /// Sets the value of TransactionsFlowed
    pub fn set_transactions_flowed(&mut self, value: u32) {
        self.transactions_flowed = Some(value);
    }

    /// Gets the value of TransactionsFlowed
    pub fn get_transactions_flowed(&self) -> Option<&u32> {
        self.transactions_flowed.as_ref()
    }

    /// Sets the value of TransactionsFlowedPerSecond
    pub fn set_transactions_flowed_per_second(&mut self, value: u32) {
        self.transactions_flowed_per_second = Some(value);
    }

    /// Gets the value of TransactionsFlowedPerSecond
    pub fn get_transactions_flowed_per_second(&self) -> Option<&u32> {
        self.transactions_flowed_per_second.as_ref()
    }
}

