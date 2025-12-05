// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// TcpIp_Fail struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TcpIp_Fail {
    #[serde(flatten)]
    pub base: TcpIp,

/// 
    #[serde(rename = "FailureCode")]
    pub failure_code: Option<u16>,

/// 
    #[serde(rename = "Proto")]
    pub proto: Option<u16>,
}

impl TcpIp_Fail {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: TcpIp::new(),
            failure_code: None,
            proto: None,
        }
    }


    /// Sets the value of FailureCode
    pub fn set_failure_code(&mut self, value: u16) {
        self.failure_code = Some(value);
    }

    /// Gets the value of FailureCode
    pub fn get_failure_code(&self) -> Option<&u16> {
        self.failure_code.as_ref()
    }

    /// Sets the value of Proto
    pub fn set_proto(&mut self, value: u16) {
        self.proto = Some(value);
    }

    /// Gets the value of Proto
    pub fn get_proto(&self) -> Option<&u16> {
        self.proto.as_ref()
    }
}

