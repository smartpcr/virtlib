// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ClusBfltHddBindingInformation struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClusBfltHddBindingInformation {

/// Binding Id.
    #[serde(rename = "BindingId")]
    pub binding_id: Option<String>,

/// Device Id.
    #[serde(rename = "CacheStoreDeviceId")]
    pub cache_store_device_id: Option<String>,

/// Id.
    #[serde(rename = "CacheStoreId")]
    pub cache_store_id: Option<String>,

/// Key.
    #[serde(rename = "CacheStoreKey")]
    pub cache_store_key: Option<String>,

/// Page Size.
    #[serde(rename = "PageSize")]
    pub page_size: Option<u32>,
}

impl ClusBfltHddBindingInformation {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            binding_id: None,
            cache_store_device_id: None,
            cache_store_id: None,
            cache_store_key: None,
            page_size: None,
        }
    }


    /// Sets the value of BindingId
    pub fn set_binding_id(&mut self, value: String) {
        self.binding_id = Some(value);
    }

    /// Gets the value of BindingId
    pub fn get_binding_id(&self) -> Option<&String> {
        self.binding_id.as_ref()
    }

    /// Sets the value of CacheStoreDeviceId
    pub fn set_cache_store_device_id(&mut self, value: String) {
        self.cache_store_device_id = Some(value);
    }

    /// Gets the value of CacheStoreDeviceId
    pub fn get_cache_store_device_id(&self) -> Option<&String> {
        self.cache_store_device_id.as_ref()
    }

    /// Sets the value of CacheStoreId
    pub fn set_cache_store_id(&mut self, value: String) {
        self.cache_store_id = Some(value);
    }

    /// Gets the value of CacheStoreId
    pub fn get_cache_store_id(&self) -> Option<&String> {
        self.cache_store_id.as_ref()
    }

    /// Sets the value of CacheStoreKey
    pub fn set_cache_store_key(&mut self, value: String) {
        self.cache_store_key = Some(value);
    }

    /// Gets the value of CacheStoreKey
    pub fn get_cache_store_key(&self) -> Option<&String> {
        self.cache_store_key.as_ref()
    }

    /// Sets the value of PageSize
    pub fn set_page_size(&mut self, value: u32) {
        self.page_size = Some(value);
    }

    /// Gets the value of PageSize
    pub fn get_page_size(&self) -> Option<&u32> {
        self.page_size.as_ref()
    }
}

