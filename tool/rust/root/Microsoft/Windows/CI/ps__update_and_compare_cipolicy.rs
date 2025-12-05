// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.CI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// PS_UpdateAndCompareCIPolicy struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PS_UpdateAndCompareCIPolicy {
}

impl PS_UpdateAndCompareCIPolicy {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// 

    /// * `file_path` -  (String)

    /// * `cmdlet_output` -  (u64)
    /// * `return_value` -  (u32)
    pub fn compare(&self, file_path: &String, cmdlet_output: &mut u64) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FilePath".to_string(), value: file_path.into() });

        let result = self.invoke_method("Compare", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `file_path` -  (String)

    /// * `cmdlet_output` -  (u64)
    /// * `return_value` -  (u32)
    pub fn update(&self, file_path: &String, cmdlet_output: &mut u64) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FilePath".to_string(), value: file_path.into() });

        let result = self.invoke_method("Update", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `cmdlet_output` -  (u64)
    /// * `return_value` -  (u32)
    pub fn delete(&self, cmdlet_output: &mut u64) -> Result<(), WmiError> {

        let result = self.invoke_method("Delete", &[])?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }

}

