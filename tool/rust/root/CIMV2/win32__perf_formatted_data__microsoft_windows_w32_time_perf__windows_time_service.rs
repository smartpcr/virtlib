// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_MicrosoftWindowsW32TimePerf_WindowsTimeService struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_MicrosoftWindowsW32TimePerf_WindowsTimeService {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "ClockFrequencyAdjustment")]
    pub clock_frequency_adjustment: Option<u32>,

/// 
    #[serde(rename = "ClockFrequencyAdjustmentPPB")]
    pub clock_frequency_adjustment_ppb: Option<u32>,

/// 
    #[serde(rename = "ComputedTimeOffset")]
    pub computed_time_offset: Option<u64>,

/// 
    #[serde(rename = "NTPClientTimeSourceCount")]
    pub ntpclient_time_source_count: Option<u32>,

/// 
    #[serde(rename = "NTPRoundtripDelay")]
    pub ntproundtrip_delay: Option<u32>,

/// 
    #[serde(rename = "NTPServerIncomingRequests")]
    pub ntpserver_incoming_requests: Option<u64>,

/// 
    #[serde(rename = "NTPServerOutgoingResponses")]
    pub ntpserver_outgoing_responses: Option<u64>,
}

impl Win32_PerfFormattedData_MicrosoftWindowsW32TimePerf_WindowsTimeService {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            clock_frequency_adjustment: None,
            clock_frequency_adjustment_ppb: None,
            computed_time_offset: None,
            ntpclient_time_source_count: None,
            ntproundtrip_delay: None,
            ntpserver_incoming_requests: None,
            ntpserver_outgoing_responses: None,
        }
    }


    /// Sets the value of ClockFrequencyAdjustment
    pub fn set_clock_frequency_adjustment(&mut self, value: u32) {
        self.clock_frequency_adjustment = Some(value);
    }

    /// Gets the value of ClockFrequencyAdjustment
    pub fn get_clock_frequency_adjustment(&self) -> Option<&u32> {
        self.clock_frequency_adjustment.as_ref()
    }

    /// Sets the value of ClockFrequencyAdjustmentPPB
    pub fn set_clock_frequency_adjustment_ppb(&mut self, value: u32) {
        self.clock_frequency_adjustment_ppb = Some(value);
    }

    /// Gets the value of ClockFrequencyAdjustmentPPB
    pub fn get_clock_frequency_adjustment_ppb(&self) -> Option<&u32> {
        self.clock_frequency_adjustment_ppb.as_ref()
    }

    /// Sets the value of ComputedTimeOffset
    pub fn set_computed_time_offset(&mut self, value: u64) {
        self.computed_time_offset = Some(value);
    }

    /// Gets the value of ComputedTimeOffset
    pub fn get_computed_time_offset(&self) -> Option<&u64> {
        self.computed_time_offset.as_ref()
    }

    /// Sets the value of NTPClientTimeSourceCount
    pub fn set_ntpclient_time_source_count(&mut self, value: u32) {
        self.ntpclient_time_source_count = Some(value);
    }

    /// Gets the value of NTPClientTimeSourceCount
    pub fn get_ntpclient_time_source_count(&self) -> Option<&u32> {
        self.ntpclient_time_source_count.as_ref()
    }

    /// Sets the value of NTPRoundtripDelay
    pub fn set_ntproundtrip_delay(&mut self, value: u32) {
        self.ntproundtrip_delay = Some(value);
    }

    /// Gets the value of NTPRoundtripDelay
    pub fn get_ntproundtrip_delay(&self) -> Option<&u32> {
        self.ntproundtrip_delay.as_ref()
    }

    /// Sets the value of NTPServerIncomingRequests
    pub fn set_ntpserver_incoming_requests(&mut self, value: u64) {
        self.ntpserver_incoming_requests = Some(value);
    }

    /// Gets the value of NTPServerIncomingRequests
    pub fn get_ntpserver_incoming_requests(&self) -> Option<&u64> {
        self.ntpserver_incoming_requests.as_ref()
    }

    /// Sets the value of NTPServerOutgoingResponses
    pub fn set_ntpserver_outgoing_responses(&mut self, value: u64) {
        self.ntpserver_outgoing_responses = Some(value);
    }

    /// Gets the value of NTPServerOutgoingResponses
    pub fn get_ntpserver_outgoing_responses(&self) -> Option<&u64> {
        self.ntpserver_outgoing_responses.as_ref()
    }
}

