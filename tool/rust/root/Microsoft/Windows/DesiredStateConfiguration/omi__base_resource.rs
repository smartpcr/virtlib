// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.DesiredStateConfiguration
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// OMI_BaseResource struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OMI_BaseResource {

/// 
    #[serde(rename = "ConfigurationName")]
    pub configuration_name: Option<String>,

/// 
    #[serde(rename = "DependsOn")]
    pub depends_on: Vec<String>,

/// 
    #[serde(rename = "ModuleName")]
    pub module_name: Option<String>,

/// 
    #[serde(rename = "ModuleVersion")]
    pub module_version: Option<String>,

/// 
    #[serde(rename = "PsDscRunAsCredential")]
    pub ps_dsc_run_as_credential: Option<MSFT_Credential>,

/// 
    #[serde(rename = "ResourceId")]
    pub resource_id: Option<String>,

/// 
    #[serde(rename = "SourceInfo")]
    pub source_info: Option<String>,
}

impl OMI_BaseResource {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            configuration_name: None,
            depends_on: Vec::new(),
            module_name: None,
            module_version: None,
            ps_dsc_run_as_credential: None,
            resource_id: None,
            source_info: None,
        }
    }


    /// Sets the value of ConfigurationName
    pub fn set_configuration_name(&mut self, value: String) {
        self.configuration_name = Some(value);
    }

    /// Gets the value of ConfigurationName
    pub fn get_configuration_name(&self) -> Option<&String> {
        self.configuration_name.as_ref()
    }

    /// Sets the value of DependsOn
    pub fn set_depends_on(&mut self, value: Vec<String>) {
        self.depends_on = value;
    }

    /// Gets the value of DependsOn
    pub fn get_depends_on(&self) -> &Vec<String> {
        &self.depends_on
    }

    /// Sets the value of ModuleName
    pub fn set_module_name(&mut self, value: String) {
        self.module_name = Some(value);
    }

    /// Gets the value of ModuleName
    pub fn get_module_name(&self) -> Option<&String> {
        self.module_name.as_ref()
    }

    /// Sets the value of ModuleVersion
    pub fn set_module_version(&mut self, value: String) {
        self.module_version = Some(value);
    }

    /// Gets the value of ModuleVersion
    pub fn get_module_version(&self) -> Option<&String> {
        self.module_version.as_ref()
    }

    /// Sets the value of PsDscRunAsCredential
    pub fn set_ps_dsc_run_as_credential(&mut self, value: MSFT_Credential) {
        self.ps_dsc_run_as_credential = Some(value);
    }

    /// Gets the value of PsDscRunAsCredential
    pub fn get_ps_dsc_run_as_credential(&self) -> Option<&MSFT_Credential> {
        self.ps_dsc_run_as_credential.as_ref()
    }

    /// Sets the value of ResourceId
    pub fn set_resource_id(&mut self, value: String) {
        self.resource_id = Some(value);
    }

    /// Gets the value of ResourceId
    pub fn get_resource_id(&self) -> Option<&String> {
        self.resource_id.as_ref()
    }

    /// Sets the value of SourceInfo
    pub fn set_source_info(&mut self, value: String) {
        self.source_info = Some(value);
    }

    /// Gets the value of SourceInfo
    pub fn get_source_info(&self) -> Option<&String> {
        self.source_info.as_ref()
    }
}

