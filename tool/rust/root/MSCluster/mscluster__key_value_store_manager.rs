// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.MSCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSCluster_KeyValueStoreManager struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSCluster_KeyValueStoreManager {

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "Path")]
    pub path: Option<String>,
}

impl MSCluster_KeyValueStoreManager {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            name: None,
            path: None,
        }
    }


    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of Path
    pub fn set_path(&mut self, value: String) {
        self.path = Some(value);
    }

    /// Gets the value of Path
    pub fn get_path(&self) -> Option<&String> {
        self.path.as_ref()
    }

/// 

    /// * `flags` -  (u32)
    /// * `name` -  (String)
    /// * `path` -  (String)

    /// * `created_key_value_store_manager` -  (MSCluster_KeyValueStoreManager)
    /// * `return_value` -  (u32)
    pub fn create_key_value_store_manager(&self, name: &String, path: &String, flags: u32, created_key_value_store_manager: &mut MSCluster_KeyValueStoreManager) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "Path".to_string(), value: path.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });

        let result = self.invoke_method("CreateKeyValueStoreManager", &args)?;
        let created_key_value_store_manager = result.get_value("CreatedKeyValueStoreManager")?;
        Ok(result.return_value)

    }


/// 

    /// * `flags` -  (u32)
    /// * `name` -  (String)

    /// * `return_value` -  (u32)
    pub fn delete_key_value_store_manager(&self, name: &String, flags: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        self.invoke_method("DeleteKeyValueStoreManager", &args)

    }


/// 

    /// * `flags` -  (u32)

    /// * `key_value_store_managers` -  (MSCluster_KeyValueStoreManager[])
    /// * `return_value` -  (u32)
    pub fn get_key_value_store_managers(&self, flags: u32, key_value_store_managers: &mut Vec<MSCluster_KeyValueStoreManager>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });

        let result = self.invoke_method("GetKeyValueStoreManagers", &args)?;
        let key_value_store_managers = result.get_value("KeyValueStoreManagers")?;
        Ok(result.return_value)

    }


/// 

    /// * `flags` -  (u32)
    /// * `name` -  (String)
    /// * `type` -  (u32)

    /// * `return_value` -  (u32)
    pub fn create_key_value_store(&self, name: &String, type: u32, flags: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "Type".to_string(), value: type.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        self.invoke_method("CreateKeyValueStore", &args)

    }


/// 

    /// * `flags` -  (u32)
    /// * `name` -  (String)

    /// * `return_value` -  (u32)
    pub fn delete_key_value_store(&self, name: &String, flags: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        self.invoke_method("DeleteKeyValueStore", &args)

    }


/// 

    /// * `flags` -  (u32)
    /// * `name` -  (String)
    /// * `path` -  (String)

    /// * `return_value` -  (u32)
    pub fn backup_key_value_store(&self, name: &String, path: &String, flags: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "Path".to_string(), value: path.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        self.invoke_method("BackupKeyValueStore", &args)

    }


/// 

    /// * `flags` -  (u32)
    /// * `name` -  (String)
    /// * `path` -  (String)

    /// * `return_value` -  (u32)
    pub fn restore_key_value_store(&self, name: &String, path: &String, flags: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "Path".to_string(), value: path.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        self.invoke_method("RestoreKeyValueStore", &args)

    }


/// 

    /// * `flags` -  (u32)

    /// * `key_value_stores` -  (MSCluster_KeyValueStore[])
    /// * `return_value` -  (u32)
    pub fn get_key_value_stores(&self, flags: u32, key_value_stores: &mut Vec<MSCluster_KeyValueStore>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });

        let result = self.invoke_method("GetKeyValueStores", &args)?;
        let key_value_stores = result.get_value("KeyValueStores")?;
        Ok(result.return_value)

    }


/// 

    /// * `flags` -  (u32)

    /// * `key_value_stores` -  (MSCluster_KeyValueStore[])
    /// * `return_value` -  (u32)
    pub fn get_all_key_value_stores(&self, flags: u32, key_value_stores: &mut Vec<MSCluster_KeyValueStore>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });

        let result = self.invoke_method("GetAllKeyValueStores", &args)?;
        let key_value_stores = result.get_value("KeyValueStores")?;
        Ok(result.return_value)

    }

}

