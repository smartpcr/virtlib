// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_MicrosoftWindowsSMBDirect_SMBDirect struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_MicrosoftWindowsSMBDirect_SMBDirect {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "AllocationFailuresConnector")]
    pub allocation_failures_connector: Option<u32>,

/// 
    #[serde(rename = "AllocationFailuresCQ")]
    pub allocation_failures_cq: Option<u32>,

/// 
    #[serde(rename = "AllocationFailuresFRMR")]
    pub allocation_failures_frmr: Option<u32>,

/// 
    #[serde(rename = "AllocationFailuresMR")]
    pub allocation_failures_mr: Option<u32>,

/// 
    #[serde(rename = "AllocationFailuresPD")]
    pub allocation_failures_pd: Option<u32>,

/// 
    #[serde(rename = "AllocationFailuresQP")]
    pub allocation_failures_qp: Option<u32>,

/// 
    #[serde(rename = "AllocationStallsConnector")]
    pub allocation_stalls_connector: Option<u32>,

/// 
    #[serde(rename = "AllocationStallsCQ")]
    pub allocation_stalls_cq: Option<u32>,

/// 
    #[serde(rename = "AllocationStallsFRMR")]
    pub allocation_stalls_frmr: Option<u32>,

/// 
    #[serde(rename = "AllocationStallsMR")]
    pub allocation_stalls_mr: Option<u32>,

/// 
    #[serde(rename = "AllocationStallsPD")]
    pub allocation_stalls_pd: Option<u32>,

/// 
    #[serde(rename = "AllocationStallsQP")]
    pub allocation_stalls_qp: Option<u32>,

/// 
    #[serde(rename = "BytesRDMAReadPersec")]
    pub bytes_rdmaread_persec: Option<u64>,

/// 
    #[serde(rename = "BytesRDMAWrittenPersec")]
    pub bytes_rdmawritten_persec: Option<u64>,

/// 
    #[serde(rename = "BytesReceivedPersec")]
    pub bytes_received_persec: Option<u64>,

/// 
    #[serde(rename = "BytesRegistered")]
    pub bytes_registered: Option<u32>,

/// 
    #[serde(rename = "BytesSentPersec")]
    pub bytes_sent_persec: Option<u64>,

/// 
    #[serde(rename = "FastregistrationsPersec")]
    pub fastregistrations_persec: Option<u64>,

/// 
    #[serde(rename = "FRMRPages")]
    pub frmrpages: Option<u32>,

/// 
    #[serde(rename = "FRMRs")]
    pub frmrs: Option<u32>,

/// 
    #[serde(rename = "InboundConnectionFailures")]
    pub inbound_connection_failures: Option<u32>,

/// 
    #[serde(rename = "InboundConnectionRefusals")]
    pub inbound_connection_refusals: Option<u32>,

/// 
    #[serde(rename = "InboundConnections")]
    pub inbound_connections: Option<u32>,

/// 
    #[serde(rename = "InboundConnectionsMax")]
    pub inbound_connections_max: Option<u32>,

/// 
    #[serde(rename = "InboundConnectionsPending")]
    pub inbound_connections_pending: Option<u32>,

/// 
    #[serde(rename = "InvalidationsPersec")]
    pub invalidations_persec: Option<u64>,

/// 
    #[serde(rename = "OutboundConnectionFailures")]
    pub outbound_connection_failures: Option<u32>,

/// 
    #[serde(rename = "OutboundConnections")]
    pub outbound_connections: Option<u32>,

/// 
    #[serde(rename = "OutboundConnectionsMax")]
    pub outbound_connections_max: Option<u32>,

/// 
    #[serde(rename = "ProtocolErrors")]
    pub protocol_errors: Option<u32>,

/// 
    #[serde(rename = "RCQNotificationsPersec")]
    pub rcqnotifications_persec: Option<u64>,

/// 
    #[serde(rename = "RCQNotificationsQueuedPersec")]
    pub rcqnotifications_queued_persec: Option<u64>,

/// 
    #[serde(rename = "RDMAReadsPersec")]
    pub rdmareads_persec: Option<u64>,

/// 
    #[serde(rename = "RDMAWritesPersec")]
    pub rdmawrites_persec: Option<u64>,

