// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.TerminalServices
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_TSApplicationFileExtensions struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_TSApplicationFileExtensions {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,
}

impl Win32_TSApplicationFileExtensions {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
        }
    }


/// Gives the list of file extensions owned by an application

    /// * `app_path` - Path to the application (String)

    /// * `extensions` - File extensions owned by the application (String[])
    /// * `return_value` -  (u32)
    pub fn file_extensions(&self, app_path: &String, extensions: &mut Vec<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AppPath".to_string(), value: app_path.into() });

        let result = self.invoke_method("FileExtensions", &args)?;
        let extensions = result.get_value("Extensions")?;
        Ok(result.return_value)

    }


/// Scans the registry to get the current file associations for an application.

    /// * `app_path` - Path to the application (String)

    /// * `file_associations` - File associations for this application (Win32_RDFileAssociation[])
    /// * `return_value` -  (u32)
    pub fn file_associations(&self, app_path: &String, file_associations: &mut Vec<Win32_RDFileAssociation>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AppPath".to_string(), value: app_path.into() });

        let result = self.invoke_method("FileAssociations", &args)?;
        let file_associations = result.get_value("FileAssociations")?;
        Ok(result.return_value)

    }

}

