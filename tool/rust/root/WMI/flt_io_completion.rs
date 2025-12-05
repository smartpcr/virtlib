// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// FltIoCompletion struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FltIoCompletion {
    #[serde(flatten)]
    pub base: FileIo,

/// 
    #[serde(rename = "CallbackDataPtr")]
    pub callback_data_ptr: Option<u32>,

/// 
    #[serde(rename = "FileContext")]
    pub file_context: Option<u32>,

/// 
    #[serde(rename = "FileObject")]
    pub file_object: Option<u32>,

/// 
    #[serde(rename = "InitialTime")]
    pub initial_time: Option<serde_json::Value>,

/// 
    #[serde(rename = "IrpPtr")]
    pub irp_ptr: Option<u32>,

/// 
    #[serde(rename = "MajorFunction")]
    pub major_function: Option<u32>,

/// 
    #[serde(rename = "RoutineAddr")]
    pub routine_addr: Option<u32>,
}

impl FltIoCompletion {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: FileIo::new(),
            callback_data_ptr: None,
            file_context: None,
            file_object: None,
            initial_time: None,
            irp_ptr: None,
            major_function: None,
            routine_addr: None,
        }
    }


    /// Sets the value of CallbackDataPtr
    pub fn set_callback_data_ptr(&mut self, value: u32) {
        self.callback_data_ptr = Some(value);
    }

    /// Gets the value of CallbackDataPtr
    pub fn get_callback_data_ptr(&self) -> Option<&u32> {
        self.callback_data_ptr.as_ref()
    }

    /// Sets the value of FileContext
    pub fn set_file_context(&mut self, value: u32) {
        self.file_context = Some(value);
    }

    /// Gets the value of FileContext
    pub fn get_file_context(&self) -> Option<&u32> {
        self.file_context.as_ref()
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

    /// Sets the value of IrpPtr
    pub fn set_irp_ptr(&mut self, value: u32) {
        self.irp_ptr = Some(value);
    }

    /// Gets the value of IrpPtr
    pub fn get_irp_ptr(&self) -> Option<&u32> {
        self.irp_ptr.as_ref()
    }

    /// Sets the value of MajorFunction
    pub fn set_major_function(&mut self, value: u32) {
        self.major_function = Some(value);
    }

    /// Gets the value of MajorFunction
    pub fn get_major_function(&self) -> Option<&u32> {
        self.major_function.as_ref()
    }

    /// Sets the value of RoutineAddr
    pub fn set_routine_addr(&mut self, value: u32) {
        self.routine_addr = Some(value);
    }

    /// Gets the value of RoutineAddr
    pub fn get_routine_addr(&self) -> Option<&u32> {
        self.routine_addr.as_ref()
    }
}

