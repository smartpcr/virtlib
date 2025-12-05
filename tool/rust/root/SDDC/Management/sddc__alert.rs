// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.SDDC.Management
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SDDC_Alert struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SDDC_Alert {

/// 
    #[serde(rename = "Actions")]
    pub actions: Option<String>,

/// 
    #[serde(rename = "Description")]
    pub description: Option<String>,

/// 
    #[serde(rename = "FaultingObjectDescription")]
    pub faulting_object_description: Option<String>,

/// 
    #[serde(rename = "FaultingObjectLocation")]
    pub faulting_object_location: Option<String>,

/// 
    #[serde(rename = "Id")]
    pub id: Option<String>,

/// 
    #[serde(rename = "Severity")]
    pub severity: Option<u16>,

/// 
    #[serde(rename = "Time")]
    pub time: Option<String>,

/// 
    #[serde(rename = "Title")]
    pub title: Option<String>,

/// 
    #[serde(rename = "Type")]
    pub type: Option<String>,
}

impl SDDC_Alert {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            actions: None,
            description: None,
            faulting_object_description: None,
            faulting_object_location: None,
            id: None,
            severity: None,
            time: None,
            title: None,
            type: None,
        }
    }


    /// Sets the value of Actions
    pub fn set_actions(&mut self, value: String) {
        self.actions = Some(value);
    }

    /// Gets the value of Actions
    pub fn get_actions(&self) -> Option<&String> {
        self.actions.as_ref()
    }

    /// Sets the value of Description
    pub fn set_description(&mut self, value: String) {
        self.description = Some(value);
    }

    /// Gets the value of Description
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// Sets the value of FaultingObjectDescription
    pub fn set_faulting_object_description(&mut self, value: String) {
        self.faulting_object_description = Some(value);
    }

    /// Gets the value of FaultingObjectDescription
    pub fn get_faulting_object_description(&self) -> Option<&String> {
        self.faulting_object_description.as_ref()
    }

    /// Sets the value of FaultingObjectLocation
    pub fn set_faulting_object_location(&mut self, value: String) {
        self.faulting_object_location = Some(value);
    }

    /// Gets the value of FaultingObjectLocation
    pub fn get_faulting_object_location(&self) -> Option<&String> {
        self.faulting_object_location.as_ref()
    }

    /// Sets the value of Id
    pub fn set_id(&mut self, value: String) {
        self.id = Some(value);
    }

    /// Gets the value of Id
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Sets the value of Severity
    pub fn set_severity(&mut self, value: u16) {
        self.severity = Some(value);
    }

    /// Gets the value of Severity
    pub fn get_severity(&self) -> Option<&u16> {
        self.severity.as_ref()
    }

    /// Sets the value of Time
    pub fn set_time(&mut self, value: String) {
        self.time = Some(value);
    }

    /// Gets the value of Time
    pub fn get_time(&self) -> Option<&String> {
        self.time.as_ref()
    }

    /// Sets the value of Title
    pub fn set_title(&mut self, value: String) {
        self.title = Some(value);
    }

    /// Gets the value of Title
    pub fn get_title(&self) -> Option<&String> {
        self.title.as_ref()
    }

    /// Sets the value of Type
    pub fn set_type(&mut self, value: String) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&String> {
        self.type.as_ref()
    }
}

