// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_Counters_GenericIKEv1AuthIPandIKEv2 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_Counters_GenericIKEv1AuthIPandIKEv2 {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "AuthIPMainModeNegotiationTime")]
    pub auth_ipmain_mode_negotiation_time: Option<u32>,

/// 
    #[serde(rename = "AuthIPQuickModeNegotiationTime")]
    pub auth_ipquick_mode_negotiation_time: Option<u32>,

/// 
    #[serde(rename = "ExtendedModeNegotiationTime")]
    pub extended_mode_negotiation_time: Option<u32>,

/// 
    #[serde(rename = "FailedNegotiations")]
    pub failed_negotiations: Option<u32>,

/// 
    #[serde(rename = "FailedNegotiationsPersec")]
    pub failed_negotiations_persec: Option<u32>,

/// 
    #[serde(rename = "IKEv1MainModeNegotiationTime")]
    pub ikev1_main_mode_negotiation_time: Option<u32>,

/// 
    #[serde(rename = "IKEv1QuickModeNegotiationTime")]
    pub ikev1_quick_mode_negotiation_time: Option<u32>,

/// 
    #[serde(rename = "IKEv2MainModeNegotiationTime")]
    pub ikev2_main_mode_negotiation_time: Option<u32>,

/// 
    #[serde(rename = "IKEv2QuickModeNegotiationTime")]
    pub ikev2_quick_mode_negotiation_time: Option<u32>,

/// 
    #[serde(rename = "InvalidPacketsReceivedPersec")]
    pub invalid_packets_received_persec: Option<u32>,

/// 
    #[serde(rename = "PacketsReceivedPersec")]
    pub packets_received_persec: Option<u32>,

/// 
    #[serde(rename = "SuccessfulNegotiations")]
    pub successful_negotiations: Option<u32>,

/// 
    #[serde(rename = "SuccessfulNegotiationsPersec")]
    pub successful_negotiations_persec: Option<u32>,
}

