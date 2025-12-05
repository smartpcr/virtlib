// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Hardware
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ManagedSystemElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ManagedSystemElement {
    #[serde(flatten)]
    pub base: CIM_ManagedElement,

/// 
    #[serde(rename = "HealthState")]
    pub health_state: Option<u16>,

/// 
    #[serde(rename = "InstallDate")]
    pub install_date: Option<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "OperationalStatus")]
    pub operational_status: Vec<u16>,

/// 
    #[serde(rename = "Status")]
    pub status: Option<String>,

/// 
    #[serde(rename = "StatusDescriptions")]
    pub status_descriptions: Vec<String>,
}

impl CIM_ManagedSystemElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ManagedElement::new(),
            health_state: None,
            install_date: None,
            name: None,
            operational_status: Vec::new(),
            status: None,
            status_descriptions: Vec::new(),
        }
    }


    /// Sets the value of HealthState
    pub fn set_health_state(&mut self, value: u16) {
        self.health_state = Some(value);
    }

    /// Gets the value of HealthState
    pub fn get_health_state(&self) -> Option<&u16> {
        self.health_state.as_ref()
    }

    /// Sets the value of InstallDate
    pub fn set_install_date(&mut self, value: String) {
        self.install_date = Some(value);
    }

    /// Gets the value of InstallDate
    pub fn get_install_date(&self) -> Option<&String> {
        self.install_date.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of OperationalStatus
    pub fn set_operational_status(&mut self, value: Vec<u16>) {
        self.operational_status = value;
    }

    /// Gets the value of OperationalStatus
    pub fn get_operational_status(&self) -> &Vec<u16> {
        &self.operational_status
    }

    /// Sets the value of Status
    pub fn set_status(&mut self, value: String) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&String> {
        self.status.as_ref()
    }

    /// Sets the value of StatusDescriptions
    pub fn set_status_descriptions(&mut self, value: Vec<String>) {
        self.status_descriptions = value;
    }

    /// Gets the value of StatusDescriptions
    pub fn get_status_descriptions(&self) -> &Vec<String> {
        &self.status_descriptions
    }
}

