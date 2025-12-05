// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_QuicCounters_QUICPerformanceDiagnostics struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_QuicCounters_QUICPerformanceDiagnostics {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "QUICAppReceivedBytesPersec")]
    pub quicapp_received_bytes_persec: Option<u32>,

/// 
    #[serde(rename = "QUICAppSentBytesPersec")]
    pub quicapp_sent_bytes_persec: Option<u32>,

/// 
    #[serde(rename = "QUICConnectionOperationsCompletedPersec")]
    pub quicconnection_operations_completed_persec: Option<u32>,

/// 
    #[serde(rename = "QUICConnectionOperationsQueued")]
    pub quicconnection_operations_queued: Option<u64>,

/// 
    #[serde(rename = "QUICConnectionOperationsQueuedPersec")]
    pub quicconnection_operations_queued_persec: Option<u32>,

/// 
    #[serde(rename = "QUICConnectionsActive")]
    pub quicconnections_active: Option<u64>,

/// 
    #[serde(rename = "QUICConnectionsConnected")]
    pub quicconnections_connected: Option<u64>,

/// 
    #[serde(rename = "QUICConnectionsCreated")]
    pub quicconnections_created: Option<u64>,

/// 
    #[serde(rename = "QUICConnectionsCreatedPersec")]
    pub quicconnections_created_persec: Option<u32>,

/// 
    #[serde(rename = "QUICConnectionsNoALPN")]
    pub quicconnections_no_alpn: Option<u64>,

/// 
    #[serde(rename = "QUICConnectionsNoALPNsPersec")]
    pub quicconnections_no_alpns_persec: Option<u32>,

/// 
    #[serde(rename = "QUICConnectionsProtocolError")]
    pub quicconnections_protocol_error: Option<u64>,

/// 
    #[serde(rename = "QUICConnectionsProtocolErrorsPersec")]
    pub quicconnections_protocol_errors_persec: Option<u32>,

/// 
    #[serde(rename = "QUICConnectionsQueued")]
    pub quicconnections_queued: Option<u64>,

/// 
    #[serde(rename = "QUICConnectionsRejected")]
    pub quicconnections_rejected: Option<u64>,

/// 
    #[serde(rename = "QUICConnectionsRejectedPersec")]
    pub quicconnections_rejected_persec: Option<u32>,

/// 
    #[serde(rename = "QUICConnectionsResumed")]
    pub quicconnections_resumed: Option<u64>,

/// 
    #[serde(rename = "QUICConnectionsResumedPersec")]
    pub quicconnections_resumed_persec: Option<u32>,

/// 
    #[serde(rename = "QUICHandshakesFailed")]
    pub quichandshakes_failed: Option<u64>,

/// 
    #[serde(rename = "QUICHandshakesFailedPersec")]
    pub quichandshakes_failed_persec: Option<u32>,

/// 
    #[serde(rename = "QUICPacketDecryptionFailuresPersec")]
    pub quicpacket_decryption_failures_persec: Option<u32>,

/// 
    #[serde(rename = "QUICPacketsDroppedPersec")]
    pub quicpackets_dropped_persec: Option<u32>,

/// 
    #[serde(rename = "QUICPacketsSuspectedLostPersec")]
    pub quicpackets_suspected_lost_persec: Option<u32>,

/// 
    #[serde(rename = "QUICPathChallengesFailed")]
    pub quicpath_challenges_failed: Option<u64>,

/// 
    #[serde(rename = "QUICPathChallengesSucceeded")]
    pub quicpath_challenges_succeeded: Option<u64>,

/// 
    #[serde(rename = "QUICStatelessResetsSent")]
    pub quicstateless_resets_sent: Option<u64>,

/// 
    #[serde(rename = "QUICStatelessRetriesSent")]
    pub quicstateless_retries_sent: Option<u64>,

/// 
    #[serde(rename = "QUICStreamsActive")]
    pub quicstreams_active: Option<u64>,

/// 
    #[serde(rename = "QUICUDPDatagramsReceivedPersec")]
    pub quicudpdatagrams_received_persec: Option<u32>,

/// 
    #[serde(rename = "QUICUDPDatagramsSentPersec")]
    pub quicudpdatagrams_sent_persec: Option<u32>,

/// 
    #[serde(rename = "QUICUDPPayloadBytesReceivedPersec")]
    pub quicudppayload_bytes_received_persec: Option<u32>,

