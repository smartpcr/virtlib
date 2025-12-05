// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.DesiredStateConfiguration
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_PartialConfiguration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_PartialConfiguration {
    #[serde(flatten)]
    pub base: OMI_MetaConfigurationResource,

/// 
    #[serde(rename = "ConfigurationSource")]
    pub configuration_source: Vec<String>,

/// 
    #[serde(rename = "DependsOn")]
    pub depends_on: Vec<String>,

/// 
    #[serde(rename = "Description")]
    pub description: Option<String>,

/// 
    #[serde(rename = "ExclusiveResources")]
    pub exclusive_resources: Vec<String>,

/// 
    #[serde(rename = "RefreshMode")]
    pub refresh_mode: Option<String>,

/// 
    #[serde(rename = "ResourceModuleSource")]
    pub resource_module_source: Vec<String>,
}

impl MSFT_PartialConfiguration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: OMI_MetaConfigurationResource::new(),
            configuration_source: Vec::new(),
            depends_on: Vec::new(),
            description: None,
            exclusive_resources: Vec::new(),
            refresh_mode: None,
            resource_module_source: Vec::new(),
        }
    }


    /// Sets the value of ConfigurationSource
    pub fn set_configuration_source(&mut self, value: Vec<String>) {
        self.configuration_source = value;
    }

    /// Gets the value of ConfigurationSource
    pub fn get_configuration_source(&self) -> &Vec<String> {
        &self.configuration_source
    }

    /// Sets the value of DependsOn
    pub fn set_depends_on(&mut self, value: Vec<String>) {
        self.depends_on = value;
    }

    /// Gets the value of DependsOn
    pub fn get_depends_on(&self) -> &Vec<String> {
        &self.depends_on
    }

    /// Sets the value of Description
    pub fn set_description(&mut self, value: String) {
        self.description = Some(value);
    }

    /// Gets the value of Description
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// Sets the value of ExclusiveResources
    pub fn set_exclusive_resources(&mut self, value: Vec<String>) {
        self.exclusive_resources = value;
    }

    /// Gets the value of ExclusiveResources
    pub fn get_exclusive_resources(&self) -> &Vec<String> {
        &self.exclusive_resources
    }

    /// Sets the value of RefreshMode
    pub fn set_refresh_mode(&mut self, value: String) {
        self.refresh_mode = Some(value);
    }

    /// Gets the value of RefreshMode
    pub fn get_refresh_mode(&self) -> Option<&String> {
        self.refresh_mode.as_ref()
    }

    /// Sets the value of ResourceModuleSource
    pub fn set_resource_module_source(&mut self, value: Vec<String>) {
        self.resource_module_source = value;
    }

    /// Gets the value of ResourceModuleSource
    pub fn get_resource_module_source(&self) -> &Vec<String> {
        &self.resource_module_source
    }
}

