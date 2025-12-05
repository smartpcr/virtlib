// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_SecuritySettingAccess struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_SecuritySettingAccess {

/// 
    #[serde(rename = "AccessMask")]
    pub access_mask: Option<u32>,

/// 
    #[serde(rename = "GuidInheritedObjectType")]
    pub guid_inherited_object_type: Option<String>,

/// 
    #[serde(rename = "GuidObjectType")]
    pub guid_object_type: Option<String>,

/// 
    #[serde(rename = "Inheritance")]
    pub inheritance: Option<u32>,

/// 
    #[serde(rename = "SecuritySetting")]
    pub security_setting: Option<Win32_SecuritySetting>,

/// 
    #[serde(rename = "Trustee")]
    pub trustee: Option<Win32_SID>,

/// 
    #[serde(rename = "Type")]
    pub type: Option<u32>,
}

impl Win32_SecuritySettingAccess {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            access_mask: None,
            guid_inherited_object_type: None,
            guid_object_type: None,
            inheritance: None,
            security_setting: None,
            trustee: None,
            type: None,
        }
    }


    /// Sets the value of AccessMask
    pub fn set_access_mask(&mut self, value: u32) {
        self.access_mask = Some(value);
    }

    /// Gets the value of AccessMask
    pub fn get_access_mask(&self) -> Option<&u32> {
        self.access_mask.as_ref()
    }

    /// Sets the value of GuidInheritedObjectType
    pub fn set_guid_inherited_object_type(&mut self, value: String) {
        self.guid_inherited_object_type = Some(value);
    }

    /// Gets the value of GuidInheritedObjectType
    pub fn get_guid_inherited_object_type(&self) -> Option<&String> {
        self.guid_inherited_object_type.as_ref()
    }

    /// Sets the value of GuidObjectType
    pub fn set_guid_object_type(&mut self, value: String) {
        self.guid_object_type = Some(value);
    }

    /// Gets the value of GuidObjectType
    pub fn get_guid_object_type(&self) -> Option<&String> {
        self.guid_object_type.as_ref()
    }

    /// Sets the value of Inheritance
    pub fn set_inheritance(&mut self, value: u32) {
        self.inheritance = Some(value);
    }

    /// Gets the value of Inheritance
    pub fn get_inheritance(&self) -> Option<&u32> {
        self.inheritance.as_ref()
    }

    /// Sets the value of SecuritySetting
    pub fn set_security_setting(&mut self, value: Win32_SecuritySetting) {
        self.security_setting = Some(value);
    }

    /// Gets the value of SecuritySetting
    pub fn get_security_setting(&self) -> Option<&Win32_SecuritySetting> {
        self.security_setting.as_ref()
    }

    /// Sets the value of Trustee
    pub fn set_trustee(&mut self, value: Win32_SID) {
        self.trustee = Some(value);
    }

    /// Gets the value of Trustee
    pub fn get_trustee(&self) -> Option<&Win32_SID> {
        self.trustee.as_ref()
    }

    /// Sets the value of Type
    pub fn set_type(&mut self, value: u32) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&u32> {
        self.type.as_ref()
    }
}

