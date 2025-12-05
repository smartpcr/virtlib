// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_RemoteFind struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_RemoteFind {

/// 
    #[serde(rename = "DesiredAccuracy")]
    pub desired_accuracy: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "MaximumAge")]
    pub maximum_age: Option<i32>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "Timeout")]
    pub timeout: Option<i32>,
}

impl MDM_RemoteFind {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            desired_accuracy: None,
            instance_id: None,
            maximum_age: None,
            parent_id: None,
            timeout: None,
        }
    }


    /// Sets the value of DesiredAccuracy
    pub fn set_desired_accuracy(&mut self, value: i32) {
        self.desired_accuracy = Some(value);
    }

    /// Gets the value of DesiredAccuracy
    pub fn get_desired_accuracy(&self) -> Option<&i32> {
        self.desired_accuracy.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of MaximumAge
    pub fn set_maximum_age(&mut self, value: i32) {
        self.maximum_age = Some(value);
    }

    /// Gets the value of MaximumAge
    pub fn get_maximum_age(&self) -> Option<&i32> {
        self.maximum_age.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of Timeout
    pub fn set_timeout(&mut self, value: i32) {
        self.timeout = Some(value);
    }

    /// Gets the value of Timeout
    pub fn get_timeout(&self) -> Option<&i32> {
        self.timeout.as_ref()
    }
}

