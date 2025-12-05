// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.TaskScheduler
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_TaskPrincipal2 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_TaskPrincipal2 {
    #[serde(flatten)]
    pub base: MSFT_TaskPrincipal,

/// 
    #[serde(rename = "ProcessTokenSidType")]
    pub process_token_sid_type: Option<TaskPrincipal2_ProcessTokenSidType>,

/// 
    #[serde(rename = "RequiredPrivilege")]
    pub required_privilege: Vec<String>,
}

impl MSFT_TaskPrincipal2 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_TaskPrincipal::new(),
            process_token_sid_type: None,
            required_privilege: Vec::new(),
        }
    }


    /// Sets the value of ProcessTokenSidType
    pub fn set_process_token_sid_type(&mut self, value: TaskPrincipal2_ProcessTokenSidType) {
        self.process_token_sid_type = Some(value);
    }

    /// Gets the value of ProcessTokenSidType
    pub fn get_process_token_sid_type(&self) -> Option<&TaskPrincipal2_ProcessTokenSidType> {
        self.process_token_sid_type.as_ref()
    }

    /// Sets the value of RequiredPrivilege
    pub fn set_required_privilege(&mut self, value: Vec<String>) {
        self.required_privilege = value;
    }

    /// Gets the value of RequiredPrivilege
    pub fn get_required_privilege(&self) -> &Vec<String> {
        &self.required_privilege
    }
}

