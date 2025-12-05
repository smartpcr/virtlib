// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_WNSConfiguration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_WNSConfiguration {

/// 
    #[serde(rename = "AppId")]
    pub app_id: Option<String>,

/// 
    #[serde(rename = "ConfigurationStatus")]
    pub configuration_status: Option<u32>,
}

impl MDM_WNSConfiguration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            app_id: None,
            configuration_status: None,
        }
    }


    /// Sets the value of AppId
    pub fn set_app_id(&mut self, value: String) {
        self.app_id = Some(value);
    }

    /// Gets the value of AppId
    pub fn get_app_id(&self) -> Option<&String> {
        self.app_id.as_ref()
    }

    /// Sets the value of ConfigurationStatus
    pub fn set_configuration_status(&mut self, value: u32) {
        self.configuration_status = Some(value);
    }

    /// Gets the value of ConfigurationStatus
    pub fn get_configuration_status(&self) -> Option<&u32> {
        self.configuration_status.as_ref()
    }

/// 

    /// * `config_string` -  (String)

    /// * `return_value` -  (u32)
    pub fn update_configuration(&self, config_string: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ConfigString".to_string(), value: config_string.into() });
        self.invoke_method("UpdateConfiguration", &args)

    }

}

