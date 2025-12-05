// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_Counters_IPsecIKEv2IPv4 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_Counters_IPsecIKEv2IPv4 {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "ActiveMainModeSAs")]
    pub active_main_mode_sas: Option<u32>,

/// 
    #[serde(rename = "ActiveQuickModeSAs")]
    pub active_quick_mode_sas: Option<u32>,

/// 
    #[serde(rename = "FailedMainModeNegotiations")]
    pub failed_main_mode_negotiations: Option<u32>,

/// 
    #[serde(rename = "FailedMainModeNegotiationsPersec")]
    pub failed_main_mode_negotiations_persec: Option<u32>,

/// 
    #[serde(rename = "FailedQuickModeNegotiations")]
    pub failed_quick_mode_negotiations: Option<u32>,

/// 
    #[serde(rename = "FailedQuickModeNegotiationsPersec")]
    pub failed_quick_mode_negotiations_persec: Option<u32>,

/// 
    #[serde(rename = "MainModeNegotiationRequestsReceived")]
    pub main_mode_negotiation_requests_received: Option<u32>,

/// 
    #[serde(rename = "MainModeNegotiationRequestsReceivedPersec")]
    pub main_mode_negotiation_requests_received_persec: Option<u32>,

/// 
    #[serde(rename = "MainModeNegotiations")]
    pub main_mode_negotiations: Option<u32>,

/// 
    #[serde(rename = "MainModeNegotiationsPersec")]
    pub main_mode_negotiations_persec: Option<u32>,

/// 
    #[serde(rename = "PendingMainModeNegotiations")]
    pub pending_main_mode_negotiations: Option<u32>,

/// 
    #[serde(rename = "PendingQuickModeNegotiations")]
    pub pending_quick_mode_negotiations: Option<u32>,

/// 
    #[serde(rename = "QuickModeNegotiations")]
    pub quick_mode_negotiations: Option<u32>,

/// 
    #[serde(rename = "QuickModeNegotiationsPersec")]
    pub quick_mode_negotiations_persec: Option<u32>,

/// 
    #[serde(rename = "SuccessfulMainModeNegotiations")]
    pub successful_main_mode_negotiations: Option<u32>,

/// 
    #[serde(rename = "SuccessfulMainModeNegotiationsPersec")]
    pub successful_main_mode_negotiations_persec: Option<u32>,

/// 
    #[serde(rename = "SuccessfulQuickModeNegotiations")]
    pub successful_quick_mode_negotiations: Option<u32>,

/// 
    #[serde(rename = "SuccessfulQuickModeNegotiationsPersec")]
    pub successful_quick_mode_negotiations_persec: Option<u32>,
}

