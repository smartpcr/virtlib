// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.TerminalServices
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_TSEnvironmentSetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_TSEnvironmentSetting {
    #[serde(flatten)]
    pub base: Win32_TerminalSetting,

/// 
    #[serde(rename = "ClientWallPaper")]
    pub client_wall_paper: Option<u32>,

/// 
    #[serde(rename = "InitialProgramPath")]
    pub initial_program_path: Option<String>,

/// 
    #[serde(rename = "InitialProgramPolicy")]
    pub initial_program_policy: Option<u32>,

/// 
    #[serde(rename = "PolicySourceClientWallPaper")]
    pub policy_source_client_wall_paper: Option<u32>,

/// 
    #[serde(rename = "PolicySourceInitialProgramPath")]
    pub policy_source_initial_program_path: Option<u32>,

/// 
    #[serde(rename = "PolicySourceStartIn")]
    pub policy_source_start_in: Option<u32>,

/// 
    #[serde(rename = "Startin")]
    pub startin: Option<String>,
}

impl Win32_TSEnvironmentSetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_TerminalSetting::new(),
            client_wall_paper: None,
            initial_program_path: None,
            initial_program_policy: None,
            policy_source_client_wall_paper: None,
            policy_source_initial_program_path: None,
            policy_source_start_in: None,
            startin: None,
        }
    }


    /// Sets the value of ClientWallPaper
    pub fn set_client_wall_paper(&mut self, value: u32) {
        self.client_wall_paper = Some(value);
    }

    /// Gets the value of ClientWallPaper
    pub fn get_client_wall_paper(&self) -> Option<&u32> {
        self.client_wall_paper.as_ref()
    }

    /// Sets the value of InitialProgramPath
    pub fn set_initial_program_path(&mut self, value: String) {
        self.initial_program_path = Some(value);
    }

    /// Gets the value of InitialProgramPath
    pub fn get_initial_program_path(&self) -> Option<&String> {
        self.initial_program_path.as_ref()
    }

    /// Sets the value of InitialProgramPolicy
    pub fn set_initial_program_policy(&mut self, value: u32) {
        self.initial_program_policy = Some(value);
    }

    /// Gets the value of InitialProgramPolicy
    pub fn get_initial_program_policy(&self) -> Option<&u32> {
        self.initial_program_policy.as_ref()
    }

    /// Sets the value of PolicySourceClientWallPaper
    pub fn set_policy_source_client_wall_paper(&mut self, value: u32) {
        self.policy_source_client_wall_paper = Some(value);
    }

    /// Gets the value of PolicySourceClientWallPaper
    pub fn get_policy_source_client_wall_paper(&self) -> Option<&u32> {
        self.policy_source_client_wall_paper.as_ref()
    }

    /// Sets the value of PolicySourceInitialProgramPath
    pub fn set_policy_source_initial_program_path(&mut self, value: u32) {
        self.policy_source_initial_program_path = Some(value);
    }

    /// Gets the value of PolicySourceInitialProgramPath
    pub fn get_policy_source_initial_program_path(&self) -> Option<&u32> {
        self.policy_source_initial_program_path.as_ref()
    }

    /// Sets the value of PolicySourceStartIn
    pub fn set_policy_source_start_in(&mut self, value: u32) {
        self.policy_source_start_in = Some(value);
    }

    /// Gets the value of PolicySourceStartIn
    pub fn get_policy_source_start_in(&self) -> Option<&u32> {
        self.policy_source_start_in.as_ref()
    }

    /// Sets the value of Startin
    pub fn set_startin(&mut self, value: String) {
        self.startin = Some(value);
    }

    /// Gets the value of Startin
    pub fn get_startin(&self) -> Option<&String> {
        self.startin.as_ref()
    }

/// 

    /// * `initial_program_path` -  (String)
    /// * `startin` -  (String)

    /// * `return_value` -  (u32)
    pub fn initial_program(&self, initial_program_path: &String, startin: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "InitialProgramPath".to_string(), value: initial_program_path.into() });
        args.push(MethodParameter { name: "Startin".to_string(), value: startin.into() });
        self.invoke_method("InitialProgram", &args)

    }


/// 

    /// * `client_wall_paper` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_client_wall_paper(&self, client_wall_paper: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ClientWallPaper".to_string(), value: client_wall_paper.into() });
        self.invoke_method("SetClientWallPaper", &args)

    }

}

