// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Config01_RemoteShell02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Config01_RemoteShell02 {

/// 
    #[serde(rename = "AllowRemoteShellAccess")]
    pub allow_remote_shell_access: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "MaxConcurrentUsers")]
    pub max_concurrent_users: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "SpecifyIdleTimeout")]
    pub specify_idle_timeout: Option<String>,

/// 
    #[serde(rename = "SpecifyMaxMemory")]
    pub specify_max_memory: Option<String>,

/// 
    #[serde(rename = "SpecifyMaxProcesses")]
    pub specify_max_processes: Option<String>,

/// 
    #[serde(rename = "SpecifyMaxRemoteShells")]
    pub specify_max_remote_shells: Option<String>,

/// 
    #[serde(rename = "SpecifyShellTimeout")]
    pub specify_shell_timeout: Option<String>,
}

impl MDM_Policy_Config01_RemoteShell02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allow_remote_shell_access: None,
            instance_id: None,
            max_concurrent_users: None,
            parent_id: None,
            specify_idle_timeout: None,
            specify_max_memory: None,
            specify_max_processes: None,
            specify_max_remote_shells: None,
            specify_shell_timeout: None,
        }
    }


    /// Sets the value of AllowRemoteShellAccess
    pub fn set_allow_remote_shell_access(&mut self, value: String) {
        self.allow_remote_shell_access = Some(value);
    }

    /// Gets the value of AllowRemoteShellAccess
    pub fn get_allow_remote_shell_access(&self) -> Option<&String> {
        self.allow_remote_shell_access.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of MaxConcurrentUsers
    pub fn set_max_concurrent_users(&mut self, value: String) {
        self.max_concurrent_users = Some(value);
    }

    /// Gets the value of MaxConcurrentUsers
    pub fn get_max_concurrent_users(&self) -> Option<&String> {
        self.max_concurrent_users.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of SpecifyIdleTimeout
    pub fn set_specify_idle_timeout(&mut self, value: String) {
        self.specify_idle_timeout = Some(value);
    }

    /// Gets the value of SpecifyIdleTimeout
    pub fn get_specify_idle_timeout(&self) -> Option<&String> {
        self.specify_idle_timeout.as_ref()
    }

    /// Sets the value of SpecifyMaxMemory
    pub fn set_specify_max_memory(&mut self, value: String) {
        self.specify_max_memory = Some(value);
    }

    /// Gets the value of SpecifyMaxMemory
    pub fn get_specify_max_memory(&self) -> Option<&String> {
        self.specify_max_memory.as_ref()
    }

    /// Sets the value of SpecifyMaxProcesses
    pub fn set_specify_max_processes(&mut self, value: String) {
        self.specify_max_processes = Some(value);
    }

    /// Gets the value of SpecifyMaxProcesses
    pub fn get_specify_max_processes(&self) -> Option<&String> {
        self.specify_max_processes.as_ref()
    }

    /// Sets the value of SpecifyMaxRemoteShells
    pub fn set_specify_max_remote_shells(&mut self, value: String) {
        self.specify_max_remote_shells = Some(value);
    }

    /// Gets the value of SpecifyMaxRemoteShells
    pub fn get_specify_max_remote_shells(&self) -> Option<&String> {
        self.specify_max_remote_shells.as_ref()
    }

    /// Sets the value of SpecifyShellTimeout
    pub fn set_specify_shell_timeout(&mut self, value: String) {
        self.specify_shell_timeout = Some(value);
    }

    /// Gets the value of SpecifyShellTimeout
    pub fn get_specify_shell_timeout(&self) -> Option<&String> {
        self.specify_shell_timeout.as_ref()
    }
}

