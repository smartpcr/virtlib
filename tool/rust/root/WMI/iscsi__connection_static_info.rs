// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ISCSI_ConnectionStaticInfo struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ISCSI_ConnectionStaticInfo {

/// **typedef** Authentication type used when establishing the connection.
    #[serde(rename = "AuthType")]
    pub auth_type: Option<ConnectionStaticInfo_AuthType>,

/// The iSCSI connection ID for this connection instance.
    #[serde(rename = "CID")]
    pub cid: Option<u16>,

/// **typedef** The name of the iSCSI data digest scheme in use within this session.
    #[serde(rename = "DataIntegrity")]
    pub data_integrity: Option<ConnectionStaticInfo_DataIntegrity>,

/// Estimated throughput of the link in bytes per second
    #[serde(rename = "EstimatedThroughput")]
    pub estimated_throughput: Option<u64>,

/// **typedef** The name of the iSCSI header digest scheme in use within this session.
    #[serde(rename = "HeaderIntegrity")]
    pub header_integrity: Option<ConnectionStaticInfo_HeaderIntegrity>,

/// The local network address used for the connection
    #[serde(rename = "LocalAddr")]
    pub local_addr: Option<ISCSI_IP_Address>,

/// The local port used for the connection
    #[serde(rename = "LocalPort")]
    pub local_port: Option<u32>,

/// Maximum Datagram size supported by the transport in bytes
    #[serde(rename = "MaxDatagramSize")]
    pub max_datagram_size: Option<u32>,

/// The maximum data payload size supported for command or data PDUs within this session.
    #[serde(rename = "MaxRecvDataSegmentLength")]
    pub max_recv_data_segment_length: Option<u32>,

/// **typedef** The transport protocol over which this connection instance is running.
    #[serde(rename = "Protocol")]
    pub protocol: Option<ConnectionStaticInfo_Protocol>,

/// The remote network address used for the connection
    #[serde(rename = "RemoteAddr")]
    pub remote_addr: Option<ISCSI_IP_Address>,

/// The remote port used for the connection
    #[serde(rename = "RemotePort")]
    pub remote_port: Option<u32>,

/// Must be zero
    #[serde(rename = "Reserved")]
    pub reserved: Option<u16>,

/// **typedef** Indicates the current state of this connection
    #[serde(rename = "State")]
    pub state: Option<ConnectionStaticInfo_State>,

/// A uniquely generated connection ID. Do not confuse this with CID.
    #[serde(rename = "UniqueConnectionId")]
    pub unique_connection_id: Option<u64>,
}

