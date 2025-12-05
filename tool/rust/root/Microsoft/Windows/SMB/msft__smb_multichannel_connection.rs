// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.SMB
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_SmbMultichannelConnection struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_SmbMultichannelConnection {

/// 
    #[serde(rename = "ClientInterfaceFriendlyName")]
    pub client_interface_friendly_name: Option<String>,

/// 
    #[serde(rename = "ClientInterfaceIndex")]
    pub client_interface_index: Option<u32>,

/// 
    #[serde(rename = "ClientIpAddress")]
    pub client_ip_address: Option<String>,

/// 
    #[serde(rename = "ClientLinkSpeed")]
    pub client_link_speed: Option<u64>,

/// 
    #[serde(rename = "ClientRdmaCapable")]
    pub client_rdma_capable: Option<bool>,

/// 
    #[serde(rename = "ClientRSSCapable")]
    pub client_rsscapable: Option<bool>,

/// 
    #[serde(rename = "CurrentChannels")]
    pub current_channels: Option<u32>,

/// 
    #[serde(rename = "Failed")]
    pub failed: Option<bool>,

/// 
    #[serde(rename = "FailureCount")]
    pub failure_count: Option<u32>,

/// 
    #[serde(rename = "MaxChannels")]
    pub max_channels: Option<u32>,

/// 
    #[serde(rename = "QuicConnectionCount")]
    pub quic_connection_count: Option<u16>,

/// 
    #[serde(rename = "RdmaConnectionCount")]
    pub rdma_connection_count: Option<u16>,

/// 
    #[serde(rename = "Selected")]
    pub selected: Option<bool>,

/// 
    #[serde(rename = "ServerInterfaceIndex")]
    pub server_interface_index: Option<u32>,

/// 
    #[serde(rename = "ServerIpAddress")]
    pub server_ip_address: Option<String>,

/// 
    #[serde(rename = "ServerLinkSpeed")]
    pub server_link_speed: Option<u64>,

/// 
    #[serde(rename = "ServerName")]
    pub server_name: Option<String>,

/// 
    #[serde(rename = "ServerRdmaCapable")]
    pub server_rdma_capable: Option<bool>,

/// 
    #[serde(rename = "ServerRSSCapable")]
    pub server_rsscapable: Option<bool>,

/// 
    #[serde(rename = "SmbInstance")]
    pub smb_instance: Option<SmbMultichannelConnection_SmbInstance>,

/// 
    #[serde(rename = "TcpConnectionCount")]
    pub tcp_connection_count: Option<u16>,
}

