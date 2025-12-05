// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// PageFault_HardFault struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PageFault_HardFault {
    #[serde(flatten)]
    pub base: PageFault_V2,

/// 
    #[serde(rename = "ByteCount")]
    pub byte_count: Option<u32>,

/// 
    #[serde(rename = "FileObject")]
    pub file_object: Option<u32>,

/// 
    #[serde(rename = "InitialTime")]
    pub initial_time: Option<serde_json::Value>,

/// 
    #[serde(rename = "ReadOffset")]
    pub read_offset: Option<u64>,

/// 
    #[serde(rename = "TThreadId")]
    pub tthread_id: Option<u32>,

/// 
    #[serde(rename = "VirtualAddress")]
    pub virtual_address: Option<u32>,
}

impl PageFault_HardFault {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: PageFault_V2::new(),
            byte_count: None,
            file_object: None,
            initial_time: None,
            read_offset: None,
            tthread_id: None,
            virtual_address: None,
        }
    }


    /// Sets the value of ByteCount
    pub fn set_byte_count(&mut self, value: u32) {
        self.byte_count = Some(value);
    }

    /// Gets the value of ByteCount
    pub fn get_byte_count(&self) -> Option<&u32> {
        self.byte_count.as_ref()
    }

    /// Sets the value of FileObject
    pub fn set_file_object(&mut self, value: u32) {
        self.file_object = Some(value);
    }

    /// Gets the value of FileObject
    pub fn get_file_object(&self) -> Option<&u32> {
        self.file_object.as_ref()
    }

    /// Sets the value of InitialTime
    pub fn set_initial_time(&mut self, value: serde_json::Value) {
        self.initial_time = Some(value);
    }

    /// Gets the value of InitialTime
    pub fn get_initial_time(&self) -> Option<&serde_json::Value> {
        self.initial_time.as_ref()
    }

    /// Sets the value of ReadOffset
    pub fn set_read_offset(&mut self, value: u64) {
        self.read_offset = Some(value);
    }

    /// Gets the value of ReadOffset
    pub fn get_read_offset(&self) -> Option<&u64> {
        self.read_offset.as_ref()
    }

    /// Sets the value of TThreadId
    pub fn set_tthread_id(&mut self, value: u32) {
        self.tthread_id = Some(value);
    }

    /// Gets the value of TThreadId
    pub fn get_tthread_id(&self) -> Option<&u32> {
        self.tthread_id.as_ref()
    }

    /// Sets the value of VirtualAddress
    pub fn set_virtual_address(&mut self, value: u32) {
        self.virtual_address = Some(value);
    }

    /// Gets the value of VirtualAddress
    pub fn get_virtual_address(&self) -> Option<&u32> {
        self.virtual_address.as_ref()
    }
}