/// 
    #[serde(rename = "QUICUDPPayloadBytesSentPersec")]
    pub quicudppayload_bytes_sent_persec: Option<u32>,

/// 
    #[serde(rename = "QUICUDPReceiveEventsPersec")]
    pub quicudpreceive_events_persec: Option<u32>,

/// 
    #[serde(rename = "QUICUDPSendCallsPersec")]
    pub quicudpsend_calls_persec: Option<u32>,

/// 
    #[serde(rename = "QUICWorkerOperationsPersec")]
    pub quicworker_operations_persec: Option<u32>,

/// 
    #[serde(rename = "QUICWorkerOperationsQueued")]
    pub quicworker_operations_queued: Option<u64>,

/// 
    #[serde(rename = "QUICWorkerOperationsQueuedPersec")]
    pub quicworker_operations_queued_persec: Option<u32>,
}

impl Win32_PerfFormattedData_QuicCounters_QUICPerformanceDiagnostics {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            quicapp_received_bytes_persec: None,
            quicapp_sent_bytes_persec: None,
            quicconnection_operations_completed_persec: None,
            quicconnection_operations_queued: None,
            quicconnection_operations_queued_persec: None,
            quicconnections_active: None,
            quicconnections_connected: None,
            quicconnections_created: None,
            quicconnections_created_persec: None,
            quicconnections_no_alpn: None,
            quicconnections_no_alpns_persec: None,
            quicconnections_protocol_error: None,
            quicconnections_protocol_errors_persec: None,
            quicconnections_queued: None,
            quicconnections_rejected: None,
            quicconnections_rejected_persec: None,
            quicconnections_resumed: None,
            quicconnections_resumed_persec: None,
            quichandshakes_failed: None,
            quichandshakes_failed_persec: None,
            quicpacket_decryption_failures_persec: None,
            quicpackets_dropped_persec: None,
            quicpackets_suspected_lost_persec: None,
            quicpath_challenges_failed: None,
            quicpath_challenges_succeeded: None,
            quicstateless_resets_sent: None,
            quicstateless_retries_sent: None,
            quicstreams_active: None,
            quicudpdatagrams_received_persec: None,
            quicudpdatagrams_sent_persec: None,
            quicudppayload_bytes_received_persec: None,
            quicudppayload_bytes_sent_persec: None,
            quicudpreceive_events_persec: None,
            quicudpsend_calls_persec: None,
            quicworker_operations_persec: None,
            quicworker_operations_queued: None,
            quicworker_operations_queued_persec: None,
        }
    }


    /// Sets the value of QUICAppReceivedBytesPersec
    pub fn set_quicapp_received_bytes_persec(&mut self, value: u32) {
        self.quicapp_received_bytes_persec = Some(value);
    }

    /// Gets the value of QUICAppReceivedBytesPersec
    pub fn get_quicapp_received_bytes_persec(&self) -> Option<&u32> {
        self.quicapp_received_bytes_persec.as_ref()
    }

    /// Sets the value of QUICAppSentBytesPersec
    pub fn set_quicapp_sent_bytes_persec(&mut self, value: u32) {
        self.quicapp_sent_bytes_persec = Some(value);
    }

    /// Gets the value of QUICAppSentBytesPersec
    pub fn get_quicapp_sent_bytes_persec(&self) -> Option<&u32> {
        self.quicapp_sent_bytes_persec.as_ref()
    }

    /// Sets the value of QUICConnectionOperationsCompletedPersec
    pub fn set_quicconnection_operations_completed_persec(&mut self, value: u32) {
        self.quicconnection_operations_completed_persec = Some(value);
    }

    /// Gets the value of QUICConnectionOperationsCompletedPersec
    pub fn get_quicconnection_operations_completed_persec(&self) -> Option<&u32> {
        self.quicconnection_operations_completed_persec.as_ref()
    }

    /// Sets the value of QUICConnectionOperationsQueued
    pub fn set_quicconnection_operations_queued(&mut self, value: u64) {
        self.quicconnection_operations_queued = Some(value);
    }

    /// Gets the value of QUICConnectionOperationsQueued
    pub fn get_quicconnection_operations_queued(&self) -> Option<&u64> {
        self.quicconnection_operations_queued.as_ref()
    }

    /// Sets the value of QUICConnectionOperationsQueuedPersec
    pub fn set_quicconnection_operations_queued_persec(&mut self, value: u32) {
        self.quicconnection_operations_queued_persec = Some(value);
    }

    /// Gets the value of QUICConnectionOperationsQueuedPersec
    pub fn get_quicconnection_operations_queued_persec(&self) -> Option<&u32> {
        self.quicconnection_operations_queued_persec.as_ref()
    }

    /// Sets the value of QUICConnectionsActive
    pub fn set_quicconnections_active(&mut self, value: u64) {
        self.quicconnections_active = Some(value);
    }

    /// Gets the value of QUICConnectionsActive
    pub fn get_quicconnections_active(&self) -> Option<&u64> {
        self.quicconnections_active.as_ref()
    }

    /// Sets the value of QUICConnectionsConnected
    pub fn set_quicconnections_connected(&mut self, value: u64) {
        self.quicconnections_connected = Some(value);
    }

    /// Gets the value of QUICConnectionsConnected
    pub fn get_quicconnections_connected(&self) -> Option<&u64> {
        self.quicconnections_connected.as_ref()
    }

    /// Sets the value of QUICConnectionsCreated
    pub fn set_quicconnections_created(&mut self, value: u64) {
        self.quicconnections_created = Some(value);
    }

    /// Gets the value of QUICConnectionsCreated
    pub fn get_quicconnections_created(&self) -> Option<&u64> {
        self.quicconnections_created.as_ref()
    }

    /// Sets the value of QUICConnectionsCreatedPersec
    pub fn set_quicconnections_created_persec(&mut self, value: u32) {
        self.quicconnections_created_persec = Some(value);
    }

    /// Gets the value of QUICConnectionsCreatedPersec
    pub fn get_quicconnections_created_persec(&self) -> Option<&u32> {
        self.quicconnections_created_persec.as_ref()
    }

    /// Sets the value of QUICConnectionsNoALPN
    pub fn set_quicconnections_no_alpn(&mut self, value: u64) {
        self.quicconnections_no_alpn = Some(value);
    }

    /// Gets the value of QUICConnectionsNoALPN
    pub fn get_quicconnections_no_alpn(&self) -> Option<&u64> {
        self.quicconnections_no_alpn.as_ref()
    }

    /// Sets the value of QUICConnectionsNoALPNsPersec
    pub fn set_quicconnections_no_alpns_persec(&mut self, value: u32) {
        self.quicconnections_no_alpns_persec = Some(value);
    }

    /// Gets the value of QUICConnectionsNoALPNsPersec
    pub fn get_quicconnections_no_alpns_persec(&self) -> Option<&u32> {
        self.quicconnections_no_alpns_persec.as_ref()
    }

    /// Sets the value of QUICConnectionsProtocolError
    pub fn set_quicconnections_protocol_error(&mut self, value: u64) {
        self.quicconnections_protocol_error = Some(value);
    }

    /// Gets the value of QUICConnectionsProtocolError
    pub fn get_quicconnections_protocol_error(&self) -> Option<&u64> {
        self.quicconnections_protocol_error.as_ref()
    }

    /// Sets the value of QUICConnectionsProtocolErrorsPersec
    pub fn set_quicconnections_protocol_errors_persec(&mut self, value: u32) {
        self.quicconnections_protocol_errors_persec = Some(value);
    }

    /// Gets the value of QUICConnectionsProtocolErrorsPersec
    pub fn get_quicconnections_protocol_errors_persec(&self) -> Option<&u32> {
        self.quicconnections_protocol_errors_persec.as_ref()
    }

    /// Sets the value of QUICConnectionsQueued
    pub fn set_quicconnections_queued(&mut self, value: u64) {
        self.quicconnections_queued = Some(value);
    }

    /// Gets the value of QUICConnectionsQueued
    pub fn get_quicconnections_queued(&self) -> Option<&u64> {
        self.quicconnections_queued.as_ref()
    }

    /// Sets the value of QUICConnectionsRejected
    pub fn set_quicconnections_rejected(&mut self, value: u64) {
        self.quicconnections_rejected = Some(value);
    }

    /// Gets the value of QUICConnectionsRejected
    pub fn get_quicconnections_rejected(&self) -> Option<&u64> {
        self.quicconnections_rejected.as_ref()
    }

    /// Sets the value of QUICConnectionsRejectedPersec
    pub fn set_quicconnections_rejected_persec(&mut self, value: u32) {
        self.quicconnections_rejected_persec = Some(value);
    }

    /// Gets the value of QUICConnectionsRejectedPersec
    pub fn get_quicconnections_rejected_persec(&self) -> Option<&u32> {
        self.quicconnections_rejected_persec.as_ref()
    }

    /// Sets the value of QUICConnectionsResumed
    pub fn set_quicconnections_resumed(&mut self, value: u64) {
        self.quicconnections_resumed = Some(value);
    }

    /// Gets the value of QUICConnectionsResumed
    pub fn get_quicconnections_resumed(&self) -> Option<&u64> {
        self.quicconnections_resumed.as_ref()
    }

    /// Sets the value of QUICConnectionsResumedPersec
    pub fn set_quicconnections_resumed_persec(&mut self, value: u32) {
        self.quicconnections_resumed_persec = Some(value);
    }

    /// Gets the value of QUICConnectionsResumedPersec
    pub fn get_quicconnections_resumed_persec(&self) -> Option<&u32> {
        self.quicconnections_resumed_persec.as_ref()
    }

    /// Sets the value of QUICHandshakesFailed
    pub fn set_quichandshakes_failed(&mut self, value: u64) {
        self.quichandshakes_failed = Some(value);
    }

    /// Gets the value of QUICHandshakesFailed
    pub fn get_quichandshakes_failed(&self) -> Option<&u64> {
        self.quichandshakes_failed.as_ref()
    }

    /// Sets the value of QUICHandshakesFailedPersec
    pub fn set_quichandshakes_failed_persec(&mut self, value: u32) {
        self.quichandshakes_failed_persec = Some(value);
    }

    /// Gets the value of QUICHandshakesFailedPersec
    pub fn get_quichandshakes_failed_persec(&self) -> Option<&u32> {
        self.quichandshakes_failed_persec.as_ref()
    }

    /// Sets the value of QUICPacketDecryptionFailuresPersec
    pub fn set_quicpacket_decryption_failures_persec(&mut self, value: u32) {
        self.quicpacket_decryption_failures_persec = Some(value);
    }

    /// Gets the value of QUICPacketDecryptionFailuresPersec
    pub fn get_quicpacket_decryption_failures_persec(&self) -> Option<&u32> {
        self.quicpacket_decryption_failures_persec.as_ref()
    }

    /// Sets the value of QUICPacketsDroppedPersec
    pub fn set_quicpackets_dropped_persec(&mut self, value: u32) {
        self.quicpackets_dropped_persec = Some(value);
    }

    /// Gets the value of QUICPacketsDroppedPersec
    pub fn get_quicpackets_dropped_persec(&self) -> Option<&u32> {
        self.quicpackets_dropped_persec.as_ref()
    }

    /// Sets the value of QUICPacketsSuspectedLostPersec
    pub fn set_quicpackets_suspected_lost_persec(&mut self, value: u32) {
        self.quicpackets_suspected_lost_persec = Some(value);
    }

    /// Gets the value of QUICPacketsSuspectedLostPersec
    pub fn get_quicpackets_suspected_lost_persec(&self) -> Option<&u32> {
        self.quicpackets_suspected_lost_persec.as_ref()
    }

    /// Sets the value of QUICPathChallengesFailed
    pub fn set_quicpath_challenges_failed(&mut self, value: u64) {
        self.quicpath_challenges_failed = Some(value);
    }

    /// Gets the value of QUICPathChallengesFailed
    pub fn get_quicpath_challenges_failed(&self) -> Option<&u64> {
        self.quicpath_challenges_failed.as_ref()
    }

    /// Sets the value of QUICPathChallengesSucceeded
    pub fn set_quicpath_challenges_succeeded(&mut self, value: u64) {
        self.quicpath_challenges_succeeded = Some(value);
    }

    /// Gets the value of QUICPathChallengesSucceeded
    pub fn get_quicpath_challenges_succeeded(&self) -> Option<&u64> {
        self.quicpath_challenges_succeeded.as_ref()
    }

    /// Sets the value of QUICStatelessResetsSent
    pub fn set_quicstateless_resets_sent(&mut self, value: u64) {
        self.quicstateless_resets_sent = Some(value);
    }

    /// Gets the value of QUICStatelessResetsSent
    pub fn get_quicstateless_resets_sent(&self) -> Option<&u64> {
        self.quicstateless_resets_sent.as_ref()
    }

    /// Sets the value of QUICStatelessRetriesSent
    pub fn set_quicstateless_retries_sent(&mut self, value: u64) {
        self.quicstateless_retries_sent = Some(value);
    }

    /// Gets the value of QUICStatelessRetriesSent
    pub fn get_quicstateless_retries_sent(&self) -> Option<&u64> {
        self.quicstateless_retries_sent.as_ref()
    }

    /// Sets the value of QUICStreamsActive
    pub fn set_quicstreams_active(&mut self, value: u64) {
        self.quicstreams_active = Some(value);
    }

    /// Gets the value of QUICStreamsActive
    pub fn get_quicstreams_active(&self) -> Option<&u64> {
        self.quicstreams_active.as_ref()
    }

    /// Sets the value of QUICUDPDatagramsReceivedPersec
    pub fn set_quicudpdatagrams_received_persec(&mut self, value: u32) {
        self.quicudpdatagrams_received_persec = Some(value);
    }

    /// Gets the value of QUICUDPDatagramsReceivedPersec
    pub fn get_quicudpdatagrams_received_persec(&self) -> Option<&u32> {
        self.quicudpdatagrams_received_persec.as_ref()
    }

    /// Sets the value of QUICUDPDatagramsSentPersec
    pub fn set_quicudpdatagrams_sent_persec(&mut self, value: u32) {
        self.quicudpdatagrams_sent_persec = Some(value);
    }

    /// Gets the value of QUICUDPDatagramsSentPersec
    pub fn get_quicudpdatagrams_sent_persec(&self) -> Option<&u32> {
        self.quicudpdatagrams_sent_persec.as_ref()
    }

    /// Sets the value of QUICUDPPayloadBytesReceivedPersec
    pub fn set_quicudppayload_bytes_received_persec(&mut self, value: u32) {
        self.quicudppayload_bytes_received_persec = Some(value);
    }

    /// Gets the value of QUICUDPPayloadBytesReceivedPersec
    pub fn get_quicudppayload_bytes_received_persec(&self) -> Option<&u32> {
        self.quicudppayload_bytes_received_persec.as_ref()
    }

    /// Sets the value of QUICUDPPayloadBytesSentPersec
    pub fn set_quicudppayload_bytes_sent_persec(&mut self, value: u32) {
        self.quicudppayload_bytes_sent_persec = Some(value);
    }

    /// Gets the value of QUICUDPPayloadBytesSentPersec
    pub fn get_quicudppayload_bytes_sent_persec(&self) -> Option<&u32> {
        self.quicudppayload_bytes_sent_persec.as_ref()
    }

    /// Sets the value of QUICUDPReceiveEventsPersec
    pub fn set_quicudpreceive_events_persec(&mut self, value: u32) {
        self.quicudpreceive_events_persec = Some(value);
    }

    /// Gets the value of QUICUDPReceiveEventsPersec
    pub fn get_quicudpreceive_events_persec(&self) -> Option<&u32> {
        self.quicudpreceive_events_persec.as_ref()
    }

    /// Sets the value of QUICUDPSendCallsPersec
    pub fn set_quicudpsend_calls_persec(&mut self, value: u32) {
        self.quicudpsend_calls_persec = Some(value);
    }

    /// Gets the value of QUICUDPSendCallsPersec
    pub fn get_quicudpsend_calls_persec(&self) -> Option<&u32> {
        self.quicudpsend_calls_persec.as_ref()
    }

    /// Sets the value of QUICWorkerOperationsPersec
    pub fn set_quicworker_operations_persec(&mut self, value: u32) {
        self.quicworker_operations_persec = Some(value);
    }

    /// Gets the value of QUICWorkerOperationsPersec
    pub fn get_quicworker_operations_persec(&self) -> Option<&u32> {
        self.quicworker_operations_persec.as_ref()
    }

    /// Sets the value of QUICWorkerOperationsQueued
    pub fn set_quicworker_operations_queued(&mut self, value: u64) {
        self.quicworker_operations_queued = Some(value);
    }

    /// Gets the value of QUICWorkerOperationsQueued
    pub fn get_quicworker_operations_queued(&self) -> Option<&u64> {
        self.quicworker_operations_queued.as_ref()
    }

    /// Sets the value of QUICWorkerOperationsQueuedPersec
    pub fn set_quicworker_operations_queued_persec(&mut self, value: u32) {
        self.quicworker_operations_queued_persec = Some(value);
    }

    /// Gets the value of QUICWorkerOperationsQueuedPersec
    pub fn get_quicworker_operations_queued_persec(&self) -> Option<&u32> {
        self.quicworker_operations_queued_persec.as_ref()
    }
}

