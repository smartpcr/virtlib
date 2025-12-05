// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ServiceThrottlingBehavior struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServiceThrottlingBehavior {
    #[serde(flatten)]
    pub base: Behavior,

/// The maximum number of messages actively processing across all dispatcher objects in a ServiceHost. 
    #[serde(rename = "MaxConcurrentCalls")]
    pub max_concurrent_calls: Option<i32>,

/// The maximum number of service objects that can execute at one time.
    #[serde(rename = "MaxConcurrentInstances")]
    pub max_concurrent_instances: Option<i32>,

/// The maximum number of sessions a host can accept at one time.
    #[serde(rename = "MaxConcurrentSessions")]
    pub max_concurrent_sessions: Option<i32>,
}

impl ServiceThrottlingBehavior {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Behavior::new(),
            max_concurrent_calls: None,
            max_concurrent_instances: None,
            max_concurrent_sessions: None,
        }
    }


    /// Sets the value of MaxConcurrentCalls
    pub fn set_max_concurrent_calls(&mut self, value: i32) {
        self.max_concurrent_calls = Some(value);
    }

    /// Gets the value of MaxConcurrentCalls
    pub fn get_max_concurrent_calls(&self) -> Option<&i32> {
        self.max_concurrent_calls.as_ref()
    }

    /// Sets the value of MaxConcurrentInstances
    pub fn set_max_concurrent_instances(&mut self, value: i32) {
        self.max_concurrent_instances = Some(value);
    }

    /// Gets the value of MaxConcurrentInstances
    pub fn get_max_concurrent_instances(&self) -> Option<&i32> {
        self.max_concurrent_instances.as_ref()
    }

    /// Sets the value of MaxConcurrentSessions
    pub fn set_max_concurrent_sessions(&mut self, value: i32) {
        self.max_concurrent_sessions = Some(value);
    }

    /// Gets the value of MaxConcurrentSessions
    pub fn get_max_concurrent_sessions(&self) -> Option<&i32> {
        self.max_concurrent_sessions.as_ref()
    }
}

