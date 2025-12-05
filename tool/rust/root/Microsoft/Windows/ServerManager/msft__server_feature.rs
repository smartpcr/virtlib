// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ServerManager
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_ServerFeature struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_ServerFeature {

/// 
    #[serde(rename = "BpaModels")]
    pub bpa_models: Vec<String>,

/// 
    #[serde(rename = "ConfigurationStatus")]
    pub configuration_status: Option<u8>,

/// 
    #[serde(rename = "DisplayName")]
    pub display_name: Option<String>,

/// 
    #[serde(rename = "EventQuery")]
    pub event_query: Option<String>,

/// 
    #[serde(rename = "Id")]
    pub id: Option<i32>,

/// 
    #[serde(rename = "ParentName")]
    pub parent_name: Option<String>,

/// 
    #[serde(rename = "Services")]
    pub services: Vec<String>,

/// 
    #[serde(rename = "State")]
    pub state: Option<u8>,

/// 
    #[serde(rename = "Type")]
    pub type: Option<u8>,

/// 
    #[serde(rename = "UniqueName")]
    pub unique_name: Option<String>,
}

impl MSFT_ServerFeature {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            bpa_models: Vec::new(),
            configuration_status: None,
            display_name: None,
            event_query: None,
            id: None,
            parent_name: None,
            services: Vec::new(),
            state: None,
            type: None,
            unique_name: None,
        }
    }


    /// Sets the value of BpaModels
    pub fn set_bpa_models(&mut self, value: Vec<String>) {
        self.bpa_models = value;
    }

    /// Gets the value of BpaModels
    pub fn get_bpa_models(&self) -> &Vec<String> {
        &self.bpa_models
    }

    /// Sets the value of ConfigurationStatus
    pub fn set_configuration_status(&mut self, value: u8) {
        self.configuration_status = Some(value);
    }

    /// Gets the value of ConfigurationStatus
    pub fn get_configuration_status(&self) -> Option<&u8> {
        self.configuration_status.as_ref()
    }

    /// Sets the value of DisplayName
    pub fn set_display_name(&mut self, value: String) {
        self.display_name = Some(value);
    }

    /// Gets the value of DisplayName
    pub fn get_display_name(&self) -> Option<&String> {
        self.display_name.as_ref()
    }

    /// Sets the value of EventQuery
    pub fn set_event_query(&mut self, value: String) {
        self.event_query = Some(value);
    }

    /// Gets the value of EventQuery
    pub fn get_event_query(&self) -> Option<&String> {
        self.event_query.as_ref()
    }

    /// Sets the value of Id
    pub fn set_id(&mut self, value: i32) {
        self.id = Some(value);
    }

    /// Gets the value of Id
    pub fn get_id(&self) -> Option<&i32> {
        self.id.as_ref()
    }

    /// Sets the value of ParentName
    pub fn set_parent_name(&mut self, value: String) {
        self.parent_name = Some(value);
    }

    /// Gets the value of ParentName
    pub fn get_parent_name(&self) -> Option<&String> {
        self.parent_name.as_ref()
    }

    /// Sets the value of Services
    pub fn set_services(&mut self, value: Vec<String>) {
        self.services = value;
    }

    /// Gets the value of Services
    pub fn get_services(&self) -> &Vec<String> {
        &self.services
    }

    /// Sets the value of State
    pub fn set_state(&mut self, value: u8) {
        self.state = Some(value);
    }

    /// Gets the value of State
    pub fn get_state(&self) -> Option<&u8> {
        self.state.as_ref()
    }

    /// Sets the value of Type
    pub fn set_type(&mut self, value: u8) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&u8> {
        self.type.as_ref()
    }

    /// Sets the value of UniqueName
    pub fn set_unique_name(&mut self, value: String) {
        self.unique_name = Some(value);
    }

    /// Gets the value of UniqueName
    pub fn get_unique_name(&self) -> Option<&String> {
        self.unique_name.as_ref()
    }
}

