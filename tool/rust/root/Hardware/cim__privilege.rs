// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Hardware
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_Privilege struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_Privilege {
    #[serde(flatten)]
    pub base: CIM_ManagedElement,

/// 
    #[serde(rename = "Activities")]
    pub activities: Vec<u16>,

/// 
    #[serde(rename = "ActivityQualifiers")]
    pub activity_qualifiers: Vec<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "PrivilegeGranted")]
    pub privilege_granted: Option<bool>,

/// 
    #[serde(rename = "QualifierFormats")]
    pub qualifier_formats: Vec<u16>,
}

impl CIM_Privilege {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ManagedElement::new(),
            activities: Vec::new(),
            activity_qualifiers: Vec::new(),
            instance_id: None,
            privilege_granted: None,
            qualifier_formats: Vec::new(),
        }
    }


    /// Sets the value of Activities
    pub fn set_activities(&mut self, value: Vec<u16>) {
        self.activities = value;
    }

    /// Gets the value of Activities
    pub fn get_activities(&self) -> &Vec<u16> {
        &self.activities
    }

    /// Sets the value of ActivityQualifiers
    pub fn set_activity_qualifiers(&mut self, value: Vec<String>) {
        self.activity_qualifiers = value;
    }

    /// Gets the value of ActivityQualifiers
    pub fn get_activity_qualifiers(&self) -> &Vec<String> {
        &self.activity_qualifiers
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of PrivilegeGranted
    pub fn set_privilege_granted(&mut self, value: bool) {
        self.privilege_granted = Some(value);
    }

    /// Gets the value of PrivilegeGranted
    pub fn get_privilege_granted(&self) -> Option<&bool> {
        self.privilege_granted.as_ref()
    }

    /// Sets the value of QualifierFormats
    pub fn set_qualifier_formats(&mut self, value: Vec<u16>) {
        self.qualifier_formats = value;
    }

    /// Gets the value of QualifierFormats
    pub fn get_qualifier_formats(&self) -> &Vec<u16> {
        &self.qualifier_formats
    }
}

