// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// BcdStore struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BcdStore {

/// A BcdStore is uniquely identified by its file path. The system store is denoted via an empty file path.
    #[serde(rename = "FilePath")]
    pub file_path: Option<String>,
}

impl BcdStore {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            file_path: None,
        }
    }


    /// Sets the value of FilePath
    pub fn set_file_path(&mut self, value: String) {
        self.file_path = Some(value);
    }

    /// Gets the value of FilePath
    pub fn get_file_path(&self) -> Option<&String> {
        self.file_path.as_ref()
    }

/// 

    /// * `file` -  (String)

    /// * `return_value` -  (bool)
    /// * `store` -  (BcdStore)
    pub fn open_store(&self, file: &String, store: &mut BcdStore) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "File".to_string(), value: file.into() });

        let result = self.invoke_method("OpenStore", &args)?;
        let store = result.get_value("Store")?;
        Ok(result.return_value)

    }


/// 

    /// * `file` -  (String)

    /// * `return_value` -  (bool)
    pub fn import_store(&self, file: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "File".to_string(), value: file.into() });
        self.invoke_method("ImportStore", &args)

    }


/// 

    /// * `file` -  (String)
    /// * `flags` -  (u32)

    /// * `return_value` -  (bool)
    pub fn import_store_with_flags(&self, file: &String, flags: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "File".to_string(), value: file.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        self.invoke_method("ImportStoreWithFlags", &args)

    }


/// 

    /// * `file` -  (String)

    /// * `return_value` -  (bool)
    pub fn export_store(&self, file: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "File".to_string(), value: file.into() });
        self.invoke_method("ExportStore", &args)

    }


/// 

    /// * `file` -  (String)

    /// * `return_value` -  (bool)
    /// * `store` -  (BcdStore)
    pub fn create_store(&self, file: &String, store: &mut BcdStore) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "File".to_string(), value: file.into() });

        let result = self.invoke_method("CreateStore", &args)?;
        let store = result.get_value("Store")?;
        Ok(result.return_value)

    }


/// 

    /// * `return_value` -  (bool)
    pub fn delete_system_store(&self) -> Result<(), WmiError> {
        self.invoke_method("DeleteSystemStore", &[])

    }


/// 

    /// * `disk` -  (String)
    /// * `return_value` -  (bool)
    pub fn get_system_disk(&self, disk: &mut String) -> Result<(), WmiError> {

        let result = self.invoke_method("GetSystemDisk", &[])?;
        let disk = result.get_value("Disk")?;
        Ok(result.return_value)

    }


/// 

    /// * `partition` -  (String)
    /// * `return_value` -  (bool)
    pub fn get_system_partition(&self, partition: &mut String) -> Result<(), WmiError> {

        let result = self.invoke_method("GetSystemPartition", &[])?;
        let partition = result.get_value("Partition")?;
        Ok(result.return_value)

    }


/// 

    /// * `partition` -  (String)

    /// * `return_value` -  (bool)
    pub fn set_system_store_device(&self, partition: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Partition".to_string(), value: partition.into() });
        self.invoke_method("SetSystemStoreDevice", &args)

    }


/// 

    /// * `type` -  (u32)

    /// * `objects` -  (BcdObject[])
    /// * `return_value` -  (bool)
    pub fn enumerate_objects(&self, type: u32, objects: &mut Vec<BcdObject>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Type".to_string(), value: type.into() });

        let result = self.invoke_method("EnumerateObjects", &args)?;
        let objects = result.get_value("Objects")?;
        Ok(result.return_value)

    }


/// 

    /// * `id` -  (String)

    /// * `object` -  (BcdObject)
    /// * `return_value` -  (bool)
    pub fn open_object(&self, id: &String, object: &mut BcdObject) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Id".to_string(), value: id.into() });

        let result = self.invoke_method("OpenObject", &args)?;
        let object = result.get_value("Object")?;
        Ok(result.return_value)

    }


/// 

    /// * `id` -  (String)
    /// * `type` -  (u32)

    /// * `object` -  (BcdObject)
    /// * `return_value` -  (bool)
    pub fn create_object(&self, id: &String, type: u32, object: &mut BcdObject) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Id".to_string(), value: id.into() });
        args.push(MethodParameter { name: "Type".to_string(), value: type.into() });

        let result = self.invoke_method("CreateObject", &args)?;
        let object = result.get_value("Object")?;
        Ok(result.return_value)

    }


/// 

    /// * `id` -  (String)

    /// * `return_value` -  (bool)
    pub fn delete_object(&self, id: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Id".to_string(), value: id.into() });
        self.invoke_method("DeleteObject", &args)

    }


/// 

    /// * `flags` -  (u32)
    /// * `source_id` -  (String)
    /// * `source_store_file` -  (String)

    /// * `object` -  (BcdObject)
    /// * `return_value` -  (bool)
    pub fn copy_object(&self, source_store_file: &String, source_id: &String, flags: u32, object: &mut BcdObject) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SourceStoreFile".to_string(), value: source_store_file.into() });
        args.push(MethodParameter { name: "SourceId".to_string(), value: source_id.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });

        let result = self.invoke_method("CopyObject", &args)?;
        let object = result.get_value("Object")?;
        Ok(result.return_value)

    }


/// 

    /// * `flags` -  (u32)
    /// * `source_store_file` -  (String)
    /// * `type` -  (u32)

    /// * `return_value` -  (bool)
    pub fn copy_objects(&self, source_store_file: &String, type: u32, flags: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SourceStoreFile".to_string(), value: source_store_file.into() });
        args.push(MethodParameter { name: "Type".to_string(), value: type.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        self.invoke_method("CopyObjects", &args)

    }

}

