// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_FileSpecification struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_FileSpecification {
    #[serde(flatten)]
    pub base: CIM_Check,

/// 
    #[serde(rename = "CheckSum")]
    pub check_sum: Option<u32>,

/// 
    #[serde(rename = "CRC1")]
    pub crc1: Option<u32>,

/// 
    #[serde(rename = "CRC2")]
    pub crc2: Option<u32>,

/// 
    #[serde(rename = "CreateTimeStamp")]
    pub create_time_stamp: Option<String>,

/// 
    #[serde(rename = "FileSize")]
    pub file_size: Option<u64>,

/// 
    #[serde(rename = "MD5Checksum")]
    pub md5_checksum: Option<String>,
}

impl CIM_FileSpecification {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Check::new(),
            check_sum: None,
            crc1: None,
            crc2: None,
            create_time_stamp: None,
            file_size: None,
            md5_checksum: None,
        }
    }


    /// Sets the value of CheckSum
    pub fn set_check_sum(&mut self, value: u32) {
        self.check_sum = Some(value);
    }

    /// Gets the value of CheckSum
    pub fn get_check_sum(&self) -> Option<&u32> {
        self.check_sum.as_ref()
    }

    /// Sets the value of CRC1
    pub fn set_crc1(&mut self, value: u32) {
        self.crc1 = Some(value);
    }

    /// Gets the value of CRC1
    pub fn get_crc1(&self) -> Option<&u32> {
        self.crc1.as_ref()
    }

    /// Sets the value of CRC2
    pub fn set_crc2(&mut self, value: u32) {
        self.crc2 = Some(value);
    }

    /// Gets the value of CRC2
    pub fn get_crc2(&self) -> Option<&u32> {
        self.crc2.as_ref()
    }

    /// Sets the value of CreateTimeStamp
    pub fn set_create_time_stamp(&mut self, value: String) {
        self.create_time_stamp = Some(value);
    }

    /// Gets the value of CreateTimeStamp
    pub fn get_create_time_stamp(&self) -> Option<&String> {
        self.create_time_stamp.as_ref()
    }

    /// Sets the value of FileSize
    pub fn set_file_size(&mut self, value: u64) {
        self.file_size = Some(value);
    }

    /// Gets the value of FileSize
    pub fn get_file_size(&self) -> Option<&u64> {
        self.file_size.as_ref()
    }

    /// Sets the value of MD5Checksum
    pub fn set_md5_checksum(&mut self, value: String) {
        self.md5_checksum = Some(value);
    }

    /// Gets the value of MD5Checksum
    pub fn get_md5_checksum(&self) -> Option<&String> {
        self.md5_checksum.as_ref()
    }
}

