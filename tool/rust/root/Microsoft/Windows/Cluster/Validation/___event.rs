// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Cluster.Validation
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __Event struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __Event {
    #[serde(flatten)]
    pub base: __IndicationRelated,

/// 
    #[serde(rename = "SECURITY_DESCRIPTOR")]
    pub security__descriptor: Vec<u8>,

/// 
    #[serde(rename = "TIME_CREATED")]
    pub time__created: Option<u64>,
}

impl __Event {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __IndicationRelated::new(),
            security__descriptor: Vec::new(),
            time__created: None,
        }
    }


    /// Sets the value of SECURITY_DESCRIPTOR
    pub fn set_security__descriptor(&mut self, value: Vec<u8>) {
        self.security__descriptor = value;
    }

    /// Gets the value of SECURITY_DESCRIPTOR
    pub fn get_security__descriptor(&self) -> &Vec<u8> {
        &self.security__descriptor
    }

    /// Sets the value of TIME_CREATED
    pub fn set_time__created(&mut self, value: u64) {
        self.time__created = Some(value);
    }

    /// Gets the value of TIME_CREATED
    pub fn get_time__created(&self) -> Option<&u64> {
        self.time__created.as_ref()
    }
}

