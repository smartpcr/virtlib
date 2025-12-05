// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetSwitchTeam_TeamMember struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetSwitchTeam_TeamMember {

/// 
    #[serde(rename = "MemberOfTheTeam")]
    pub member_of_the_team: Option<MSFT_NetSwitchTeamMember>,

/// 
    #[serde(rename = "TeamOfTheMember")]
    pub team_of_the_member: Option<MSFT_NetSwitchTeam>,
}

impl MSFT_NetSwitchTeam_TeamMember {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            member_of_the_team: None,
            team_of_the_member: None,
        }
    }


    /// Sets the value of MemberOfTheTeam
    pub fn set_member_of_the_team(&mut self, value: MSFT_NetSwitchTeamMember) {
        self.member_of_the_team = Some(value);
    }

    /// Gets the value of MemberOfTheTeam
    pub fn get_member_of_the_team(&self) -> Option<&MSFT_NetSwitchTeamMember> {
        self.member_of_the_team.as_ref()
    }

    /// Sets the value of TeamOfTheMember
    pub fn set_team_of_the_member(&mut self, value: MSFT_NetSwitchTeam) {
        self.team_of_the_member = Some(value);
    }

    /// Gets the value of TeamOfTheMember
    pub fn get_team_of_the_member(&self) -> Option<&MSFT_NetSwitchTeam> {
        self.team_of_the_member.as_ref()
    }
}

