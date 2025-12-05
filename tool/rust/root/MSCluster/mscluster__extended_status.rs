// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.MSCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSCluster_ExtendedStatus struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSCluster_ExtendedStatus {
    #[serde(flatten)]
    pub base: __ExtendedStatus,

/// 
    #[serde(rename = "ErrorType")]
    pub error_type: Option<u32>,
}

impl MSCluster_ExtendedStatus {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __ExtendedStatus::new(),
            error_type: None,
        }
    }


    /// Sets the value of ErrorType
    pub fn set_error_type(&mut self, value: u32) {
        self.error_type = Some(value);
    }

    /// Gets the value of ErrorType
    pub fn get_error_type(&self) -> Option<&u32> {
        self.error_type.as_ref()
    }
}

