// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ManagementTools
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_MTRegistryValue struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_MTRegistryValue {
    #[serde(flatten)]
    pub base: MSFT_MTRegistryObject,

/// 
    #[serde(rename = "Status")]
    pub status: Option<u16>,

/// 
    #[serde(rename = "Type")]
    pub type: Option<u32>,
}

impl MSFT_MTRegistryValue {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_MTRegistryObject::new(),
            status: None,
            type: None,
        }
    }


    /// Sets the value of Status
    pub fn set_status(&mut self, value: u16) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&u16> {
        self.status.as_ref()
    }

    /// Sets the value of Type
    pub fn set_type(&mut self, value: u32) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&u32> {
        self.type.as_ref()
    }

/// 

    /// * `new_name` -  (String)

    /// * `result` -  (MSFT_MTRegistryValue)
    /// * `return_value` -  (u32)
    pub fn rename(&self, new_name: &String, result: &mut MSFT_MTRegistryValue) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "NewName".to_string(), value: new_name.into() });

        let result = self.invoke_method("Rename", &args)?;
        let result = result.get_value("Result")?;
        Ok(result.return_value)

    }


/// 

    /// * `name` -  (String)

    /// * `result` -  (MSFT_MTRegistryValue)
    /// * `return_value` -  (u32)
    pub fn get_value(&self, name: &String, result: &mut MSFT_MTRegistryValue) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });

        let result = self.invoke_method("GetValue", &args)?;
        let result = result.get_value("Result")?;
        Ok(result.return_value)

    }

}

