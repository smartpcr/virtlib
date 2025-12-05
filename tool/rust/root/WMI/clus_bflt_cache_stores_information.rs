// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ClusBfltCacheStoresInformation struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClusBfltCacheStoresInformation {

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// Cache Store Info.
    #[serde(rename = "CacheStoreInfo")]
    pub cache_store_info: Vec<ClusBfltCacheStoreInformation>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// Number of Cache Stores.
    #[serde(rename = "NumberOfCacheStores")]
    pub number_of_cache_stores: Option<u32>,
}

impl ClusBfltCacheStoresInformation {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            active: None,
            cache_store_info: Vec::new(),
            instance_name: None,
            number_of_cache_stores: None,
        }
    }


    /// Sets the value of Active
    pub fn set_active(&mut self, value: bool) {
        self.active = Some(value);
    }

    /// Gets the value of Active
    pub fn get_active(&self) -> Option<&bool> {
        self.active.as_ref()
    }

    /// Sets the value of CacheStoreInfo
    pub fn set_cache_store_info(&mut self, value: Vec<ClusBfltCacheStoreInformation>) {
        self.cache_store_info = value;
    }

    /// Gets the value of CacheStoreInfo
    pub fn get_cache_store_info(&self) -> &Vec<ClusBfltCacheStoreInformation> {
        &self.cache_store_info
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of NumberOfCacheStores
    pub fn set_number_of_cache_stores(&mut self, value: u32) {
        self.number_of_cache_stores = Some(value);
    }

    /// Gets the value of NumberOfCacheStores
    pub fn get_number_of_cache_stores(&self) -> Option<&u32> {
        self.number_of_cache_stores.as_ref()
    }
}

