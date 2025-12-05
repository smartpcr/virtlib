// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetAdapter_QosSettings struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetAdapter_QosSettings {

/// 
    #[serde(rename = "BandwidthAssignmentTable")]
    pub bandwidth_assignment_table: Vec<u8>,

/// 
    #[serde(rename = "ClassificationEnabled")]
    pub classification_enabled: Option<bool>,

/// 
    #[serde(rename = "ClassificationTable")]
    pub classification_table: Vec<MSFT_NetAdapter_QosClassificationElement>,

/// 
    #[serde(rename = "FlowControlEnabled")]
    pub flow_control_enabled: Option<bool>,

/// 
    #[serde(rename = "NumberOfClassificationElements")]
    pub number_of_classification_elements: Option<u32>,

/// 
    #[serde(rename = "PriorityAssignmentTable")]
    pub priority_assignment_table: Vec<u8>,

/// 
    #[serde(rename = "PriorityFlowControlEnableArray")]
    pub priority_flow_control_enable_array: Vec<bool>,

/// 
    #[serde(rename = "TransmissionSelectionEnabled")]
    pub transmission_selection_enabled: Option<bool>,

/// 
    #[serde(rename = "TsaAssignmentTable")]
    pub tsa_assignment_table: Vec<u8>,
}

impl MSFT_NetAdapter_QosSettings {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            bandwidth_assignment_table: Vec::new(),
            classification_enabled: None,
            classification_table: Vec::new(),
            flow_control_enabled: None,
            number_of_classification_elements: None,
            priority_assignment_table: Vec::new(),
            priority_flow_control_enable_array: Vec::new(),
            transmission_selection_enabled: None,
            tsa_assignment_table: Vec::new(),
        }
    }


    /// Sets the value of BandwidthAssignmentTable
    pub fn set_bandwidth_assignment_table(&mut self, value: Vec<u8>) {
        self.bandwidth_assignment_table = value;
    }

    /// Gets the value of BandwidthAssignmentTable
    pub fn get_bandwidth_assignment_table(&self) -> &Vec<u8> {
        &self.bandwidth_assignment_table
    }

    /// Sets the value of ClassificationEnabled
    pub fn set_classification_enabled(&mut self, value: bool) {
        self.classification_enabled = Some(value);
    }

    /// Gets the value of ClassificationEnabled
    pub fn get_classification_enabled(&self) -> Option<&bool> {
        self.classification_enabled.as_ref()
    }

    /// Sets the value of ClassificationTable
    pub fn set_classification_table(&mut self, value: Vec<MSFT_NetAdapter_QosClassificationElement>) {
        self.classification_table = value;
    }

    /// Gets the value of ClassificationTable
    pub fn get_classification_table(&self) -> &Vec<MSFT_NetAdapter_QosClassificationElement> {
        &self.classification_table
    }

    /// Sets the value of FlowControlEnabled
    pub fn set_flow_control_enabled(&mut self, value: bool) {
        self.flow_control_enabled = Some(value);
    }

    /// Gets the value of FlowControlEnabled
    pub fn get_flow_control_enabled(&self) -> Option<&bool> {
        self.flow_control_enabled.as_ref()
    }

    /// Sets the value of NumberOfClassificationElements
    pub fn set_number_of_classification_elements(&mut self, value: u32) {
        self.number_of_classification_elements = Some(value);
    }

    /// Gets the value of NumberOfClassificationElements
    pub fn get_number_of_classification_elements(&self) -> Option<&u32> {
        self.number_of_classification_elements.as_ref()
    }

    /// Sets the value of PriorityAssignmentTable
    pub fn set_priority_assignment_table(&mut self, value: Vec<u8>) {
        self.priority_assignment_table = value;
    }

    /// Gets the value of PriorityAssignmentTable
    pub fn get_priority_assignment_table(&self) -> &Vec<u8> {
        &self.priority_assignment_table
    }

    /// Sets the value of PriorityFlowControlEnableArray
    pub fn set_priority_flow_control_enable_array(&mut self, value: Vec<bool>) {
        self.priority_flow_control_enable_array = value;
    }

    /// Gets the value of PriorityFlowControlEnableArray
    pub fn get_priority_flow_control_enable_array(&self) -> &Vec<bool> {
        &self.priority_flow_control_enable_array
    }

    /// Sets the value of TransmissionSelectionEnabled
    pub fn set_transmission_selection_enabled(&mut self, value: bool) {
        self.transmission_selection_enabled = Some(value);
    }

    /// Gets the value of TransmissionSelectionEnabled
    pub fn get_transmission_selection_enabled(&self) -> Option<&bool> {
        self.transmission_selection_enabled.as_ref()
    }

    /// Sets the value of TsaAssignmentTable
    pub fn set_tsa_assignment_table(&mut self, value: Vec<u8>) {
        self.tsa_assignment_table = value;
    }

    /// Gets the value of TsaAssignmentTable
    pub fn get_tsa_assignment_table(&self) -> &Vec<u8> {
        &self.tsa_assignment_table
    }
}

