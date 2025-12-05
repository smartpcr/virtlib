// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.SMB
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_SmbOpenFile struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_SmbOpenFile {

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
    #[serde(rename = "ContinuouslyAvailable")]
    pub continuously_available: Option<bool>,

/// 
    #[serde(rename = "Encrypted")]
    pub encrypted: Option<bool>,

/// 
    #[serde(rename = "FileId")]
    pub file_id: Option<u64>,

/// 
    #[serde(rename = "Locks")]
    pub locks: Option<u32>,

/// 
    #[serde(rename = "Path")]
    pub path: Option<String>,

/// 
    #[serde(rename = "Permissions")]
    pub permissions: Option<u32>,

/// 
    #[serde(rename = "ScopeName")]
    pub scope_name: Option<String>,

/// 
    #[serde(rename = "SessionId")]
    pub session_id: Option<u64>,

/// 
    #[serde(rename = "ShareRelativePath")]
    pub share_relative_path: Option<String>,

/// 
    #[serde(rename = "Signed")]
    pub signed: Option<bool>,

/// 
    #[serde(rename = "SmbInstance")]
    pub smb_instance: Option<SmbOpenFile_SmbInstance>,
}

impl MSFT_SmbOpenFile {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            client_computer_name: None,
            client_user_name: None,
            cluster_node_name: None,
            continuously_available: None,
            encrypted: None,
            file_id: None,
            locks: None,
            path: None,
            permissions: None,
            scope_name: None,
            session_id: None,
            share_relative_path: None,
            signed: None,
            smb_instance: None,
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

    /// Sets the value of ContinuouslyAvailable
    pub fn set_continuously_available(&mut self, value: bool) {
        self.continuously_available = Some(value);
    }

    /// Gets the value of ContinuouslyAvailable
    pub fn get_continuously_available(&self) -> Option<&bool> {
        self.continuously_available.as_ref()
    }

    /// Sets the value of Encrypted
    pub fn set_encrypted(&mut self, value: bool) {
        self.encrypted = Some(value);
    }

    /// Gets the value of Encrypted
    pub fn get_encrypted(&self) -> Option<&bool> {
        self.encrypted.as_ref()
    }

    /// Sets the value of FileId
    pub fn set_file_id(&mut self, value: u64) {
        self.file_id = Some(value);
    }

    /// Gets the value of FileId
    pub fn get_file_id(&self) -> Option<&u64> {
        self.file_id.as_ref()
    }

    /// Sets the value of Locks
    pub fn set_locks(&mut self, value: u32) {
        self.locks = Some(value);
    }

    /// Gets the value of Locks
    pub fn get_locks(&self) -> Option<&u32> {
        self.locks.as_ref()
    }

    /// Sets the value of Path
    pub fn set_path(&mut self, value: String) {
        self.path = Some(value);
    }

    /// Gets the value of Path
    pub fn get_path(&self) -> Option<&String> {
        self.path.as_ref()
    }

    /// Sets the value of Permissions
    pub fn set_permissions(&mut self, value: u32) {
        self.permissions = Some(value);
    }

    /// Gets the value of Permissions
    pub fn get_permissions(&self) -> Option<&u32> {
        self.permissions.as_ref()
    }

    /// Sets the value of ScopeName
    pub fn set_scope_name(&mut self, value: String) {
        self.scope_name = Some(value);
    }

    /// Gets the value of ScopeName
    pub fn get_scope_name(&self) -> Option<&String> {
        self.scope_name.as_ref()
    }

    /// Sets the value of SessionId
    pub fn set_session_id(&mut self, value: u64) {
        self.session_id = Some(value);
    }

    /// Gets the value of SessionId
    pub fn get_session_id(&self) -> Option<&u64> {
        self.session_id.as_ref()
    }

    /// Sets the value of ShareRelativePath
    pub fn set_share_relative_path(&mut self, value: String) {
        self.share_relative_path = Some(value);
    }

    /// Gets the value of ShareRelativePath
    pub fn get_share_relative_path(&self) -> Option<&String> {
        self.share_relative_path.as_ref()
    }

    /// Sets the value of Signed
    pub fn set_signed(&mut self, value: bool) {
        self.signed = Some(value);
    }

    /// Gets the value of Signed
    pub fn get_signed(&self) -> Option<&bool> {
        self.signed.as_ref()
    }

    /// Sets the value of SmbInstance
    pub fn set_smb_instance(&mut self, value: SmbOpenFile_SmbInstance) {
        self.smb_instance = Some(value);
    }

    /// Gets the value of SmbInstance
    pub fn get_smb_instance(&self) -> Option<&SmbOpenFile_SmbInstance> {
        self.smb_instance.as_ref()
    }

/// 

    /// * `return_value` -  (u32)
    pub fn force_close(&self) -> Result<(), WmiError> {
        self.invoke_method("ForceClose", &[])

    }

}

