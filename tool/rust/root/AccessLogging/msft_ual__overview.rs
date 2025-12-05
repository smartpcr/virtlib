// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.AccessLogging
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MsftUal_Overview struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MsftUal_Overview {

/// The date and time when a role component or product, becomes operational through UAL.
    #[serde(rename = "FirstSeen")]
    pub first_seen: Option<String>,

/// The UAL assigned, or registered, GUID representing the server role, or installed product. When a role or product reports its UAL data, this GUID accompanies the payload.
    #[serde(rename = "GUID")]
    pub guid: Option<String>,

/// The date and time when a role component, or product, was last operational through UAL.
    #[serde(rename = "LastSeen")]
    pub last_seen: Option<String>,

/// The name of the software parent product, or product line, that is providing User Access Logging data. This is also associated with a RoleName, and a RoleGuid.
    #[serde(rename = "ProductName")]
    pub product_name: Option<String>,

/// The name of the role, component, or sub-product that is providing User Access Logging data. This is also associated with a ProductName, and a RoleGuid.
    #[serde(rename = "RoleName")]
    pub role_name: Option<String>,
}

impl MsftUal_Overview {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            first_seen: None,
            guid: None,
            last_seen: None,
            product_name: None,
            role_name: None,
        }
    }


    /// Sets the value of FirstSeen
    pub fn set_first_seen(&mut self, value: String) {
        self.first_seen = Some(value);
    }

    /// Gets the value of FirstSeen
    pub fn get_first_seen(&self) -> Option<&String> {
        self.first_seen.as_ref()
    }

    /// Sets the value of GUID
    pub fn set_guid(&mut self, value: String) {
        self.guid = Some(value);
    }

    /// Gets the value of GUID
    pub fn get_guid(&self) -> Option<&String> {
        self.guid.as_ref()
    }

    /// Sets the value of LastSeen
    pub fn set_last_seen(&mut self, value: String) {
        self.last_seen = Some(value);
    }

    /// Gets the value of LastSeen
    pub fn get_last_seen(&self) -> Option<&String> {
        self.last_seen.as_ref()
    }

    /// Sets the value of ProductName
    pub fn set_product_name(&mut self, value: String) {
        self.product_name = Some(value);
    }

    /// Gets the value of ProductName
    pub fn get_product_name(&self) -> Option<&String> {
        self.product_name.as_ref()
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

