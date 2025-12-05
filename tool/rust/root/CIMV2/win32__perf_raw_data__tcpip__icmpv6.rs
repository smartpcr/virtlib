// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_Tcpip_ICMPv6 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_Tcpip_ICMPv6 {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

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
    #[serde(rename = "ReceivedDestUnreachable")]
    pub received_dest_unreachable: Option<u32>,

/// 
    #[serde(rename = "ReceivedEchoPersec")]
    pub received_echo_persec: Option<u32>,

/// 
    #[serde(rename = "ReceivedEchoReplyPersec")]
    pub received_echo_reply_persec: Option<u32>,

/// 
    #[serde(rename = "ReceivedMembershipQuery")]
    pub received_membership_query: Option<u32>,

/// 
    #[serde(rename = "ReceivedMembershipReduction")]
    pub received_membership_reduction: Option<u32>,

/// 
    #[serde(rename = "ReceivedMembershipReport")]
    pub received_membership_report: Option<u32>,

/// 
    #[serde(rename = "ReceivedNeighborAdvert")]
    pub received_neighbor_advert: Option<u32>,

/// 
    #[serde(rename = "ReceivedNeighborSolicit")]
    pub received_neighbor_solicit: Option<u32>,

/// 
    #[serde(rename = "ReceivedPacketTooBig")]
    pub received_packet_too_big: Option<u32>,

/// 
    #[serde(rename = "ReceivedParameterProblem")]
    pub received_parameter_problem: Option<u32>,

/// 
    #[serde(rename = "ReceivedRedirectPersec")]
    pub received_redirect_persec: Option<u32>,

/// 
    #[serde(rename = "ReceivedRouterAdvert")]
    pub received_router_advert: Option<u32>,

/// 
    #[serde(rename = "ReceivedRouterSolicit")]
    pub received_router_solicit: Option<u32>,

/// 
    #[serde(rename = "ReceivedTimeExceeded")]
    pub received_time_exceeded: Option<u32>,

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
    #[serde(rename = "SentMembershipQuery")]
    pub sent_membership_query: Option<u32>,

/// 
    #[serde(rename = "SentMembershipReduction")]
    pub sent_membership_reduction: Option<u32>,

/// 
    #[serde(rename = "SentMembershipReport")]
    pub sent_membership_report: Option<u32>,

/// 
    #[serde(rename = "SentNeighborAdvert")]
    pub sent_neighbor_advert: Option<u32>,

/// 
    #[serde(rename = "SentNeighborSolicit")]
    pub sent_neighbor_solicit: Option<u32>,

/// 
    #[serde(rename = "SentPacketTooBig")]
    pub sent_packet_too_big: Option<u32>,

/// 
    #[serde(rename = "SentParameterProblem")]
    pub sent_parameter_problem: Option<u32>,

/// 
    #[serde(rename = "SentRedirectPersec")]
    pub sent_redirect_persec: Option<u32>,

/// 
    #[serde(rename = "SentRouterAdvert")]
    pub sent_router_advert: Option<u32>,

/// 
    #[serde(rename = "SentRouterSolicit")]
    pub sent_router_solicit: Option<u32>,

/// 
    #[serde(rename = "SentTimeExceeded")]
    pub sent_time_exceeded: Option<u32>,
}

