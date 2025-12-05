// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_NvspNicVRSSStats_HyperVVirtualNetworkAdapterVRSS struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_NvspNicVRSSStats_HyperVVirtualNetworkAdapterVRSS {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "PostmoveReceivePacketsPerSecond")]
    pub postmove_receive_packets_per_second: Option<u64>,

/// 
    #[serde(rename = "PostmoveReceivePacketsTotal")]
    pub postmove_receive_packets_total: Option<u64>,

/// 
    #[serde(rename = "PostmoveSendPacketCompletionsPerSecond")]
    pub postmove_send_packet_completions_per_second: Option<u64>,

/// 
    #[serde(rename = "PostmoveSendPacketCompletionsTotal")]
    pub postmove_send_packet_completions_total: Option<u64>,

/// 
    #[serde(rename = "PostmoveSendPacketsPerSecond")]
    pub postmove_send_packets_per_second: Option<u64>,

/// 
    #[serde(rename = "PostmoveSendPacketsTotal")]
    pub postmove_send_packets_total: Option<u64>,

/// 
    #[serde(rename = "ReceivePacketPerSecond")]
    pub receive_packet_per_second: Option<u64>,

/// 
    #[serde(rename = "ReceivePacketTotal")]
    pub receive_packet_total: Option<u64>,

/// 
    #[serde(rename = "ReceiveProcessor")]
    pub receive_processor: Option<u32>,

/// 
    #[serde(rename = "ReceiveProcessorGroup")]
    pub receive_processor_group: Option<u32>,

/// 
    #[serde(rename = "SendPacketCompletionsPerSecond")]
    pub send_packet_completions_per_second: Option<u64>,

/// 
    #[serde(rename = "SendPacketCompletionsTotal")]
    pub send_packet_completions_total: Option<u64>,

/// 
    #[serde(rename = "SendPacketPerSecond")]
    pub send_packet_per_second: Option<u64>,

/// 
    #[serde(rename = "SendPacketTotal")]
    pub send_packet_total: Option<u64>,

/// 
    #[serde(rename = "SendProcessor")]
    pub send_processor: Option<u32>,

/// 
    #[serde(rename = "SendProcessorGroup")]
    pub send_processor_group: Option<u32>,
}

