// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_LogicalFile struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_LogicalFile {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// 
    #[serde(rename = "AccessMask")]
    pub access_mask: Option<u32>,

/// 
    #[serde(rename = "Archive")]
    pub archive: Option<bool>,

/// 
    #[serde(rename = "Compressed")]
    pub compressed: Option<bool>,

/// 
    #[serde(rename = "CompressionMethod")]
    pub compression_method: Option<String>,

/// 
    #[serde(rename = "CreationClassName")]
    pub creation_class_name: Option<String>,

/// 
    #[serde(rename = "CreationDate")]
    pub creation_date: Option<String>,

/// 
    #[serde(rename = "CSCreationClassName")]
    pub cscreation_class_name: Option<String>,

/// 
    #[serde(rename = "CSName")]
    pub csname: Option<String>,

/// 
    #[serde(rename = "Drive")]
    pub drive: Option<String>,

/// 
    #[serde(rename = "EightDotThreeFileName")]
    pub eight_dot_three_file_name: Option<String>,

/// 
    #[serde(rename = "Encrypted")]
    pub encrypted: Option<bool>,

/// 
    #[serde(rename = "EncryptionMethod")]
    pub encryption_method: Option<String>,

/// 
    #[serde(rename = "Extension")]
    pub extension: Option<String>,

/// 
    #[serde(rename = "FileName")]
    pub file_name: Option<String>,

/// 
    #[serde(rename = "FileSize")]
    pub file_size: Option<u64>,

/// 
    #[serde(rename = "FileType")]
    pub file_type: Option<String>,

/// 
    #[serde(rename = "FSCreationClassName")]
    pub fscreation_class_name: Option<String>,

/// 
    #[serde(rename = "FSName")]
    pub fsname: Option<String>,

/// 
    #[serde(rename = "Hidden")]
    pub hidden: Option<bool>,

/// 
    #[serde(rename = "InUseCount")]
    pub in_use_count: Option<u64>,

/// 
    #[serde(rename = "LastAccessed")]
    pub last_accessed: Option<String>,

/// 
    #[serde(rename = "LastModified")]
    pub last_modified: Option<String>,

/// 
    #[serde(rename = "Path")]
    pub path: Option<String>,

/// 
    #[serde(rename = "Readable")]
    pub readable: Option<bool>,

/// 
    #[serde(rename = "System")]
    pub system: Option<bool>,

/// 
    #[serde(rename = "Writeable")]
    pub writeable: Option<bool>,
}

