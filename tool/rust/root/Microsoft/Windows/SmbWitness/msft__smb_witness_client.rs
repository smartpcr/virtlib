// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.SmbWitness
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_SmbWitnessClient struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_SmbWitnessClient {

/// 
    #[serde(rename = "ClientName")]
    pub client_name: Option<String>,

/// 
    #[serde(rename = "FileServerNodeName")]
    pub file_server_node_name: Option<String>,

/// 
    #[serde(rename = "Flags")]
    pub flags: Option<u32>,

/// 
    #[serde(rename = "IPAddress")]
    pub ipaddress: Option<String>,

/// 
    #[serde(rename = "NetworkName")]
    pub network_name: Option<String>,

/// 
    #[serde(rename = "NotificationsCancelled")]
    pub notifications_cancelled: Option<u32>,

/// 
    #[serde(rename = "NotificationsSent")]
    pub notifications_sent: Option<u32>,

/// 
    #[serde(rename = "QueuedNotifications")]
    pub queued_notifications: Option<u32>,

/// 
    #[serde(rename = "ResourcesMonitored")]
    pub resources_monitored: Option<u32>,

/// 
    #[serde(rename = "ShareName")]
    pub share_name: Option<String>,

/// 
    #[serde(rename = "State")]
    pub state: Option<SmbWitnessClient_State>,

/// 
    #[serde(rename = "WitnessNodeName")]
    pub witness_node_name: Option<String>,
}

impl MSFT_SmbWitnessClient {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            client_name: None,
            file_server_node_name: None,
            flags: None,
            ipaddress: None,
            network_name: None,
            notifications_cancelled: None,
            notifications_sent: None,
            queued_notifications: None,
            resources_monitored: None,
            share_name: None,
            state: None,
            witness_node_name: None,
        }
    }


    /// Sets the value of ClientName
    pub fn set_client_name(&mut self, value: String) {
        self.client_name = Some(value);
    }

    /// Gets the value of ClientName
    pub fn get_client_name(&self) -> Option<&String> {
        self.client_name.as_ref()
    }

    /// Sets the value of FileServerNodeName
    pub fn set_file_server_node_name(&mut self, value: String) {
        self.file_server_node_name = Some(value);
    }

    /// Gets the value of FileServerNodeName
    pub fn get_file_server_node_name(&self) -> Option<&String> {
        self.file_server_node_name.as_ref()
    }

    /// Sets the value of Flags
    pub fn set_flags(&mut self, value: u32) {
        self.flags = Some(value);
    }

    /// Gets the value of Flags
    pub fn get_flags(&self) -> Option<&u32> {
        self.flags.as_ref()
    }

    /// Sets the value of IPAddress
    pub fn set_ipaddress(&mut self, value: String) {
        self.ipaddress = Some(value);
    }

    /// Gets the value of IPAddress
    pub fn get_ipaddress(&self) -> Option<&String> {
        self.ipaddress.as_ref()
    }

    /// Sets the value of NetworkName
    pub fn set_network_name(&mut self, value: String) {
        self.network_name = Some(value);
    }

    /// Gets the value of NetworkName
    pub fn get_network_name(&self) -> Option<&String> {
        self.network_name.as_ref()
    }

    /// Sets the value of NotificationsCancelled
    pub fn set_notifications_cancelled(&mut self, value: u32) {
        self.notifications_cancelled = Some(value);
    }

    /// Gets the value of NotificationsCancelled
    pub fn get_notifications_cancelled(&self) -> Option<&u32> {
        self.notifications_cancelled.as_ref()
    }

    /// Sets the value of NotificationsSent
    pub fn set_notifications_sent(&mut self, value: u32) {
        self.notifications_sent = Some(value);
    }

    /// Gets the value of NotificationsSent
    pub fn get_notifications_sent(&self) -> Option<&u32> {
        self.notifications_sent.as_ref()
    }

    /// Sets the value of QueuedNotifications
    pub fn set_queued_notifications(&mut self, value: u32) {
        self.queued_notifications = Some(value);
    }

    /// Gets the value of QueuedNotifications
    pub fn get_queued_notifications(&self) -> Option<&u32> {
        self.queued_notifications.as_ref()
    }

    /// Sets the value of ResourcesMonitored
    pub fn set_resources_monitored(&mut self, value: u32) {
        self.resources_monitored = Some(value);
    }

    /// Gets the value of ResourcesMonitored
    pub fn get_resources_monitored(&self) -> Option<&u32> {
        self.resources_monitored.as_ref()
    }

    /// Sets the value of ShareName
    pub fn set_share_name(&mut self, value: String) {
        self.share_name = Some(value);
    }

    /// Gets the value of ShareName
    pub fn get_share_name(&self) -> Option<&String> {
        self.share_name.as_ref()
    }

    /// Sets the value of State
    pub fn set_state(&mut self, value: SmbWitnessClient_State) {
        self.state = Some(value);
    }

    /// Gets the value of State
    pub fn get_state(&self) -> Option<&SmbWitnessClient_State> {
        self.state.as_ref()
    }

    /// Sets the value of WitnessNodeName
    pub fn set_witness_node_name(&mut self, value: String) {
        self.witness_node_name = Some(value);
    }

    /// Gets the value of WitnessNodeName
    pub fn get_witness_node_name(&self) -> Option<&String> {
        self.witness_node_name.as_ref()
    }

/// 

    /// * `client_name` -  (String)
    /// * `destination_node` -  (String)
    /// * `network_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn move_client(&self, client_name: &String, destination_node: &String, network_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ClientName".to_string(), value: client_name.into() });
        args.push(MethodParameter { name: "DestinationNode".to_string(), value: destination_node.into() });
        args.push(MethodParameter { name: "NetworkName".to_string(), value: network_name.into() });
        self.invoke_method("MoveClient", &args)

    }

}