impl Win32_PerfRawData_NvspNicVRSSStats_HyperVVirtualNetworkAdapterVRSS {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            postmove_receive_packets_per_second: None,
            postmove_receive_packets_total: None,
            postmove_send_packet_completions_per_second: None,
            postmove_send_packet_completions_total: None,
            postmove_send_packets_per_second: None,
            postmove_send_packets_total: None,
            receive_packet_per_second: None,
            receive_packet_total: None,
            receive_processor: None,
            receive_processor_group: None,
            send_packet_completions_per_second: None,
            send_packet_completions_total: None,
            send_packet_per_second: None,
            send_packet_total: None,
            send_processor: None,
            send_processor_group: None,
        }
    }


    /// Sets the value of PostmoveReceivePacketsPerSecond
    pub fn set_postmove_receive_packets_per_second(&mut self, value: u64) {
        self.postmove_receive_packets_per_second = Some(value);
    }

    /// Gets the value of PostmoveReceivePacketsPerSecond
    pub fn get_postmove_receive_packets_per_second(&self) -> Option<&u64> {
        self.postmove_receive_packets_per_second.as_ref()
    }

    /// Sets the value of PostmoveReceivePacketsTotal
    pub fn set_postmove_receive_packets_total(&mut self, value: u64) {
        self.postmove_receive_packets_total = Some(value);
    }

    /// Gets the value of PostmoveReceivePacketsTotal
    pub fn get_postmove_receive_packets_total(&self) -> Option<&u64> {
        self.postmove_receive_packets_total.as_ref()
    }

    /// Sets the value of PostmoveSendPacketCompletionsPerSecond
    pub fn set_postmove_send_packet_completions_per_second(&mut self, value: u64) {
        self.postmove_send_packet_completions_per_second = Some(value);
    }

    /// Gets the value of PostmoveSendPacketCompletionsPerSecond
    pub fn get_postmove_send_packet_completions_per_second(&self) -> Option<&u64> {
        self.postmove_send_packet_completions_per_second.as_ref()
    }

    /// Sets the value of PostmoveSendPacketCompletionsTotal
    pub fn set_postmove_send_packet_completions_total(&mut self, value: u64) {
        self.postmove_send_packet_completions_total = Some(value);
    }

    /// Gets the value of PostmoveSendPacketCompletionsTotal
    pub fn get_postmove_send_packet_completions_total(&self) -> Option<&u64> {
        self.postmove_send_packet_completions_total.as_ref()
    }

    /// Sets the value of PostmoveSendPacketsPerSecond
    pub fn set_postmove_send_packets_per_second(&mut self, value: u64) {
        self.postmove_send_packets_per_second = Some(value);
    }

    /// Gets the value of PostmoveSendPacketsPerSecond
    pub fn get_postmove_send_packets_per_second(&self) -> Option<&u64> {
        self.postmove_send_packets_per_second.as_ref()
    }

    /// Sets the value of PostmoveSendPacketsTotal
    pub fn set_postmove_send_packets_total(&mut self, value: u64) {
        self.postmove_send_packets_total = Some(value);
    }

    /// Gets the value of PostmoveSendPacketsTotal
    pub fn get_postmove_send_packets_total(&self) -> Option<&u64> {
        self.postmove_send_packets_total.as_ref()
    }

    /// Sets the value of ReceivePacketPerSecond
    pub fn set_receive_packet_per_second(&mut self, value: u64) {
        self.receive_packet_per_second = Some(value);
    }

    /// Gets the value of ReceivePacketPerSecond
    pub fn get_receive_packet_per_second(&self) -> Option<&u64> {
        self.receive_packet_per_second.as_ref()
    }

    /// Sets the value of ReceivePacketTotal
    pub fn set_receive_packet_total(&mut self, value: u64) {
        self.receive_packet_total = Some(value);
    }

    /// Gets the value of ReceivePacketTotal
    pub fn get_receive_packet_total(&self) -> Option<&u64> {
        self.receive_packet_total.as_ref()
    }

    /// Sets the value of ReceiveProcessor
    pub fn set_receive_processor(&mut self, value: u32) {
        self.receive_processor = Some(value);
    }

    /// Gets the value of ReceiveProcessor
    pub fn get_receive_processor(&self) -> Option<&u32> {
        self.receive_processor.as_ref()
    }

    /// Sets the value of ReceiveProcessorGroup
    pub fn set_receive_processor_group(&mut self, value: u32) {
        self.receive_processor_group = Some(value);
    }

    /// Gets the value of ReceiveProcessorGroup
    pub fn get_receive_processor_group(&self) -> Option<&u32> {
        self.receive_processor_group.as_ref()
    }

    /// Sets the value of SendPacketCompletionsPerSecond
    pub fn set_send_packet_completions_per_second(&mut self, value: u64) {
        self.send_packet_completions_per_second = Some(value);
    }

    /// Gets the value of SendPacketCompletionsPerSecond
    pub fn get_send_packet_completions_per_second(&self) -> Option<&u64> {
        self.send_packet_completions_per_second.as_ref()
    }

    /// Sets the value of SendPacketCompletionsTotal
    pub fn set_send_packet_completions_total(&mut self, value: u64) {
        self.send_packet_completions_total = Some(value);
    }

    /// Gets the value of SendPacketCompletionsTotal
    pub fn get_send_packet_completions_total(&self) -> Option<&u64> {
        self.send_packet_completions_total.as_ref()
    }

    /// Sets the value of SendPacketPerSecond
    pub fn set_send_packet_per_second(&mut self, value: u64) {
        self.send_packet_per_second = Some(value);
    }

    /// Gets the value of SendPacketPerSecond
    pub fn get_send_packet_per_second(&self) -> Option<&u64> {
        self.send_packet_per_second.as_ref()
    }

    /// Sets the value of SendPacketTotal
    pub fn set_send_packet_total(&mut self, value: u64) {
        self.send_packet_total = Some(value);
    }

    /// Gets the value of SendPacketTotal
    pub fn get_send_packet_total(&self) -> Option<&u64> {
        self.send_packet_total.as_ref()
    }

    /// Sets the value of SendProcessor
    pub fn set_send_processor(&mut self, value: u32) {
        self.send_processor = Some(value);
    }

    /// Gets the value of SendProcessor
    pub fn get_send_processor(&self) -> Option<&u32> {
        self.send_processor.as_ref()
    }

    /// Sets the value of SendProcessorGroup
    pub fn set_send_processor_group(&mut self, value: u32) {
        self.send_processor_group = Some(value);
    }

    /// Gets the value of SendProcessorGroup
    pub fn get_send_processor_group(&self) -> Option<&u32> {
        self.send_processor_group.as_ref()
    }
}