impl Win32_PerfFormattedData_Counters_IPsecIKEv2IPv4 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            active_main_mode_sas: None,
            active_quick_mode_sas: None,
            failed_main_mode_negotiations: None,
            failed_main_mode_negotiations_persec: None,
            failed_quick_mode_negotiations: None,
            failed_quick_mode_negotiations_persec: None,
            main_mode_negotiation_requests_received: None,
            main_mode_negotiation_requests_received_persec: None,
            main_mode_negotiations: None,
            main_mode_negotiations_persec: None,
            pending_main_mode_negotiations: None,
            pending_quick_mode_negotiations: None,
            quick_mode_negotiations: None,
            quick_mode_negotiations_persec: None,
            successful_main_mode_negotiations: None,
            successful_main_mode_negotiations_persec: None,
            successful_quick_mode_negotiations: None,
            successful_quick_mode_negotiations_persec: None,
        }
    }


    /// Sets the value of ActiveMainModeSAs
    pub fn set_active_main_mode_sas(&mut self, value: u32) {
        self.active_main_mode_sas = Some(value);
    }

    /// Gets the value of ActiveMainModeSAs
    pub fn get_active_main_mode_sas(&self) -> Option<&u32> {
        self.active_main_mode_sas.as_ref()
    }

    /// Sets the value of ActiveQuickModeSAs
    pub fn set_active_quick_mode_sas(&mut self, value: u32) {
        self.active_quick_mode_sas = Some(value);
    }

    /// Gets the value of ActiveQuickModeSAs
    pub fn get_active_quick_mode_sas(&self) -> Option<&u32> {
        self.active_quick_mode_sas.as_ref()
    }

    /// Sets the value of FailedMainModeNegotiations
    pub fn set_failed_main_mode_negotiations(&mut self, value: u32) {
        self.failed_main_mode_negotiations = Some(value);
    }

    /// Gets the value of FailedMainModeNegotiations
    pub fn get_failed_main_mode_negotiations(&self) -> Option<&u32> {
        self.failed_main_mode_negotiations.as_ref()
    }

    /// Sets the value of FailedMainModeNegotiationsPersec
    pub fn set_failed_main_mode_negotiations_persec(&mut self, value: u32) {
        self.failed_main_mode_negotiations_persec = Some(value);
    }

    /// Gets the value of FailedMainModeNegotiationsPersec
    pub fn get_failed_main_mode_negotiations_persec(&self) -> Option<&u32> {
        self.failed_main_mode_negotiations_persec.as_ref()
    }

    /// Sets the value of FailedQuickModeNegotiations
    pub fn set_failed_quick_mode_negotiations(&mut self, value: u32) {
        self.failed_quick_mode_negotiations = Some(value);
    }

    /// Gets the value of FailedQuickModeNegotiations
    pub fn get_failed_quick_mode_negotiations(&self) -> Option<&u32> {
        self.failed_quick_mode_negotiations.as_ref()
    }

    /// Sets the value of FailedQuickModeNegotiationsPersec
    pub fn set_failed_quick_mode_negotiations_persec(&mut self, value: u32) {
        self.failed_quick_mode_negotiations_persec = Some(value);
    }

    /// Gets the value of FailedQuickModeNegotiationsPersec
    pub fn get_failed_quick_mode_negotiations_persec(&self) -> Option<&u32> {
        self.failed_quick_mode_negotiations_persec.as_ref()
    }

    /// Sets the value of MainModeNegotiationRequestsReceived
    pub fn set_main_mode_negotiation_requests_received(&mut self, value: u32) {
        self.main_mode_negotiation_requests_received = Some(value);
    }

    /// Gets the value of MainModeNegotiationRequestsReceived
    pub fn get_main_mode_negotiation_requests_received(&self) -> Option<&u32> {
        self.main_mode_negotiation_requests_received.as_ref()
    }

    /// Sets the value of MainModeNegotiationRequestsReceivedPersec
    pub fn set_main_mode_negotiation_requests_received_persec(&mut self, value: u32) {
        self.main_mode_negotiation_requests_received_persec = Some(value);
    }

    /// Gets the value of MainModeNegotiationRequestsReceivedPersec
    pub fn get_main_mode_negotiation_requests_received_persec(&self) -> Option<&u32> {
        self.main_mode_negotiation_requests_received_persec.as_ref()
    }

    /// Sets the value of MainModeNegotiations
    pub fn set_main_mode_negotiations(&mut self, value: u32) {
        self.main_mode_negotiations = Some(value);
    }

    /// Gets the value of MainModeNegotiations
    pub fn get_main_mode_negotiations(&self) -> Option<&u32> {
        self.main_mode_negotiations.as_ref()
    }

    /// Sets the value of MainModeNegotiationsPersec
    pub fn set_main_mode_negotiations_persec(&mut self, value: u32) {
        self.main_mode_negotiations_persec = Some(value);
    }

    /// Gets the value of MainModeNegotiationsPersec
    pub fn get_main_mode_negotiations_persec(&self) -> Option<&u32> {
        self.main_mode_negotiations_persec.as_ref()
    }

    /// Sets the value of PendingMainModeNegotiations
    pub fn set_pending_main_mode_negotiations(&mut self, value: u32) {
        self.pending_main_mode_negotiations = Some(value);
    }

    /// Gets the value of PendingMainModeNegotiations
    pub fn get_pending_main_mode_negotiations(&self) -> Option<&u32> {
        self.pending_main_mode_negotiations.as_ref()
    }

    /// Sets the value of PendingQuickModeNegotiations
    pub fn set_pending_quick_mode_negotiations(&mut self, value: u32) {
        self.pending_quick_mode_negotiations = Some(value);
    }

    /// Gets the value of PendingQuickModeNegotiations
    pub fn get_pending_quick_mode_negotiations(&self) -> Option<&u32> {
        self.pending_quick_mode_negotiations.as_ref()
    }

    /// Sets the value of QuickModeNegotiations
    pub fn set_quick_mode_negotiations(&mut self, value: u32) {
        self.quick_mode_negotiations = Some(value);
    }

    /// Gets the value of QuickModeNegotiations
    pub fn get_quick_mode_negotiations(&self) -> Option<&u32> {
        self.quick_mode_negotiations.as_ref()
    }

    /// Sets the value of QuickModeNegotiationsPersec
    pub fn set_quick_mode_negotiations_persec(&mut self, value: u32) {
        self.quick_mode_negotiations_persec = Some(value);
    }

    /// Gets the value of QuickModeNegotiationsPersec
    pub fn get_quick_mode_negotiations_persec(&self) -> Option<&u32> {
        self.quick_mode_negotiations_persec.as_ref()
    }

    /// Sets the value of SuccessfulMainModeNegotiations
    pub fn set_successful_main_mode_negotiations(&mut self, value: u32) {
        self.successful_main_mode_negotiations = Some(value);
    }

    /// Gets the value of SuccessfulMainModeNegotiations
    pub fn get_successful_main_mode_negotiations(&self) -> Option<&u32> {
        self.successful_main_mode_negotiations.as_ref()
    }

    /// Sets the value of SuccessfulMainModeNegotiationsPersec
    pub fn set_successful_main_mode_negotiations_persec(&mut self, value: u32) {
        self.successful_main_mode_negotiations_persec = Some(value);
    }

    /// Gets the value of SuccessfulMainModeNegotiationsPersec
    pub fn get_successful_main_mode_negotiations_persec(&self) -> Option<&u32> {
        self.successful_main_mode_negotiations_persec.as_ref()
    }

    /// Sets the value of SuccessfulQuickModeNegotiations
    pub fn set_successful_quick_mode_negotiations(&mut self, value: u32) {
        self.successful_quick_mode_negotiations = Some(value);
    }

    /// Gets the value of SuccessfulQuickModeNegotiations
    pub fn get_successful_quick_mode_negotiations(&self) -> Option<&u32> {
        self.successful_quick_mode_negotiations.as_ref()
    }

    /// Sets the value of SuccessfulQuickModeNegotiationsPersec
    pub fn set_successful_quick_mode_negotiations_persec(&mut self, value: u32) {
        self.successful_quick_mode_negotiations_persec = Some(value);
    }

    /// Gets the value of SuccessfulQuickModeNegotiationsPersec
    pub fn get_successful_quick_mode_negotiations_persec(&self) -> Option<&u32> {
        self.successful_quick_mode_negotiations_persec.as_ref()
    }
}

