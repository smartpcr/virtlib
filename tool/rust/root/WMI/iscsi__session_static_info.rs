// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ISCSI_SessionStaticInfo struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ISCSI_SessionStaticInfo {

/// The number of connections that currently belong to this session
    #[serde(rename = "ConnectionCount")]
    pub connection_count: Option<u16>,

/// List of ISCSI_ConnectionStaticInfo.  ConnectionCount specifies the number of elements in the array. NOTE: This is a variable length array.
    #[serde(rename = "ConnectionsList")]
    pub connections_list: Vec<ISCSI_ConnectionStaticInfo>,

/// If FALSE indicates that data PDUs within sequences may be in any order. If TRUE indicates that data PDUs within sequences must be at continuously increasing addresses, with no gaps or overlay between PDUs.
    #[serde(rename = "DataPduInOrder")]
    pub data_pdu_in_order: Option<bool>,

/// If FALSE indicates that data PDU Sequences may be transferred in any order.  If TRUE indicates that data PDU sequences must be transferred using continuously increasing offsets, except during error recovery.
    #[serde(rename = "DataSequenceInOrder")]
    pub data_sequence_in_order: Option<bool>,

/// The level of error recovery negotiated between the initiator and the target.
    #[serde(rename = "ErrorRecoveryLevel")]
    pub error_recovery_level: Option<u8>,

/// The maximum length supported for unsolicited data sent within this session
    #[serde(rename = "FirstBurstLength")]
    pub first_burst_length: Option<u32>,

/// If TRUE indicates whether the initiator and target have agreed to support immediate commands on this session.
    #[serde(rename = "ImmediateData")]
    pub immediate_data: Option<bool>,

/// If TRUE, the initiator must wait for an R2T before sending data to the target.  If FALSE, the initiator may send data immediately, within limits set by FirstBurstSize and the expected data transfer length of the request.
    #[serde(rename = "InitialR2t")]
    pub initial_r2t: Option<bool>,

/// Initiator node name used to establish the session
    #[serde(rename = "InitiatoriSCSIName")]
    pub initiatori_scsiname: Option<String>,

/// Initiator-defined portion of the iSCSI Session ID
    #[serde(rename = "ISID")]
    pub isid: Vec<u8>,

/// The maximum number of bytes which can be sent within a single sequence of Data-In or Data-Out PDUs
    #[serde(rename = "MaxBurstLength")]
    pub max_burst_length: Option<u32>,

/// The maximum number of connections that will be allowed within this session
    #[serde(rename = "MaxConnections")]
    pub max_connections: Option<u32>,

/// The maximum number of outstanding request-to-transmit (R2T) per task within this session
    #[serde(rename = "MaxOutstandingR2t")]
    pub max_outstanding_r2t: Option<u32>,

/// iSCSI node name of the target
    #[serde(rename = "TargetiSCSIName")]
    pub targeti_scsiname: Option<String>,

/// Target-defined portion of the iSCSI Session ID
    #[serde(rename = "TSID")]
    pub tsid: Option<u16>,

/// **typedef** Type of iSCSI session
    #[serde(rename = "Type")]
    pub type: Option<SessionStaticInfo_Type>,

/// A uniquely generated session ID, it is the same id returned by the LoginToTarget method.  Do not confuse this with ISID or SSID.
    #[serde(rename = "UniqueSessionId")]
    pub unique_session_id: Option<u64>,
}

