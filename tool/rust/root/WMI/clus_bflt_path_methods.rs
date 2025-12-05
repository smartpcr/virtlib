// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ClusBfltPathMethods struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClusBfltPathMethods {

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,
}

impl ClusBfltPathMethods {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            active: None,
            instance_name: None,
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

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

/// Reinitializes disk partition with GPT, trims SSD disk

    /// * `flags` - Flags (u32)
    /// * `path_id` - Path Id (u32)
    pub fn re_initialize_disk(&self, path_id: u32, flags: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PathId".to_string(), value: path_id.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        self.invoke_method("ReInitializeDisk", &args)

    }


/// Creates Cache Store on SSD

    /// * `page_size` - Page Size (u32)
    /// * `path_id` - Path Id (u32)
    /// * `reserve_percentage` - Reserve Percentage (u32)
    /// * `unused_size` - Unused Size (u64)
    pub fn create_ssd_cache_store(&self, path_id: u32, reserve_percentage: u32, unused_size: u64, page_size: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PathId".to_string(), value: path_id.into() });
        args.push(MethodParameter { name: "ReservePercentage".to_string(), value: reserve_percentage.into() });
        args.push(MethodParameter { name: "UnusedSize".to_string(), value: unused_size.into() });
        args.push(MethodParameter { name: "PageSize".to_string(), value: page_size.into() });
        self.invoke_method("CreateSsdCacheStore", &args)

    }


/// Prepares HDD Disk

    /// * `options` - Options (u32)
    /// * `path_id` - Path Id (u32)
    pub fn prepare_hdd_for_cache(&self, path_id: u32, options: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PathId".to_string(), value: path_id.into() });
        args.push(MethodParameter { name: "Options".to_string(), value: options.into() });
        self.invoke_method("PrepareHddForCache", &args)

    }


/// Binds Hdd to Cache Store

    /// * `attributes` - Attributes (u32)
    /// * `cache_store_id` - CacheStoreId (String)
    /// * `path_id` - Path Id (u32)
    pub fn bind_hdd_to_cache_store(&self, path_id: u32, attributes: u32, cache_store_id: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PathId".to_string(), value: path_id.into() });
        args.push(MethodParameter { name: "Attributes".to_string(), value: attributes.into() });
        args.push(MethodParameter { name: "CacheStoreId".to_string(), value: cache_store_id.into() });
        self.invoke_method("BindHddToCacheStore", &args)

    }


/// Unbinds Hdd from Cache Store

    /// * `flags` - Flags (u32)
    /// * `path_id` - Path Id (u32)
    pub fn un_bind_hdd(&self, path_id: u32, flags: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PathId".to_string(), value: path_id.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        self.invoke_method("UnBindHdd", &args)

    }


/// Queries Hdd Binding

    /// * `path_id` - Path Id (u32)

    /// * `binding_info` - BindingInfo (ClusBfltHddBindingInformation)
    pub fn query_hdd_binding(&self, path_id: u32, binding_info: &mut ClusBfltHddBindingInformation) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PathId".to_string(), value: path_id.into() });

        let result = self.invoke_method("QueryHddBinding", &args)?;
        let binding_info = result.get_value("BindingInfo")?;
        Ok(result.return_value)

    }


/// Queries Cache Stores

    /// * `path_id` - Path Id (u32)

    /// * `cache_stores` - CacheStores (ClusBfltCacheStoresInformation)
    pub fn query_cache_stores(&self, path_id: u32, cache_stores: &mut ClusBfltCacheStoresInformation) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PathId".to_string(), value: path_id.into() });

        let result = self.invoke_method("QueryCacheStores", &args)?;
        let cache_stores = result.get_value("CacheStores")?;
        Ok(result.return_value)

    }


/// Queries Cache Store Binding Records

    /// * `cache_store_id` - Cache Store Id (String)
    /// * `path_id` - Path Id (u32)

    /// * `binding_records` - BindingRecords (ClusBfltSsdBindingRecords)
    pub fn query_ssd_binding_records(&self, path_id: u32, cache_store_id: &String, binding_records: &mut ClusBfltSsdBindingRecords) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PathId".to_string(), value: path_id.into() });
        args.push(MethodParameter { name: "CacheStoreId".to_string(), value: cache_store_id.into() });

        let result = self.invoke_method("QuerySsdBindingRecords", &args)?;
        let binding_records = result.get_value("BindingRecords")?;
        Ok(result.return_value)

    }


/// Disabled Cache Store Binding

    /// * `binding_id` - Binding Id (String)
    /// * `cache_store_id` - Cache Store Id (String)
    /// * `path_id` - Path Id (u32)
    pub fn disable_ssd_binding(&self, path_id: u32, cache_store_id: &String, binding_id: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PathId".to_string(), value: path_id.into() });
        args.push(MethodParameter { name: "CacheStoreId".to_string(), value: cache_store_id.into() });
        args.push(MethodParameter { name: "BindingId".to_string(), value: binding_id.into() });
        self.invoke_method("DisableSsdBinding", &args)

    }


/// Gets Path Id of a Given Type by Device Guid

    /// * `attributes` - Attributes (u32)
    /// * `attributes_mask` - AttributesMask (u32)
    /// * `device_guid` - Device Id (String)
    /// * `path_type` - PathType (u32)
    /// * `timeout` - Timeout (u32)

    /// * `path_id` - PathId (u32)
    pub fn get_path_id_by_device_guid(&self, device_guid: &String, path_type: u32, attributes: u32, attributes_mask: u32, timeout: u32, path_id: &mut u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DeviceGuid".to_string(), value: device_guid.into() });
        args.push(MethodParameter { name: "PathType".to_string(), value: path_type.into() });
        args.push(MethodParameter { name: "Attributes".to_string(), value: attributes.into() });
        args.push(MethodParameter { name: "AttributesMask".to_string(), value: attributes_mask.into() });
        args.push(MethodParameter { name: "Timeout".to_string(), value: timeout.into() });

        let result = self.invoke_method("GetPathIdByDeviceGuid", &args)?;
        let path_id = result.get_value("PathId")?;
        Ok(result.return_value)

    }


/// Trims the disk

    /// * `path_id` - Path Id (u32)
    pub fn trim_disk(&self, path_id: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PathId".to_string(), value: path_id.into() });
        self.invoke_method("TrimDisk", &args)

    }


/// Disabled Cache Store Binding

    /// * `attributes` - Attributes (u32)
    /// * `attributes_mask` - AttributesMask (u32)
    /// * `binding_id` - Binding Id (String)
    /// * `cache_store_id` - Cache Store Id (String)
    /// * `path_id` - Path Id (u32)
    pub fn set_ssd_binding_attributes(&self, path_id: u32, attributes: u32, attributes_mask: u32, cache_store_id: &String, binding_id: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PathId".to_string(), value: path_id.into() });
        args.push(MethodParameter { name: "Attributes".to_string(), value: attributes.into() });
        args.push(MethodParameter { name: "AttributesMask".to_string(), value: attributes_mask.into() });
        args.push(MethodParameter { name: "CacheStoreId".to_string(), value: cache_store_id.into() });
        args.push(MethodParameter { name: "BindingId".to_string(), value: binding_id.into() });
        self.invoke_method("SetSsdBindingAttributes", &args)

    }

}

