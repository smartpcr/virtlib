// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_OfflineFilesChangeInfo struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_OfflineFilesChangeInfo {

/// 
    #[serde(rename = "CreatedOffline")]
    pub created_offline: Option<bool>,

/// 
    #[serde(rename = "DeletedOffline")]
    pub deleted_offline: Option<bool>,

/// 
    #[serde(rename = "Dirty")]
    pub dirty: Option<bool>,

/// 
    #[serde(rename = "ModifiedAttributes")]
    pub modified_attributes: Option<bool>,

/// 
    #[serde(rename = "ModifiedData")]
    pub modified_data: Option<bool>,

/// 
    #[serde(rename = "ModifiedTime")]
    pub modified_time: Option<bool>,
}

impl Win32_OfflineFilesChangeInfo {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            created_offline: None,
            deleted_offline: None,
            dirty: None,
            modified_attributes: None,
            modified_data: None,
            modified_time: None,
        }
    }


    /// Sets the value of CreatedOffline
    pub fn set_created_offline(&mut self, value: bool) {
        self.created_offline = Some(value);
    }

    /// Gets the value of CreatedOffline
    pub fn get_created_offline(&self) -> Option<&bool> {
        self.created_offline.as_ref()
    }

    /// Sets the value of DeletedOffline
    pub fn set_deleted_offline(&mut self, value: bool) {
        self.deleted_offline = Some(value);
    }

    /// Gets the value of DeletedOffline
    pub fn get_deleted_offline(&self) -> Option<&bool> {
        self.deleted_offline.as_ref()
    }

    /// Sets the value of Dirty
    pub fn set_dirty(&mut self, value: bool) {
        self.dirty = Some(value);
    }

    /// Gets the value of Dirty
    pub fn get_dirty(&self) -> Option<&bool> {
        self.dirty.as_ref()
    }

    /// Sets the value of ModifiedAttributes
    pub fn set_modified_attributes(&mut self, value: bool) {
        self.modified_attributes = Some(value);
    }

    /// Gets the value of ModifiedAttributes
    pub fn get_modified_attributes(&self) -> Option<&bool> {
        self.modified_attributes.as_ref()
    }

    /// Sets the value of ModifiedData
    pub fn set_modified_data(&mut self, value: bool) {
        self.modified_data = Some(value);
    }

    /// Gets the value of ModifiedData
    pub fn get_modified_data(&self) -> Option<&bool> {
        self.modified_data.as_ref()
    }

    /// Sets the value of ModifiedTime
    pub fn set_modified_time(&mut self, value: bool) {
        self.modified_time = Some(value);
    }

    /// Gets the value of ModifiedTime
    pub fn get_modified_time(&self) -> Option<&bool> {
        self.modified_time.as_ref()
    }
}

