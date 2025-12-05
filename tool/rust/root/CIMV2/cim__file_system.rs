// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_FileSystem struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_FileSystem {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// 
    #[serde(rename = "AvailableSpace")]
    pub available_space: Option<u64>,

/// 
    #[serde(rename = "BlockSize")]
    pub block_size: Option<u64>,

/// 
    #[serde(rename = "CasePreserved")]
    pub case_preserved: Option<bool>,

/// 
    #[serde(rename = "CaseSensitive")]
    pub case_sensitive: Option<bool>,

/// 
    #[serde(rename = "CodeSet")]
    pub code_set: Vec<u16>,

/// 
    #[serde(rename = "CompressionMethod")]
    pub compression_method: Option<String>,

/// 
    #[serde(rename = "CreationClassName")]
    pub creation_class_name: Option<String>,

/// 
    #[serde(rename = "CSCreationClassName")]
    pub cscreation_class_name: Option<String>,

/// 
    #[serde(rename = "CSName")]
    pub csname: Option<String>,

/// 
    #[serde(rename = "EncryptionMethod")]
    pub encryption_method: Option<String>,

/// 
    #[serde(rename = "FileSystemSize")]
    pub file_system_size: Option<u64>,

/// 
    #[serde(rename = "MaxFileNameLength")]
    pub max_file_name_length: Option<u32>,

/// 
    #[serde(rename = "ReadOnly")]
    pub read_only: Option<bool>,

/// 
    #[serde(rename = "Root")]
    pub root: Option<String>,
}

impl CIM_FileSystem {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
            available_space: None,
            block_size: None,
            case_preserved: None,
            case_sensitive: None,
            code_set: Vec::new(),
            compression_method: None,
            creation_class_name: None,
            cscreation_class_name: None,
            csname: None,
            encryption_method: None,
            file_system_size: None,
            max_file_name_length: None,
            read_only: None,
            root: None,
        }
    }


    /// Sets the value of AvailableSpace
    pub fn set_available_space(&mut self, value: u64) {
        self.available_space = Some(value);
    }

    /// Gets the value of AvailableSpace
    pub fn get_available_space(&self) -> Option<&u64> {
        self.available_space.as_ref()
    }

    /// Sets the value of BlockSize
    pub fn set_block_size(&mut self, value: u64) {
        self.block_size = Some(value);
    }

    /// Gets the value of BlockSize
    pub fn get_block_size(&self) -> Option<&u64> {
        self.block_size.as_ref()
    }

    /// Sets the value of CasePreserved
    pub fn set_case_preserved(&mut self, value: bool) {
        self.case_preserved = Some(value);
    }

    /// Gets the value of CasePreserved
    pub fn get_case_preserved(&self) -> Option<&bool> {
        self.case_preserved.as_ref()
    }

    /// Sets the value of CaseSensitive
    pub fn set_case_sensitive(&mut self, value: bool) {
        self.case_sensitive = Some(value);
    }

    /// Gets the value of CaseSensitive
    pub fn get_case_sensitive(&self) -> Option<&bool> {
        self.case_sensitive.as_ref()
    }

    /// Sets the value of CodeSet
    pub fn set_code_set(&mut self, value: Vec<u16>) {
        self.code_set = value;
    }

    /// Gets the value of CodeSet
    pub fn get_code_set(&self) -> &Vec<u16> {
        &self.code_set
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

    /// Sets the value of EncryptionMethod
    pub fn set_encryption_method(&mut self, value: String) {
        self.encryption_method = Some(value);
    }

    /// Gets the value of EncryptionMethod
    pub fn get_encryption_method(&self) -> Option<&String> {
        self.encryption_method.as_ref()
    }

    /// Sets the value of FileSystemSize
    pub fn set_file_system_size(&mut self, value: u64) {
        self.file_system_size = Some(value);
    }

    /// Gets the value of FileSystemSize
    pub fn get_file_system_size(&self) -> Option<&u64> {
        self.file_system_size.as_ref()
    }

    /// Sets the value of MaxFileNameLength
    pub fn set_max_file_name_length(&mut self, value: u32) {
        self.max_file_name_length = Some(value);
    }

    /// Gets the value of MaxFileNameLength
    pub fn get_max_file_name_length(&self) -> Option<&u32> {
        self.max_file_name_length.as_ref()
    }

    /// Sets the value of ReadOnly
    pub fn set_read_only(&mut self, value: bool) {
        self.read_only = Some(value);
    }

    /// Gets the value of ReadOnly
    pub fn get_read_only(&self) -> Option<&bool> {
        self.read_only.as_ref()
    }

    /// Sets the value of Root
    pub fn set_root(&mut self, value: String) {
        self.root = Some(value);
    }

    /// Gets the value of Root
    pub fn get_root(&self) -> Option<&String> {
        self.root.as_ref()
    }
}

