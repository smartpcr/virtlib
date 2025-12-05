// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_iSCSISession struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_iSCSISession {

/// 
    #[serde(rename = "AuthenticationType")]
    pub authentication_type: Option<String>,

/// 
    #[serde(rename = "InitiatorInstanceName")]
    pub initiator_instance_name: Option<String>,

/// 
    #[serde(rename = "InitiatorNodeAddress")]
    pub initiator_node_address: Option<String>,

/// 
    #[serde(rename = "InitiatorPortalAddress")]
    pub initiator_portal_address: Option<String>,

/// 
    #[serde(rename = "InitiatorSideIdentifier")]
    pub initiator_side_identifier: Option<String>,

/// 
    #[serde(rename = "IsConnected")]
    pub is_connected: Option<bool>,

/// 
    #[serde(rename = "IsDataDigest")]
    pub is_data_digest: Option<bool>,

/// 
    #[serde(rename = "IsDiscovered")]
    pub is_discovered: Option<bool>,

/// 
    #[serde(rename = "IsHeaderDigest")]
    pub is_header_digest: Option<bool>,

/// 
    #[serde(rename = "IsPersistent")]
    pub is_persistent: Option<bool>,

/// 
    #[serde(rename = "NumberOfConnections")]
    pub number_of_connections: Option<u32>,

/// 
    #[serde(rename = "SessionIdentifier")]
    pub session_identifier: Option<String>,

/// 
    #[serde(rename = "TargetNodeAddress")]
    pub target_node_address: Option<String>,

/// 
    #[serde(rename = "TargetSideIdentifier")]
    pub target_side_identifier: Option<String>,
}

impl MSFT_iSCSISession {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            authentication_type: None,
            initiator_instance_name: None,
            initiator_node_address: None,
            initiator_portal_address: None,
            initiator_side_identifier: None,
            is_connected: None,
            is_data_digest: None,
            is_discovered: None,
            is_header_digest: None,
            is_persistent: None,
            number_of_connections: None,
            session_identifier: None,
            target_node_address: None,
            target_side_identifier: None,
        }
    }


    /// Sets the value of AuthenticationType
    pub fn set_authentication_type(&mut self, value: String) {
        self.authentication_type = Some(value);
    }

    /// Gets the value of AuthenticationType
    pub fn get_authentication_type(&self) -> Option<&String> {
        self.authentication_type.as_ref()
    }

    /// Sets the value of InitiatorInstanceName
    pub fn set_initiator_instance_name(&mut self, value: String) {
        self.initiator_instance_name = Some(value);
    }

    /// Gets the value of InitiatorInstanceName
    pub fn get_initiator_instance_name(&self) -> Option<&String> {
        self.initiator_instance_name.as_ref()
    }

    /// Sets the value of InitiatorNodeAddress
    pub fn set_initiator_node_address(&mut self, value: String) {
        self.initiator_node_address = Some(value);
    }

    /// Gets the value of InitiatorNodeAddress
    pub fn get_initiator_node_address(&self) -> Option<&String> {
        self.initiator_node_address.as_ref()
    }

    /// Sets the value of InitiatorPortalAddress
    pub fn set_initiator_portal_address(&mut self, value: String) {
        self.initiator_portal_address = Some(value);
    }

    /// Gets the value of InitiatorPortalAddress
    pub fn get_initiator_portal_address(&self) -> Option<&String> {
        self.initiator_portal_address.as_ref()
    }

    /// Sets the value of InitiatorSideIdentifier
    pub fn set_initiator_side_identifier(&mut self, value: String) {
        self.initiator_side_identifier = Some(value);
    }

    /// Gets the value of InitiatorSideIdentifier
    pub fn get_initiator_side_identifier(&self) -> Option<&String> {
        self.initiator_side_identifier.as_ref()
    }

    /// Sets the value of IsConnected
    pub fn set_is_connected(&mut self, value: bool) {
        self.is_connected = Some(value);
    }

    /// Gets the value of IsConnected
    pub fn get_is_connected(&self) -> Option<&bool> {
        self.is_connected.as_ref()
    }

    /// Sets the value of IsDataDigest
    pub fn set_is_data_digest(&mut self, value: bool) {
        self.is_data_digest = Some(value);
    }

    /// Gets the value of IsDataDigest
    pub fn get_is_data_digest(&self) -> Option<&bool> {
        self.is_data_digest.as_ref()
    }

    /// Sets the value of IsDiscovered
    pub fn set_is_discovered(&mut self, value: bool) {
        self.is_discovered = Some(value);
    }

    /// Gets the value of IsDiscovered
    pub fn get_is_discovered(&self) -> Option<&bool> {
        self.is_discovered.as_ref()
    }

    /// Sets the value of IsHeaderDigest
    pub fn set_is_header_digest(&mut self, value: bool) {
        self.is_header_digest = Some(value);
    }

    /// Gets the value of IsHeaderDigest
    pub fn get_is_header_digest(&self) -> Option<&bool> {
        self.is_header_digest.as_ref()
    }

    /// Sets the value of IsPersistent
    pub fn set_is_persistent(&mut self, value: bool) {
        self.is_persistent = Some(value);
    }

    /// Gets the value of IsPersistent
    pub fn get_is_persistent(&self) -> Option<&bool> {
        self.is_persistent.as_ref()
    }

    /// Sets the value of NumberOfConnections
    pub fn set_number_of_connections(&mut self, value: u32) {
        self.number_of_connections = Some(value);
    }

    /// Gets the value of NumberOfConnections
    pub fn get_number_of_connections(&self) -> Option<&u32> {
        self.number_of_connections.as_ref()
    }

    /// Sets the value of SessionIdentifier
    pub fn set_session_identifier(&mut self, value: String) {
        self.session_identifier = Some(value);
    }

    /// Gets the value of SessionIdentifier
    pub fn get_session_identifier(&self) -> Option<&String> {
        self.session_identifier.as_ref()
    }

    /// Sets the value of TargetNodeAddress
    pub fn set_target_node_address(&mut self, value: String) {
        self.target_node_address = Some(value);
    }

    /// Gets the value of TargetNodeAddress
    pub fn get_target_node_address(&self) -> Option<&String> {
        self.target_node_address.as_ref()
    }

    /// Sets the value of TargetSideIdentifier
    pub fn set_target_side_identifier(&mut self, value: String) {
        self.target_side_identifier = Some(value);
    }

    /// Gets the value of TargetSideIdentifier
    pub fn get_target_side_identifier(&self) -> Option<&String> {
        self.target_side_identifier.as_ref()
    }

/// 

    /// * `chap_secret` -  (String)
    /// * `chap_username` -  (String)
    /// * `is_multipath_enabled` -  (bool)

    /// * `return_value` -  (u32)
    pub fn register(&self, is_multipath_enabled: bool, chap_username: &String, chap_secret: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "IsMultipathEnabled".to_string(), value: is_multipath_enabled.into() });
        args.push(MethodParameter { name: "ChapUsername".to_string(), value: chap_username.into() });
        args.push(MethodParameter { name: "ChapSecret".to_string(), value: chap_secret.into() });
        self.invoke_method("Register", &args)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn unregister(&self) -> Result<(), WmiError> {
        self.invoke_method("Unregister", &[])

    }


/// 

    /// * `chap_secret` -  (String)

    /// * `return_value` -  (u32)
    pub fn set_chapsecret(&self, chap_secret: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ChapSecret".to_string(), value: chap_secret.into() });
        self.invoke_method("SetCHAPSecret", &args)

    }

}

