// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_OfflineFilesFileSysInfo struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_OfflineFilesFileSysInfo {

/// 
    #[serde(rename = "LocalAttributes")]
    pub local_attributes: Option<u32>,

/// 
    #[serde(rename = "LocalChangeTime")]
    pub local_change_time: Option<String>,

/// 
    #[serde(rename = "LocalCreationTime")]
    pub local_creation_time: Option<String>,

/// 
    #[serde(rename = "LocalLastAccessTime")]
    pub local_last_access_time: Option<String>,

/// 
    #[serde(rename = "LocalLastWriteTime")]
    pub local_last_write_time: Option<String>,

/// 
    #[serde(rename = "LocalSize")]
    pub local_size: Option<i64>,

/// 
    #[serde(rename = "OriginalAttributes")]
    pub original_attributes: Option<u32>,

/// 
    #[serde(rename = "OriginalChangeTime")]
    pub original_change_time: Option<String>,

/// 
    #[serde(rename = "OriginalCreationTime")]
    pub original_creation_time: Option<String>,

/// 
    #[serde(rename = "OriginalLastAccessTime")]
    pub original_last_access_time: Option<String>,

/// 
    #[serde(rename = "OriginalLastWriteTime")]
    pub original_last_write_time: Option<String>,

/// 
    #[serde(rename = "OriginalSize")]
    pub original_size: Option<i64>,

/// 
    #[serde(rename = "RemoteAttributes")]
    pub remote_attributes: Option<u32>,

/// 
    #[serde(rename = "RemoteChangeTime")]
    pub remote_change_time: Option<String>,

/// 
    #[serde(rename = "RemoteCreationTime")]
    pub remote_creation_time: Option<String>,

/// 
    #[serde(rename = "RemoteLastAccessTime")]
    pub remote_last_access_time: Option<String>,

/// 
    #[serde(rename = "RemoteLastWriteTime")]
    pub remote_last_write_time: Option<String>,

/// 
    #[serde(rename = "RemoteSize")]
    pub remote_size: Option<i64>,
}

