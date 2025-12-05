// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSiSCSI_InitiatorNodeFailureEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSiSCSI_InitiatorNodeFailureEvent {
    #[serde(flatten)]
    pub base: __ExtrinsicEvent,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// Timestamp denoting time failure occured
    #[serde(rename = "FailureTime")]
    pub failure_time: Option<u64>,

/// **typedef** Types of initiator node failure
    #[serde(rename = "FailureType")]
    pub failure_type: Option<InitiatorNodeFailureEvent_FailureType>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// Network address of target involved in failure
    #[serde(rename = "TargetFailureAddr")]
    pub target_failure_addr: Option<ISCSI_IP_Address>,

/// Name of target involved in failure
    #[serde(rename = "TargetFailureName")]
    pub target_failure_name: Option<String>,
}

impl MSiSCSI_InitiatorNodeFailureEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __ExtrinsicEvent::new(),
            active: None,
            failure_time: None,
            failure_type: None,
            instance_name: None,
            target_failure_addr: None,
            target_failure_name: None,
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

    /// Sets the value of FailureTime
    pub fn set_failure_time(&mut self, value: u64) {
        self.failure_time = Some(value);
    }

    /// Gets the value of FailureTime
    pub fn get_failure_time(&self) -> Option<&u64> {
        self.failure_time.as_ref()
    }

    /// Sets the value of FailureType
    pub fn set_failure_type(&mut self, value: InitiatorNodeFailureEvent_FailureType) {
        self.failure_type = Some(value);
    }

    /// Gets the value of FailureType
    pub fn get_failure_type(&self) -> Option<&InitiatorNodeFailureEvent_FailureType> {
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

    /// Sets the value of TargetFailureAddr
    pub fn set_target_failure_addr(&mut self, value: ISCSI_IP_Address) {
        self.target_failure_addr = Some(value);
    }

    /// Gets the value of TargetFailureAddr
    pub fn get_target_failure_addr(&self) -> Option<&ISCSI_IP_Address> {
        self.target_failure_addr.as_ref()
    }

    /// Sets the value of TargetFailureName
    pub fn set_target_failure_name(&mut self, value: String) {
        self.target_failure_name = Some(value);
    }

    /// Gets the value of TargetFailureName
    pub fn get_target_failure_name(&self) -> Option<&String> {
        self.target_failure_name.as_ref()
    }
}

