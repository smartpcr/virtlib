// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_ReceiveFilterParameters struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_ReceiveFilterParameters {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "FieldParameters")]
    pub field_parameters: Vec<MSNdis_ReceiveFilterFieldParameters>,

/// 
    #[serde(rename = "FieldParametersArrayElementSize")]
    pub field_parameters_array_element_size: Option<u32>,

/// 
    #[serde(rename = "FieldParametersArrayNumElements")]
    pub field_parameters_array_num_elements: Option<u32>,

/// 
    #[serde(rename = "FieldParametersArrayOffset")]
    pub field_parameters_array_offset: Option<u32>,

/// 
    #[serde(rename = "FilterId")]
    pub filter_id: Option<u32>,

/// 
    #[serde(rename = "FilterType")]
    pub filter_type: Option<u32>,

/// 
    #[serde(rename = "Flags")]
    pub flags: Option<u32>,

/// 
    #[serde(rename = "Header")]
    pub header: Option<MSNdis_ObjectHeader>,

/// 
    #[serde(rename = "QueueId")]
    pub queue_id: Option<u32>,

/// 
    #[serde(rename = "RequestedFilterIdBitCount")]
    pub requested_filter_id_bit_count: Option<u32>,
}

impl MSNdis_ReceiveFilterParameters {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            field_parameters: Vec::new(),
            field_parameters_array_element_size: None,
            field_parameters_array_num_elements: None,
            field_parameters_array_offset: None,
            filter_id: None,
            filter_type: None,
            flags: None,
            header: None,
            queue_id: None,
            requested_filter_id_bit_count: None,
        }
    }


    /// Sets the value of FieldParameters
    pub fn set_field_parameters(&mut self, value: Vec<MSNdis_ReceiveFilterFieldParameters>) {
        self.field_parameters = value;
    }

    /// Gets the value of FieldParameters
    pub fn get_field_parameters(&self) -> &Vec<MSNdis_ReceiveFilterFieldParameters> {
        &self.field_parameters
    }

    /// Sets the value of FieldParametersArrayElementSize
    pub fn set_field_parameters_array_element_size(&mut self, value: u32) {
        self.field_parameters_array_element_size = Some(value);
    }

    /// Gets the value of FieldParametersArrayElementSize
    pub fn get_field_parameters_array_element_size(&self) -> Option<&u32> {
        self.field_parameters_array_element_size.as_ref()
    }

    /// Sets the value of FieldParametersArrayNumElements
    pub fn set_field_parameters_array_num_elements(&mut self, value: u32) {
        self.field_parameters_array_num_elements = Some(value);
    }

    /// Gets the value of FieldParametersArrayNumElements
    pub fn get_field_parameters_array_num_elements(&self) -> Option<&u32> {
        self.field_parameters_array_num_elements.as_ref()
    }

    /// Sets the value of FieldParametersArrayOffset
    pub fn set_field_parameters_array_offset(&mut self, value: u32) {
        self.field_parameters_array_offset = Some(value);
    }

    /// Gets the value of FieldParametersArrayOffset
    pub fn get_field_parameters_array_offset(&self) -> Option<&u32> {
        self.field_parameters_array_offset.as_ref()
    }

    /// Sets the value of FilterId
    pub fn set_filter_id(&mut self, value: u32) {
        self.filter_id = Some(value);
    }

    /// Gets the value of FilterId
    pub fn get_filter_id(&self) -> Option<&u32> {
        self.filter_id.as_ref()
    }

    /// Sets the value of FilterType
    pub fn set_filter_type(&mut self, value: u32) {
        self.filter_type = Some(value);
    }

    /// Gets the value of FilterType
    pub fn get_filter_type(&self) -> Option<&u32> {
        self.filter_type.as_ref()
    }

    /// Sets the value of Flags
    pub fn set_flags(&mut self, value: u32) {
        self.flags = Some(value);
    }

    /// Gets the value of Flags
    pub fn get_flags(&self) -> Option<&u32> {
        self.flags.as_ref()
    }

    /// Sets the value of Header
    pub fn set_header(&mut self, value: MSNdis_ObjectHeader) {
        self.header = Some(value);
    }

    /// Gets the value of Header
    pub fn get_header(&self) -> Option<&MSNdis_ObjectHeader> {
        self.header.as_ref()
    }

    /// Sets the value of QueueId
    pub fn set_queue_id(&mut self, value: u32) {
        self.queue_id = Some(value);
    }

    /// Gets the value of QueueId
    pub fn get_queue_id(&self) -> Option<&u32> {
        self.queue_id.as_ref()
    }

    /// Sets the value of RequestedFilterIdBitCount
    pub fn set_requested_filter_id_bit_count(&mut self, value: u32) {
        self.requested_filter_id_bit_count = Some(value);
    }

    /// Gets the value of RequestedFilterIdBitCount
    pub fn get_requested_filter_id_bit_count(&self) -> Option<&u32> {
        self.requested_filter_id_bit_count.as_ref()
    }
}

