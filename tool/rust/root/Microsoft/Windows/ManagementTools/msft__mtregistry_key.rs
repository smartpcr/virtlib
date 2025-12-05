// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ManagementTools
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_MTRegistryKey struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_MTRegistryKey {
    #[serde(flatten)]
    pub base: MSFT_MTRegistryObject,

/// 
    #[serde(rename = "Modified")]
    pub modified: Option<String>,

/// 
    #[serde(rename = "SubKeyCount")]
    pub sub_key_count: Option<u32>,

/// 
    #[serde(rename = "ValueCount")]
    pub value_count: Option<u32>,
}

impl MSFT_MTRegistryKey {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_MTRegistryObject::new(),
            modified: None,
            sub_key_count: None,
            value_count: None,
        }
    }


    /// Sets the value of Modified
    pub fn set_modified(&mut self, value: String) {
        self.modified = Some(value);
    }

    /// Gets the value of Modified
    pub fn get_modified(&self) -> Option<&String> {
        self.modified.as_ref()
    }

    /// Sets the value of SubKeyCount
    pub fn set_sub_key_count(&mut self, value: u32) {
        self.sub_key_count = Some(value);
    }

    /// Gets the value of SubKeyCount
    pub fn get_sub_key_count(&self) -> Option<&u32> {
        self.sub_key_count.as_ref()
    }

    /// Sets the value of ValueCount
    pub fn set_value_count(&mut self, value: u32) {
        self.value_count = Some(value);
    }

    /// Gets the value of ValueCount
    pub fn get_value_count(&self) -> Option<&u32> {
        self.value_count.as_ref()
    }

/// 

    /// * `results` -  (MSFT_MTRegistryKey[])
    /// * `return_value` -  (u32)
    pub fn get_sub_keys(&self, results: &mut Vec<MSFT_MTRegistryKey>) -> Result<(), WmiError> {

        let result = self.invoke_method("GetSubKeys", &[])?;
        let results = result.get_value("Results")?;
        Ok(result.return_value)

    }


/// 

    /// * `results` -  (MSFT_MTRegistryValue[])
    /// * `return_value` -  (u32)
    pub fn get_values(&self, results: &mut Vec<MSFT_MTRegistryValue>) -> Result<(), WmiError> {

        let result = self.invoke_method("GetValues", &[])?;
        let results = result.get_value("Results")?;
        Ok(result.return_value)

    }


/// 

    /// * `new_name` -  (String)

    /// * `result` -  (MSFT_MTRegistryKey)
    /// * `return_value` -  (u32)
    pub fn rename(&self, new_name: &String, result: &mut MSFT_MTRegistryKey) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "NewName".to_string(), value: new_name.into() });

        let result = self.invoke_method("Rename", &args)?;
        let result = result.get_value("Result")?;
        Ok(result.return_value)

    }


/// 

    /// * `name` -  (String)

    /// * `result` -  (MSFT_MTRegistryKey)
    /// * `return_value` -  (u32)
    pub fn get_key(&self, name: &String, result: &mut MSFT_MTRegistryKey) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });

        let result = self.invoke_method("GetKey", &args)?;
        let result = result.get_value("Result")?;
        Ok(result.return_value)

    }

}

