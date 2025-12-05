// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// WorkflowRuntimeBehavior struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WorkflowRuntimeBehavior {
    #[serde(flatten)]
    pub base: Behavior,

/// Specifies the interval after which idle workflow instances in memory are terminated.
    #[serde(rename = "CachedInstanceExpiration")]
    pub cached_instance_expiration: Option<String>,
}

impl WorkflowRuntimeBehavior {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Behavior::new(),
            cached_instance_expiration: None,
        }
    }


    /// Sets the value of CachedInstanceExpiration
    pub fn set_cached_instance_expiration(&mut self, value: String) {
        self.cached_instance_expiration = Some(value);
    }

    /// Gets the value of CachedInstanceExpiration
    pub fn get_cached_instance_expiration(&self) -> Option<&String> {
        self.cached_instance_expiration.as_ref()
    }
}

