// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_FileIntegrity struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_FileIntegrity {

/// 
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

/// 
    #[serde(rename = "Enforced")]
    pub enforced: Option<bool>,

/// 
    #[serde(rename = "FileName")]
    pub file_name: Option<String>,
}

impl MSFT_FileIntegrity {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            enabled: None,
            enforced: None,
            file_name: None,
        }
    }


    /// Sets the value of Enabled
    pub fn set_enabled(&mut self, value: bool) {
        self.enabled = Some(value);
    }

    /// Gets the value of Enabled
    pub fn get_enabled(&self) -> Option<&bool> {
        self.enabled.as_ref()
    }

    /// Sets the value of Enforced
    pub fn set_enforced(&mut self, value: bool) {
        self.enforced = Some(value);
    }

    /// Gets the value of Enforced
    pub fn get_enforced(&self) -> Option<&bool> {
        self.enforced.as_ref()
    }

    /// Sets the value of FileName
    pub fn set_file_name(&mut self, value: String) {
        self.file_name = Some(value);
    }

    /// Gets the value of FileName
    pub fn get_file_name(&self) -> Option<&String> {
        self.file_name.as_ref()
    }

/// 

    /// * `file_name` -  (String)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `file_integrity` -  (MSFT_FileIntegrity)
    /// * `return_value` -  (u32)
    pub fn get(&self, file_name: &String, file_integrity: &mut MSFT_FileIntegrity, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FileName".to_string(), value: file_name.into() });

        let result = self.invoke_method("Get", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        let file_integrity = result.get_value("FileIntegrity")?;
        Ok(result.return_value)

    }


/// 

    /// * `file_name` -  (String)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn repair(&self, file_name: &String, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FileName".to_string(), value: file_name.into() });

        let result = self.invoke_method("Repair", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `enable` -  (bool)
    /// * `enforce` -  (bool)
    /// * `file_name` -  (String)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn set(&self, file_name: &String, enable: bool, enforce: bool, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FileName".to_string(), value: file_name.into() });
        args.push(MethodParameter { name: "Enable".to_string(), value: enable.into() });
        args.push(MethodParameter { name: "Enforce".to_string(), value: enforce.into() });

        let result = self.invoke_method("Set", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }

}

