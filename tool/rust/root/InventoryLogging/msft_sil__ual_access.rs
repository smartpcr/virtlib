// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.InventoryLogging
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MsftSil_UalAccess struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MsftSil_UalAccess {
    #[serde(flatten)]
    pub base: MsftSil_Data,

/// 
    #[serde(rename = "ProductName")]
    pub product_name: Option<String>,

/// 
    #[serde(rename = "RoleGuid")]
    pub role_guid: Option<String>,

/// 
    #[serde(rename = "RoleName")]
    pub role_name: Option<String>,

/// 
    #[serde(rename = "SampleDate")]
    pub sample_date: Option<String>,

/// 
    #[serde(rename = "UniqueDeviceAccessCount")]
    pub unique_device_access_count: Option<u32>,

/// 
    #[serde(rename = "UniqueUserAccessCount")]
    pub unique_user_access_count: Option<u32>,
}

impl MsftSil_UalAccess {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MsftSil_Data::new(),
            product_name: None,
            role_guid: None,
            role_name: None,
            sample_date: None,
            unique_device_access_count: None,
            unique_user_access_count: None,
        }
    }


    /// Sets the value of ProductName
    pub fn set_product_name(&mut self, value: String) {
        self.product_name = Some(value);
    }

    /// Gets the value of ProductName
    pub fn get_product_name(&self) -> Option<&String> {
        self.product_name.as_ref()
    }

    /// Sets the value of RoleGuid
    pub fn set_role_guid(&mut self, value: String) {
        self.role_guid = Some(value);
    }

    /// Gets the value of RoleGuid
    pub fn get_role_guid(&self) -> Option<&String> {
        self.role_guid.as_ref()
    }

    /// Sets the value of RoleName
    pub fn set_role_name(&mut self, value: String) {
        self.role_name = Some(value);
    }

    /// Gets the value of RoleName
    pub fn get_role_name(&self) -> Option<&String> {
        self.role_name.as_ref()
    }

    /// Sets the value of SampleDate
    pub fn set_sample_date(&mut self, value: String) {
        self.sample_date = Some(value);
    }

    /// Gets the value of SampleDate
    pub fn get_sample_date(&self) -> Option<&String> {
        self.sample_date.as_ref()
    }

    /// Sets the value of UniqueDeviceAccessCount
    pub fn set_unique_device_access_count(&mut self, value: u32) {
        self.unique_device_access_count = Some(value);
    }

    /// Gets the value of UniqueDeviceAccessCount
    pub fn get_unique_device_access_count(&self) -> Option<&u32> {
        self.unique_device_access_count.as_ref()
    }

    /// Sets the value of UniqueUserAccessCount
    pub fn set_unique_user_access_count(&mut self, value: u32) {
        self.unique_user_access_count = Some(value);
    }

    /// Gets the value of UniqueUserAccessCount
    pub fn get_unique_user_access_count(&self) -> Option<&u32> {
        self.unique_user_access_count.as_ref()
    }
}