impl Win32_OfflineFilesFileSysInfo {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            local_attributes: None,
            local_change_time: None,
            local_creation_time: None,
            local_last_access_time: None,
            local_last_write_time: None,
            local_size: None,
            original_attributes: None,
            original_change_time: None,
            original_creation_time: None,
            original_last_access_time: None,
            original_last_write_time: None,
            original_size: None,
            remote_attributes: None,
            remote_change_time: None,
            remote_creation_time: None,
            remote_last_access_time: None,
            remote_last_write_time: None,
            remote_size: None,
        }
    }


    /// Sets the value of LocalAttributes
    pub fn set_local_attributes(&mut self, value: u32) {
        self.local_attributes = Some(value);
    }

    /// Gets the value of LocalAttributes
    pub fn get_local_attributes(&self) -> Option<&u32> {
        self.local_attributes.as_ref()
    }

    /// Sets the value of LocalChangeTime
    pub fn set_local_change_time(&mut self, value: String) {
        self.local_change_time = Some(value);
    }

    /// Gets the value of LocalChangeTime
    pub fn get_local_change_time(&self) -> Option<&String> {
        self.local_change_time.as_ref()
    }

    /// Sets the value of LocalCreationTime
    pub fn set_local_creation_time(&mut self, value: String) {
        self.local_creation_time = Some(value);
    }

    /// Gets the value of LocalCreationTime
    pub fn get_local_creation_time(&self) -> Option<&String> {
        self.local_creation_time.as_ref()
    }

    /// Sets the value of LocalLastAccessTime
    pub fn set_local_last_access_time(&mut self, value: String) {
        self.local_last_access_time = Some(value);
    }

    /// Gets the value of LocalLastAccessTime
    pub fn get_local_last_access_time(&self) -> Option<&String> {
        self.local_last_access_time.as_ref()
    }

    /// Sets the value of LocalLastWriteTime
    pub fn set_local_last_write_time(&mut self, value: String) {
        self.local_last_write_time = Some(value);
    }

    /// Gets the value of LocalLastWriteTime
    pub fn get_local_last_write_time(&self) -> Option<&String> {
        self.local_last_write_time.as_ref()
    }

    /// Sets the value of LocalSize
    pub fn set_local_size(&mut self, value: i64) {
        self.local_size = Some(value);
    }

    /// Gets the value of LocalSize
    pub fn get_local_size(&self) -> Option<&i64> {
        self.local_size.as_ref()
    }

    /// Sets the value of OriginalAttributes
    pub fn set_original_attributes(&mut self, value: u32) {
        self.original_attributes = Some(value);
    }

    /// Gets the value of OriginalAttributes
    pub fn get_original_attributes(&self) -> Option<&u32> {
        self.original_attributes.as_ref()
    }

    /// Sets the value of OriginalChangeTime
    pub fn set_original_change_time(&mut self, value: String) {
        self.original_change_time = Some(value);
    }

    /// Gets the value of OriginalChangeTime
    pub fn get_original_change_time(&self) -> Option<&String> {
        self.original_change_time.as_ref()
    }

    /// Sets the value of OriginalCreationTime
    pub fn set_original_creation_time(&mut self, value: String) {
        self.original_creation_time = Some(value);
    }

    /// Gets the value of OriginalCreationTime
    pub fn get_original_creation_time(&self) -> Option<&String> {
        self.original_creation_time.as_ref()
    }

    /// Sets the value of OriginalLastAccessTime
    pub fn set_original_last_access_time(&mut self, value: String) {
        self.original_last_access_time = Some(value);
    }

    /// Gets the value of OriginalLastAccessTime
    pub fn get_original_last_access_time(&self) -> Option<&String> {
        self.original_last_access_time.as_ref()
    }

    /// Sets the value of OriginalLastWriteTime
    pub fn set_original_last_write_time(&mut self, value: String) {
        self.original_last_write_time = Some(value);
    }

    /// Gets the value of OriginalLastWriteTime
    pub fn get_original_last_write_time(&self) -> Option<&String> {
        self.original_last_write_time.as_ref()
    }

    /// Sets the value of OriginalSize
    pub fn set_original_size(&mut self, value: i64) {
        self.original_size = Some(value);
    }

    /// Gets the value of OriginalSize
    pub fn get_original_size(&self) -> Option<&i64> {
        self.original_size.as_ref()
    }

    /// Sets the value of RemoteAttributes
    pub fn set_remote_attributes(&mut self, value: u32) {
        self.remote_attributes = Some(value);
    }

    /// Gets the value of RemoteAttributes
    pub fn get_remote_attributes(&self) -> Option<&u32> {
        self.remote_attributes.as_ref()
    }

    /// Sets the value of RemoteChangeTime
    pub fn set_remote_change_time(&mut self, value: String) {
        self.remote_change_time = Some(value);
    }

    /// Gets the value of RemoteChangeTime
    pub fn get_remote_change_time(&self) -> Option<&String> {
        self.remote_change_time.as_ref()
    }

    /// Sets the value of RemoteCreationTime
    pub fn set_remote_creation_time(&mut self, value: String) {
        self.remote_creation_time = Some(value);
    }

    /// Gets the value of RemoteCreationTime
    pub fn get_remote_creation_time(&self) -> Option<&String> {
        self.remote_creation_time.as_ref()
    }

    /// Sets the value of RemoteLastAccessTime
    pub fn set_remote_last_access_time(&mut self, value: String) {
        self.remote_last_access_time = Some(value);
    }

    /// Gets the value of RemoteLastAccessTime
    pub fn get_remote_last_access_time(&self) -> Option<&String> {
        self.remote_last_access_time.as_ref()
    }

    /// Sets the value of RemoteLastWriteTime
    pub fn set_remote_last_write_time(&mut self, value: String) {
        self.remote_last_write_time = Some(value);
    }

    /// Gets the value of RemoteLastWriteTime
    pub fn get_remote_last_write_time(&self) -> Option<&String> {
        self.remote_last_write_time.as_ref()
    }

    /// Sets the value of RemoteSize
    pub fn set_remote_size(&mut self, value: i64) {
        self.remote_size = Some(value);
    }

    /// Gets the value of RemoteSize
    pub fn get_remote_size(&self) -> Option<&i64> {
        self.remote_size.as_ref()
    }
}

