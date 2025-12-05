// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_QueryReceiveFilterGlobalParameters struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_QueryReceiveFilterGlobalParameters {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,
}

impl MSNdis_QueryReceiveFilterGlobalParameters {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            active: None,
            instance_name: None,
        }
    }


    /// Sets the value of Active
    pub fn set_active(&mut self, value: bool) {
        self.active = Some(value);
    }

    /// Gets the value of Active
    pub fn get_active(&self) -> Option<&bool> {
        self.active.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

/// 

    /// * `header` -  (MSNdis_WmiMethodHeader)

    /// * `receive_filter_global_parameters` -  (MSNdis_ReceiveFilterGlobalParameters)
    pub fn wmi_query_receive_filter_global_parameters(&self, header: MSNdis_WmiMethodHeader, receive_filter_global_parameters: &mut MSNdis_ReceiveFilterGlobalParameters) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Header".to_string(), value: header.into() });

        let result = self.invoke_method("WmiQueryReceiveFilterGlobalParameters", &args)?;
        let receive_filter_global_parameters = result.get_value("ReceiveFilterGlobalParameters")?;
        Ok(result.return_value)

    }

}

