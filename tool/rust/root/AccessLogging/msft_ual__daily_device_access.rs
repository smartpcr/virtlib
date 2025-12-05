// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.AccessLogging
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MsftUal_DailyDeviceAccess struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MsftUal_DailyDeviceAccess {

/// The number of accesses of a role, or installed product, on the local server from a unique client device.
    #[serde(rename = "AccessCount")]
    pub access_count: Option<u32>,

/// The date that a device accessed a role, or installed product, on the local server.
    #[serde(rename = "AccessDate")]
    pub access_date: Option<String>,

/// The IP address of a client device that accompanies the UAL payload from installed roles and products.
    #[serde(rename = "IPAddress")]
    pub ipaddress: Option<String>,

/// The name of the software parent product, or product line, that is providing User Access Logging data. This is also associated with a RoleName, and a RoleGuid.
    #[serde(rename = "ProductName")]
    pub product_name: Option<String>,

/// The UAL assigned, or registered, GUID representing the server role, or installed product. When a role or product reports its UAL data, this GUID accompanies the payload.
    #[serde(rename = "RoleGuid")]
    pub role_guid: Option<String>,

/// The name of the role, component, or sub-product that is providing User Access Logging data. This is also associated with a ProductName, and a RoleGuid.
    #[serde(rename = "RoleName")]
    pub role_name: Option<String>,
}

impl MsftUal_DailyDeviceAccess {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            access_count: None,
            access_date: None,
            ipaddress: None,
            product_name: None,
            role_guid: None,
            role_name: None,
        }
    }


    /// Sets the value of AccessCount
    pub fn set_access_count(&mut self, value: u32) {
        self.access_count = Some(value);
    }

    /// Gets the value of AccessCount
    pub fn get_access_count(&self) -> Option<&u32> {
        self.access_count.as_ref()
    }

    /// Sets the value of AccessDate
    pub fn set_access_date(&mut self, value: String) {
        self.access_date = Some(value);
    }

    /// Gets the value of AccessDate
    pub fn get_access_date(&self) -> Option<&String> {
        self.access_date.as_ref()
    }

    /// Sets the value of IPAddress
    pub fn set_ipaddress(&mut self, value: String) {
        self.ipaddress = Some(value);
    }

    /// Gets the value of IPAddress
    pub fn get_ipaddress(&self) -> Option<&String> {
        self.ipaddress.as_ref()
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
}

