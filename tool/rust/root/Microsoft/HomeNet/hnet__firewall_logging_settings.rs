// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.HomeNet
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// HNet_FirewallLoggingSettings struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HNet_FirewallLoggingSettings {

/// 
    #[serde(rename = "Id")]
    pub id: Option<String>,

/// 
    #[serde(rename = "LogConnections")]
    pub log_connections: Option<bool>,

/// 
    #[serde(rename = "LogDroppedPackets")]
    pub log_dropped_packets: Option<bool>,

/// 
    #[serde(rename = "MaxFileSize")]
    pub max_file_size: Option<u32>,

/// 
    #[serde(rename = "Path")]
    pub path: Option<String>,
}

impl HNet_FirewallLoggingSettings {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            id: None,
            log_connections: None,
            log_dropped_packets: None,
            max_file_size: None,
            path: None,
        }
    }


    /// Sets the value of Id
    pub fn set_id(&mut self, value: String) {
        self.id = Some(value);
    }

    /// Gets the value of Id
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Sets the value of LogConnections
    pub fn set_log_connections(&mut self, value: bool) {
        self.log_connections = Some(value);
    }

    /// Gets the value of LogConnections
    pub fn get_log_connections(&self) -> Option<&bool> {
        self.log_connections.as_ref()
    }

    /// Sets the value of LogDroppedPackets
    pub fn set_log_dropped_packets(&mut self, value: bool) {
        self.log_dropped_packets = Some(value);
    }

    /// Gets the value of LogDroppedPackets
    pub fn get_log_dropped_packets(&self) -> Option<&bool> {
        self.log_dropped_packets.as_ref()
    }

    /// Sets the value of MaxFileSize
    pub fn set_max_file_size(&mut self, value: u32) {
        self.max_file_size = Some(value);
    }

    /// Gets the value of MaxFileSize
    pub fn get_max_file_size(&self) -> Option<&u32> {
        self.max_file_size.as_ref()
    }

    /// Sets the value of Path
    pub fn set_path(&mut self, value: String) {
        self.path = Some(value);
    }

    /// Gets the value of Path
    pub fn get_path(&self) -> Option<&String> {
        self.path.as_ref()
    }
}

