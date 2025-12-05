// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_TCPIPCounters_TCPIPTransportLayerPacketDropCounters struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_TCPIPCounters_TCPIPTransportLayerPacketDropCounters {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "InetDiscardAcceptInspection")]
    pub inet_discard_accept_inspection: Option<u64>,

/// 
    #[serde(rename = "InetDiscardAcceptRedirection")]
    pub inet_discard_accept_redirection: Option<u64>,

/// 
    #[serde(rename = "InetDiscardAckInvalid")]
    pub inet_discard_ack_invalid: Option<u64>,

/// 
    #[serde(rename = "InetDiscardChecksumInvalid")]
    pub inet_discard_checksum_invalid: Option<u64>,

/// 
    #[serde(rename = "InetDiscardClosedWindow")]
    pub inet_discard_closed_window: Option<u64>,

/// 
    #[serde(rename = "InetDiscardConnectedPath")]
    pub inet_discard_connected_path: Option<u64>,

/// 
    #[serde(rename = "InetDiscardDestinationMulticast")]
    pub inet_discard_destination_multicast: Option<u64>,

/// 
    #[serde(rename = "InetDiscardDuplicateSegment")]
    pub inet_discard_duplicate_segment: Option<u64>,

/// 
    #[serde(rename = "InetDiscardEndpointNotFound")]
    pub inet_discard_endpoint_not_found: Option<u64>,

/// 
    #[serde(rename = "InetDiscardExpectedSyn")]
    pub inet_discard_expected_syn: Option<u64>,

/// 
    #[serde(rename = "InetDiscardFinReceived")]
    pub inet_discard_fin_received: Option<u64>,

/// 
    #[serde(rename = "InetDiscardFinWait2")]
    pub inet_discard_fin_wait2: Option<u64>,

/// 
    #[serde(rename = "InetDiscardHeaderInvalid")]
    pub inet_discard_header_invalid: Option<u64>,

/// 
    #[serde(rename = "InetDiscardLandAttack")]
    pub inet_discard_land_attack: Option<u64>,

/// 
    #[serde(rename = "InetDiscardListenerInvalidFlags")]
    pub inet_discard_listener_invalid_flags: Option<u64>,

/// 
    #[serde(rename = "InetDiscardMissedReset")]
    pub inet_discard_missed_reset: Option<u64>,

/// 
    #[serde(rename = "InetDiscardOutsideWindow")]
    pub inet_discard_outside_window: Option<u64>,

/// 
    #[serde(rename = "InetDiscardPauseAccept")]
    pub inet_discard_pause_accept: Option<u64>,

/// 
    #[serde(rename = "InetDiscardPawsFailed")]
    pub inet_discard_paws_failed: Option<u64>,

/// 
    #[serde(rename = "InetDiscardReassemblyConflict")]
    pub inet_discard_reassembly_conflict: Option<u64>,

/// 
    #[serde(rename = "InetDiscardReceiveInspection")]
    pub inet_discard_receive_inspection: Option<u64>,

/// 
    #[serde(rename = "InetDiscardRst")]
    pub inet_discard_rst: Option<u64>,

/// 
    #[serde(rename = "InetDiscardSessionState")]
    pub inet_discard_session_state: Option<u64>,

/// 
    #[serde(rename = "InetDiscardSimultaneousConnect")]
    pub inet_discard_simultaneous_connect: Option<u64>,

/// 
    #[serde(rename = "InetDiscardSourceUnspecified")]
    pub inet_discard_source_unspecified: Option<u64>,

/// 
    #[serde(rename = "InetDiscardSynAckWithFastopenCookieRequest")]
    pub inet_discard_syn_ack_with_fastopen_cookie_request: Option<u64>,

/// 
    #[serde(rename = "InetDiscardSynAttack")]
    pub inet_discard_syn_attack: Option<u64>,

/// 
    #[serde(rename = "InetDiscardSynRcvdSyn")]
    pub inet_discard_syn_rcvd_syn: Option<u64>,

/// 
    #[serde(rename = "InetDiscardTcbNotInTcbTable")]
    pub inet_discard_tcb_not_in_tcb_table: Option<u64>,

/// 
    #[serde(rename = "InetDiscardTcbRemoved")]
    pub inet_discard_tcb_removed: Option<u64>,

