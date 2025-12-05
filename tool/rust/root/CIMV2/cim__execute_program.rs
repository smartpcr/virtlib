// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ExecuteProgram struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ExecuteProgram {
    #[serde(flatten)]
    pub base: CIM_Action,

/// 
    #[serde(rename = "CommandLine")]
    pub command_line: Option<String>,

/// 
    #[serde(rename = "ProgramPath")]
    pub program_path: Option<String>,
}

impl CIM_ExecuteProgram {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Action::new(),
            command_line: None,
            program_path: None,
        }
    }


    /// Sets the value of CommandLine
    pub fn set_command_line(&mut self, value: String) {
        self.command_line = Some(value);
    }

    /// Gets the value of CommandLine
    pub fn get_command_line(&self) -> Option<&String> {
        self.command_line.as_ref()
    }

    /// Sets the value of ProgramPath
    pub fn set_program_path(&mut self, value: String) {
        self.program_path = Some(value);
    }

    /// Gets the value of ProgramPath
    pub fn get_program_path(&self) -> Option<&String> {
        self.program_path.as_ref()
    }
}

