// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.DesiredStateConfiguration
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// OMI_MetaConfigurationResource struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OMI_MetaConfigurationResource {

/// 
    #[serde(rename = "ResourceId")]
    pub resource_id: Option<String>,

/// 
    #[serde(rename = "SourceInfo")]
    pub source_info: Option<String>,
}

impl OMI_MetaConfigurationResource {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            resource_id: None,
            source_info: None,
        }
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

