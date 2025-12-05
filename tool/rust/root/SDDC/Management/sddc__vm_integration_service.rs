// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.SDDC.Management
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SDDC_VmIntegrationService struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SDDC_VmIntegrationService {

/// 
    #[serde(rename = "IsEnabled")]
    pub is_enabled: Option<bool>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

impl SDDC_VmIntegrationService {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            is_enabled: None,
            name: None,
        }
    }


    /// Sets the value of IsEnabled
    pub fn set_is_enabled(&mut self, value: bool) {
        self.is_enabled = Some(value);
    }

    /// Gets the value of IsEnabled
    pub fn get_is_enabled(&self) -> Option<&bool> {
        self.is_enabled.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }
}

