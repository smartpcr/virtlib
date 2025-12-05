// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.Security.MicrosoftVolumeEncryption
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __ACE struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __ACE {
    #[serde(flatten)]
    pub base: __SecurityRelatedClass,

/// 
    #[serde(rename = "AccessMask")]
    pub access_mask: Option<u32>,

/// 
    #[serde(rename = "AceFlags")]
    pub ace_flags: Option<u32>,

/// 
    #[serde(rename = "AceType")]
    pub ace_type: Option<u32>,

/// 
    #[serde(rename = "GuidInheritedObjectType")]
    pub guid_inherited_object_type: Option<String>,

/// 
    #[serde(rename = "GuidObjectType")]
    pub guid_object_type: Option<String>,

/// 
    #[serde(rename = "TIME_CREATED")]
    pub time__created: Option<u64>,

/// 
    #[serde(rename = "Trustee")]
    pub trustee: Option<__Trustee>,
}

impl __ACE {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __SecurityRelatedClass::new(),
            access_mask: None,
            ace_flags: None,
            ace_type: None,
            guid_inherited_object_type: None,
            guid_object_type: None,
            time__created: None,
            trustee: None,
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

    /// Sets the value of AceFlags
    pub fn set_ace_flags(&mut self, value: u32) {
        self.ace_flags = Some(value);
    }

    /// Gets the value of AceFlags
    pub fn get_ace_flags(&self) -> Option<&u32> {
        self.ace_flags.as_ref()
    }

    /// Sets the value of AceType
    pub fn set_ace_type(&mut self, value: u32) {
        self.ace_type = Some(value);
    }

    /// Gets the value of AceType
    pub fn get_ace_type(&self) -> Option<&u32> {
        self.ace_type.as_ref()
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

    /// Sets the value of TIME_CREATED
    pub fn set_time__created(&mut self, value: u64) {
        self.time__created = Some(value);
    }

    /// Gets the value of TIME_CREATED
    pub fn get_time__created(&self) -> Option<&u64> {
        self.time__created.as_ref()
    }

    /// Sets the value of Trustee
    pub fn set_trustee(&mut self, value: __Trustee) {
        self.trustee = Some(value);
    }

    /// Gets the value of Trustee
    pub fn get_trustee(&self) -> Option<&__Trustee> {
        self.trustee.as_ref()
    }
}

