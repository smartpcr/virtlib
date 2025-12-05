// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// DiskIo_TypeGroup1 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DiskIo_TypeGroup1 {
    #[serde(flatten)]
    pub base: DiskIo,

/// 
    #[serde(rename = "ByteOffset")]
    pub byte_offset: Option<u64>,

/// 
    #[serde(rename = "DiskNumber")]
    pub disk_number: Option<u32>,

/// 
    #[serde(rename = "FileObject")]
    pub file_object: Option<u32>,

/// 
    #[serde(rename = "HighResResponseTime")]
    pub high_res_response_time: Option<u64>,

/// 
    #[serde(rename = "Irp")]
    pub irp: Option<u32>,

/// 
    #[serde(rename = "IrpFlags")]
    pub irp_flags: Option<u32>,

/// 
    #[serde(rename = "IssuingThreadId")]
    pub issuing_thread_id: Option<u32>,

/// 
    #[serde(rename = "Reserved")]
    pub reserved: Option<u32>,

/// 
    #[serde(rename = "TransferSize")]
    pub transfer_size: Option<u32>,
}

impl DiskIo_TypeGroup1 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: DiskIo::new(),
            byte_offset: None,
            disk_number: None,
            file_object: None,
            high_res_response_time: None,
            irp: None,
            irp_flags: None,
            issuing_thread_id: None,
            reserved: None,
            transfer_size: None,
        }
    }


    /// Sets the value of ByteOffset
    pub fn set_byte_offset(&mut self, value: u64) {
        self.byte_offset = Some(value);
    }

    /// Gets the value of ByteOffset
    pub fn get_byte_offset(&self) -> Option<&u64> {
        self.byte_offset.as_ref()
    }

    /// Sets the value of DiskNumber
    pub fn set_disk_number(&mut self, value: u32) {
        self.disk_number = Some(value);
    }

    /// Gets the value of DiskNumber
    pub fn get_disk_number(&self) -> Option<&u32> {
        self.disk_number.as_ref()
    }

    /// Sets the value of FileObject
    pub fn set_file_object(&mut self, value: u32) {
        self.file_object = Some(value);
    }

    /// Gets the value of FileObject
    pub fn get_file_object(&self) -> Option<&u32> {
        self.file_object.as_ref()
    }

    /// Sets the value of HighResResponseTime
    pub fn set_high_res_response_time(&mut self, value: u64) {
        self.high_res_response_time = Some(value);
    }

    /// Gets the value of HighResResponseTime
    pub fn get_high_res_response_time(&self) -> Option<&u64> {
        self.high_res_response_time.as_ref()
    }

    /// Sets the value of Irp
    pub fn set_irp(&mut self, value: u32) {
        self.irp = Some(value);
    }

    /// Gets the value of Irp
    pub fn get_irp(&self) -> Option<&u32> {
        self.irp.as_ref()
    }

    /// Sets the value of IrpFlags
    pub fn set_irp_flags(&mut self, value: u32) {
        self.irp_flags = Some(value);
    }

    /// Gets the value of IrpFlags
    pub fn get_irp_flags(&self) -> Option<&u32> {
        self.irp_flags.as_ref()
    }

    /// Sets the value of IssuingThreadId
    pub fn set_issuing_thread_id(&mut self, value: u32) {
        self.issuing_thread_id = Some(value);
    }

    /// Gets the value of IssuingThreadId
    pub fn get_issuing_thread_id(&self) -> Option<&u32> {
        self.issuing_thread_id.as_ref()
    }

    /// Sets the value of Reserved
    pub fn set_reserved(&mut self, value: u32) {
        self.reserved = Some(value);
    }

    /// Gets the value of Reserved
    pub fn get_reserved(&self) -> Option<&u32> {
        self.reserved.as_ref()
    }

    /// Sets the value of TransferSize
    pub fn set_transfer_size(&mut self, value: u32) {
        self.transfer_size = Some(value);
    }

    /// Gets the value of TransferSize
    pub fn get_transfer_size(&self) -> Option<&u32> {
        self.transfer_size.as_ref()
    }
}