impl CIM_LogicalFile {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
            access_mask: None,
            archive: None,
            compressed: None,
            compression_method: None,
            creation_class_name: None,
            creation_date: None,
            cscreation_class_name: None,
            csname: None,
            drive: None,
            eight_dot_three_file_name: None,
            encrypted: None,
            encryption_method: None,
            extension: None,
            file_name: None,
            file_size: None,
            file_type: None,
            fscreation_class_name: None,
            fsname: None,
            hidden: None,
            in_use_count: None,
            last_accessed: None,
            last_modified: None,
            path: None,
            readable: None,
            system: None,
            writeable: None,
        }
    }


    /// Sets the value of AccessMask
    pub fn set_access_mask(&mut self, value: u32) {
        self.access_mask = Some(value);
    }

    /// Gets the value of AccessMask
    pub fn get_access_mask(&self) -> Option<&u32> {
        self.access_mask.as_ref()
    }

    /// Sets the value of Archive
    pub fn set_archive(&mut self, value: bool) {
        self.archive = Some(value);
    }

    /// Gets the value of Archive
    pub fn get_archive(&self) -> Option<&bool> {
        self.archive.as_ref()
    }

    /// Sets the value of Compressed
    pub fn set_compressed(&mut self, value: bool) {
        self.compressed = Some(value);
    }

    /// Gets the value of Compressed
    pub fn get_compressed(&self) -> Option<&bool> {
        self.compressed.as_ref()
    }

    /// Sets the value of CompressionMethod
    pub fn set_compression_method(&mut self, value: String) {
        self.compression_method = Some(value);
    }

    /// Gets the value of CompressionMethod
    pub fn get_compression_method(&self) -> Option<&String> {
        self.compression_method.as_ref()
    }

    /// Sets the value of CreationClassName
    pub fn set_creation_class_name(&mut self, value: String) {
        self.creation_class_name = Some(value);
    }

    /// Gets the value of CreationClassName
    pub fn get_creation_class_name(&self) -> Option<&String> {
        self.creation_class_name.as_ref()
    }

    /// Sets the value of CreationDate
    pub fn set_creation_date(&mut self, value: String) {
        self.creation_date = Some(value);
    }

    /// Gets the value of CreationDate
    pub fn get_creation_date(&self) -> Option<&String> {
        self.creation_date.as_ref()
    }

    /// Sets the value of CSCreationClassName
    pub fn set_cscreation_class_name(&mut self, value: String) {
        self.cscreation_class_name = Some(value);
    }

    /// Gets the value of CSCreationClassName
    pub fn get_cscreation_class_name(&self) -> Option<&String> {
        self.cscreation_class_name.as_ref()
    }

    /// Sets the value of CSName
    pub fn set_csname(&mut self, value: String) {
        self.csname = Some(value);
    }

    /// Gets the value of CSName
    pub fn get_csname(&self) -> Option<&String> {
        self.csname.as_ref()
    }

    /// Sets the value of Drive
    pub fn set_drive(&mut self, value: String) {
        self.drive = Some(value);
    }

    /// Gets the value of Drive
    pub fn get_drive(&self) -> Option<&String> {
        self.drive.as_ref()
    }

    /// Sets the value of EightDotThreeFileName
    pub fn set_eight_dot_three_file_name(&mut self, value: String) {
        self.eight_dot_three_file_name = Some(value);
    }

    /// Gets the value of EightDotThreeFileName
    pub fn get_eight_dot_three_file_name(&self) -> Option<&String> {
        self.eight_dot_three_file_name.as_ref()
    }

    /// Sets the value of Encrypted
    pub fn set_encrypted(&mut self, value: bool) {
        self.encrypted = Some(value);
    }

    /// Gets the value of Encrypted
    pub fn get_encrypted(&self) -> Option<&bool> {
        self.encrypted.as_ref()
    }

    /// Sets the value of EncryptionMethod
    pub fn set_encryption_method(&mut self, value: String) {
        self.encryption_method = Some(value);
    }

    /// Gets the value of EncryptionMethod
    pub fn get_encryption_method(&self) -> Option<&String> {
        self.encryption_method.as_ref()
    }

    /// Sets the value of Extension
    pub fn set_extension(&mut self, value: String) {
        self.extension = Some(value);
    }

    /// Gets the value of Extension
    pub fn get_extension(&self) -> Option<&String> {
        self.extension.as_ref()
    }

    /// Sets the value of FileName
    pub fn set_file_name(&mut self, value: String) {
        self.file_name = Some(value);
    }

    /// Gets the value of FileName
    pub fn get_file_name(&self) -> Option<&String> {
        self.file_name.as_ref()
    }

    /// Sets the value of FileSize
    pub fn set_file_size(&mut self, value: u64) {
        self.file_size = Some(value);
    }

    /// Gets the value of FileSize
    pub fn get_file_size(&self) -> Option<&u64> {
        self.file_size.as_ref()
    }

    /// Sets the value of FileType
    pub fn set_file_type(&mut self, value: String) {
        self.file_type = Some(value);
    }

    /// Gets the value of FileType
    pub fn get_file_type(&self) -> Option<&String> {
        self.file_type.as_ref()
    }

    /// Sets the value of FSCreationClassName
    pub fn set_fscreation_class_name(&mut self, value: String) {
        self.fscreation_class_name = Some(value);
    }

    /// Gets the value of FSCreationClassName
    pub fn get_fscreation_class_name(&self) -> Option<&String> {
        self.fscreation_class_name.as_ref()
    }

    /// Sets the value of FSName
    pub fn set_fsname(&mut self, value: String) {
        self.fsname = Some(value);
    }

    /// Gets the value of FSName
    pub fn get_fsname(&self) -> Option<&String> {
        self.fsname.as_ref()
    }

    /// Sets the value of Hidden
    pub fn set_hidden(&mut self, value: bool) {
        self.hidden = Some(value);
    }

    /// Gets the value of Hidden
    pub fn get_hidden(&self) -> Option<&bool> {
        self.hidden.as_ref()
    }

    /// Sets the value of InUseCount
    pub fn set_in_use_count(&mut self, value: u64) {
        self.in_use_count = Some(value);
    }

    /// Gets the value of InUseCount
    pub fn get_in_use_count(&self) -> Option<&u64> {
        self.in_use_count.as_ref()
    }

    /// Sets the value of LastAccessed
    pub fn set_last_accessed(&mut self, value: String) {
        self.last_accessed = Some(value);
    }

    /// Gets the value of LastAccessed
    pub fn get_last_accessed(&self) -> Option<&String> {
        self.last_accessed.as_ref()
    }

    /// Sets the value of LastModified
    pub fn set_last_modified(&mut self, value: String) {
        self.last_modified = Some(value);
    }

    /// Gets the value of LastModified
    pub fn get_last_modified(&self) -> Option<&String> {
        self.last_modified.as_ref()
    }

    /// Sets the value of Path
    pub fn set_path(&mut self, value: String) {
        self.path = Some(value);
    }

    /// Gets the value of Path
    pub fn get_path(&self) -> Option<&String> {
        self.path.as_ref()
    }

    /// Sets the value of Readable
    pub fn set_readable(&mut self, value: bool) {
        self.readable = Some(value);
    }

    /// Gets the value of Readable
    pub fn get_readable(&self) -> Option<&bool> {
        self.readable.as_ref()
    }

    /// Sets the value of System
    pub fn set_system(&mut self, value: bool) {
        self.system = Some(value);
    }

    /// Gets the value of System
    pub fn get_system(&self) -> Option<&bool> {
        self.system.as_ref()
    }

    /// Sets the value of Writeable
    pub fn set_writeable(&mut self, value: bool) {
        self.writeable = Some(value);
    }

    /// Gets the value of Writeable
    pub fn get_writeable(&self) -> Option<&bool> {
        self.writeable.as_ref()
    }

