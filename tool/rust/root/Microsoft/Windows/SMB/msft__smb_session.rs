// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.SMB
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_SmbSession struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_SmbSession {

/// 
    #[serde(rename = "ClientComputerName")]
    pub client_computer_name: Option<String>,

/// 
    #[serde(rename = "ClientUserName")]
    pub client_user_name: Option<String>,

/// 
    #[serde(rename = "ClusterNodeName")]
    pub cluster_node_name: Option<String>,

/// 
    #[serde(rename = "Dialect")]
    pub dialect: Option<String>,

/// 
    #[serde(rename = "NumOpens")]
    pub num_opens: Option<u64>,

/// 
    #[serde(rename = "ScopeName")]
    pub scope_name: Option<String>,

/// 
    #[serde(rename = "SecondsExists")]
    pub seconds_exists: Option<u32>,

/// 
    #[serde(rename = "SecondsIdle")]
    pub seconds_idle: Option<u32>,

/// 
    #[serde(rename = "SessionId")]
    pub session_id: Option<u64>,

/// 
    #[serde(rename = "SmbInstance")]
    pub smb_instance: Option<SmbSession_SmbInstance>,

/// 
    #[serde(rename = "TransportName")]
    pub transport_name: Option<String>,
}

impl MSFT_SmbSession {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            client_computer_name: None,
            client_user_name: None,
            cluster_node_name: None,
            dialect: None,
            num_opens: None,
            scope_name: None,
            seconds_exists: None,
            seconds_idle: None,
            session_id: None,
            smb_instance: None,
            transport_name: None,
        }
    }


    /// Sets the value of ClientComputerName
    pub fn set_client_computer_name(&mut self, value: String) {
        self.client_computer_name = Some(value);
    }

    /// Gets the value of ClientComputerName
    pub fn get_client_computer_name(&self) -> Option<&String> {
        self.client_computer_name.as_ref()
    }

    /// Sets the value of ClientUserName
    pub fn set_client_user_name(&mut self, value: String) {
        self.client_user_name = Some(value);
    }

    /// Gets the value of ClientUserName
    pub fn get_client_user_name(&self) -> Option<&String> {
        self.client_user_name.as_ref()
    }

    /// Sets the value of ClusterNodeName
    pub fn set_cluster_node_name(&mut self, value: String) {
        self.cluster_node_name = Some(value);
    }

    /// Gets the value of ClusterNodeName
    pub fn get_cluster_node_name(&self) -> Option<&String> {
        self.cluster_node_name.as_ref()
    }

    /// Sets the value of Dialect
    pub fn set_dialect(&mut self, value: String) {
        self.dialect = Some(value);
    }

    /// Gets the value of Dialect
    pub fn get_dialect(&self) -> Option<&String> {
        self.dialect.as_ref()
    }

    /// Sets the value of NumOpens
    pub fn set_num_opens(&mut self, value: u64) {
        self.num_opens = Some(value);
    }

    /// Gets the value of NumOpens
    pub fn get_num_opens(&self) -> Option<&u64> {
        self.num_opens.as_ref()
    }

    /// Sets the value of ScopeName
    pub fn set_scope_name(&mut self, value: String) {
        self.scope_name = Some(value);
    }

    /// Gets the value of ScopeName
    pub fn get_scope_name(&self) -> Option<&String> {
        self.scope_name.as_ref()
    }

    /// Sets the value of SecondsExists
    pub fn set_seconds_exists(&mut self, value: u32) {
        self.seconds_exists = Some(value);
    }

    /// Gets the value of SecondsExists
    pub fn get_seconds_exists(&self) -> Option<&u32> {
        self.seconds_exists.as_ref()
    }

    /// Sets the value of SecondsIdle
    pub fn set_seconds_idle(&mut self, value: u32) {
        self.seconds_idle = Some(value);
    }

    /// Gets the value of SecondsIdle
    pub fn get_seconds_idle(&self) -> Option<&u32> {
        self.seconds_idle.as_ref()
    }

    /// Sets the value of SessionId
    pub fn set_session_id(&mut self, value: u64) {
        self.session_id = Some(value);
    }

    /// Gets the value of SessionId
    pub fn get_session_id(&self) -> Option<&u64> {
        self.session_id.as_ref()
    }

    /// Sets the value of SmbInstance
    pub fn set_smb_instance(&mut self, value: SmbSession_SmbInstance) {
        self.smb_instance = Some(value);
    }

    /// Gets the value of SmbInstance
    pub fn get_smb_instance(&self) -> Option<&SmbSession_SmbInstance> {
        self.smb_instance.as_ref()
    }

    /// Sets the value of TransportName
    pub fn set_transport_name(&mut self, value: String) {
        self.transport_name = Some(value);
    }

    /// Gets the value of TransportName
    pub fn get_transport_name(&self) -> Option<&String> {
        self.transport_name.as_ref()
    }

/// 

    /// * `return_value` -  (u32)
    pub fn force_close(&self) -> Result<(), WmiError> {
        self.invoke_method("ForceClose", &[])

    }

}

