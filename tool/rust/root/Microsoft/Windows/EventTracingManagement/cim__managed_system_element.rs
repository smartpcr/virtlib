// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.EventTracingManagement
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ManagedSystemElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ManagedSystemElement {
    #[serde(flatten)]
    pub base: CIM_ManagedElement,

/// 
    #[serde(rename = "CommunicationStatus")]
    pub communication_status: Option<u16>,

/// 
    #[serde(rename = "DetailedStatus")]
    pub detailed_status: Option<u16>,

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
    #[serde(rename = "OperatingStatus")]
    pub operating_status: Option<u16>,

/// 
    #[serde(rename = "OperationalStatus")]
    pub operational_status: Vec<u16>,

/// 
    #[serde(rename = "PrimaryStatus")]
    pub primary_status: Option<u16>,

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
            communication_status: None,
            detailed_status: None,
            health_state: None,
            install_date: None,
            name: None,
            operating_status: None,
            operational_status: Vec::new(),
            primary_status: None,
            status: None,
            status_descriptions: Vec::new(),
        }
    }


    /// Sets the value of CommunicationStatus
    pub fn set_communication_status(&mut self, value: u16) {
        self.communication_status = Some(value);
    }

    /// Gets the value of CommunicationStatus
    pub fn get_communication_status(&self) -> Option<&u16> {
        self.communication_status.as_ref()
    }

    /// Sets the value of DetailedStatus
    pub fn set_detailed_status(&mut self, value: u16) {
        self.detailed_status = Some(value);
    }

    /// Gets the value of DetailedStatus
    pub fn get_detailed_status(&self) -> Option<&u16> {
        self.detailed_status.as_ref()
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

    /// Sets the value of OperatingStatus
    pub fn set_operating_status(&mut self, value: u16) {
        self.operating_status = Some(value);
    }

    /// Gets the value of OperatingStatus
    pub fn get_operating_status(&self) -> Option<&u16> {
        self.operating_status.as_ref()
    }

    /// Sets the value of OperationalStatus
    pub fn set_operational_status(&mut self, value: Vec<u16>) {
        self.operational_status = value;
    }

    /// Gets the value of OperationalStatus
    pub fn get_operational_status(&self) -> &Vec<u16> {
        &self.operational_status
    }

    /// Sets the value of PrimaryStatus
    pub fn set_primary_status(&mut self, value: u16) {
        self.primary_status = Some(value);
    }

    /// Gets the value of PrimaryStatus
    pub fn get_primary_status(&self) -> Option<&u16> {
        self.primary_status.as_ref()
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

