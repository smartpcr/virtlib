// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_Counters_TeredoRelay struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_Counters_TeredoRelay {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "InTeredoRelayErrorPacketsDestinationError")]
    pub in_teredo_relay_error_packets_destination_error: Option<u32>,

/// 
    #[serde(rename = "InTeredoRelayErrorPacketsHeaderError")]
    pub in_teredo_relay_error_packets_header_error: Option<u32>,

/// 
    #[serde(rename = "InTeredoRelayErrorPacketsSourceError")]
    pub in_teredo_relay_error_packets_source_error: Option<u32>,

/// 
    #[serde(rename = "InTeredoRelayErrorPacketsTotal")]
    pub in_teredo_relay_error_packets_total: Option<u32>,

/// 
    #[serde(rename = "InTeredoRelaySuccessPacketsBubbles")]
    pub in_teredo_relay_success_packets_bubbles: Option<u32>,

/// 
    #[serde(rename = "InTeredoRelaySuccessPacketsDataPackets")]
    pub in_teredo_relay_success_packets_data_packets: Option<u64>,

/// 
    #[serde(rename = "InTeredoRelaySuccessPacketsDataPacketsKernelMode")]
    pub in_teredo_relay_success_packets_data_packets_kernel_mode: Option<u64>,

/// 
    #[serde(rename = "InTeredoRelaySuccessPacketsDataPacketsUserMode")]
    pub in_teredo_relay_success_packets_data_packets_user_mode: Option<u64>,

/// 
    #[serde(rename = "InTeredoRelaySuccessPacketsTotal")]
    pub in_teredo_relay_success_packets_total: Option<u64>,

/// 
    #[serde(rename = "InTeredoRelayTotalPacketsSuccessError")]
    pub in_teredo_relay_total_packets_success_error: Option<u32>,

/// 
    #[serde(rename = "InTeredoRelayTotalPacketsSuccessErrorPersec")]
    pub in_teredo_relay_total_packets_success_error_persec: Option<u32>,

/// 
    #[serde(rename = "OutTeredoRelayErrorPackets")]
    pub out_teredo_relay_error_packets: Option<u32>,

/// 
    #[serde(rename = "OutTeredoRelayErrorPacketsDestinationError")]
    pub out_teredo_relay_error_packets_destination_error: Option<u32>,

/// 
    #[serde(rename = "OutTeredoRelayErrorPacketsHeaderError")]
    pub out_teredo_relay_error_packets_header_error: Option<u32>,

/// 
    #[serde(rename = "OutTeredoRelayErrorPacketsSourceError")]
    pub out_teredo_relay_error_packets_source_error: Option<u32>,

/// 
    #[serde(rename = "OutTeredoRelaySuccessPackets")]
    pub out_teredo_relay_success_packets: Option<u64>,

/// 
    #[serde(rename = "OutTeredoRelaySuccessPacketsBubbles")]
    pub out_teredo_relay_success_packets_bubbles: Option<u32>,

/// 
    #[serde(rename = "OutTeredoRelaySuccessPacketsDataPackets")]
    pub out_teredo_relay_success_packets_data_packets: Option<u64>,

/// 
    #[serde(rename = "OutTeredoRelaySuccessPacketsDataPacketsKernelMode")]
    pub out_teredo_relay_success_packets_data_packets_kernel_mode: Option<u64>,

/// 
    #[serde(rename = "OutTeredoRelaySuccessPacketsDataPacketsUserMode")]
    pub out_teredo_relay_success_packets_data_packets_user_mode: Option<u64>,

/// 
    #[serde(rename = "OutTeredoRelayTotalPacketsSuccessError")]
    pub out_teredo_relay_total_packets_success_error: Option<u32>,

/// 
    #[serde(rename = "OutTeredoRelayTotalPacketsSuccessErrorPersec")]
    pub out_teredo_relay_total_packets_success_error_persec: Option<u32>,
}