/// 
    #[serde(rename = "ReceivesControlPersec")]
    pub receives_control_persec: Option<u64>,

/// 
    #[serde(rename = "ReceivesPersec")]
    pub receives_persec: Option<u64>,

/// 
    #[serde(rename = "RemoteInvalidationsPersec")]
    pub remote_invalidations_persec: Option<u64>,

/// 
    #[serde(rename = "SCQNotificationsPersec")]
    pub scqnotifications_persec: Option<u64>,

/// 
    #[serde(rename = "SCQNotificationsQueuedPersec")]
    pub scqnotifications_queued_persec: Option<u64>,

/// 
    #[serde(rename = "SendsControlPersec")]
    pub sends_control_persec: Option<u64>,

/// 
    #[serde(rename = "SendsPersec")]
    pub sends_persec: Option<u64>,

/// 
    #[serde(rename = "SQRequestsPersec")]
    pub sqrequests_persec: Option<u64>,

/// 
    #[serde(rename = "SQRequestsPersecunsignaled")]
    pub sqrequests_persecunsignaled: Option<u64>,

/// 
    #[serde(rename = "StallsFRMRPersec")]
    pub stalls_frmrpersec: Option<u64>,

/// 
    #[serde(rename = "StallsRDMAReadPersec")]
    pub stalls_rdmaread_persec: Option<u64>,

/// 
    #[serde(rename = "StallsSendCreditPersec")]
    pub stalls_send_credit_persec: Option<u64>,

/// 
    #[serde(rename = "StallsSQPersec")]
    pub stalls_sqpersec: Option<u64>,

/// 
    #[serde(rename = "TotalConnections")]
    pub total_connections: Option<u32>,

/// 
    #[serde(rename = "TotalConnectionsMax")]
    pub total_connections_max: Option<u32>,
}

