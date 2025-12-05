// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetSwitchTeam struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetSwitchTeam {
    #[serde(flatten)]
    pub base: MSFT_NetImPlatTeam,
}

impl MSFT_NetSwitchTeam {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetImPlatTeam::new(),
        }
    }


/// 

    /// * `name` -  (String)
    /// * `team_members` -  (String[])

    /// * `return_value` -  (u32)
    pub fn create(&self, name: &String, team_members: &Vec<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "TeamMembers".to_string(), value: team_members.into() });
        self.invoke_method("Create", &args)

    }


/// 

    /// * `name` -  (String)
    /// * `new_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn rename(&self, name: &String, new_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "NewName".to_string(), value: new_name.into() });
        self.invoke_method("Rename", &args)

    }


/// 

    /// * `name` -  (String)
    /// * `team` -  (String)

    /// * `return_value` -  (u32)
    pub fn add_member(&self, name: &String, team: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "Team".to_string(), value: team.into() });
        self.invoke_method("AddMember", &args)

    }


/// 

    /// * `name` -  (String)
    /// * `team` -  (String)

    /// * `return_value` -  (u32)
    pub fn remove_member(&self, name: &String, team: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "Team".to_string(), value: team.into() });
        self.invoke_method("RemoveMember", &args)

    }

}

