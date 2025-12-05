// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Config01_RemoteDesktopServices02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Config01_RemoteDesktopServices02 {

/// 
    #[serde(rename = "AllowUsersToConnectRemotely")]
    pub allow_users_to_connect_remotely: Option<String>,

/// 
    #[serde(rename = "ClientConnectionEncryptionLevel")]
    pub client_connection_encryption_level: Option<String>,

/// 
    #[serde(rename = "DoNotAllowDriveRedirection")]
    pub do_not_allow_drive_redirection: Option<String>,

/// 
    #[serde(rename = "DoNotAllowPasswordSaving")]
    pub do_not_allow_password_saving: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "PromptForPasswordUponConnection")]
    pub prompt_for_password_upon_connection: Option<String>,

/// 
    #[serde(rename = "RequireSecureRPCCommunication")]
    pub require_secure_rpccommunication: Option<String>,
}

impl MDM_Policy_Config01_RemoteDesktopServices02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allow_users_to_connect_remotely: None,
            client_connection_encryption_level: None,
            do_not_allow_drive_redirection: None,
            do_not_allow_password_saving: None,
            instance_id: None,
            parent_id: None,
            prompt_for_password_upon_connection: None,
            require_secure_rpccommunication: None,
        }
    }


    /// Sets the value of AllowUsersToConnectRemotely
    pub fn set_allow_users_to_connect_remotely(&mut self, value: String) {
        self.allow_users_to_connect_remotely = Some(value);
    }

    /// Gets the value of AllowUsersToConnectRemotely
    pub fn get_allow_users_to_connect_remotely(&self) -> Option<&String> {
        self.allow_users_to_connect_remotely.as_ref()
    }

    /// Sets the value of ClientConnectionEncryptionLevel
    pub fn set_client_connection_encryption_level(&mut self, value: String) {
        self.client_connection_encryption_level = Some(value);
    }

    /// Gets the value of ClientConnectionEncryptionLevel
    pub fn get_client_connection_encryption_level(&self) -> Option<&String> {
        self.client_connection_encryption_level.as_ref()
    }

    /// Sets the value of DoNotAllowDriveRedirection
    pub fn set_do_not_allow_drive_redirection(&mut self, value: String) {
        self.do_not_allow_drive_redirection = Some(value);
    }

    /// Gets the value of DoNotAllowDriveRedirection
    pub fn get_do_not_allow_drive_redirection(&self) -> Option<&String> {
        self.do_not_allow_drive_redirection.as_ref()
    }

    /// Sets the value of DoNotAllowPasswordSaving
    pub fn set_do_not_allow_password_saving(&mut self, value: String) {
        self.do_not_allow_password_saving = Some(value);
    }

    /// Gets the value of DoNotAllowPasswordSaving
    pub fn get_do_not_allow_password_saving(&self) -> Option<&String> {
        self.do_not_allow_password_saving.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of PromptForPasswordUponConnection
    pub fn set_prompt_for_password_upon_connection(&mut self, value: String) {
        self.prompt_for_password_upon_connection = Some(value);
    }

    /// Gets the value of PromptForPasswordUponConnection
    pub fn get_prompt_for_password_upon_connection(&self) -> Option<&String> {
        self.prompt_for_password_upon_connection.as_ref()
    }

    /// Sets the value of RequireSecureRPCCommunication
    pub fn set_require_secure_rpccommunication(&mut self, value: String) {
        self.require_secure_rpccommunication = Some(value);
    }

    /// Gets the value of RequireSecureRPCCommunication
    pub fn get_require_secure_rpccommunication(&self) -> Option<&String> {
        self.require_secure_rpccommunication.as_ref()
    }
}

