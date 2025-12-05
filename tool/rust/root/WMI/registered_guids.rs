// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RegisteredGuids struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RegisteredGuids {

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "EnableFlags")]
    pub enable_flags: Option<u32>,

/// 
    #[serde(rename = "EnableLevel")]
    pub enable_level: Option<u32>,

/// 
    #[serde(rename = "GuidType")]
    pub guid_type: Option<u32>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "IsEnabled")]
    pub is_enabled: Option<bool>,

/// 
    #[serde(rename = "LoggerId")]
    pub logger_id: Option<u32>,
}

impl RegisteredGuids {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            active: None,
            enable_flags: None,
            enable_level: None,
            guid_type: None,
            instance_name: None,
            is_enabled: None,
            logger_id: None,
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

    /// Sets the value of EnableFlags
    pub fn set_enable_flags(&mut self, value: u32) {
        self.enable_flags = Some(value);
    }

    /// Gets the value of EnableFlags
    pub fn get_enable_flags(&self) -> Option<&u32> {
        self.enable_flags.as_ref()
    }

    /// Sets the value of EnableLevel
    pub fn set_enable_level(&mut self, value: u32) {
        self.enable_level = Some(value);
    }

    /// Gets the value of EnableLevel
    pub fn get_enable_level(&self) -> Option<&u32> {
        self.enable_level.as_ref()
    }

    /// Sets the value of GuidType
    pub fn set_guid_type(&mut self, value: u32) {
        self.guid_type = Some(value);
    }

    /// Gets the value of GuidType
    pub fn get_guid_type(&self) -> Option<&u32> {
        self.guid_type.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of IsEnabled
    pub fn set_is_enabled(&mut self, value: bool) {
        self.is_enabled = Some(value);
    }

    /// Gets the value of IsEnabled
    pub fn get_is_enabled(&self) -> Option<&bool> {
        self.is_enabled.as_ref()
    }

    /// Sets the value of LoggerId
    pub fn set_logger_id(&mut self, value: u32) {
        self.logger_id = Some(value);
    }

    /// Gets the value of LoggerId
    pub fn get_logger_id(&self) -> Option<&u32> {
        self.logger_id.as_ref()
    }
}

