// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_Tcpip_ICMP struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_Tcpip_ICMP {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "MessagesOutboundErrors")]
    pub messages_outbound_errors: Option<u32>,

/// 
    #[serde(rename = "MessagesPersec")]
    pub messages_persec: Option<u32>,

/// 
    #[serde(rename = "MessagesReceivedErrors")]
    pub messages_received_errors: Option<u32>,

/// 
    #[serde(rename = "MessagesReceivedPersec")]
    pub messages_received_persec: Option<u32>,

/// 
    #[serde(rename = "MessagesSentPersec")]
    pub messages_sent_persec: Option<u32>,

/// 
    #[serde(rename = "ReceivedAddressMask")]
    pub received_address_mask: Option<u32>,

/// 
    #[serde(rename = "ReceivedAddressMaskReply")]
    pub received_address_mask_reply: Option<u32>,

/// 
    #[serde(rename = "ReceivedDestUnreachable")]
    pub received_dest_unreachable: Option<u32>,

/// 
    #[serde(rename = "ReceivedEchoPersec")]
    pub received_echo_persec: Option<u32>,

/// 
    #[serde(rename = "ReceivedEchoReplyPersec")]
    pub received_echo_reply_persec: Option<u32>,

/// 
    #[serde(rename = "ReceivedParameterProblem")]
    pub received_parameter_problem: Option<u32>,

/// 
    #[serde(rename = "ReceivedRedirectPersec")]
    pub received_redirect_persec: Option<u32>,

/// 
    #[serde(rename = "ReceivedSourceQuench")]
    pub received_source_quench: Option<u32>,

/// 
    #[serde(rename = "ReceivedTimeExceeded")]
    pub received_time_exceeded: Option<u32>,

/// 
    #[serde(rename = "ReceivedTimestampPersec")]
    pub received_timestamp_persec: Option<u32>,

/// 
    #[serde(rename = "ReceivedTimestampReplyPersec")]
    pub received_timestamp_reply_persec: Option<u32>,

/// 
    #[serde(rename = "SentAddressMask")]
    pub sent_address_mask: Option<u32>,

/// 
    #[serde(rename = "SentAddressMaskReply")]
    pub sent_address_mask_reply: Option<u32>,

/// 
    #[serde(rename = "SentDestinationUnreachable")]
    pub sent_destination_unreachable: Option<u32>,

/// 
    #[serde(rename = "SentEchoPersec")]
    pub sent_echo_persec: Option<u32>,

/// 
    #[serde(rename = "SentEchoReplyPersec")]
    pub sent_echo_reply_persec: Option<u32>,

/// 
    #[serde(rename = "SentParameterProblem")]
    pub sent_parameter_problem: Option<u32>,

/// 
    #[serde(rename = "SentRedirectPersec")]
    pub sent_redirect_persec: Option<u32>,

/// 
    #[serde(rename = "SentSourceQuench")]
    pub sent_source_quench: Option<u32>,

/// 
    #[serde(rename = "SentTimeExceeded")]
    pub sent_time_exceeded: Option<u32>,

/// 
    #[serde(rename = "SentTimestampPersec")]
    pub sent_timestamp_persec: Option<u32>,

/// 
    #[serde(rename = "SentTimestampReplyPersec")]
    pub sent_timestamp_reply_persec: Option<u32>,
}

