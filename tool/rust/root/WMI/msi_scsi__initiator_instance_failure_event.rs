// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSiSCSI_InitiatorInstanceFailureEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSiSCSI_InitiatorInstanceFailureEvent {
    #[serde(flatten)]
    pub base: __ExtrinsicEvent,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// **typedef** Type of failure
    #[serde(rename = "FailureType")]
    pub failure_type: Option<InitiatorInstanceFailureEvent_FailureType>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// Name of target involved in failure
    #[serde(rename = "RemoteNodeName")]
    pub remote_node_name: Option<String>,
}

impl MSiSCSI_InitiatorInstanceFailureEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __ExtrinsicEvent::new(),
            active: None,
            failure_type: None,
            instance_name: None,
            remote_node_name: None,
        }
    }


    /// Sets the value of Active
    pub fn set_active(&mut self, value: bool) {
        self.active = Some(value);
    }

    /// Gets the value of Active
    pub fn get_active(&self) -> Option<&bool> {
        self.active.as_ref()
    }

    /// Sets the value of FailureType
    pub fn set_failure_type(&mut self, value: InitiatorInstanceFailureEvent_FailureType) {
        self.failure_type = Some(value);
    }

    /// Gets the value of FailureType
    pub fn get_failure_type(&self) -> Option<&InitiatorInstanceFailureEvent_FailureType> {
        self.failure_type.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of RemoteNodeName
    pub fn set_remote_node_name(&mut self, value: String) {
        self.remote_node_name = Some(value);
    }

    /// Gets the value of RemoteNodeName
    pub fn get_remote_node_name(&self) -> Option<&String> {
        self.remote_node_name.as_ref()
    }
}

