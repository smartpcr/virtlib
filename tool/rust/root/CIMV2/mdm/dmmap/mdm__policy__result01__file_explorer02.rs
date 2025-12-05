// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Result01_FileExplorer02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Result01_FileExplorer02 {

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "TurnOffDataExecutionPreventionForExplorer")]
    pub turn_off_data_execution_prevention_for_explorer: Option<String>,

/// 
    #[serde(rename = "TurnOffHeapTerminationOnCorruption")]
    pub turn_off_heap_termination_on_corruption: Option<String>,
}

impl MDM_Policy_Result01_FileExplorer02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            instance_id: None,
            parent_id: None,
            turn_off_data_execution_prevention_for_explorer: None,
            turn_off_heap_termination_on_corruption: None,
        }
    }


    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of TurnOffDataExecutionPreventionForExplorer
    pub fn set_turn_off_data_execution_prevention_for_explorer(&mut self, value: String) {
        self.turn_off_data_execution_prevention_for_explorer = Some(value);
    }

    /// Gets the value of TurnOffDataExecutionPreventionForExplorer
    pub fn get_turn_off_data_execution_prevention_for_explorer(&self) -> Option<&String> {
        self.turn_off_data_execution_prevention_for_explorer.as_ref()
    }

    /// Sets the value of TurnOffHeapTerminationOnCorruption
    pub fn set_turn_off_heap_termination_on_corruption(&mut self, value: String) {
        self.turn_off_heap_termination_on_corruption = Some(value);
    }

    /// Gets the value of TurnOffHeapTerminationOnCorruption
    pub fn get_turn_off_heap_termination_on_corruption(&self) -> Option<&String> {
        self.turn_off_heap_termination_on_corruption.as_ref()
    }
}