impl Win32_PerfRawData_Tcpip_ICMPv6 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            messages_outbound_errors: None,
            messages_persec: None,
            messages_received_errors: None,
            messages_received_persec: None,
            messages_sent_persec: None,
            received_dest_unreachable: None,
            received_echo_persec: None,
            received_echo_reply_persec: None,
            received_membership_query: None,
            received_membership_reduction: None,
            received_membership_report: None,
            received_neighbor_advert: None,
            received_neighbor_solicit: None,
            received_packet_too_big: None,
            received_parameter_problem: None,
            received_redirect_persec: None,
            received_router_advert: None,
            received_router_solicit: None,
            received_time_exceeded: None,
            sent_destination_unreachable: None,
            sent_echo_persec: None,
            sent_echo_reply_persec: None,
            sent_membership_query: None,
            sent_membership_reduction: None,
            sent_membership_report: None,
            sent_neighbor_advert: None,
            sent_neighbor_solicit: None,
            sent_packet_too_big: None,
            sent_parameter_problem: None,
            sent_redirect_persec: None,
            sent_router_advert: None,
            sent_router_solicit: None,
            sent_time_exceeded: None,
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

    /// Sets the value of ReceivedMembershipQuery
    pub fn set_received_membership_query(&mut self, value: u32) {
        self.received_membership_query = Some(value);
    }

    /// Gets the value of ReceivedMembershipQuery
    pub fn get_received_membership_query(&self) -> Option<&u32> {
        self.received_membership_query.as_ref()
    }

    /// Sets the value of ReceivedMembershipReduction
    pub fn set_received_membership_reduction(&mut self, value: u32) {
        self.received_membership_reduction = Some(value);
    }

    /// Gets the value of ReceivedMembershipReduction
    pub fn get_received_membership_reduction(&self) -> Option<&u32> {
        self.received_membership_reduction.as_ref()
    }

    /// Sets the value of ReceivedMembershipReport
    pub fn set_received_membership_report(&mut self, value: u32) {
        self.received_membership_report = Some(value);
    }

    /// Gets the value of ReceivedMembershipReport
    pub fn get_received_membership_report(&self) -> Option<&u32> {
        self.received_membership_report.as_ref()
    }

    /// Sets the value of ReceivedNeighborAdvert
    pub fn set_received_neighbor_advert(&mut self, value: u32) {
        self.received_neighbor_advert = Some(value);
    }

    /// Gets the value of ReceivedNeighborAdvert
    pub fn get_received_neighbor_advert(&self) -> Option<&u32> {
        self.received_neighbor_advert.as_ref()
    }

    /// Sets the value of ReceivedNeighborSolicit
    pub fn set_received_neighbor_solicit(&mut self, value: u32) {
        self.received_neighbor_solicit = Some(value);
    }

    /// Gets the value of ReceivedNeighborSolicit
    pub fn get_received_neighbor_solicit(&self) -> Option<&u32> {
        self.received_neighbor_solicit.as_ref()
    }

    /// Sets the value of ReceivedPacketTooBig
    pub fn set_received_packet_too_big(&mut self, value: u32) {
        self.received_packet_too_big = Some(value);
    }

    /// Gets the value of ReceivedPacketTooBig
    pub fn get_received_packet_too_big(&self) -> Option<&u32> {
        self.received_packet_too_big.as_ref()
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

    /// Sets the value of ReceivedRouterAdvert
    pub fn set_received_router_advert(&mut self, value: u32) {
        self.received_router_advert = Some(value);
    }

    /// Gets the value of ReceivedRouterAdvert
    pub fn get_received_router_advert(&self) -> Option<&u32> {
        self.received_router_advert.as_ref()
    }

    /// Sets the value of ReceivedRouterSolicit
    pub fn set_received_router_solicit(&mut self, value: u32) {
        self.received_router_solicit = Some(value);
    }

    /// Gets the value of ReceivedRouterSolicit
    pub fn get_received_router_solicit(&self) -> Option<&u32> {
        self.received_router_solicit.as_ref()
    }

    /// Sets the value of ReceivedTimeExceeded
    pub fn set_received_time_exceeded(&mut self, value: u32) {
        self.received_time_exceeded = Some(value);
    }

    /// Gets the value of ReceivedTimeExceeded
    pub fn get_received_time_exceeded(&self) -> Option<&u32> {
        self.received_time_exceeded.as_ref()
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

    /// Sets the value of SentMembershipQuery
    pub fn set_sent_membership_query(&mut self, value: u32) {
        self.sent_membership_query = Some(value);
    }

    /// Gets the value of SentMembershipQuery
    pub fn get_sent_membership_query(&self) -> Option<&u32> {
        self.sent_membership_query.as_ref()
    }

    /// Sets the value of SentMembershipReduction
    pub fn set_sent_membership_reduction(&mut self, value: u32) {
        self.sent_membership_reduction = Some(value);
    }

    /// Gets the value of SentMembershipReduction
    pub fn get_sent_membership_reduction(&self) -> Option<&u32> {
        self.sent_membership_reduction.as_ref()
    }

    /// Sets the value of SentMembershipReport
    pub fn set_sent_membership_report(&mut self, value: u32) {
        self.sent_membership_report = Some(value);
    }

    /// Gets the value of SentMembershipReport
    pub fn get_sent_membership_report(&self) -> Option<&u32> {
        self.sent_membership_report.as_ref()
    }

    /// Sets the value of SentNeighborAdvert
    pub fn set_sent_neighbor_advert(&mut self, value: u32) {
        self.sent_neighbor_advert = Some(value);
    }

    /// Gets the value of SentNeighborAdvert
    pub fn get_sent_neighbor_advert(&self) -> Option<&u32> {
        self.sent_neighbor_advert.as_ref()
    }

    /// Sets the value of SentNeighborSolicit
    pub fn set_sent_neighbor_solicit(&mut self, value: u32) {
        self.sent_neighbor_solicit = Some(value);
    }

    /// Gets the value of SentNeighborSolicit
    pub fn get_sent_neighbor_solicit(&self) -> Option<&u32> {
        self.sent_neighbor_solicit.as_ref()
    }

    /// Sets the value of SentPacketTooBig
    pub fn set_sent_packet_too_big(&mut self, value: u32) {
        self.sent_packet_too_big = Some(value);
    }

    /// Gets the value of SentPacketTooBig
    pub fn get_sent_packet_too_big(&self) -> Option<&u32> {
        self.sent_packet_too_big.as_ref()
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

    /// Sets the value of SentRouterAdvert
    pub fn set_sent_router_advert(&mut self, value: u32) {
        self.sent_router_advert = Some(value);
    }

    /// Gets the value of SentRouterAdvert
    pub fn get_sent_router_advert(&self) -> Option<&u32> {
        self.sent_router_advert.as_ref()
    }

    /// Sets the value of SentRouterSolicit
    pub fn set_sent_router_solicit(&mut self, value: u32) {
        self.sent_router_solicit = Some(value);
    }

    /// Gets the value of SentRouterSolicit
    pub fn get_sent_router_solicit(&self) -> Option<&u32> {
        self.sent_router_solicit.as_ref()
    }

    /// Sets the value of SentTimeExceeded
    pub fn set_sent_time_exceeded(&mut self, value: u32) {
        self.sent_time_exceeded = Some(value);
    }

    /// Gets the value of SentTimeExceeded
    pub fn get_sent_time_exceeded(&self) -> Option<&u32> {
        self.sent_time_exceeded.as_ref()
    }
}

