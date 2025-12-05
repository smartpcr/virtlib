// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ClusBfltCacheStoreInformation struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClusBfltCacheStoreInformation {

/// Device Guid.
    #[serde(rename = "DeviceGuid")]
    pub device_guid: Option<String>,

/// Id.
    #[serde(rename = "Id")]
    pub id: Option<String>,

/// Key.
    #[serde(rename = "key")]
    pub key: Option<String>,

/// Page Size.
    #[serde(rename = "PageSize")]
    pub page_size: Option<u32>,

/// PathId.
    #[serde(rename = "PathId")]
    pub path_id: Option<u32>,

/// Status.
    #[serde(rename = "Status")]
    pub status: Option<u32>,

/// Store Size.
    #[serde(rename = "StoreSize")]
    pub store_size: Option<u64>,
}

impl ClusBfltCacheStoreInformation {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            device_guid: None,
            id: None,
            key: None,
            page_size: None,
            path_id: None,
            status: None,
            store_size: None,
        }
    }


    /// Sets the value of DeviceGuid
    pub fn set_device_guid(&mut self, value: String) {
        self.device_guid = Some(value);
    }

    /// Gets the value of DeviceGuid
    pub fn get_device_guid(&self) -> Option<&String> {
        self.device_guid.as_ref()
    }

    /// Sets the value of Id
    pub fn set_id(&mut self, value: String) {
        self.id = Some(value);
    }

    /// Gets the value of Id
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Sets the value of key
    pub fn set_key(&mut self, value: String) {
        self.key = Some(value);
    }

    /// Gets the value of key
    pub fn get_key(&self) -> Option<&String> {
        self.key.as_ref()
    }

    /// Sets the value of PageSize
    pub fn set_page_size(&mut self, value: u32) {
        self.page_size = Some(value);
    }

    /// Gets the value of PageSize
    pub fn get_page_size(&self) -> Option<&u32> {
        self.page_size.as_ref()
    }

    /// Sets the value of PathId
    pub fn set_path_id(&mut self, value: u32) {
        self.path_id = Some(value);
    }

    /// Gets the value of PathId
    pub fn get_path_id(&self) -> Option<&u32> {
        self.path_id.as_ref()
    }

    /// Sets the value of Status
    pub fn set_status(&mut self, value: u32) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&u32> {
        self.status.as_ref()
    }

    /// Sets the value of StoreSize
    pub fn set_store_size(&mut self, value: u64) {
        self.store_size = Some(value);
    }

    /// Gets the value of StoreSize
    pub fn get_store_size(&self) -> Option<&u64> {
        self.store_size.as_ref()
    }
}

