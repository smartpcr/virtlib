// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.TerminalServices
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_RemoteAppChangeEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_RemoteAppChangeEvent {
    #[serde(flatten)]
    pub base: __ExtrinsicEvent,

/// Type of operation corresponding to the event
    #[serde(rename = "OperationType")]
    pub operation_type: Option<RemoteAppChangeEvent_OperationType>,

/// Object changed by the operation corresponding to the event
    #[serde(rename = "TargetInstance")]
    pub target_instance: Option<serde_json::Value>,
}

impl Win32_RemoteAppChangeEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __ExtrinsicEvent::new(),
            operation_type: None,
            target_instance: None,
        }
    }


    /// Sets the value of OperationType
    pub fn set_operation_type(&mut self, value: RemoteAppChangeEvent_OperationType) {
        self.operation_type = Some(value);
    }

    /// Gets the value of OperationType
    pub fn get_operation_type(&self) -> Option<&RemoteAppChangeEvent_OperationType> {
        self.operation_type.as_ref()
    }

    /// Sets the value of TargetInstance
    pub fn set_target_instance(&mut self, value: serde_json::Value) {
        self.target_instance = Some(value);
    }

    /// Gets the value of TargetInstance
    pub fn get_target_instance(&self) -> Option<&serde_json::Value> {
        self.target_instance.as_ref()
    }
}

