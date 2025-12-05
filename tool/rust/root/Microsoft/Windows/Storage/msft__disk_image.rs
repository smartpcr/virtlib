// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_DiskImage struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_DiskImage {

/// 
    #[serde(rename = "Attached")]
    pub attached: Option<bool>,

/// 
    #[serde(rename = "BlockSize")]
    pub block_size: Option<u64>,

/// 
    #[serde(rename = "DevicePath")]
    pub device_path: Option<String>,

/// 
    #[serde(rename = "FileSize")]
    pub file_size: Option<u64>,

/// 
    #[serde(rename = "ImagePath")]
    pub image_path: Option<String>,

/// 
    #[serde(rename = "LogicalSectorSize")]
    pub logical_sector_size: Option<u64>,

/// 
    #[serde(rename = "Number")]
    pub number: Option<u32>,

/// 
    #[serde(rename = "Size")]
    pub size: Option<u64>,

/// 
    #[serde(rename = "StorageType")]
    pub storage_type: Option<u32>,
}

impl MSFT_DiskImage {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            attached: None,
            block_size: None,
            device_path: None,
            file_size: None,
            image_path: None,
            logical_sector_size: None,
            number: None,
            size: None,
            storage_type: None,
        }
    }


    /// Sets the value of Attached
    pub fn set_attached(&mut self, value: bool) {
        self.attached = Some(value);
    }

    /// Gets the value of Attached
    pub fn get_attached(&self) -> Option<&bool> {
        self.attached.as_ref()
    }

    /// Sets the value of BlockSize
    pub fn set_block_size(&mut self, value: u64) {
        self.block_size = Some(value);
    }

    /// Gets the value of BlockSize
    pub fn get_block_size(&self) -> Option<&u64> {
        self.block_size.as_ref()
    }

    /// Sets the value of DevicePath
    pub fn set_device_path(&mut self, value: String) {
        self.device_path = Some(value);
    }

    /// Gets the value of DevicePath
    pub fn get_device_path(&self) -> Option<&String> {
        self.device_path.as_ref()
    }

    /// Sets the value of FileSize
    pub fn set_file_size(&mut self, value: u64) {
        self.file_size = Some(value);
    }

    /// Gets the value of FileSize
    pub fn get_file_size(&self) -> Option<&u64> {
        self.file_size.as_ref()
    }

    /// Sets the value of ImagePath
    pub fn set_image_path(&mut self, value: String) {
        self.image_path = Some(value);
    }

    /// Gets the value of ImagePath
    pub fn get_image_path(&self) -> Option<&String> {
        self.image_path.as_ref()
    }

    /// Sets the value of LogicalSectorSize
    pub fn set_logical_sector_size(&mut self, value: u64) {
        self.logical_sector_size = Some(value);
    }

    /// Gets the value of LogicalSectorSize
    pub fn get_logical_sector_size(&self) -> Option<&u64> {
        self.logical_sector_size.as_ref()
    }

    /// Sets the value of Number
    pub fn set_number(&mut self, value: u32) {
        self.number = Some(value);
    }

    /// Gets the value of Number
    pub fn get_number(&self) -> Option<&u32> {
        self.number.as_ref()
    }

    /// Sets the value of Size
    pub fn set_size(&mut self, value: u64) {
        self.size = Some(value);
    }

    /// Gets the value of Size
    pub fn get_size(&self) -> Option<&u64> {
        self.size.as_ref()
    }

    /// Sets the value of StorageType
    pub fn set_storage_type(&mut self, value: u32) {
        self.storage_type = Some(value);
    }

    /// Gets the value of StorageType
    pub fn get_storage_type(&self) -> Option<&u32> {
        self.storage_type.as_ref()
    }

/// 

    /// * `access` -  (u16)
    /// * `no_drive_letter` -  (bool)

    /// * `disk_image` -  (MSFT_DiskImage)
    /// * `return_value` -  (u32)
    pub fn mount(&self, access: u16, no_drive_letter: bool, disk_image: &mut MSFT_DiskImage) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Access".to_string(), value: access.into() });
        args.push(MethodParameter { name: "NoDriveLetter".to_string(), value: no_drive_letter.into() });

        let result = self.invoke_method("Mount", &args)?;
        let disk_image = result.get_value("DiskImage")?;
        Ok(result.return_value)

    }


/// 

    /// * `disk_image` -  (MSFT_DiskImage)
    /// * `return_value` -  (u32)
    pub fn dismount(&self, disk_image: &mut MSFT_DiskImage) -> Result<(), WmiError> {

        let result = self.invoke_method("Dismount", &[])?;
        let disk_image = result.get_value("DiskImage")?;
        Ok(result.return_value)

    }

}

