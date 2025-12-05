// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_OfflineFilesItem struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_OfflineFilesItem {

/// 
    #[serde(rename = "ChangeInfo")]
    pub change_info: Option<Win32_OfflineFilesChangeInfo>,

/// 
    #[serde(rename = "ConnectionInfo")]
    pub connection_info: Option<Win32_OfflineFilesConnectionInfo>,

/// 
    #[serde(rename = "DirtyInfo")]
    pub dirty_info: Option<Win32_OfflineFilesDirtyInfo>,

/// 
    #[serde(rename = "Encrypted")]
    pub encrypted: Option<bool>,

/// 
    #[serde(rename = "FileSysInfo")]
    pub file_sys_info: Option<Win32_OfflineFilesFileSysInfo>,

/// 
    #[serde(rename = "ItemName")]
    pub item_name: Option<String>,

/// 
    #[serde(rename = "ItemPath")]
    pub item_path: Option<String>,

/// 
    #[serde(rename = "ItemType")]
    pub item_type: Option<u32>,

/// 
    #[serde(rename = "ParentItemPath")]
    pub parent_item_path: Option<String>,

/// 
    #[serde(rename = "PinInfo")]
    pub pin_info: Option<Win32_OfflineFilesPinInfo>,

/// 
    #[serde(rename = "Sparse")]
    pub sparse: Option<bool>,

/// 
    #[serde(rename = "SuspendInfo")]
    pub suspend_info: Option<Win32_OfflineFilesSuspendInfo>,
}

impl Win32_OfflineFilesItem {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            change_info: None,
            connection_info: None,
            dirty_info: None,
            encrypted: None,
            file_sys_info: None,
            item_name: None,
            item_path: None,
            item_type: None,
            parent_item_path: None,
            pin_info: None,
            sparse: None,
            suspend_info: None,
        }
    }


    /// Sets the value of ChangeInfo
    pub fn set_change_info(&mut self, value: Win32_OfflineFilesChangeInfo) {
        self.change_info = Some(value);
    }

    /// Gets the value of ChangeInfo
    pub fn get_change_info(&self) -> Option<&Win32_OfflineFilesChangeInfo> {
        self.change_info.as_ref()
    }

    /// Sets the value of ConnectionInfo
    pub fn set_connection_info(&mut self, value: Win32_OfflineFilesConnectionInfo) {
        self.connection_info = Some(value);
    }

    /// Gets the value of ConnectionInfo
    pub fn get_connection_info(&self) -> Option<&Win32_OfflineFilesConnectionInfo> {
        self.connection_info.as_ref()
    }

    /// Sets the value of DirtyInfo
    pub fn set_dirty_info(&mut self, value: Win32_OfflineFilesDirtyInfo) {
        self.dirty_info = Some(value);
    }

    /// Gets the value of DirtyInfo
    pub fn get_dirty_info(&self) -> Option<&Win32_OfflineFilesDirtyInfo> {
        self.dirty_info.as_ref()
    }

    /// Sets the value of Encrypted
    pub fn set_encrypted(&mut self, value: bool) {
        self.encrypted = Some(value);
    }

    /// Gets the value of Encrypted
    pub fn get_encrypted(&self) -> Option<&bool> {
        self.encrypted.as_ref()
    }

    /// Sets the value of FileSysInfo
    pub fn set_file_sys_info(&mut self, value: Win32_OfflineFilesFileSysInfo) {
        self.file_sys_info = Some(value);
    }

    /// Gets the value of FileSysInfo
    pub fn get_file_sys_info(&self) -> Option<&Win32_OfflineFilesFileSysInfo> {
        self.file_sys_info.as_ref()
    }

    /// Sets the value of ItemName
    pub fn set_item_name(&mut self, value: String) {
        self.item_name = Some(value);
    }

    /// Gets the value of ItemName
    pub fn get_item_name(&self) -> Option<&String> {
        self.item_name.as_ref()
    }

    /// Sets the value of ItemPath
    pub fn set_item_path(&mut self, value: String) {
        self.item_path = Some(value);
    }

    /// Gets the value of ItemPath
    pub fn get_item_path(&self) -> Option<&String> {
        self.item_path.as_ref()
    }

    /// Sets the value of ItemType
    pub fn set_item_type(&mut self, value: u32) {
        self.item_type = Some(value);
    }

    /// Gets the value of ItemType
    pub fn get_item_type(&self) -> Option<&u32> {
        self.item_type.as_ref()
    }

    /// Sets the value of ParentItemPath
    pub fn set_parent_item_path(&mut self, value: String) {
        self.parent_item_path = Some(value);
    }

    /// Gets the value of ParentItemPath
    pub fn get_parent_item_path(&self) -> Option<&String> {
        self.parent_item_path.as_ref()
    }

    /// Sets the value of PinInfo
    pub fn set_pin_info(&mut self, value: Win32_OfflineFilesPinInfo) {
        self.pin_info = Some(value);
    }

    /// Gets the value of PinInfo
    pub fn get_pin_info(&self) -> Option<&Win32_OfflineFilesPinInfo> {
        self.pin_info.as_ref()
    }

    /// Sets the value of Sparse
    pub fn set_sparse(&mut self, value: bool) {
        self.sparse = Some(value);
    }

    /// Gets the value of Sparse
    pub fn get_sparse(&self) -> Option<&bool> {
        self.sparse.as_ref()
    }

    /// Sets the value of SuspendInfo
    pub fn set_suspend_info(&mut self, value: Win32_OfflineFilesSuspendInfo) {
        self.suspend_info = Some(value);
    }

    /// Gets the value of SuspendInfo
    pub fn get_suspend_info(&self) -> Option<&Win32_OfflineFilesSuspendInfo> {
        self.suspend_info.as_ref()
    }
}