/// 
    #[serde(rename = "InetDiscardTimeWaitTcb")]
    pub inet_discard_time_wait_tcb: Option<u64>,

/// 
    #[serde(rename = "InetDiscardTimeWaitTcbReceivedRstOutsideWindow")]
    pub inet_discard_time_wait_tcb_received_rst_outside_window: Option<u64>,

/// 
    #[serde(rename = "InetDiscardTimeWaitTcbSynAndOtherFlags")]
    pub inet_discard_time_wait_tcb_syn_and_other_flags: Option<u64>,

/// 
    #[serde(rename = "InetDiscardUrgentDeliveryAllocationFailure")]
    pub inet_discard_urgent_delivery_allocation_failure: Option<u64>,
}

impl Win32_PerfFormattedData_TCPIPCounters_TCPIPTransportLayerPacketDropCounters {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            inet_discard_accept_inspection: None,
            inet_discard_accept_redirection: None,
            inet_discard_ack_invalid: None,
            inet_discard_checksum_invalid: None,
            inet_discard_closed_window: None,
            inet_discard_connected_path: None,
            inet_discard_destination_multicast: None,
            inet_discard_duplicate_segment: None,
            inet_discard_endpoint_not_found: None,
            inet_discard_expected_syn: None,
            inet_discard_fin_received: None,
            inet_discard_fin_wait2: None,
            inet_discard_header_invalid: None,
            inet_discard_land_attack: None,
            inet_discard_listener_invalid_flags: None,
            inet_discard_missed_reset: None,
            inet_discard_outside_window: None,
            inet_discard_pause_accept: None,
            inet_discard_paws_failed: None,
            inet_discard_reassembly_conflict: None,
            inet_discard_receive_inspection: None,
            inet_discard_rst: None,
            inet_discard_session_state: None,
            inet_discard_simultaneous_connect: None,
            inet_discard_source_unspecified: None,
            inet_discard_syn_ack_with_fastopen_cookie_request: None,
            inet_discard_syn_attack: None,
            inet_discard_syn_rcvd_syn: None,
            inet_discard_tcb_not_in_tcb_table: None,
            inet_discard_tcb_removed: None,
            inet_discard_time_wait_tcb: None,
            inet_discard_time_wait_tcb_received_rst_outside_window: None,
            inet_discard_time_wait_tcb_syn_and_other_flags: None,
            inet_discard_urgent_delivery_allocation_failure: None,
        }
    }


    /// Sets the value of InetDiscardAcceptInspection
    pub fn set_inet_discard_accept_inspection(&mut self, value: u64) {
        self.inet_discard_accept_inspection = Some(value);
    }

    /// Gets the value of InetDiscardAcceptInspection
    pub fn get_inet_discard_accept_inspection(&self) -> Option<&u64> {
        self.inet_discard_accept_inspection.as_ref()
    }

    /// Sets the value of InetDiscardAcceptRedirection
    pub fn set_inet_discard_accept_redirection(&mut self, value: u64) {
        self.inet_discard_accept_redirection = Some(value);
    }

    /// Gets the value of InetDiscardAcceptRedirection
    pub fn get_inet_discard_accept_redirection(&self) -> Option<&u64> {
        self.inet_discard_accept_redirection.as_ref()
    }

    /// Sets the value of InetDiscardAckInvalid
    pub fn set_inet_discard_ack_invalid(&mut self, value: u64) {
        self.inet_discard_ack_invalid = Some(value);
    }

    /// Gets the value of InetDiscardAckInvalid
    pub fn get_inet_discard_ack_invalid(&self) -> Option<&u64> {
        self.inet_discard_ack_invalid.as_ref()
    }

    /// Sets the value of InetDiscardChecksumInvalid
    pub fn set_inet_discard_checksum_invalid(&mut self, value: u64) {
        self.inet_discard_checksum_invalid = Some(value);
    }

    /// Gets the value of InetDiscardChecksumInvalid
    pub fn get_inet_discard_checksum_invalid(&self) -> Option<&u64> {
        self.inet_discard_checksum_invalid.as_ref()
    }

    /// Sets the value of InetDiscardClosedWindow
    pub fn set_inet_discard_closed_window(&mut self, value: u64) {
        self.inet_discard_closed_window = Some(value);
    }

    /// Gets the value of InetDiscardClosedWindow
    pub fn get_inet_discard_closed_window(&self) -> Option<&u64> {
        self.inet_discard_closed_window.as_ref()
    }

    /// Sets the value of InetDiscardConnectedPath
    pub fn set_inet_discard_connected_path(&mut self, value: u64) {
        self.inet_discard_connected_path = Some(value);
    }

    /// Gets the value of InetDiscardConnectedPath
    pub fn get_inet_discard_connected_path(&self) -> Option<&u64> {
        self.inet_discard_connected_path.as_ref()
    }

    /// Sets the value of InetDiscardDestinationMulticast
    pub fn set_inet_discard_destination_multicast(&mut self, value: u64) {
        self.inet_discard_destination_multicast = Some(value);
    }

    /// Gets the value of InetDiscardDestinationMulticast
    pub fn get_inet_discard_destination_multicast(&self) -> Option<&u64> {
        self.inet_discard_destination_multicast.as_ref()
    }

    /// Sets the value of InetDiscardDuplicateSegment
    pub fn set_inet_discard_duplicate_segment(&mut self, value: u64) {
        self.inet_discard_duplicate_segment = Some(value);
    }

    /// Gets the value of InetDiscardDuplicateSegment
    pub fn get_inet_discard_duplicate_segment(&self) -> Option<&u64> {
        self.inet_discard_duplicate_segment.as_ref()
    }

    /// Sets the value of InetDiscardEndpointNotFound
    pub fn set_inet_discard_endpoint_not_found(&mut self, value: u64) {
        self.inet_discard_endpoint_not_found = Some(value);
    }

    /// Gets the value of InetDiscardEndpointNotFound
    pub fn get_inet_discard_endpoint_not_found(&self) -> Option<&u64> {
        self.inet_discard_endpoint_not_found.as_ref()
    }

    /// Sets the value of InetDiscardExpectedSyn
    pub fn set_inet_discard_expected_syn(&mut self, value: u64) {
        self.inet_discard_expected_syn = Some(value);
    }

    /// Gets the value of InetDiscardExpectedSyn
    pub fn get_inet_discard_expected_syn(&self) -> Option<&u64> {
        self.inet_discard_expected_syn.as_ref()
    }

    /// Sets the value of InetDiscardFinReceived
    pub fn set_inet_discard_fin_received(&mut self, value: u64) {
        self.inet_discard_fin_received = Some(value);
    }

    /// Gets the value of InetDiscardFinReceived
    pub fn get_inet_discard_fin_received(&self) -> Option<&u64> {
        self.inet_discard_fin_received.as_ref()
    }

    /// Sets the value of InetDiscardFinWait2
    pub fn set_inet_discard_fin_wait2(&mut self, value: u64) {
        self.inet_discard_fin_wait2 = Some(value);
    }

    /// Gets the value of InetDiscardFinWait2
    pub fn get_inet_discard_fin_wait2(&self) -> Option<&u64> {
        self.inet_discard_fin_wait2.as_ref()
    }

    /// Sets the value of InetDiscardHeaderInvalid
    pub fn set_inet_discard_header_invalid(&mut self, value: u64) {
        self.inet_discard_header_invalid = Some(value);
    }

    /// Gets the value of InetDiscardHeaderInvalid
    pub fn get_inet_discard_header_invalid(&self) -> Option<&u64> {
        self.inet_discard_header_invalid.as_ref()
    }

    /// Sets the value of InetDiscardLandAttack
    pub fn set_inet_discard_land_attack(&mut self, value: u64) {
        self.inet_discard_land_attack = Some(value);
    }

    /// Gets the value of InetDiscardLandAttack
    pub fn get_inet_discard_land_attack(&self) -> Option<&u64> {
        self.inet_discard_land_attack.as_ref()
    }

    /// Sets the value of InetDiscardListenerInvalidFlags
    pub fn set_inet_discard_listener_invalid_flags(&mut self, value: u64) {
        self.inet_discard_listener_invalid_flags = Some(value);
    }

    /// Gets the value of InetDiscardListenerInvalidFlags
    pub fn get_inet_discard_listener_invalid_flags(&self) -> Option<&u64> {
        self.inet_discard_listener_invalid_flags.as_ref()
    }

    /// Sets the value of InetDiscardMissedReset
    pub fn set_inet_discard_missed_reset(&mut self, value: u64) {
        self.inet_discard_missed_reset = Some(value);
    }

    /// Gets the value of InetDiscardMissedReset
    pub fn get_inet_discard_missed_reset(&self) -> Option<&u64> {
        self.inet_discard_missed_reset.as_ref()
    }

    /// Sets the value of InetDiscardOutsideWindow
    pub fn set_inet_discard_outside_window(&mut self, value: u64) {
        self.inet_discard_outside_window = Some(value);
    }

    /// Gets the value of InetDiscardOutsideWindow
    pub fn get_inet_discard_outside_window(&self) -> Option<&u64> {
        self.inet_discard_outside_window.as_ref()
    }

    /// Sets the value of InetDiscardPauseAccept
    pub fn set_inet_discard_pause_accept(&mut self, value: u64) {
        self.inet_discard_pause_accept = Some(value);
    }

    /// Gets the value of InetDiscardPauseAccept
    pub fn get_inet_discard_pause_accept(&self) -> Option<&u64> {
        self.inet_discard_pause_accept.as_ref()
    }

    /// Sets the value of InetDiscardPawsFailed
    pub fn set_inet_discard_paws_failed(&mut self, value: u64) {
        self.inet_discard_paws_failed = Some(value);
    }

    /// Gets the value of InetDiscardPawsFailed
    pub fn get_inet_discard_paws_failed(&self) -> Option<&u64> {
        self.inet_discard_paws_failed.as_ref()
    }

    /// Sets the value of InetDiscardReassemblyConflict
    pub fn set_inet_discard_reassembly_conflict(&mut self, value: u64) {
        self.inet_discard_reassembly_conflict = Some(value);
    }

    /// Gets the value of InetDiscardReassemblyConflict
    pub fn get_inet_discard_reassembly_conflict(&self) -> Option<&u64> {
        self.inet_discard_reassembly_conflict.as_ref()
    }

    /// Sets the value of InetDiscardReceiveInspection
    pub fn set_inet_discard_receive_inspection(&mut self, value: u64) {
        self.inet_discard_receive_inspection = Some(value);
    }

    /// Gets the value of InetDiscardReceiveInspection
    pub fn get_inet_discard_receive_inspection(&self) -> Option<&u64> {
        self.inet_discard_receive_inspection.as_ref()
    }

    /// Sets the value of InetDiscardRst
    pub fn set_inet_discard_rst(&mut self, value: u64) {
        self.inet_discard_rst = Some(value);
    }

    /// Gets the value of InetDiscardRst
    pub fn get_inet_discard_rst(&self) -> Option<&u64> {
        self.inet_discard_rst.as_ref()
    }

    /// Sets the value of InetDiscardSessionState
    pub fn set_inet_discard_session_state(&mut self, value: u64) {
        self.inet_discard_session_state = Some(value);
    }

    /// Gets the value of InetDiscardSessionState
    pub fn get_inet_discard_session_state(&self) -> Option<&u64> {
        self.inet_discard_session_state.as_ref()
    }

    /// Sets the value of InetDiscardSimultaneousConnect
    pub fn set_inet_discard_simultaneous_connect(&mut self, value: u64) {
        self.inet_discard_simultaneous_connect = Some(value);
    }

    /// Gets the value of InetDiscardSimultaneousConnect
    pub fn get_inet_discard_simultaneous_connect(&self) -> Option<&u64> {
        self.inet_discard_simultaneous_connect.as_ref()
    }

    /// Sets the value of InetDiscardSourceUnspecified
    pub fn set_inet_discard_source_unspecified(&mut self, value: u64) {
        self.inet_discard_source_unspecified = Some(value);
    }

    /// Gets the value of InetDiscardSourceUnspecified
    pub fn get_inet_discard_source_unspecified(&self) -> Option<&u64> {
        self.inet_discard_source_unspecified.as_ref()
    }

    /// Sets the value of InetDiscardSynAckWithFastopenCookieRequest
    pub fn set_inet_discard_syn_ack_with_fastopen_cookie_request(&mut self, value: u64) {
        self.inet_discard_syn_ack_with_fastopen_cookie_request = Some(value);
    }

    /// Gets the value of InetDiscardSynAckWithFastopenCookieRequest
    pub fn get_inet_discard_syn_ack_with_fastopen_cookie_request(&self) -> Option<&u64> {
        self.inet_discard_syn_ack_with_fastopen_cookie_request.as_ref()
    }

    /// Sets the value of InetDiscardSynAttack
    pub fn set_inet_discard_syn_attack(&mut self, value: u64) {
        self.inet_discard_syn_attack = Some(value);
    }

    /// Gets the value of InetDiscardSynAttack
    pub fn get_inet_discard_syn_attack(&self) -> Option<&u64> {
        self.inet_discard_syn_attack.as_ref()
    }

    /// Sets the value of InetDiscardSynRcvdSyn
    pub fn set_inet_discard_syn_rcvd_syn(&mut self, value: u64) {
        self.inet_discard_syn_rcvd_syn = Some(value);
    }

    /// Gets the value of InetDiscardSynRcvdSyn
    pub fn get_inet_discard_syn_rcvd_syn(&self) -> Option<&u64> {
        self.inet_discard_syn_rcvd_syn.as_ref()
    }

    /// Sets the value of InetDiscardTcbNotInTcbTable
    pub fn set_inet_discard_tcb_not_in_tcb_table(&mut self, value: u64) {
        self.inet_discard_tcb_not_in_tcb_table = Some(value);
    }

    /// Gets the value of InetDiscardTcbNotInTcbTable
    pub fn get_inet_discard_tcb_not_in_tcb_table(&self) -> Option<&u64> {
        self.inet_discard_tcb_not_in_tcb_table.as_ref()
    }

    /// Sets the value of InetDiscardTcbRemoved
    pub fn set_inet_discard_tcb_removed(&mut self, value: u64) {
        self.inet_discard_tcb_removed = Some(value);
    }

    /// Gets the value of InetDiscardTcbRemoved
    pub fn get_inet_discard_tcb_removed(&self) -> Option<&u64> {
        self.inet_discard_tcb_removed.as_ref()
    }

    /// Sets the value of InetDiscardTimeWaitTcb
    pub fn set_inet_discard_time_wait_tcb(&mut self, value: u64) {
        self.inet_discard_time_wait_tcb = Some(value);
    }

    /// Gets the value of InetDiscardTimeWaitTcb
    pub fn get_inet_discard_time_wait_tcb(&self) -> Option<&u64> {
        self.inet_discard_time_wait_tcb.as_ref()
    }

    /// Sets the value of InetDiscardTimeWaitTcbReceivedRstOutsideWindow
    pub fn set_inet_discard_time_wait_tcb_received_rst_outside_window(&mut self, value: u64) {
        self.inet_discard_time_wait_tcb_received_rst_outside_window = Some(value);
    }

    /// Gets the value of InetDiscardTimeWaitTcbReceivedRstOutsideWindow
    pub fn get_inet_discard_time_wait_tcb_received_rst_outside_window(&self) -> Option<&u64> {
        self.inet_discard_time_wait_tcb_received_rst_outside_window.as_ref()
    }

    /// Sets the value of InetDiscardTimeWaitTcbSynAndOtherFlags
    pub fn set_inet_discard_time_wait_tcb_syn_and_other_flags(&mut self, value: u64) {
        self.inet_discard_time_wait_tcb_syn_and_other_flags = Some(value);
    }

    /// Gets the value of InetDiscardTimeWaitTcbSynAndOtherFlags
    pub fn get_inet_discard_time_wait_tcb_syn_and_other_flags(&self) -> Option<&u64> {
        self.inet_discard_time_wait_tcb_syn_and_other_flags.as_ref()
    }

    /// Sets the value of InetDiscardUrgentDeliveryAllocationFailure
    pub fn set_inet_discard_urgent_delivery_allocation_failure(&mut self, value: u64) {
        self.inet_discard_urgent_delivery_allocation_failure = Some(value);
    }

    /// Gets the value of InetDiscardUrgentDeliveryAllocationFailure
    pub fn get_inet_discard_urgent_delivery_allocation_failure(&self) -> Option<&u64> {
        self.inet_discard_urgent_delivery_allocation_failure.as_ref()
    }
}