impl ISCSI_ConnectionStaticInfo {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            auth_type: None,
            cid: None,
            data_integrity: None,
            estimated_throughput: None,
            header_integrity: None,
            local_addr: None,
            local_port: None,
            max_datagram_size: None,
            max_recv_data_segment_length: None,
            protocol: None,
            remote_addr: None,
            remote_port: None,
            reserved: None,
            state: None,
            unique_connection_id: None,
        }
    }


    /// Sets the value of AuthType
    pub fn set_auth_type(&mut self, value: ConnectionStaticInfo_AuthType) {
        self.auth_type = Some(value);
    }

    /// Gets the value of AuthType
    pub fn get_auth_type(&self) -> Option<&ConnectionStaticInfo_AuthType> {
        self.auth_type.as_ref()
    }

    /// Sets the value of CID
    pub fn set_cid(&mut self, value: u16) {
        self.cid = Some(value);
    }

    /// Gets the value of CID
    pub fn get_cid(&self) -> Option<&u16> {
        self.cid.as_ref()
    }

    /// Sets the value of DataIntegrity
    pub fn set_data_integrity(&mut self, value: ConnectionStaticInfo_DataIntegrity) {
        self.data_integrity = Some(value);
    }

    /// Gets the value of DataIntegrity
    pub fn get_data_integrity(&self) -> Option<&ConnectionStaticInfo_DataIntegrity> {
        self.data_integrity.as_ref()
    }

    /// Sets the value of EstimatedThroughput
    pub fn set_estimated_throughput(&mut self, value: u64) {
        self.estimated_throughput = Some(value);
    }

    /// Gets the value of EstimatedThroughput
    pub fn get_estimated_throughput(&self) -> Option<&u64> {
        self.estimated_throughput.as_ref()
    }

    /// Sets the value of HeaderIntegrity
    pub fn set_header_integrity(&mut self, value: ConnectionStaticInfo_HeaderIntegrity) {
        self.header_integrity = Some(value);
    }

    /// Gets the value of HeaderIntegrity
    pub fn get_header_integrity(&self) -> Option<&ConnectionStaticInfo_HeaderIntegrity> {
        self.header_integrity.as_ref()
    }

    /// Sets the value of LocalAddr
    pub fn set_local_addr(&mut self, value: ISCSI_IP_Address) {
        self.local_addr = Some(value);
    }

    /// Gets the value of LocalAddr
    pub fn get_local_addr(&self) -> Option<&ISCSI_IP_Address> {
        self.local_addr.as_ref()
    }

    /// Sets the value of LocalPort
    pub fn set_local_port(&mut self, value: u32) {
        self.local_port = Some(value);
    }

    /// Gets the value of LocalPort
    pub fn get_local_port(&self) -> Option<&u32> {
        self.local_port.as_ref()
    }

    /// Sets the value of MaxDatagramSize
    pub fn set_max_datagram_size(&mut self, value: u32) {
        self.max_datagram_size = Some(value);
    }

    /// Gets the value of MaxDatagramSize
    pub fn get_max_datagram_size(&self) -> Option<&u32> {
        self.max_datagram_size.as_ref()
    }

    /// Sets the value of MaxRecvDataSegmentLength
    pub fn set_max_recv_data_segment_length(&mut self, value: u32) {
        self.max_recv_data_segment_length = Some(value);
    }

    /// Gets the value of MaxRecvDataSegmentLength
    pub fn get_max_recv_data_segment_length(&self) -> Option<&u32> {
        self.max_recv_data_segment_length.as_ref()
    }

    /// Sets the value of Protocol
    pub fn set_protocol(&mut self, value: ConnectionStaticInfo_Protocol) {
        self.protocol = Some(value);
    }

    /// Gets the value of Protocol
    pub fn get_protocol(&self) -> Option<&ConnectionStaticInfo_Protocol> {
        self.protocol.as_ref()
    }

    /// Sets the value of RemoteAddr
    pub fn set_remote_addr(&mut self, value: ISCSI_IP_Address) {
        self.remote_addr = Some(value);
    }

    /// Gets the value of RemoteAddr
    pub fn get_remote_addr(&self) -> Option<&ISCSI_IP_Address> {
        self.remote_addr.as_ref()
    }

    /// Sets the value of RemotePort
    pub fn set_remote_port(&mut self, value: u32) {
        self.remote_port = Some(value);
    }

    /// Gets the value of RemotePort
    pub fn get_remote_port(&self) -> Option<&u32> {
        self.remote_port.as_ref()
    }

    /// Sets the value of Reserved
    pub fn set_reserved(&mut self, value: u16) {
        self.reserved = Some(value);
    }

    /// Gets the value of Reserved
    pub fn get_reserved(&self) -> Option<&u16> {
        self.reserved.as_ref()
    }

    /// Sets the value of State
    pub fn set_state(&mut self, value: ConnectionStaticInfo_State) {
        self.state = Some(value);
    }

    /// Gets the value of State
    pub fn get_state(&self) -> Option<&ConnectionStaticInfo_State> {
        self.state.as_ref()
    }

    /// Sets the value of UniqueConnectionId
    pub fn set_unique_connection_id(&mut self, value: u64) {
        self.unique_connection_id = Some(value);
    }

    /// Gets the value of UniqueConnectionId
    pub fn get_unique_connection_id(&self) -> Option<&u64> {
        self.unique_connection_id.as_ref()
    }
}