/// 

    /// * `return_value` -  (u32)
    pub fn take_owner_ship(&self) -> Result<(), WmiError> {
        self.invoke_method("TakeOwnerShip", &[])

    }


/// 

    /// * `option` -  (u32)
    /// * `security_descriptor` -  (Win32_SecurityDescriptor)

    /// * `return_value` -  (u32)
    pub fn change_security_permissions(&self, security_descriptor: Win32_SecurityDescriptor, option: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SecurityDescriptor".to_string(), value: security_descriptor.into() });
        args.push(MethodParameter { name: "Option".to_string(), value: option.into() });
        self.invoke_method("ChangeSecurityPermissions", &args)

    }


/// 

    /// * `file_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn copy(&self, file_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FileName".to_string(), value: file_name.into() });
        self.invoke_method("Copy", &args)

    }


/// 

    /// * `file_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn rename(&self, file_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FileName".to_string(), value: file_name.into() });
        self.invoke_method("Rename", &args)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn delete(&self) -> Result<(), WmiError> {
        self.invoke_method("Delete", &[])

    }


/// 

    /// * `return_value` -  (u32)
    pub fn compress(&self) -> Result<(), WmiError> {
        self.invoke_method("Compress", &[])

    }


/// 

    /// * `return_value` -  (u32)
    pub fn uncompress(&self) -> Result<(), WmiError> {
        self.invoke_method("Uncompress", &[])

    }


/// 

    /// * `recursive` -  (bool)
    /// * `start_file_name` -  (String)

    /// * `return_value` -  (u32)
    /// * `stop_file_name` -  (String)
    pub fn take_owner_ship_ex(&self, stop_file_name: &mut String, start_file_name: &Option<String>, recursive: Option<bool>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        if let Some(val) = start_file_name {
            args.push(MethodParameter { name: "StartFileName".to_string(), value: val.into() });
        }
        if let Some(val) = recursive {
            args.push(MethodParameter { name: "Recursive".to_string(), value: val.into() });
        }

        let result = self.invoke_method("TakeOwnerShipEx", &args)?;
        let stop_file_name = result.get_value("StopFileName")?;
        Ok(result.return_value)

    }


