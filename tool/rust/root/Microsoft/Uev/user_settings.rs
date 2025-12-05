// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Uev
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// UserSettings struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UserSettings {
}

impl UserSettings {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// Restore users' application and Windows settings back to the original settings state.

    /// * `restore_type` - Restore Type (u32)
    /// * `template_id` - Template ID (String)
    pub fn restore_by_template_id(&self, template_id: &String, restore_type: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TemplateId".to_string(), value: template_id.into() });
        args.push(MethodParameter { name: "RestoreType".to_string(), value: restore_type.into() });
        self.invoke_method("RestoreByTemplateId", &args)

    }


/// Export a settings package file to a XML file listing all the settings in the package.

    /// * `absolute_path_to_package` - Absolute path to the settings package file (String)

    /// * `return_value` -  (String)
    pub fn export_package(&self, absolute_path_to_package: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AbsolutePathToPackage".to_string(), value: absolute_path_to_package.into() });
        self.invoke_method("ExportPackage", &args)

    }


/// Get the version of a settings package.

    /// * `absolute_path_to_package` - Absolute path to the settings package file (String)

    /// * `return_value` -  (u32)
    pub fn get_package_version(&self, absolute_path_to_package: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AbsolutePathToPackage".to_string(), value: absolute_path_to_package.into() });
        self.invoke_method("GetPackageVersion", &args)

    }


/// Check if the user live ID is connected.

    /// * `return_value` -  (bool)
    pub fn is_user_msa_connected(&self) -> Result<(), WmiError> {
        self.invoke_method("IsUserMsaConnected", &[])

    }


/// Restores the backup packages from another machine. Returns false if a restore is already pending, true otherwise

    /// * `computer_name` - Fully qualified computer name from which to restore packages (String)

    /// * `return_value` -  (bool)
    pub fn restore_backup(&self, computer_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ComputerName".to_string(), value: computer_name.into() });
        self.invoke_method("RestoreBackup", &args)

    }

}