impl ISCSI_SessionStaticInfo {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            connection_count: None,
            connections_list: Vec::new(),
            data_pdu_in_order: None,
            data_sequence_in_order: None,
            error_recovery_level: None,
            first_burst_length: None,
            immediate_data: None,
            initial_r2t: None,
            initiatori_scsiname: None,
            isid: Vec::new(),
            max_burst_length: None,
            max_connections: None,
            max_outstanding_r2t: None,
            targeti_scsiname: None,
            tsid: None,
            type: None,
            unique_session_id: None,
        }
    }


    /// Sets the value of ConnectionCount
    pub fn set_connection_count(&mut self, value: u16) {
        self.connection_count = Some(value);
    }

    /// Gets the value of ConnectionCount
    pub fn get_connection_count(&self) -> Option<&u16> {
        self.connection_count.as_ref()
    }

    /// Sets the value of ConnectionsList
    pub fn set_connections_list(&mut self, value: Vec<ISCSI_ConnectionStaticInfo>) {
        self.connections_list = value;
    }

    /// Gets the value of ConnectionsList
    pub fn get_connections_list(&self) -> &Vec<ISCSI_ConnectionStaticInfo> {
        &self.connections_list
    }

    /// Sets the value of DataPduInOrder
    pub fn set_data_pdu_in_order(&mut self, value: bool) {
        self.data_pdu_in_order = Some(value);
    }

    /// Gets the value of DataPduInOrder
    pub fn get_data_pdu_in_order(&self) -> Option<&bool> {
        self.data_pdu_in_order.as_ref()
    }

    /// Sets the value of DataSequenceInOrder
    pub fn set_data_sequence_in_order(&mut self, value: bool) {
        self.data_sequence_in_order = Some(value);
    }

    /// Gets the value of DataSequenceInOrder
    pub fn get_data_sequence_in_order(&self) -> Option<&bool> {
        self.data_sequence_in_order.as_ref()
    }

    /// Sets the value of ErrorRecoveryLevel
    pub fn set_error_recovery_level(&mut self, value: u8) {
        self.error_recovery_level = Some(value);
    }

    /// Gets the value of ErrorRecoveryLevel
    pub fn get_error_recovery_level(&self) -> Option<&u8> {
        self.error_recovery_level.as_ref()
    }

    /// Sets the value of FirstBurstLength
    pub fn set_first_burst_length(&mut self, value: u32) {
        self.first_burst_length = Some(value);
    }

    /// Gets the value of FirstBurstLength
    pub fn get_first_burst_length(&self) -> Option<&u32> {
        self.first_burst_length.as_ref()
    }

    /// Sets the value of ImmediateData
    pub fn set_immediate_data(&mut self, value: bool) {
        self.immediate_data = Some(value);
    }

    /// Gets the value of ImmediateData
    pub fn get_immediate_data(&self) -> Option<&bool> {
        self.immediate_data.as_ref()
    }

    /// Sets the value of InitialR2t
    pub fn set_initial_r2t(&mut self, value: bool) {
        self.initial_r2t = Some(value);
    }

    /// Gets the value of InitialR2t
    pub fn get_initial_r2t(&self) -> Option<&bool> {
        self.initial_r2t.as_ref()
    }

    /// Sets the value of InitiatoriSCSIName
    pub fn set_initiatori_scsiname(&mut self, value: String) {
        self.initiatori_scsiname = Some(value);
    }

    /// Gets the value of InitiatoriSCSIName
    pub fn get_initiatori_scsiname(&self) -> Option<&String> {
        self.initiatori_scsiname.as_ref()
    }

    /// Sets the value of ISID
    pub fn set_isid(&mut self, value: Vec<u8>) {
        self.isid = value;
    }

    /// Gets the value of ISID
    pub fn get_isid(&self) -> &Vec<u8> {
        &self.isid
    }

    /// Sets the value of MaxBurstLength
    pub fn set_max_burst_length(&mut self, value: u32) {
        self.max_burst_length = Some(value);
    }

    /// Gets the value of MaxBurstLength
    pub fn get_max_burst_length(&self) -> Option<&u32> {
        self.max_burst_length.as_ref()
    }

    /// Sets the value of MaxConnections
    pub fn set_max_connections(&mut self, value: u32) {
        self.max_connections = Some(value);
    }

    /// Gets the value of MaxConnections
    pub fn get_max_connections(&self) -> Option<&u32> {
        self.max_connections.as_ref()
    }

    /// Sets the value of MaxOutstandingR2t
    pub fn set_max_outstanding_r2t(&mut self, value: u32) {
        self.max_outstanding_r2t = Some(value);
    }

    /// Gets the value of MaxOutstandingR2t
    pub fn get_max_outstanding_r2t(&self) -> Option<&u32> {
        self.max_outstanding_r2t.as_ref()
    }

    /// Sets the value of TargetiSCSIName
    pub fn set_targeti_scsiname(&mut self, value: String) {
        self.targeti_scsiname = Some(value);
    }

    /// Gets the value of TargetiSCSIName
    pub fn get_targeti_scsiname(&self) -> Option<&String> {
        self.targeti_scsiname.as_ref()
    }

    /// Sets the value of TSID
    pub fn set_tsid(&mut self, value: u16) {
        self.tsid = Some(value);
    }

    /// Gets the value of TSID
    pub fn get_tsid(&self) -> Option<&u16> {
        self.tsid.as_ref()
    }

    /// Sets the value of Type
    pub fn set_type(&mut self, value: SessionStaticInfo_Type) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&SessionStaticInfo_Type> {
        self.type.as_ref()
    }

    /// Sets the value of UniqueSessionId
    pub fn set_unique_session_id(&mut self, value: u64) {
        self.unique_session_id = Some(value);
    }

    /// Gets the value of UniqueSessionId
    pub fn get_unique_session_id(&self) -> Option<&u64> {
        self.unique_session_id.as_ref()
    }
}