impl Win32_PerfRawData_Counters_TeredoRelay {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            in_teredo_relay_error_packets_destination_error: None,
            in_teredo_relay_error_packets_header_error: None,
            in_teredo_relay_error_packets_source_error: None,
            in_teredo_relay_error_packets_total: None,
            in_teredo_relay_success_packets_bubbles: None,
            in_teredo_relay_success_packets_data_packets: None,
            in_teredo_relay_success_packets_data_packets_kernel_mode: None,
            in_teredo_relay_success_packets_data_packets_user_mode: None,
            in_teredo_relay_success_packets_total: None,
            in_teredo_relay_total_packets_success_error: None,
            in_teredo_relay_total_packets_success_error_persec: None,
            out_teredo_relay_error_packets: None,
            out_teredo_relay_error_packets_destination_error: None,
            out_teredo_relay_error_packets_header_error: None,
            out_teredo_relay_error_packets_source_error: None,
            out_teredo_relay_success_packets: None,
            out_teredo_relay_success_packets_bubbles: None,
            out_teredo_relay_success_packets_data_packets: None,
            out_teredo_relay_success_packets_data_packets_kernel_mode: None,
            out_teredo_relay_success_packets_data_packets_user_mode: None,
            out_teredo_relay_total_packets_success_error: None,
            out_teredo_relay_total_packets_success_error_persec: None,
        }
    }


    /// Sets the value of InTeredoRelayErrorPacketsDestinationError
    pub fn set_in_teredo_relay_error_packets_destination_error(&mut self, value: u32) {
        self.in_teredo_relay_error_packets_destination_error = Some(value);
    }

    /// Gets the value of InTeredoRelayErrorPacketsDestinationError
    pub fn get_in_teredo_relay_error_packets_destination_error(&self) -> Option<&u32> {
        self.in_teredo_relay_error_packets_destination_error.as_ref()
    }

    /// Sets the value of InTeredoRelayErrorPacketsHeaderError
    pub fn set_in_teredo_relay_error_packets_header_error(&mut self, value: u32) {
        self.in_teredo_relay_error_packets_header_error = Some(value);
    }

    /// Gets the value of InTeredoRelayErrorPacketsHeaderError
    pub fn get_in_teredo_relay_error_packets_header_error(&self) -> Option<&u32> {
        self.in_teredo_relay_error_packets_header_error.as_ref()
    }

    /// Sets the value of InTeredoRelayErrorPacketsSourceError
    pub fn set_in_teredo_relay_error_packets_source_error(&mut self, value: u32) {
        self.in_teredo_relay_error_packets_source_error = Some(value);
    }

    /// Gets the value of InTeredoRelayErrorPacketsSourceError
    pub fn get_in_teredo_relay_error_packets_source_error(&self) -> Option<&u32> {
        self.in_teredo_relay_error_packets_source_error.as_ref()
    }

    /// Sets the value of InTeredoRelayErrorPacketsTotal
    pub fn set_in_teredo_relay_error_packets_total(&mut self, value: u32) {
        self.in_teredo_relay_error_packets_total = Some(value);
    }

    /// Gets the value of InTeredoRelayErrorPacketsTotal
    pub fn get_in_teredo_relay_error_packets_total(&self) -> Option<&u32> {
        self.in_teredo_relay_error_packets_total.as_ref()
    }

    /// Sets the value of InTeredoRelaySuccessPacketsBubbles
    pub fn set_in_teredo_relay_success_packets_bubbles(&mut self, value: u32) {
        self.in_teredo_relay_success_packets_bubbles = Some(value);
    }

    /// Gets the value of InTeredoRelaySuccessPacketsBubbles
    pub fn get_in_teredo_relay_success_packets_bubbles(&self) -> Option<&u32> {
        self.in_teredo_relay_success_packets_bubbles.as_ref()
    }

    /// Sets the value of InTeredoRelaySuccessPacketsDataPackets
    pub fn set_in_teredo_relay_success_packets_data_packets(&mut self, value: u64) {
        self.in_teredo_relay_success_packets_data_packets = Some(value);
    }

    /// Gets the value of InTeredoRelaySuccessPacketsDataPackets
    pub fn get_in_teredo_relay_success_packets_data_packets(&self) -> Option<&u64> {
        self.in_teredo_relay_success_packets_data_packets.as_ref()
    }

    /// Sets the value of InTeredoRelaySuccessPacketsDataPacketsKernelMode
    pub fn set_in_teredo_relay_success_packets_data_packets_kernel_mode(&mut self, value: u64) {
        self.in_teredo_relay_success_packets_data_packets_kernel_mode = Some(value);
    }

    /// Gets the value of InTeredoRelaySuccessPacketsDataPacketsKernelMode
    pub fn get_in_teredo_relay_success_packets_data_packets_kernel_mode(&self) -> Option<&u64> {
        self.in_teredo_relay_success_packets_data_packets_kernel_mode.as_ref()
    }

    /// Sets the value of InTeredoRelaySuccessPacketsDataPacketsUserMode
    pub fn set_in_teredo_relay_success_packets_data_packets_user_mode(&mut self, value: u64) {
        self.in_teredo_relay_success_packets_data_packets_user_mode = Some(value);
    }

    /// Gets the value of InTeredoRelaySuccessPacketsDataPacketsUserMode
    pub fn get_in_teredo_relay_success_packets_data_packets_user_mode(&self) -> Option<&u64> {
        self.in_teredo_relay_success_packets_data_packets_user_mode.as_ref()
    }

    /// Sets the value of InTeredoRelaySuccessPacketsTotal
    pub fn set_in_teredo_relay_success_packets_total(&mut self, value: u64) {
        self.in_teredo_relay_success_packets_total = Some(value);
    }

    /// Gets the value of InTeredoRelaySuccessPacketsTotal
    pub fn get_in_teredo_relay_success_packets_total(&self) -> Option<&u64> {
        self.in_teredo_relay_success_packets_total.as_ref()
    }

    /// Sets the value of InTeredoRelayTotalPacketsSuccessError
    pub fn set_in_teredo_relay_total_packets_success_error(&mut self, value: u32) {
        self.in_teredo_relay_total_packets_success_error = Some(value);
    }

    /// Gets the value of InTeredoRelayTotalPacketsSuccessError
    pub fn get_in_teredo_relay_total_packets_success_error(&self) -> Option<&u32> {
        self.in_teredo_relay_total_packets_success_error.as_ref()
    }

    /// Sets the value of InTeredoRelayTotalPacketsSuccessErrorPersec
    pub fn set_in_teredo_relay_total_packets_success_error_persec(&mut self, value: u32) {
        self.in_teredo_relay_total_packets_success_error_persec = Some(value);
    }

    /// Gets the value of InTeredoRelayTotalPacketsSuccessErrorPersec
    pub fn get_in_teredo_relay_total_packets_success_error_persec(&self) -> Option<&u32> {
        self.in_teredo_relay_total_packets_success_error_persec.as_ref()
    }

    /// Sets the value of OutTeredoRelayErrorPackets
    pub fn set_out_teredo_relay_error_packets(&mut self, value: u32) {
        self.out_teredo_relay_error_packets = Some(value);
    }

    /// Gets the value of OutTeredoRelayErrorPackets
    pub fn get_out_teredo_relay_error_packets(&self) -> Option<&u32> {
        self.out_teredo_relay_error_packets.as_ref()
    }

    /// Sets the value of OutTeredoRelayErrorPacketsDestinationError
    pub fn set_out_teredo_relay_error_packets_destination_error(&mut self, value: u32) {
        self.out_teredo_relay_error_packets_destination_error = Some(value);
    }

    /// Gets the value of OutTeredoRelayErrorPacketsDestinationError
    pub fn get_out_teredo_relay_error_packets_destination_error(&self) -> Option<&u32> {
        self.out_teredo_relay_error_packets_destination_error.as_ref()
    }

    /// Sets the value of OutTeredoRelayErrorPacketsHeaderError
    pub fn set_out_teredo_relay_error_packets_header_error(&mut self, value: u32) {
        self.out_teredo_relay_error_packets_header_error = Some(value);
    }

    /// Gets the value of OutTeredoRelayErrorPacketsHeaderError
    pub fn get_out_teredo_relay_error_packets_header_error(&self) -> Option<&u32> {
        self.out_teredo_relay_error_packets_header_error.as_ref()
    }

    /// Sets the value of OutTeredoRelayErrorPacketsSourceError
    pub fn set_out_teredo_relay_error_packets_source_error(&mut self, value: u32) {
        self.out_teredo_relay_error_packets_source_error = Some(value);
    }

    /// Gets the value of OutTeredoRelayErrorPacketsSourceError
    pub fn get_out_teredo_relay_error_packets_source_error(&self) -> Option<&u32> {
        self.out_teredo_relay_error_packets_source_error.as_ref()
    }

    /// Sets the value of OutTeredoRelaySuccessPackets
    pub fn set_out_teredo_relay_success_packets(&mut self, value: u64) {
        self.out_teredo_relay_success_packets = Some(value);
    }

    /// Gets the value of OutTeredoRelaySuccessPackets
    pub fn get_out_teredo_relay_success_packets(&self) -> Option<&u64> {
        self.out_teredo_relay_success_packets.as_ref()
    }

    /// Sets the value of OutTeredoRelaySuccessPacketsBubbles
    pub fn set_out_teredo_relay_success_packets_bubbles(&mut self, value: u32) {
        self.out_teredo_relay_success_packets_bubbles = Some(value);
    }

    /// Gets the value of OutTeredoRelaySuccessPacketsBubbles
    pub fn get_out_teredo_relay_success_packets_bubbles(&self) -> Option<&u32> {
        self.out_teredo_relay_success_packets_bubbles.as_ref()
    }

    /// Sets the value of OutTeredoRelaySuccessPacketsDataPackets
    pub fn set_out_teredo_relay_success_packets_data_packets(&mut self, value: u64) {
        self.out_teredo_relay_success_packets_data_packets = Some(value);
    }

    /// Gets the value of OutTeredoRelaySuccessPacketsDataPackets
    pub fn get_out_teredo_relay_success_packets_data_packets(&self) -> Option<&u64> {
        self.out_teredo_relay_success_packets_data_packets.as_ref()
    }

    /// Sets the value of OutTeredoRelaySuccessPacketsDataPacketsKernelMode
    pub fn set_out_teredo_relay_success_packets_data_packets_kernel_mode(&mut self, value: u64) {
        self.out_teredo_relay_success_packets_data_packets_kernel_mode = Some(value);
    }

    /// Gets the value of OutTeredoRelaySuccessPacketsDataPacketsKernelMode
    pub fn get_out_teredo_relay_success_packets_data_packets_kernel_mode(&self) -> Option<&u64> {
        self.out_teredo_relay_success_packets_data_packets_kernel_mode.as_ref()
    }

    /// Sets the value of OutTeredoRelaySuccessPacketsDataPacketsUserMode
    pub fn set_out_teredo_relay_success_packets_data_packets_user_mode(&mut self, value: u64) {
        self.out_teredo_relay_success_packets_data_packets_user_mode = Some(value);
    }

    /// Gets the value of OutTeredoRelaySuccessPacketsDataPacketsUserMode
    pub fn get_out_teredo_relay_success_packets_data_packets_user_mode(&self) -> Option<&u64> {
        self.out_teredo_relay_success_packets_data_packets_user_mode.as_ref()
    }

    /// Sets the value of OutTeredoRelayTotalPacketsSuccessError
    pub fn set_out_teredo_relay_total_packets_success_error(&mut self, value: u32) {
        self.out_teredo_relay_total_packets_success_error = Some(value);
    }

    /// Gets the value of OutTeredoRelayTotalPacketsSuccessError
    pub fn get_out_teredo_relay_total_packets_success_error(&self) -> Option<&u32> {
        self.out_teredo_relay_total_packets_success_error.as_ref()
    }

    /// Sets the value of OutTeredoRelayTotalPacketsSuccessErrorPersec
    pub fn set_out_teredo_relay_total_packets_success_error_persec(&mut self, value: u32) {
        self.out_teredo_relay_total_packets_success_error_persec = Some(value);
    }

    /// Gets the value of OutTeredoRelayTotalPacketsSuccessErrorPersec
    pub fn get_out_teredo_relay_total_packets_success_error_persec(&self) -> Option<&u32> {
        self.out_teredo_relay_total_packets_success_error_persec.as_ref()
    }
}

