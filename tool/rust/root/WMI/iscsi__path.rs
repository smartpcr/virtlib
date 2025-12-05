// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ISCSI_Path struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ISCSI_Path {

/// Status of the path - connected, disconnected, reconnecting
    #[serde(rename = "ConnectionStatus")]
    pub connection_status: Option<Path_ConnectionStatus>,

/// Estimated speed of the connection in MegaBits Per Second
    #[serde(rename = "EstimatedLinkSpeed")]
    pub estimated_link_speed: Option<u64>,

/// Weight assigned to the path
    #[serde(rename = "PathWeight")]
    pub path_weight: Option<u32>,

/// Flag set to 1 if the path is a primary path, 0 otherwise.
    #[serde(rename = "PrimaryPath")]
    pub primary_path: Option<u32>,

/// Flag set to 1 if TCP offload is supported for this connection, 0 otherwise.
    #[serde(rename = "TCPOffLoadAvailable")]
    pub tcpoff_load_available: Option<u32>,

/// iSCSI Unique connection id
    #[serde(rename = "UniqueConnectionId")]
    pub unique_connection_id: Option<u64>,
}

impl ISCSI_Path {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            connection_status: None,
            estimated_link_speed: None,
            path_weight: None,
            primary_path: None,
            tcpoff_load_available: None,
            unique_connection_id: None,
        }
    }


    /// Sets the value of ConnectionStatus
    pub fn set_connection_status(&mut self, value: Path_ConnectionStatus) {
        self.connection_status = Some(value);
    }

    /// Gets the value of ConnectionStatus
    pub fn get_connection_status(&self) -> Option<&Path_ConnectionStatus> {
        self.connection_status.as_ref()
    }

    /// Sets the value of EstimatedLinkSpeed
    pub fn set_estimated_link_speed(&mut self, value: u64) {
        self.estimated_link_speed = Some(value);
    }

    /// Gets the value of EstimatedLinkSpeed
    pub fn get_estimated_link_speed(&self) -> Option<&u64> {
        self.estimated_link_speed.as_ref()
    }

    /// Sets the value of PathWeight
    pub fn set_path_weight(&mut self, value: u32) {
        self.path_weight = Some(value);
    }

    /// Gets the value of PathWeight
    pub fn get_path_weight(&self) -> Option<&u32> {
        self.path_weight.as_ref()
    }

    /// Sets the value of PrimaryPath
    pub fn set_primary_path(&mut self, value: u32) {
        self.primary_path = Some(value);
    }

    /// Gets the value of PrimaryPath
    pub fn get_primary_path(&self) -> Option<&u32> {
        self.primary_path.as_ref()
    }

    /// Sets the value of TCPOffLoadAvailable
    pub fn set_tcpoff_load_available(&mut self, value: u32) {
        self.tcpoff_load_available = Some(value);
    }

    /// Gets the value of TCPOffLoadAvailable
    pub fn get_tcpoff_load_available(&self) -> Option<&u32> {
        self.tcpoff_load_available.as_ref()
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