/// 

    /// * `option` -  (u32)
    /// * `recursive` -  (bool)
    /// * `security_descriptor` -  (Win32_SecurityDescriptor)
    /// * `start_file_name` -  (String)

    /// * `return_value` -  (u32)
    /// * `stop_file_name` -  (String)
    pub fn change_security_permissions_ex(&self, security_descriptor: Win32_SecurityDescriptor, option: u32, stop_file_name: &mut String, start_file_name: &Option<String>, recursive: Option<bool>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SecurityDescriptor".to_string(), value: security_descriptor.into() });
        args.push(MethodParameter { name: "Option".to_string(), value: option.into() });
        if let Some(val) = start_file_name {
            args.push(MethodParameter { name: "StartFileName".to_string(), value: val.into() });
        }
        if let Some(val) = recursive {
            args.push(MethodParameter { name: "Recursive".to_string(), value: val.into() });
        }

        let result = self.invoke_method("ChangeSecurityPermissionsEx", &args)?;
        let stop_file_name = result.get_value("StopFileName")?;
        Ok(result.return_value)

    }


/// 

    /// * `file_name` -  (String)
    /// * `recursive` -  (bool)
    /// * `start_file_name` -  (String)

    /// * `return_value` -  (u32)
    /// * `stop_file_name` -  (String)
    pub fn copy_ex(&self, file_name: &String, stop_file_name: &mut String, start_file_name: &Option<String>, recursive: Option<bool>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FileName".to_string(), value: file_name.into() });
        if let Some(val) = start_file_name {
            args.push(MethodParameter { name: "StartFileName".to_string(), value: val.into() });
        }
        if let Some(val) = recursive {
            args.push(MethodParameter { name: "Recursive".to_string(), value: val.into() });
        }

        let result = self.invoke_method("CopyEx", &args)?;
        let stop_file_name = result.get_value("StopFileName")?;
        Ok(result.return_value)

    }


/// 

    /// * `start_file_name` -  (String)

    /// * `return_value` -  (u32)
    /// * `stop_file_name` -  (String)
    pub fn delete_ex(&self, stop_file_name: &mut String, start_file_name: &Option<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        if let Some(val) = start_file_name {
            args.push(MethodParameter { name: "StartFileName".to_string(), value: val.into() });
        }

        let result = self.invoke_method("DeleteEx", &args)?;
        let stop_file_name = result.get_value("StopFileName")?;
        Ok(result.return_value)

    }


/// 

    /// * `recursive` -  (bool)
    /// * `start_file_name` -  (String)

    /// * `return_value` -  (u32)
    /// * `stop_file_name` -  (String)
    pub fn compress_ex(&self, stop_file_name: &mut String, start_file_name: &Option<String>, recursive: Option<bool>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        if let Some(val) = start_file_name {
            args.push(MethodParameter { name: "StartFileName".to_string(), value: val.into() });
        }
        if let Some(val) = recursive {
            args.push(MethodParameter { name: "Recursive".to_string(), value: val.into() });
        }

        let result = self.invoke_method("CompressEx", &args)?;
        let stop_file_name = result.get_value("StopFileName")?;
        Ok(result.return_value)

    }


/// 

    /// * `recursive` -  (bool)
    /// * `start_file_name` -  (String)

    /// * `return_value` -  (u32)
    /// * `stop_file_name` -  (String)
    pub fn uncompress_ex(&self, stop_file_name: &mut String, start_file_name: &Option<String>, recursive: Option<bool>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        if let Some(val) = start_file_name {
            args.push(MethodParameter { name: "StartFileName".to_string(), value: val.into() });
        }
        if let Some(val) = recursive {
            args.push(MethodParameter { name: "Recursive".to_string(), value: val.into() });
        }

        let result = self.invoke_method("UncompressEx", &args)?;
        let stop_file_name = result.get_value("StopFileName")?;
        Ok(result.return_value)

    }


/// 

    /// * `permissions` -  (u32)

    /// * `return_value` -  (bool)
    pub fn get_effective_permission(&self, permissions: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Permissions".to_string(), value: permissions.into() });
        self.invoke_method("GetEffectivePermission", &args)

    }

}