impl Win32_PerfFormattedData_Tcpip_ICMP {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            messages_outbound_errors: None,
            messages_persec: None,
            messages_received_errors: None,
            messages_received_persec: None,
            messages_sent_persec: None,
            received_address_mask: None,
            received_address_mask_reply: None,
            received_dest_unreachable: None,
            received_echo_persec: None,
            received_echo_reply_persec: None,
            received_parameter_problem: None,
            received_redirect_persec: None,
            received_source_quench: None,
            received_time_exceeded: None,
            received_timestamp_persec: None,
            received_timestamp_reply_persec: None,
            sent_address_mask: None,
            sent_address_mask_reply: None,
            sent_destination_unreachable: None,
            sent_echo_persec: None,
            sent_echo_reply_persec: None,
            sent_parameter_problem: None,
            sent_redirect_persec: None,
            sent_source_quench: None,
            sent_time_exceeded: None,
            sent_timestamp_persec: None,
            sent_timestamp_reply_persec: None,
        }
    }


    /// Sets the value of MessagesOutboundErrors
    pub fn set_messages_outbound_errors(&mut self, value: u32) {
        self.messages_outbound_errors = Some(value);
    }

    /// Gets the value of MessagesOutboundErrors
    pub fn get_messages_outbound_errors(&self) -> Option<&u32> {
        self.messages_outbound_errors.as_ref()
    }

    /// Sets the value of MessagesPersec
    pub fn set_messages_persec(&mut self, value: u32) {
        self.messages_persec = Some(value);
    }

    /// Gets the value of MessagesPersec
    pub fn get_messages_persec(&self) -> Option<&u32> {
        self.messages_persec.as_ref()
    }

    /// Sets the value of MessagesReceivedErrors
    pub fn set_messages_received_errors(&mut self, value: u32) {
        self.messages_received_errors = Some(value);
    }

    /// Gets the value of MessagesReceivedErrors
    pub fn get_messages_received_errors(&self) -> Option<&u32> {
        self.messages_received_errors.as_ref()
    }

    /// Sets the value of MessagesReceivedPersec
    pub fn set_messages_received_persec(&mut self, value: u32) {
        self.messages_received_persec = Some(value);
    }

    /// Gets the value of MessagesReceivedPersec
    pub fn get_messages_received_persec(&self) -> Option<&u32> {
        self.messages_received_persec.as_ref()
    }

    /// Sets the value of MessagesSentPersec
    pub fn set_messages_sent_persec(&mut self, value: u32) {
        self.messages_sent_persec = Some(value);
    }

    /// Gets the value of MessagesSentPersec
    pub fn get_messages_sent_persec(&self) -> Option<&u32> {
        self.messages_sent_persec.as_ref()
    }

    /// Sets the value of ReceivedAddressMask
    pub fn set_received_address_mask(&mut self, value: u32) {
        self.received_address_mask = Some(value);
    }

    /// Gets the value of ReceivedAddressMask
    pub fn get_received_address_mask(&self) -> Option<&u32> {
        self.received_address_mask.as_ref()
    }

    /// Sets the value of ReceivedAddressMaskReply
    pub fn set_received_address_mask_reply(&mut self, value: u32) {
        self.received_address_mask_reply = Some(value);
    }

    /// Gets the value of ReceivedAddressMaskReply
    pub fn get_received_address_mask_reply(&self) -> Option<&u32> {
        self.received_address_mask_reply.as_ref()
    }

    /// Sets the value of ReceivedDestUnreachable
    pub fn set_received_dest_unreachable(&mut self, value: u32) {
        self.received_dest_unreachable = Some(value);
    }

    /// Gets the value of ReceivedDestUnreachable
    pub fn get_received_dest_unreachable(&self) -> Option<&u32> {
        self.received_dest_unreachable.as_ref()
    }

    /// Sets the value of ReceivedEchoPersec
    pub fn set_received_echo_persec(&mut self, value: u32) {
        self.received_echo_persec = Some(value);
    }

    /// Gets the value of ReceivedEchoPersec
    pub fn get_received_echo_persec(&self) -> Option<&u32> {
        self.received_echo_persec.as_ref()
    }

    /// Sets the value of ReceivedEchoReplyPersec
    pub fn set_received_echo_reply_persec(&mut self, value: u32) {
        self.received_echo_reply_persec = Some(value);
    }

    /// Gets the value of ReceivedEchoReplyPersec
    pub fn get_received_echo_reply_persec(&self) -> Option<&u32> {
        self.received_echo_reply_persec.as_ref()
    }

    /// Sets the value of ReceivedParameterProblem
    pub fn set_received_parameter_problem(&mut self, value: u32) {
        self.received_parameter_problem = Some(value);
    }

    /// Gets the value of ReceivedParameterProblem
    pub fn get_received_parameter_problem(&self) -> Option<&u32> {
        self.received_parameter_problem.as_ref()
    }

    /// Sets the value of ReceivedRedirectPersec
    pub fn set_received_redirect_persec(&mut self, value: u32) {
        self.received_redirect_persec = Some(value);
    }

    /// Gets the value of ReceivedRedirectPersec
    pub fn get_received_redirect_persec(&self) -> Option<&u32> {
        self.received_redirect_persec.as_ref()
    }

    /// Sets the value of ReceivedSourceQuench
    pub fn set_received_source_quench(&mut self, value: u32) {
        self.received_source_quench = Some(value);
    }

    /// Gets the value of ReceivedSourceQuench
    pub fn get_received_source_quench(&self) -> Option<&u32> {
        self.received_source_quench.as_ref()
    }

    /// Sets the value of ReceivedTimeExceeded
    pub fn set_received_time_exceeded(&mut self, value: u32) {
        self.received_time_exceeded = Some(value);
    }

    /// Gets the value of ReceivedTimeExceeded
    pub fn get_received_time_exceeded(&self) -> Option<&u32> {
        self.received_time_exceeded.as_ref()
    }

    /// Sets the value of ReceivedTimestampPersec
    pub fn set_received_timestamp_persec(&mut self, value: u32) {
        self.received_timestamp_persec = Some(value);
    }

    /// Gets the value of ReceivedTimestampPersec
    pub fn get_received_timestamp_persec(&self) -> Option<&u32> {
        self.received_timestamp_persec.as_ref()
    }

    /// Sets the value of ReceivedTimestampReplyPersec
    pub fn set_received_timestamp_reply_persec(&mut self, value: u32) {
        self.received_timestamp_reply_persec = Some(value);
    }

    /// Gets the value of ReceivedTimestampReplyPersec
    pub fn get_received_timestamp_reply_persec(&self) -> Option<&u32> {
        self.received_timestamp_reply_persec.as_ref()
    }

    /// Sets the value of SentAddressMask
    pub fn set_sent_address_mask(&mut self, value: u32) {
        self.sent_address_mask = Some(value);
    }

    /// Gets the value of SentAddressMask
    pub fn get_sent_address_mask(&self) -> Option<&u32> {
        self.sent_address_mask.as_ref()
    }

    /// Sets the value of SentAddressMaskReply
    pub fn set_sent_address_mask_reply(&mut self, value: u32) {
        self.sent_address_mask_reply = Some(value);
    }

    /// Gets the value of SentAddressMaskReply
    pub fn get_sent_address_mask_reply(&self) -> Option<&u32> {
        self.sent_address_mask_reply.as_ref()
    }

    /// Sets the value of SentDestinationUnreachable
    pub fn set_sent_destination_unreachable(&mut self, value: u32) {
        self.sent_destination_unreachable = Some(value);
    }

    /// Gets the value of SentDestinationUnreachable
    pub fn get_sent_destination_unreachable(&self) -> Option<&u32> {
        self.sent_destination_unreachable.as_ref()
    }

    /// Sets the value of SentEchoPersec
    pub fn set_sent_echo_persec(&mut self, value: u32) {
        self.sent_echo_persec = Some(value);
    }

    /// Gets the value of SentEchoPersec
    pub fn get_sent_echo_persec(&self) -> Option<&u32> {
        self.sent_echo_persec.as_ref()
    }

    /// Sets the value of SentEchoReplyPersec
    pub fn set_sent_echo_reply_persec(&mut self, value: u32) {
        self.sent_echo_reply_persec = Some(value);
    }

    /// Gets the value of SentEchoReplyPersec
    pub fn get_sent_echo_reply_persec(&self) -> Option<&u32> {
        self.sent_echo_reply_persec.as_ref()
    }

    /// Sets the value of SentParameterProblem
    pub fn set_sent_parameter_problem(&mut self, value: u32) {
        self.sent_parameter_problem = Some(value);
    }

    /// Gets the value of SentParameterProblem
    pub fn get_sent_parameter_problem(&self) -> Option<&u32> {
        self.sent_parameter_problem.as_ref()
    }

    /// Sets the value of SentRedirectPersec
    pub fn set_sent_redirect_persec(&mut self, value: u32) {
        self.sent_redirect_persec = Some(value);
    }

    /// Gets the value of SentRedirectPersec
    pub fn get_sent_redirect_persec(&self) -> Option<&u32> {
        self.sent_redirect_persec.as_ref()
    }

    /// Sets the value of SentSourceQuench
    pub fn set_sent_source_quench(&mut self, value: u32) {
        self.sent_source_quench = Some(value);
    }

    /// Gets the value of SentSourceQuench
    pub fn get_sent_source_quench(&self) -> Option<&u32> {
        self.sent_source_quench.as_ref()
    }

    /// Sets the value of SentTimeExceeded
    pub fn set_sent_time_exceeded(&mut self, value: u32) {
        self.sent_time_exceeded = Some(value);
    }

    /// Gets the value of SentTimeExceeded
    pub fn get_sent_time_exceeded(&self) -> Option<&u32> {
        self.sent_time_exceeded.as_ref()
    }

    /// Sets the value of SentTimestampPersec
    pub fn set_sent_timestamp_persec(&mut self, value: u32) {
        self.sent_timestamp_persec = Some(value);
    }

    /// Gets the value of SentTimestampPersec
    pub fn get_sent_timestamp_persec(&self) -> Option<&u32> {
        self.sent_timestamp_persec.as_ref()
    }

    /// Sets the value of SentTimestampReplyPersec
    pub fn set_sent_timestamp_reply_persec(&mut self, value: u32) {
        self.sent_timestamp_reply_persec = Some(value);
    }

    /// Gets the value of SentTimestampReplyPersec
    pub fn get_sent_timestamp_reply_persec(&self) -> Option<&u32> {
        self.sent_timestamp_reply_persec.as_ref()
    }
}