impl Win32_PerfRawData_Counters_GenericIKEv1AuthIPandIKEv2 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            auth_ipmain_mode_negotiation_time: None,
            auth_ipquick_mode_negotiation_time: None,
            extended_mode_negotiation_time: None,
            failed_negotiations: None,
            failed_negotiations_persec: None,
            ikev1_main_mode_negotiation_time: None,
            ikev1_quick_mode_negotiation_time: None,
            ikev2_main_mode_negotiation_time: None,
            ikev2_quick_mode_negotiation_time: None,
            invalid_packets_received_persec: None,
            packets_received_persec: None,
            successful_negotiations: None,
            successful_negotiations_persec: None,
        }
    }


    /// Sets the value of AuthIPMainModeNegotiationTime
    pub fn set_auth_ipmain_mode_negotiation_time(&mut self, value: u32) {
        self.auth_ipmain_mode_negotiation_time = Some(value);
    }

    /// Gets the value of AuthIPMainModeNegotiationTime
    pub fn get_auth_ipmain_mode_negotiation_time(&self) -> Option<&u32> {
        self.auth_ipmain_mode_negotiation_time.as_ref()
    }

    /// Sets the value of AuthIPQuickModeNegotiationTime
    pub fn set_auth_ipquick_mode_negotiation_time(&mut self, value: u32) {
        self.auth_ipquick_mode_negotiation_time = Some(value);
    }

    /// Gets the value of AuthIPQuickModeNegotiationTime
    pub fn get_auth_ipquick_mode_negotiation_time(&self) -> Option<&u32> {
        self.auth_ipquick_mode_negotiation_time.as_ref()
    }

    /// Sets the value of ExtendedModeNegotiationTime
    pub fn set_extended_mode_negotiation_time(&mut self, value: u32) {
        self.extended_mode_negotiation_time = Some(value);
    }

    /// Gets the value of ExtendedModeNegotiationTime
    pub fn get_extended_mode_negotiation_time(&self) -> Option<&u32> {
        self.extended_mode_negotiation_time.as_ref()
    }

    /// Sets the value of FailedNegotiations
    pub fn set_failed_negotiations(&mut self, value: u32) {
        self.failed_negotiations = Some(value);
    }

    /// Gets the value of FailedNegotiations
    pub fn get_failed_negotiations(&self) -> Option<&u32> {
        self.failed_negotiations.as_ref()
    }

    /// Sets the value of FailedNegotiationsPersec
    pub fn set_failed_negotiations_persec(&mut self, value: u32) {
        self.failed_negotiations_persec = Some(value);
    }

    /// Gets the value of FailedNegotiationsPersec
    pub fn get_failed_negotiations_persec(&self) -> Option<&u32> {
        self.failed_negotiations_persec.as_ref()
    }

    /// Sets the value of IKEv1MainModeNegotiationTime
    pub fn set_ikev1_main_mode_negotiation_time(&mut self, value: u32) {
        self.ikev1_main_mode_negotiation_time = Some(value);
    }

    /// Gets the value of IKEv1MainModeNegotiationTime
    pub fn get_ikev1_main_mode_negotiation_time(&self) -> Option<&u32> {
        self.ikev1_main_mode_negotiation_time.as_ref()
    }

    /// Sets the value of IKEv1QuickModeNegotiationTime
    pub fn set_ikev1_quick_mode_negotiation_time(&mut self, value: u32) {
        self.ikev1_quick_mode_negotiation_time = Some(value);
    }

    /// Gets the value of IKEv1QuickModeNegotiationTime
    pub fn get_ikev1_quick_mode_negotiation_time(&self) -> Option<&u32> {
        self.ikev1_quick_mode_negotiation_time.as_ref()
    }

    /// Sets the value of IKEv2MainModeNegotiationTime
    pub fn set_ikev2_main_mode_negotiation_time(&mut self, value: u32) {
        self.ikev2_main_mode_negotiation_time = Some(value);
    }

    /// Gets the value of IKEv2MainModeNegotiationTime
    pub fn get_ikev2_main_mode_negotiation_time(&self) -> Option<&u32> {
        self.ikev2_main_mode_negotiation_time.as_ref()
    }

    /// Sets the value of IKEv2QuickModeNegotiationTime
    pub fn set_ikev2_quick_mode_negotiation_time(&mut self, value: u32) {
        self.ikev2_quick_mode_negotiation_time = Some(value);
    }

    /// Gets the value of IKEv2QuickModeNegotiationTime
    pub fn get_ikev2_quick_mode_negotiation_time(&self) -> Option<&u32> {
        self.ikev2_quick_mode_negotiation_time.as_ref()
    }

    /// Sets the value of InvalidPacketsReceivedPersec
    pub fn set_invalid_packets_received_persec(&mut self, value: u32) {
        self.invalid_packets_received_persec = Some(value);
    }

    /// Gets the value of InvalidPacketsReceivedPersec
    pub fn get_invalid_packets_received_persec(&self) -> Option<&u32> {
        self.invalid_packets_received_persec.as_ref()
    }

    /// Sets the value of PacketsReceivedPersec
    pub fn set_packets_received_persec(&mut self, value: u32) {
        self.packets_received_persec = Some(value);
    }

    /// Gets the value of PacketsReceivedPersec
    pub fn get_packets_received_persec(&self) -> Option<&u32> {
        self.packets_received_persec.as_ref()
    }

    /// Sets the value of SuccessfulNegotiations
    pub fn set_successful_negotiations(&mut self, value: u32) {
        self.successful_negotiations = Some(value);
    }

    /// Gets the value of SuccessfulNegotiations
    pub fn get_successful_negotiations(&self) -> Option<&u32> {
        self.successful_negotiations.as_ref()
    }

    /// Sets the value of SuccessfulNegotiationsPersec
    pub fn set_successful_negotiations_persec(&mut self, value: u32) {
        self.successful_negotiations_persec = Some(value);
    }

    /// Gets the value of SuccessfulNegotiationsPersec
    pub fn get_successful_negotiations_persec(&self) -> Option<&u32> {
        self.successful_negotiations_persec.as_ref()
    }
}

