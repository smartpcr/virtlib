// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.TerminalServices
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_TSExpandEnvironmentVariables struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_TSExpandEnvironmentVariables {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,
}

impl Win32_TSExpandEnvironmentVariables {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
        }
    }


/// Expands System Defined Environment Variables

    /// * `original_string` - String that contains the environment variables to expand (String)

    /// * `expanded_string` - String with the environment variables expanded (String)
    /// * `return_value` -  (u32)
    pub fn environment_variables(&self, original_string: &String, expanded_string: &mut String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "OriginalString".to_string(), value: original_string.into() });

        let result = self.invoke_method("EnvironmentVariables", &args)?;
        let expanded_string = result.get_value("ExpandedString")?;
        Ok(result.return_value)

    }

}

