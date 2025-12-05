// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.AccessLogging
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MsftUal_Dns struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MsftUal_Dns {

/// The host name of the client. This is associated with IPAddress.
    #[serde(rename = "HostName")]
    pub host_name: Option<String>,

/// The IP address of a DNS client record. This is associated with hostname.
    #[serde(rename = "IPAddress")]
    pub ipaddress: Option<String>,

/// The date and time when a DNS client record was last seen in DNS.
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
}

impl MsftUal_Dns {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            host_name: None,
            ipaddress: None,
            last_seen: None,
            product_name: None,
            role_guid: None,
            role_name: None,
        }
    }


    /// Sets the value of HostName
    pub fn set_host_name(&mut self, value: String) {
        self.host_name = Some(value);
    }

    /// Gets the value of HostName
    pub fn get_host_name(&self) -> Option<&String> {
        self.host_name.as_ref()
    }

    /// Sets the value of IPAddress
    pub fn set_ipaddress(&mut self, value: String) {
        self.ipaddress = Some(value);
    }

    /// Gets the value of IPAddress
    pub fn get_ipaddress(&self) -> Option<&String> {
        self.ipaddress.as_ref()
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
}

