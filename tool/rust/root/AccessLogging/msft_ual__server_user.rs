// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.AccessLogging
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MsftUal_ServerUser struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MsftUal_ServerUser {

/// The incremental counter of client user accesses for a particular client user.
    #[serde(rename = "ActivityCount")]
    pub activity_count: Option<u32>,

/// Unit identification for the local server.
    #[serde(rename = "ChassisSerialNumber")]
    pub chassis_serial_number: Option<String>,

/// The date and time when a client user name is first seen by a server.
    #[serde(rename = "FirstSeen")]
    pub first_seen: Option<String>,

/// The date and time when a client user name is last seen by a server.
    #[serde(rename = "LastSeen")]
    pub last_seen: Option<String>,

/// The client user name that accompanies the UAL payload from installed roles and products, if applicable.
    #[serde(rename = "UserName")]
    pub user_name: Option<String>,

/// SMBIOS reported universally unique identifier for this server unit.
    #[serde(rename = "UUID")]
    pub uuid: Option<String>,
}

impl MsftUal_ServerUser {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            activity_count: None,
            chassis_serial_number: None,
            first_seen: None,
            last_seen: None,
            user_name: None,
            uuid: None,
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

    /// Sets the value of ChassisSerialNumber
    pub fn set_chassis_serial_number(&mut self, value: String) {
        self.chassis_serial_number = Some(value);
    }

    /// Gets the value of ChassisSerialNumber
    pub fn get_chassis_serial_number(&self) -> Option<&String> {
        self.chassis_serial_number.as_ref()
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

    /// Sets the value of UserName
    pub fn set_user_name(&mut self, value: String) {
        self.user_name = Some(value);
    }

    /// Gets the value of UserName
    pub fn get_user_name(&self) -> Option<&String> {
        self.user_name.as_ref()
    }

    /// Sets the value of UUID
    pub fn set_uuid(&mut self, value: String) {
        self.uuid = Some(value);
    }

    /// Gets the value of UUID
    pub fn get_uuid(&self) -> Option<&String> {
        self.uuid.as_ref()
    }
}

