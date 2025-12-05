// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_NetworkConnection struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_NetworkConnection {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// 
    #[serde(rename = "AccessMask")]
    pub access_mask: Option<u32>,

/// 
    #[serde(rename = "Comment")]
    pub comment: Option<String>,

/// 
    #[serde(rename = "ConnectionState")]
    pub connection_state: Option<String>,

/// 
    #[serde(rename = "ConnectionType")]
    pub connection_type: Option<String>,

/// 
    #[serde(rename = "DisplayType")]
    pub display_type: Option<String>,

/// 
    #[serde(rename = "LocalName")]
    pub local_name: Option<String>,

/// 
    #[serde(rename = "Persistent")]
    pub persistent: Option<bool>,

/// 
    #[serde(rename = "ProviderName")]
    pub provider_name: Option<String>,

/// 
    #[serde(rename = "RemoteName")]
    pub remote_name: Option<String>,

/// 
    #[serde(rename = "RemotePath")]
    pub remote_path: Option<String>,

/// 
    #[serde(rename = "ResourceType")]
    pub resource_type: Option<String>,

/// 
    #[serde(rename = "UserName")]
    pub user_name: Option<String>,
}

impl Win32_NetworkConnection {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
            access_mask: None,
            comment: None,
            connection_state: None,
            connection_type: None,
            display_type: None,
            local_name: None,
            persistent: None,
            provider_name: None,
            remote_name: None,
            remote_path: None,
            resource_type: None,
            user_name: None,
        }
    }


    /// Sets the value of AccessMask
    pub fn set_access_mask(&mut self, value: u32) {
        self.access_mask = Some(value);
    }

    /// Gets the value of AccessMask
    pub fn get_access_mask(&self) -> Option<&u32> {
        self.access_mask.as_ref()
    }

    /// Sets the value of Comment
    pub fn set_comment(&mut self, value: String) {
        self.comment = Some(value);
    }

    /// Gets the value of Comment
    pub fn get_comment(&self) -> Option<&String> {
        self.comment.as_ref()
    }

    /// Sets the value of ConnectionState
    pub fn set_connection_state(&mut self, value: String) {
        self.connection_state = Some(value);
    }

    /// Gets the value of ConnectionState
    pub fn get_connection_state(&self) -> Option<&String> {
        self.connection_state.as_ref()
    }

    /// Sets the value of ConnectionType
    pub fn set_connection_type(&mut self, value: String) {
        self.connection_type = Some(value);
    }

    /// Gets the value of ConnectionType
    pub fn get_connection_type(&self) -> Option<&String> {
        self.connection_type.as_ref()
    }

    /// Sets the value of DisplayType
    pub fn set_display_type(&mut self, value: String) {
        self.display_type = Some(value);
    }

    /// Gets the value of DisplayType
    pub fn get_display_type(&self) -> Option<&String> {
        self.display_type.as_ref()
    }

    /// Sets the value of LocalName
    pub fn set_local_name(&mut self, value: String) {
        self.local_name = Some(value);
    }

    /// Gets the value of LocalName
    pub fn get_local_name(&self) -> Option<&String> {
        self.local_name.as_ref()
    }

    /// Sets the value of Persistent
    pub fn set_persistent(&mut self, value: bool) {
        self.persistent = Some(value);
    }

    /// Gets the value of Persistent
    pub fn get_persistent(&self) -> Option<&bool> {
        self.persistent.as_ref()
    }

    /// Sets the value of ProviderName
    pub fn set_provider_name(&mut self, value: String) {
        self.provider_name = Some(value);
    }

    /// Gets the value of ProviderName
    pub fn get_provider_name(&self) -> Option<&String> {
        self.provider_name.as_ref()
    }

    /// Sets the value of RemoteName
    pub fn set_remote_name(&mut self, value: String) {
        self.remote_name = Some(value);
    }

    /// Gets the value of RemoteName
    pub fn get_remote_name(&self) -> Option<&String> {
        self.remote_name.as_ref()
    }

    /// Sets the value of RemotePath
    pub fn set_remote_path(&mut self, value: String) {
        self.remote_path = Some(value);
    }

    /// Gets the value of RemotePath
    pub fn get_remote_path(&self) -> Option<&String> {
        self.remote_path.as_ref()
    }

    /// Sets the value of ResourceType
    pub fn set_resource_type(&mut self, value: String) {
        self.resource_type = Some(value);
    }

    /// Gets the value of ResourceType
    pub fn get_resource_type(&self) -> Option<&String> {
        self.resource_type.as_ref()
    }

    /// Sets the value of UserName
    pub fn set_user_name(&mut self, value: String) {
        self.user_name = Some(value);
    }

    /// Gets the value of UserName
    pub fn get_user_name(&self) -> Option<&String> {
        self.user_name.as_ref()
    }
}

