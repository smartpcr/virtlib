// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageFaultEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageFaultEvent {
    #[serde(flatten)]
    pub base: MSFT_StorageEvent,

/// 
    #[serde(rename = "ChangeType")]
    pub change_type: Option<u16>,

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
    #[serde(rename = "FaultType")]
    pub fault_type: Option<String>,

/// 
    #[serde(rename = "Reason")]
    pub reason: Option<String>,

/// 
    #[serde(rename = "RecommendedActions")]
    pub recommended_actions: Vec<String>,

/// 
    #[serde(rename = "SourceUniqueId")]
    pub source_unique_id: Option<String>,

/// 
    #[serde(rename = "StorageSubsystemUniqueId")]
    pub storage_subsystem_unique_id: Option<String>,
}

impl MSFT_StorageFaultEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_StorageEvent::new(),
            change_type: None,
            fault_id: None,
            faulting_object_description: None,
            faulting_object_location: None,
            faulting_object_type: None,
            faulting_object_unique_id: None,
            fault_type: None,
            reason: None,
            recommended_actions: Vec::new(),
            source_unique_id: None,
            storage_subsystem_unique_id: None,
        }
    }


    /// Sets the value of ChangeType
    pub fn set_change_type(&mut self, value: u16) {
        self.change_type = Some(value);
    }

    /// Gets the value of ChangeType
    pub fn get_change_type(&self) -> Option<&u16> {
        self.change_type.as_ref()
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

    /// Sets the value of FaultType
    pub fn set_fault_type(&mut self, value: String) {
        self.fault_type = Some(value);
    }

    /// Gets the value of FaultType
    pub fn get_fault_type(&self) -> Option<&String> {
        self.fault_type.as_ref()
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

    /// Sets the value of SourceUniqueId
    pub fn set_source_unique_id(&mut self, value: String) {
        self.source_unique_id = Some(value);
    }

    /// Gets the value of SourceUniqueId
    pub fn get_source_unique_id(&self) -> Option<&String> {
        self.source_unique_id.as_ref()
    }

    /// Sets the value of StorageSubsystemUniqueId
    pub fn set_storage_subsystem_unique_id(&mut self, value: String) {
        self.storage_subsystem_unique_id = Some(value);
    }

    /// Gets the value of StorageSubsystemUniqueId
    pub fn get_storage_subsystem_unique_id(&self) -> Option<&String> {
        self.storage_subsystem_unique_id.as_ref()
    }
}