impl MSFT_SmbMultichannelConnection {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            client_interface_friendly_name: None,
            client_interface_index: None,
            client_ip_address: None,
            client_link_speed: None,
            client_rdma_capable: None,
            client_rsscapable: None,
            current_channels: None,
            failed: None,
            failure_count: None,
            max_channels: None,
            quic_connection_count: None,
            rdma_connection_count: None,
            selected: None,
            server_interface_index: None,
            server_ip_address: None,
            server_link_speed: None,
            server_name: None,
            server_rdma_capable: None,
            server_rsscapable: None,
            smb_instance: None,
            tcp_connection_count: None,
        }
    }


    /// Sets the value of ClientInterfaceFriendlyName
    pub fn set_client_interface_friendly_name(&mut self, value: String) {
        self.client_interface_friendly_name = Some(value);
    }

    /// Gets the value of ClientInterfaceFriendlyName
    pub fn get_client_interface_friendly_name(&self) -> Option<&String> {
        self.client_interface_friendly_name.as_ref()
    }

    /// Sets the value of ClientInterfaceIndex
    pub fn set_client_interface_index(&mut self, value: u32) {
        self.client_interface_index = Some(value);
    }

    /// Gets the value of ClientInterfaceIndex
    pub fn get_client_interface_index(&self) -> Option<&u32> {
        self.client_interface_index.as_ref()
    }

    /// Sets the value of ClientIpAddress
    pub fn set_client_ip_address(&mut self, value: String) {
        self.client_ip_address = Some(value);
    }

    /// Gets the value of ClientIpAddress
    pub fn get_client_ip_address(&self) -> Option<&String> {
        self.client_ip_address.as_ref()
    }

    /// Sets the value of ClientLinkSpeed
    pub fn set_client_link_speed(&mut self, value: u64) {
        self.client_link_speed = Some(value);
    }

    /// Gets the value of ClientLinkSpeed
    pub fn get_client_link_speed(&self) -> Option<&u64> {
        self.client_link_speed.as_ref()
    }

    /// Sets the value of ClientRdmaCapable
    pub fn set_client_rdma_capable(&mut self, value: bool) {
        self.client_rdma_capable = Some(value);
    }

    /// Gets the value of ClientRdmaCapable
    pub fn get_client_rdma_capable(&self) -> Option<&bool> {
        self.client_rdma_capable.as_ref()
    }

    /// Sets the value of ClientRSSCapable
    pub fn set_client_rsscapable(&mut self, value: bool) {
        self.client_rsscapable = Some(value);
    }

    /// Gets the value of ClientRSSCapable
    pub fn get_client_rsscapable(&self) -> Option<&bool> {
        self.client_rsscapable.as_ref()
    }

    /// Sets the value of CurrentChannels
    pub fn set_current_channels(&mut self, value: u32) {
        self.current_channels = Some(value);
    }

    /// Gets the value of CurrentChannels
    pub fn get_current_channels(&self) -> Option<&u32> {
        self.current_channels.as_ref()
    }

    /// Sets the value of Failed
    pub fn set_failed(&mut self, value: bool) {
        self.failed = Some(value);
    }

    /// Gets the value of Failed
    pub fn get_failed(&self) -> Option<&bool> {
        self.failed.as_ref()
    }

    /// Sets the value of FailureCount
    pub fn set_failure_count(&mut self, value: u32) {
        self.failure_count = Some(value);
    }

    /// Gets the value of FailureCount
    pub fn get_failure_count(&self) -> Option<&u32> {
        self.failure_count.as_ref()
    }

    /// Sets the value of MaxChannels
    pub fn set_max_channels(&mut self, value: u32) {
        self.max_channels = Some(value);
    }

    /// Gets the value of MaxChannels
    pub fn get_max_channels(&self) -> Option<&u32> {
        self.max_channels.as_ref()
    }

    /// Sets the value of QuicConnectionCount
    pub fn set_quic_connection_count(&mut self, value: u16) {
        self.quic_connection_count = Some(value);
    }

    /// Gets the value of QuicConnectionCount
    pub fn get_quic_connection_count(&self) -> Option<&u16> {
        self.quic_connection_count.as_ref()
    }

    /// Sets the value of RdmaConnectionCount
    pub fn set_rdma_connection_count(&mut self, value: u16) {
        self.rdma_connection_count = Some(value);
    }

    /// Gets the value of RdmaConnectionCount
    pub fn get_rdma_connection_count(&self) -> Option<&u16> {
        self.rdma_connection_count.as_ref()
    }

    /// Sets the value of Selected
    pub fn set_selected(&mut self, value: bool) {
        self.selected = Some(value);
    }

    /// Gets the value of Selected
    pub fn get_selected(&self) -> Option<&bool> {
        self.selected.as_ref()
    }

    /// Sets the value of ServerInterfaceIndex
    pub fn set_server_interface_index(&mut self, value: u32) {
        self.server_interface_index = Some(value);
    }

    /// Gets the value of ServerInterfaceIndex
    pub fn get_server_interface_index(&self) -> Option<&u32> {
        self.server_interface_index.as_ref()
    }

    /// Sets the value of ServerIpAddress
    pub fn set_server_ip_address(&mut self, value: String) {
        self.server_ip_address = Some(value);
    }

    /// Gets the value of ServerIpAddress
    pub fn get_server_ip_address(&self) -> Option<&String> {
        self.server_ip_address.as_ref()
    }

    /// Sets the value of ServerLinkSpeed
    pub fn set_server_link_speed(&mut self, value: u64) {
        self.server_link_speed = Some(value);
    }

    /// Gets the value of ServerLinkSpeed
    pub fn get_server_link_speed(&self) -> Option<&u64> {
        self.server_link_speed.as_ref()
    }

    /// Sets the value of ServerName
    pub fn set_server_name(&mut self, value: String) {
        self.server_name = Some(value);
    }

    /// Gets the value of ServerName
    pub fn get_server_name(&self) -> Option<&String> {
        self.server_name.as_ref()
    }

    /// Sets the value of ServerRdmaCapable
    pub fn set_server_rdma_capable(&mut self, value: bool) {
        self.server_rdma_capable = Some(value);
    }

    /// Gets the value of ServerRdmaCapable
    pub fn get_server_rdma_capable(&self) -> Option<&bool> {
        self.server_rdma_capable.as_ref()
    }

    /// Sets the value of ServerRSSCapable
    pub fn set_server_rsscapable(&mut self, value: bool) {
        self.server_rsscapable = Some(value);
    }

    /// Gets the value of ServerRSSCapable
    pub fn get_server_rsscapable(&self) -> Option<&bool> {
        self.server_rsscapable.as_ref()
    }

    /// Sets the value of SmbInstance
    pub fn set_smb_instance(&mut self, value: SmbMultichannelConnection_SmbInstance) {
        self.smb_instance = Some(value);
    }

    /// Gets the value of SmbInstance
    pub fn get_smb_instance(&self) -> Option<&SmbMultichannelConnection_SmbInstance> {
        self.smb_instance.as_ref()
    }

    /// Sets the value of TcpConnectionCount
    pub fn set_tcp_connection_count(&mut self, value: u16) {
        self.tcp_connection_count = Some(value);
    }

    /// Gets the value of TcpConnectionCount
    pub fn get_tcp_connection_count(&self) -> Option<&u16> {
        self.tcp_connection_count.as_ref()
    }

/// 

    /// * `server_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn refresh(&self, server_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ServerName".to_string(), value: server_name.into() });
        self.invoke_method("Refresh", &args)

    }

}

