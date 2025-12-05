// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageDiagnoseResult struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageDiagnoseResult {

/// 
    #[serde(rename = "FaultId")]
    pub fault_id: Option<String>,

/// 
    #[serde(rename = "FaultingObjectDescription")]
    pub faulting_object_description: Option<String>,

/// 
    #[serde(rename = "FaultingObjectLocation")]
    pub faulting_object_location: Option<String>,

/// 
    #[serde(rename = "FaultingObjectType")]
    pub faulting_object_type: Option<String>,

/// 
    #[serde(rename = "FaultingObjectUniqueId")]
    pub faulting_object_unique_id: Option<String>,

/// 
    #[serde(rename = "FaultTime")]
    pub fault_time: Option<String>,

/// 
    #[serde(rename = "FaultType")]
    pub fault_type: Option<String>,

/// 
    #[serde(rename = "PerceivedSeverity")]
    pub perceived_severity: Option<u16>,

/// 
    #[serde(rename = "Reason")]
    pub reason: Option<String>,

/// 
    #[serde(rename = "RecommendedActions")]
    pub recommended_actions: Vec<String>,
}

impl MSFT_StorageDiagnoseResult {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            fault_id: None,
            faulting_object_description: None,
            faulting_object_location: None,
            faulting_object_type: None,
            faulting_object_unique_id: None,
            fault_time: None,
            fault_type: None,
            perceived_severity: None,
            reason: None,
            recommended_actions: Vec::new(),
        }
    }


    /// Sets the value of FaultId
    pub fn set_fault_id(&mut self, value: String) {
        self.fault_id = Some(value);
    }

    /// Gets the value of FaultId
    pub fn get_fault_id(&self) -> Option<&String> {
        self.fault_id.as_ref()
    }

    /// Sets the value of FaultingObjectDescription
    pub fn set_faulting_object_description(&mut self, value: String) {
        self.faulting_object_description = Some(value);
    }

    /// Gets the value of FaultingObjectDescription
    pub fn get_faulting_object_description(&self) -> Option<&String> {
        self.faulting_object_description.as_ref()
    }

    /// Sets the value of FaultingObjectLocation
    pub fn set_faulting_object_location(&mut self, value: String) {
        self.faulting_object_location = Some(value);
    }

    /// Gets the value of FaultingObjectLocation
    pub fn get_faulting_object_location(&self) -> Option<&String> {
        self.faulting_object_location.as_ref()
    }

    /// Sets the value of FaultingObjectType
    pub fn set_faulting_object_type(&mut self, value: String) {
        self.faulting_object_type = Some(value);
    }

    /// Gets the value of FaultingObjectType
    pub fn get_faulting_object_type(&self) -> Option<&String> {
        self.faulting_object_type.as_ref()
    }

    /// Sets the value of FaultingObjectUniqueId
    pub fn set_faulting_object_unique_id(&mut self, value: String) {
        self.faulting_object_unique_id = Some(value);
    }

    /// Gets the value of FaultingObjectUniqueId
    pub fn get_faulting_object_unique_id(&self) -> Option<&String> {
        self.faulting_object_unique_id.as_ref()
    }

    /// Sets the value of FaultTime
    pub fn set_fault_time(&mut self, value: String) {
        self.fault_time = Some(value);
    }

    /// Gets the value of FaultTime
    pub fn get_fault_time(&self) -> Option<&String> {
        self.fault_time.as_ref()
    }

    /// Sets the value of FaultType
    pub fn set_fault_type(&mut self, value: String) {
        self.fault_type = Some(value);
    }

    /// Gets the value of FaultType
    pub fn get_fault_type(&self) -> Option<&String> {
        self.fault_type.as_ref()
    }

    /// Sets the value of PerceivedSeverity
    pub fn set_perceived_severity(&mut self, value: u16) {
        self.perceived_severity = Some(value);
    }

    /// Gets the value of PerceivedSeverity
    pub fn get_perceived_severity(&self) -> Option<&u16> {
        self.perceived_severity.as_ref()
    }

    /// Sets the value of Reason
    pub fn set_reason(&mut self, value: String) {
        self.reason = Some(value);
    }

    /// Gets the value of Reason
    pub fn get_reason(&self) -> Option<&String> {
        self.reason.as_ref()
    }

    /// Sets the value of RecommendedActions
    pub fn set_recommended_actions(&mut self, value: Vec<String>) {
        self.recommended_actions = value;
    }

    /// Gets the value of RecommendedActions
    pub fn get_recommended_actions(&self) -> &Vec<String> {
        &self.recommended_actions
    }
}

