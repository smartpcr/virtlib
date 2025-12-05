// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_ReceiveQueueInfoArray struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_ReceiveQueueInfoArray {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "ElementSize")]
    pub element_size: Option<u32>,

/// 
    #[serde(rename = "FirstElementOffset")]
    pub first_element_offset: Option<u32>,

/// 
    #[serde(rename = "Header")]
    pub header: Option<MSNdis_ObjectHeader>,

/// 
    #[serde(rename = "NumElements")]
    pub num_elements: Option<u32>,

/// 
    #[serde(rename = "Queue")]
    pub queue: Vec<MSNdis_ReceiveQueueInfo>,
}

impl MSNdis_ReceiveQueueInfoArray {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            element_size: None,
            first_element_offset: None,
            header: None,
            num_elements: None,
            queue: Vec::new(),
        }
    }


    /// Sets the value of ElementSize
    pub fn set_element_size(&mut self, value: u32) {
        self.element_size = Some(value);
    }

    /// Gets the value of ElementSize
    pub fn get_element_size(&self) -> Option<&u32> {
        self.element_size.as_ref()
    }

    /// Sets the value of FirstElementOffset
    pub fn set_first_element_offset(&mut self, value: u32) {
        self.first_element_offset = Some(value);
    }

    /// Gets the value of FirstElementOffset
    pub fn get_first_element_offset(&self) -> Option<&u32> {
        self.first_element_offset.as_ref()
    }

    /// Sets the value of Header
    pub fn set_header(&mut self, value: MSNdis_ObjectHeader) {
        self.header = Some(value);
    }

    /// Gets the value of Header
    pub fn get_header(&self) -> Option<&MSNdis_ObjectHeader> {
        self.header.as_ref()
    }

    /// Sets the value of NumElements
    pub fn set_num_elements(&mut self, value: u32) {
        self.num_elements = Some(value);
    }

    /// Gets the value of NumElements
    pub fn get_num_elements(&self) -> Option<&u32> {
        self.num_elements.as_ref()
    }

    /// Sets the value of Queue
    pub fn set_queue(&mut self, value: Vec<MSNdis_ReceiveQueueInfo>) {
        self.queue = value;
    }

    /// Gets the value of Queue
    pub fn get_queue(&self) -> &Vec<MSNdis_ReceiveQueueInfo> {
        &self.queue
    }
}

