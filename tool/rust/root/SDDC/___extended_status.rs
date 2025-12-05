// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.SDDC
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __ExtendedStatus struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __ExtendedStatus {
    #[serde(flatten)]
    pub base: __NotifyStatus,

/// 
    #[serde(rename = "Description")]
    pub description: Option<String>,

/// 
    #[serde(rename = "Operation")]
    pub operation: Option<String>,

/// 
    #[serde(rename = "ParameterInfo")]
    pub parameter_info: Option<String>,

/// 
    #[serde(rename = "ProviderName")]
    pub provider_name: Option<String>,
}

impl __ExtendedStatus {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __NotifyStatus::new(),
            description: None,
            operation: None,
            parameter_info: None,
            provider_name: None,
        }
    }


    /// Sets the value of Description
    pub fn set_description(&mut self, value: String) {
        self.description = Some(value);
    }

    /// Gets the value of Description
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// Sets the value of Operation
    pub fn set_operation(&mut self, value: String) {
        self.operation = Some(value);
    }

    /// Gets the value of Operation
    pub fn get_operation(&self) -> Option<&String> {
        self.operation.as_ref()
    }

    /// Sets the value of ParameterInfo
    pub fn set_parameter_info(&mut self, value: String) {
        self.parameter_info = Some(value);
    }

    /// Gets the value of ParameterInfo
    pub fn get_parameter_info(&self) -> Option<&String> {
        self.parameter_info.as_ref()
    }

    /// Sets the value of ProviderName
    pub fn set_provider_name(&mut self, value: String) {
        self.provider_name = Some(value);
    }

    /// Gets the value of ProviderName
    pub fn get_provider_name(&self) -> Option<&String> {
        self.provider_name.as_ref()
    }
}

