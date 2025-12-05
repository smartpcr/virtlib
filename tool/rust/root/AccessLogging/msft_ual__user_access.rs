// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.AccessLogging
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MsftUal_UserAccess struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MsftUal_UserAccess {

/// The incremental counter of client user accesses for a particular client user.
    #[serde(rename = "ActivityCount")]
    pub activity_count: Option<u32>,

/// The date and time when a client name is first seen by a role or product.
    #[serde(rename = "FirstSeen")]
    pub first_seen: Option<String>,

/// The date and time when a client name is last seen by a role or product.
    #[serde(rename = "LastSeen")]
    pub last_seen: Option<String>,

/// The name of the software parent product, or product line, that is providing User Access Logging data. This is also associated with a RoleName, and a RoleGuid.
    #[serde(rename = "ProductName")]
    pub product_name: Option<String>,

/// The UAL assigned, or registered, GUID representing the server role, or installed product. When a role or product reports its UAL data, this GUID accompanies the payload.
    #[serde(rename = "RoleGuid")]
    pub role_guid: Option<String>,

/// The name of the role, component, or sub-product that is providing User Access Logging data. This is also associated with a ProductName, and a RoleGuid.
    #[serde(rename = "RoleName")]
    pub role_name: Option<String>,

/// A unique GUID for a tenant client of an installed role or product which accompanies the UAL data payload, if applicable.
    #[serde(rename = "TenantIdentifier")]
    pub tenant_identifier: Option<String>,

/// The client user name that accompanies the UAL payload from installed roles and products, if applicable.
    #[serde(rename = "UserName")]
    pub user_name: Option<String>,
}

impl MsftUal_UserAccess {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            activity_count: None,
            first_seen: None,
            last_seen: None,
            product_name: None,
            role_guid: None,
            role_name: None,
            tenant_identifier: None,
            user_name: None,
        }
    }


    /// Sets the value of ActivityCount
    pub fn set_activity_count(&mut self, value: u32) {
        self.activity_count = Some(value);
    }

    /// Gets the value of ActivityCount
    pub fn get_activity_count(&self) -> Option<&u32> {
        self.activity_count.as_ref()
    }

    /// Sets the value of FirstSeen
    pub fn set_first_seen(&mut self, value: String) {
        self.first_seen = Some(value);
    }

    /// Gets the value of FirstSeen
    pub fn get_first_seen(&self) -> Option<&String> {
        self.first_seen.as_ref()
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

    /// Sets the value of TenantIdentifier
    pub fn set_tenant_identifier(&mut self, value: String) {
        self.tenant_identifier = Some(value);
    }

    /// Gets the value of TenantIdentifier
    pub fn get_tenant_identifier(&self) -> Option<&String> {
        self.tenant_identifier.as_ref()
    }

    /// Sets the value of UserName
    pub fn set_user_name(&mut self, value: String) {
        self.user_name = Some(value);
    }

    /// Gets the value of UserName
    pub fn get_user_name(&self) -> Option<&String> {
        self.user_name.as_ref()
    }
}