impl Win32_PerfFormattedData_MicrosoftWindowsSMBDirect_SMBDirect {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            allocation_failures_connector: None,
            allocation_failures_cq: None,
            allocation_failures_frmr: None,
            allocation_failures_mr: None,
            allocation_failures_pd: None,
            allocation_failures_qp: None,
            allocation_stalls_connector: None,
            allocation_stalls_cq: None,
            allocation_stalls_frmr: None,
            allocation_stalls_mr: None,
            allocation_stalls_pd: None,
            allocation_stalls_qp: None,
            bytes_rdmaread_persec: None,
            bytes_rdmawritten_persec: None,
            bytes_received_persec: None,
            bytes_registered: None,
            bytes_sent_persec: None,
            fastregistrations_persec: None,
            frmrpages: None,
            frmrs: None,
            inbound_connection_failures: None,
            inbound_connection_refusals: None,
            inbound_connections: None,
            inbound_connections_max: None,
            inbound_connections_pending: None,
            invalidations_persec: None,
            outbound_connection_failures: None,
            outbound_connections: None,
            outbound_connections_max: None,
            protocol_errors: None,
            rcqnotifications_persec: None,
            rcqnotifications_queued_persec: None,
            rdmareads_persec: None,
            rdmawrites_persec: None,
            receives_control_persec: None,
            receives_persec: None,
            remote_invalidations_persec: None,
            scqnotifications_persec: None,
            scqnotifications_queued_persec: None,
            sends_control_persec: None,
            sends_persec: None,
            sqrequests_persec: None,
            sqrequests_persecunsignaled: None,
            stalls_frmrpersec: None,
            stalls_rdmaread_persec: None,
            stalls_send_credit_persec: None,
            stalls_sqpersec: None,
            total_connections: None,
            total_connections_max: None,
        }
    }


    /// Sets the value of AllocationFailuresConnector
    pub fn set_allocation_failures_connector(&mut self, value: u32) {
        self.allocation_failures_connector = Some(value);
    }

    /// Gets the value of AllocationFailuresConnector
    pub fn get_allocation_failures_connector(&self) -> Option<&u32> {
        self.allocation_failures_connector.as_ref()
    }

    /// Sets the value of AllocationFailuresCQ
    pub fn set_allocation_failures_cq(&mut self, value: u32) {
        self.allocation_failures_cq = Some(value);
    }

    /// Gets the value of AllocationFailuresCQ
    pub fn get_allocation_failures_cq(&self) -> Option<&u32> {
        self.allocation_failures_cq.as_ref()
    }

    /// Sets the value of AllocationFailuresFRMR
    pub fn set_allocation_failures_frmr(&mut self, value: u32) {
        self.allocation_failures_frmr = Some(value);
    }

    /// Gets the value of AllocationFailuresFRMR
    pub fn get_allocation_failures_frmr(&self) -> Option<&u32> {
        self.allocation_failures_frmr.as_ref()
    }

    /// Sets the value of AllocationFailuresMR
    pub fn set_allocation_failures_mr(&mut self, value: u32) {
        self.allocation_failures_mr = Some(value);
    }

    /// Gets the value of AllocationFailuresMR
    pub fn get_allocation_failures_mr(&self) -> Option<&u32> {
        self.allocation_failures_mr.as_ref()
    }

    /// Sets the value of AllocationFailuresPD
    pub fn set_allocation_failures_pd(&mut self, value: u32) {
        self.allocation_failures_pd = Some(value);
    }

    /// Gets the value of AllocationFailuresPD
    pub fn get_allocation_failures_pd(&self) -> Option<&u32> {
        self.allocation_failures_pd.as_ref()
    }

    /// Sets the value of AllocationFailuresQP
    pub fn set_allocation_failures_qp(&mut self, value: u32) {
        self.allocation_failures_qp = Some(value);
    }

    /// Gets the value of AllocationFailuresQP
    pub fn get_allocation_failures_qp(&self) -> Option<&u32> {
        self.allocation_failures_qp.as_ref()
    }

    /// Sets the value of AllocationStallsConnector
    pub fn set_allocation_stalls_connector(&mut self, value: u32) {
        self.allocation_stalls_connector = Some(value);
    }

    /// Gets the value of AllocationStallsConnector
    pub fn get_allocation_stalls_connector(&self) -> Option<&u32> {
        self.allocation_stalls_connector.as_ref()
    }

    /// Sets the value of AllocationStallsCQ
    pub fn set_allocation_stalls_cq(&mut self, value: u32) {
        self.allocation_stalls_cq = Some(value);
    }

    /// Gets the value of AllocationStallsCQ
    pub fn get_allocation_stalls_cq(&self) -> Option<&u32> {
        self.allocation_stalls_cq.as_ref()
    }

    /// Sets the value of AllocationStallsFRMR
    pub fn set_allocation_stalls_frmr(&mut self, value: u32) {
        self.allocation_stalls_frmr = Some(value);
    }

    /// Gets the value of AllocationStallsFRMR
    pub fn get_allocation_stalls_frmr(&self) -> Option<&u32> {
        self.allocation_stalls_frmr.as_ref()
    }

    /// Sets the value of AllocationStallsMR
    pub fn set_allocation_stalls_mr(&mut self, value: u32) {
        self.allocation_stalls_mr = Some(value);
    }

    /// Gets the value of AllocationStallsMR
    pub fn get_allocation_stalls_mr(&self) -> Option<&u32> {
        self.allocation_stalls_mr.as_ref()
    }

    /// Sets the value of AllocationStallsPD
    pub fn set_allocation_stalls_pd(&mut self, value: u32) {
        self.allocation_stalls_pd = Some(value);
    }

    /// Gets the value of AllocationStallsPD
    pub fn get_allocation_stalls_pd(&self) -> Option<&u32> {
        self.allocation_stalls_pd.as_ref()
    }

    /// Sets the value of AllocationStallsQP
    pub fn set_allocation_stalls_qp(&mut self, value: u32) {
        self.allocation_stalls_qp = Some(value);
    }

    /// Gets the value of AllocationStallsQP
    pub fn get_allocation_stalls_qp(&self) -> Option<&u32> {
        self.allocation_stalls_qp.as_ref()
    }

    /// Sets the value of BytesRDMAReadPersec
    pub fn set_bytes_rdmaread_persec(&mut self, value: u64) {
        self.bytes_rdmaread_persec = Some(value);
    }

    /// Gets the value of BytesRDMAReadPersec
    pub fn get_bytes_rdmaread_persec(&self) -> Option<&u64> {
        self.bytes_rdmaread_persec.as_ref()
    }

    /// Sets the value of BytesRDMAWrittenPersec
    pub fn set_bytes_rdmawritten_persec(&mut self, value: u64) {
        self.bytes_rdmawritten_persec = Some(value);
    }

    /// Gets the value of BytesRDMAWrittenPersec
    pub fn get_bytes_rdmawritten_persec(&self) -> Option<&u64> {
        self.bytes_rdmawritten_persec.as_ref()
    }

    /// Sets the value of BytesReceivedPersec
    pub fn set_bytes_received_persec(&mut self, value: u64) {
        self.bytes_received_persec = Some(value);
    }

    /// Gets the value of BytesReceivedPersec
    pub fn get_bytes_received_persec(&self) -> Option<&u64> {
        self.bytes_received_persec.as_ref()
    }

    /// Sets the value of BytesRegistered
    pub fn set_bytes_registered(&mut self, value: u32) {
        self.bytes_registered = Some(value);
    }

    /// Gets the value of BytesRegistered
    pub fn get_bytes_registered(&self) -> Option<&u32> {
        self.bytes_registered.as_ref()
    }

    /// Sets the value of BytesSentPersec
    pub fn set_bytes_sent_persec(&mut self, value: u64) {
        self.bytes_sent_persec = Some(value);
    }

    /// Gets the value of BytesSentPersec
    pub fn get_bytes_sent_persec(&self) -> Option<&u64> {
        self.bytes_sent_persec.as_ref()
    }

    /// Sets the value of FastregistrationsPersec
    pub fn set_fastregistrations_persec(&mut self, value: u64) {
        self.fastregistrations_persec = Some(value);
    }

    /// Gets the value of FastregistrationsPersec
    pub fn get_fastregistrations_persec(&self) -> Option<&u64> {
        self.fastregistrations_persec.as_ref()
    }

    /// Sets the value of FRMRPages
    pub fn set_frmrpages(&mut self, value: u32) {
        self.frmrpages = Some(value);
    }

    /// Gets the value of FRMRPages
    pub fn get_frmrpages(&self) -> Option<&u32> {
        self.frmrpages.as_ref()
    }

    /// Sets the value of FRMRs
    pub fn set_frmrs(&mut self, value: u32) {
        self.frmrs = Some(value);
    }

    /// Gets the value of FRMRs
    pub fn get_frmrs(&self) -> Option<&u32> {
        self.frmrs.as_ref()
    }

    /// Sets the value of InboundConnectionFailures
    pub fn set_inbound_connection_failures(&mut self, value: u32) {
        self.inbound_connection_failures = Some(value);
    }

    /// Gets the value of InboundConnectionFailures
    pub fn get_inbound_connection_failures(&self) -> Option<&u32> {
        self.inbound_connection_failures.as_ref()
    }

    /// Sets the value of InboundConnectionRefusals
    pub fn set_inbound_connection_refusals(&mut self, value: u32) {
        self.inbound_connection_refusals = Some(value);
    }

    /// Gets the value of InboundConnectionRefusals
    pub fn get_inbound_connection_refusals(&self) -> Option<&u32> {
        self.inbound_connection_refusals.as_ref()
    }

    /// Sets the value of InboundConnections
    pub fn set_inbound_connections(&mut self, value: u32) {
        self.inbound_connections = Some(value);
    }

    /// Gets the value of InboundConnections
    pub fn get_inbound_connections(&self) -> Option<&u32> {
        self.inbound_connections.as_ref()
    }

    /// Sets the value of InboundConnectionsMax
    pub fn set_inbound_connections_max(&mut self, value: u32) {
        self.inbound_connections_max = Some(value);
    }

    /// Gets the value of InboundConnectionsMax
    pub fn get_inbound_connections_max(&self) -> Option<&u32> {
        self.inbound_connections_max.as_ref()
    }

    /// Sets the value of InboundConnectionsPending
    pub fn set_inbound_connections_pending(&mut self, value: u32) {
        self.inbound_connections_pending = Some(value);
    }

    /// Gets the value of InboundConnectionsPending
    pub fn get_inbound_connections_pending(&self) -> Option<&u32> {
        self.inbound_connections_pending.as_ref()
    }

    /// Sets the value of InvalidationsPersec
    pub fn set_invalidations_persec(&mut self, value: u64) {
        self.invalidations_persec = Some(value);
    }

    /// Gets the value of InvalidationsPersec
    pub fn get_invalidations_persec(&self) -> Option<&u64> {
        self.invalidations_persec.as_ref()
    }

    /// Sets the value of OutboundConnectionFailures
    pub fn set_outbound_connection_failures(&mut self, value: u32) {
        self.outbound_connection_failures = Some(value);
    }

    /// Gets the value of OutboundConnectionFailures
    pub fn get_outbound_connection_failures(&self) -> Option<&u32> {
        self.outbound_connection_failures.as_ref()
    }

    /// Sets the value of OutboundConnections
    pub fn set_outbound_connections(&mut self, value: u32) {
        self.outbound_connections = Some(value);
    }

    /// Gets the value of OutboundConnections
    pub fn get_outbound_connections(&self) -> Option<&u32> {
        self.outbound_connections.as_ref()
    }

    /// Sets the value of OutboundConnectionsMax
    pub fn set_outbound_connections_max(&mut self, value: u32) {
        self.outbound_connections_max = Some(value);
    }

    /// Gets the value of OutboundConnectionsMax
    pub fn get_outbound_connections_max(&self) -> Option<&u32> {
        self.outbound_connections_max.as_ref()
    }

    /// Sets the value of ProtocolErrors
    pub fn set_protocol_errors(&mut self, value: u32) {
        self.protocol_errors = Some(value);
    }

    /// Gets the value of ProtocolErrors
    pub fn get_protocol_errors(&self) -> Option<&u32> {
        self.protocol_errors.as_ref()
    }

    /// Sets the value of RCQNotificationsPersec
    pub fn set_rcqnotifications_persec(&mut self, value: u64) {
        self.rcqnotifications_persec = Some(value);
    }

    /// Gets the value of RCQNotificationsPersec
    pub fn get_rcqnotifications_persec(&self) -> Option<&u64> {
        self.rcqnotifications_persec.as_ref()
    }

    /// Sets the value of RCQNotificationsQueuedPersec
    pub fn set_rcqnotifications_queued_persec(&mut self, value: u64) {
        self.rcqnotifications_queued_persec = Some(value);
    }

    /// Gets the value of RCQNotificationsQueuedPersec
    pub fn get_rcqnotifications_queued_persec(&self) -> Option<&u64> {
        self.rcqnotifications_queued_persec.as_ref()
    }

    /// Sets the value of RDMAReadsPersec
    pub fn set_rdmareads_persec(&mut self, value: u64) {
        self.rdmareads_persec = Some(value);
    }

    /// Gets the value of RDMAReadsPersec
    pub fn get_rdmareads_persec(&self) -> Option<&u64> {
        self.rdmareads_persec.as_ref()
    }

    /// Sets the value of RDMAWritesPersec
    pub fn set_rdmawrites_persec(&mut self, value: u64) {
        self.rdmawrites_persec = Some(value);
    }

    /// Gets the value of RDMAWritesPersec
    pub fn get_rdmawrites_persec(&self) -> Option<&u64> {
        self.rdmawrites_persec.as_ref()
    }

    /// Sets the value of ReceivesControlPersec
    pub fn set_receives_control_persec(&mut self, value: u64) {
        self.receives_control_persec = Some(value);
    }

    /// Gets the value of ReceivesControlPersec
    pub fn get_receives_control_persec(&self) -> Option<&u64> {
        self.receives_control_persec.as_ref()
    }

    /// Sets the value of ReceivesPersec
    pub fn set_receives_persec(&mut self, value: u64) {
        self.receives_persec = Some(value);
    }

    /// Gets the value of ReceivesPersec
    pub fn get_receives_persec(&self) -> Option<&u64> {
        self.receives_persec.as_ref()
    }

    /// Sets the value of RemoteInvalidationsPersec
    pub fn set_remote_invalidations_persec(&mut self, value: u64) {
        self.remote_invalidations_persec = Some(value);
    }

    /// Gets the value of RemoteInvalidationsPersec
    pub fn get_remote_invalidations_persec(&self) -> Option<&u64> {
        self.remote_invalidations_persec.as_ref()
    }

    /// Sets the value of SCQNotificationsPersec
    pub fn set_scqnotifications_persec(&mut self, value: u64) {
        self.scqnotifications_persec = Some(value);
    }

    /// Gets the value of SCQNotificationsPersec
    pub fn get_scqnotifications_persec(&self) -> Option<&u64> {
        self.scqnotifications_persec.as_ref()
    }

    /// Sets the value of SCQNotificationsQueuedPersec
    pub fn set_scqnotifications_queued_persec(&mut self, value: u64) {
        self.scqnotifications_queued_persec = Some(value);
    }

    /// Gets the value of SCQNotificationsQueuedPersec
    pub fn get_scqnotifications_queued_persec(&self) -> Option<&u64> {
        self.scqnotifications_queued_persec.as_ref()
    }

    /// Sets the value of SendsControlPersec
    pub fn set_sends_control_persec(&mut self, value: u64) {
        self.sends_control_persec = Some(value);
    }

    /// Gets the value of SendsControlPersec
    pub fn get_sends_control_persec(&self) -> Option<&u64> {
        self.sends_control_persec.as_ref()
    }

    /// Sets the value of SendsPersec
    pub fn set_sends_persec(&mut self, value: u64) {
        self.sends_persec = Some(value);
    }

    /// Gets the value of SendsPersec
    pub fn get_sends_persec(&self) -> Option<&u64> {
        self.sends_persec.as_ref()
    }

    /// Sets the value of SQRequestsPersec
    pub fn set_sqrequests_persec(&mut self, value: u64) {
        self.sqrequests_persec = Some(value);
    }

    /// Gets the value of SQRequestsPersec
    pub fn get_sqrequests_persec(&self) -> Option<&u64> {
        self.sqrequests_persec.as_ref()
    }

    /// Sets the value of SQRequestsPersecunsignaled
    pub fn set_sqrequests_persecunsignaled(&mut self, value: u64) {
        self.sqrequests_persecunsignaled = Some(value);
    }

    /// Gets the value of SQRequestsPersecunsignaled
    pub fn get_sqrequests_persecunsignaled(&self) -> Option<&u64> {
        self.sqrequests_persecunsignaled.as_ref()
    }

    /// Sets the value of StallsFRMRPersec
    pub fn set_stalls_frmrpersec(&mut self, value: u64) {
        self.stalls_frmrpersec = Some(value);
    }

    /// Gets the value of StallsFRMRPersec
    pub fn get_stalls_frmrpersec(&self) -> Option<&u64> {
        self.stalls_frmrpersec.as_ref()
    }

    /// Sets the value of StallsRDMAReadPersec
    pub fn set_stalls_rdmaread_persec(&mut self, value: u64) {
        self.stalls_rdmaread_persec = Some(value);
    }

    /// Gets the value of StallsRDMAReadPersec
    pub fn get_stalls_rdmaread_persec(&self) -> Option<&u64> {
        self.stalls_rdmaread_persec.as_ref()
    }

    /// Sets the value of StallsSendCreditPersec
    pub fn set_stalls_send_credit_persec(&mut self, value: u64) {
        self.stalls_send_credit_persec = Some(value);
    }

    /// Gets the value of StallsSendCreditPersec
    pub fn get_stalls_send_credit_persec(&self) -> Option<&u64> {
        self.stalls_send_credit_persec.as_ref()
    }

    /// Sets the value of StallsSQPersec
    pub fn set_stalls_sqpersec(&mut self, value: u64) {
        self.stalls_sqpersec = Some(value);
    }

    /// Gets the value of StallsSQPersec
    pub fn get_stalls_sqpersec(&self) -> Option<&u64> {
        self.stalls_sqpersec.as_ref()
    }

    /// Sets the value of TotalConnections
    pub fn set_total_connections(&mut self, value: u32) {
        self.total_connections = Some(value);
    }

    /// Gets the value of TotalConnections
    pub fn get_total_connections(&self) -> Option<&u32> {
        self.total_connections.as_ref()
    }

    /// Sets the value of TotalConnectionsMax
    pub fn set_total_connections_max(&mut self, value: u32) {
        self.total_connections_max = Some(value);
    }

    /// Gets the value of TotalConnectionsMax
    pub fn get_total_connections_max(&self) -> Option<&u32> {
        self.total_connections_max.as_ref()
    }
}

