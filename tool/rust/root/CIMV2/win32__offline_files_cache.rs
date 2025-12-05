// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_OfflineFilesCache struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_OfflineFilesCache {

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

/// 
    #[serde(rename = "Location")]
    pub location: Option<String>,
}

impl Win32_OfflineFilesCache {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            active: None,
            enabled: None,
            location: None,
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

    /// Sets the value of Enabled
    pub fn set_enabled(&mut self, value: bool) {
        self.enabled = Some(value);
    }

    /// Gets the value of Enabled
    pub fn get_enabled(&self) -> Option<&bool> {
        self.enabled.as_ref()
    }

    /// Sets the value of Location
    pub fn set_location(&mut self, value: String) {
        self.location = Some(value);
    }

    /// Gets the value of Location
    pub fn get_location(&self) -> Option<&String> {
        self.location.as_ref()
    }

/// 

    /// * `enable` -  (bool)

    /// * `reboot_required` -  (bool)
    /// * `return_value` -  (u32)
    pub fn enable(&self, enable: bool, reboot_required: &mut bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Enable".to_string(), value: enable.into() });

        let result = self.invoke_method("Enable", &args)?;
        let reboot_required = result.get_value("RebootRequired")?;
        Ok(result.return_value)

    }


/// 

    /// * `new_path` -  (String)
    /// * `original_path` -  (String)
    /// * `replace_if_exists` -  (bool)

    /// * `return_value` -  (u32)
    pub fn rename_item(&self, original_path: &String, new_path: &String, replace_if_exists: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "OriginalPath".to_string(), value: original_path.into() });
        args.push(MethodParameter { name: "NewPath".to_string(), value: new_path.into() });
        args.push(MethodParameter { name: "ReplaceIfExists".to_string(), value: replace_if_exists.into() });
        self.invoke_method("RenameItem", &args)

    }


/// 

    /// * `new_path` -  (String)
    /// * `original_path` -  (String)
    /// * `replace_if_exists` -  (bool)

    /// * `return_value` -  (u32)
    pub fn rename_item_ex(&self, original_path: &String, new_path: &String, replace_if_exists: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "OriginalPath".to_string(), value: original_path.into() });
        args.push(MethodParameter { name: "NewPath".to_string(), value: new_path.into() });
        args.push(MethodParameter { name: "ReplaceIfExists".to_string(), value: replace_if_exists.into() });
        self.invoke_method("RenameItemEx", &args)

    }


/// 

    /// * `flags` -  (u32)
    /// * `paths` -  (String[])

    /// * `return_value` -  (u32)
    pub fn synchronize(&self, paths: &Vec<String>, flags: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Paths".to_string(), value: paths.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        self.invoke_method("Synchronize", &args)

    }


/// 

    /// * `deep` -  (bool)
    /// * `flags` -  (u32)
    /// * `paths` -  (String[])

    /// * `return_value` -  (u32)
    pub fn pin(&self, paths: &Vec<String>, flags: u32, deep: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Paths".to_string(), value: paths.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        args.push(MethodParameter { name: "Deep".to_string(), value: deep.into() });
        self.invoke_method("Pin", &args)

    }


/// 

    /// * `deep` -  (bool)
    /// * `flags` -  (u32)
    /// * `paths` -  (String[])

    /// * `return_value` -  (u32)
    pub fn unpin(&self, paths: &Vec<String>, flags: u32, deep: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Paths".to_string(), value: paths.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        args.push(MethodParameter { name: "Deep".to_string(), value: deep.into() });
        self.invoke_method("Unpin", &args)

    }


/// 

    /// * `flags` -  (u32)
    /// * `paths` -  (String[])

    /// * `return_value` -  (u32)
    pub fn delete_items(&self, paths: &Vec<String>, flags: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Paths".to_string(), value: paths.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        self.invoke_method("DeleteItems", &args)

    }


/// 

    /// * `encrypt` -  (bool)
    /// * `flags` -  (u32)

    /// * `return_value` -  (u32)
    pub fn encrypt(&self, encrypt: bool, flags: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Encrypt".to_string(), value: encrypt.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        self.invoke_method("Encrypt", &args)

    }


/// 

    /// * `path` -  (String)
    /// * `suspend` -  (bool)

    /// * `return_value` -  (u32)
    pub fn suspend_root(&self, path: &String, suspend: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Path".to_string(), value: path.into() });
        args.push(MethodParameter { name: "Suspend".to_string(), value: suspend.into() });
        self.invoke_method("SuspendRoot", &args)

    }


/// 

    /// * `flags` -  (u32)
    /// * `force` -  (bool)
    /// * `path` -  (String)

    /// * `open_files` -  (bool)
    /// * `return_value` -  (u32)
    pub fn transition_offline(&self, path: &String, force: bool, flags: u32, open_files: &mut bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Path".to_string(), value: path.into() });
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });

        let result = self.invoke_method("TransitionOffline", &args)?;
        let open_files = result.get_value("OpenFiles")?;
        Ok(result.return_value)

    }


/// 

    /// * `flags` -  (u32)
    /// * `path` -  (String)

    /// * `return_value` -  (u32)
    pub fn transition_online(&self, path: &String, flags: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Path".to_string(), value: path.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        self.invoke_method("TransitionOnline", &args)

    }

}

