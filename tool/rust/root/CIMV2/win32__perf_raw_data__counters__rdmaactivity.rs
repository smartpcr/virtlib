// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_Counters_RDMAActivity struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_Counters_RDMAActivity {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "RDMAAcceptedConnections")]
    pub rdmaaccepted_connections: Option<u32>,

/// 
    #[serde(rename = "RDMAActiveConnections")]
    pub rdmaactive_connections: Option<u32>,

/// 
    #[serde(rename = "RDMACompletionQueueErrors")]
    pub rdmacompletion_queue_errors: Option<u32>,

/// 
    #[serde(rename = "RDMAConnectionErrors")]
    pub rdmaconnection_errors: Option<u32>,

/// 
    #[serde(rename = "RDMAFailedConnectionAttempts")]
    pub rdmafailed_connection_attempts: Option<u32>,

/// 
    #[serde(rename = "RDMAInboundBytesPersec")]
    pub rdmainbound_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "RDMAInboundFramesPersec")]
    pub rdmainbound_frames_persec: Option<u64>,

/// 
    #[serde(rename = "RDMAInitiatedConnections")]
    pub rdmainitiated_connections: Option<u32>,

/// 
    #[serde(rename = "RDMAOutboundBytesPersec")]
    pub rdmaoutbound_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "RDMAOutboundFramesPersec")]
    pub rdmaoutbound_frames_persec: Option<u64>,
}

impl Win32_PerfRawData_Counters_RDMAActivity {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            rdmaaccepted_connections: None,
            rdmaactive_connections: None,
            rdmacompletion_queue_errors: None,
            rdmaconnection_errors: None,
            rdmafailed_connection_attempts: None,
            rdmainbound_bytes_persec: None,
            rdmainbound_frames_persec: None,
            rdmainitiated_connections: None,
            rdmaoutbound_bytes_persec: None,
            rdmaoutbound_frames_persec: None,
        }
    }


    /// Sets the value of RDMAAcceptedConnections
    pub fn set_rdmaaccepted_connections(&mut self, value: u32) {
        self.rdmaaccepted_connections = Some(value);
    }

    /// Gets the value of RDMAAcceptedConnections
    pub fn get_rdmaaccepted_connections(&self) -> Option<&u32> {
        self.rdmaaccepted_connections.as_ref()
    }

    /// Sets the value of RDMAActiveConnections
    pub fn set_rdmaactive_connections(&mut self, value: u32) {
        self.rdmaactive_connections = Some(value);
    }

    /// Gets the value of RDMAActiveConnections
    pub fn get_rdmaactive_connections(&self) -> Option<&u32> {
        self.rdmaactive_connections.as_ref()
    }

    /// Sets the value of RDMACompletionQueueErrors
    pub fn set_rdmacompletion_queue_errors(&mut self, value: u32) {
        self.rdmacompletion_queue_errors = Some(value);
    }

    /// Gets the value of RDMACompletionQueueErrors
    pub fn get_rdmacompletion_queue_errors(&self) -> Option<&u32> {
        self.rdmacompletion_queue_errors.as_ref()
    }

    /// Sets the value of RDMAConnectionErrors
    pub fn set_rdmaconnection_errors(&mut self, value: u32) {
        self.rdmaconnection_errors = Some(value);
    }

    /// Gets the value of RDMAConnectionErrors
    pub fn get_rdmaconnection_errors(&self) -> Option<&u32> {
        self.rdmaconnection_errors.as_ref()
    }

    /// Sets the value of RDMAFailedConnectionAttempts
    pub fn set_rdmafailed_connection_attempts(&mut self, value: u32) {
        self.rdmafailed_connection_attempts = Some(value);
    }

    /// Gets the value of RDMAFailedConnectionAttempts
    pub fn get_rdmafailed_connection_attempts(&self) -> Option<&u32> {
        self.rdmafailed_connection_attempts.as_ref()
    }

    /// Sets the value of RDMAInboundBytesPersec
    pub fn set_rdmainbound_bytes_persec(&mut self, value: u64) {
        self.rdmainbound_bytes_persec = Some(value);
    }

    /// Gets the value of RDMAInboundBytesPersec
    pub fn get_rdmainbound_bytes_persec(&self) -> Option<&u64> {
        self.rdmainbound_bytes_persec.as_ref()
    }

    /// Sets the value of RDMAInboundFramesPersec
    pub fn set_rdmainbound_frames_persec(&mut self, value: u64) {
        self.rdmainbound_frames_persec = Some(value);
    }

    /// Gets the value of RDMAInboundFramesPersec
    pub fn get_rdmainbound_frames_persec(&self) -> Option<&u64> {
        self.rdmainbound_frames_persec.as_ref()
    }

    /// Sets the value of RDMAInitiatedConnections
    pub fn set_rdmainitiated_connections(&mut self, value: u32) {
        self.rdmainitiated_connections = Some(value);
    }

    /// Gets the value of RDMAInitiatedConnections
    pub fn get_rdmainitiated_connections(&self) -> Option<&u32> {
        self.rdmainitiated_connections.as_ref()
    }

    /// Sets the value of RDMAOutboundBytesPersec
    pub fn set_rdmaoutbound_bytes_persec(&mut self, value: u64) {
        self.rdmaoutbound_bytes_persec = Some(value);
    }

    /// Gets the value of RDMAOutboundBytesPersec
    pub fn get_rdmaoutbound_bytes_persec(&self) -> Option<&u64> {
        self.rdmaoutbound_bytes_persec.as_ref()
    }

    /// Sets the value of RDMAOutboundFramesPersec
    pub fn set_rdmaoutbound_frames_persec(&mut self, value: u64) {
        self.rdmaoutbound_frames_persec = Some(value);
    }

    /// Gets the value of RDMAOutboundFramesPersec
    pub fn get_rdmaoutbound_frames_persec(&self) -> Option<&u64> {
        self.rdmaoutbound_frames_persec.as_ref()
    }
}

