// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_FolderRedirectionPolicySetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_FolderRedirectionPolicySetting {
    #[serde(flatten)]
    pub base: RSOP_PolicySetting,

/// 
    #[serde(rename = "configurationControl")]
    pub configuration_control: Option<u32>,

/// 
    #[serde(rename = "grantType")]
    pub grant_type: Option<bool>,

/// 
    #[serde(rename = "installationType")]
    pub installation_type: Option<u32>,

/// 
    #[serde(rename = "moveType")]
    pub move_type: Option<bool>,

/// 
    #[serde(rename = "parentFolderId")]
    pub parent_folder_id: Option<String>,

/// 
    #[serde(rename = "policyRemoval")]
    pub policy_removal: Option<u32>,

/// 
    #[serde(rename = "primaryComputerEvaluation")]
    pub primary_computer_evaluation: Option<u32>,

/// 
    #[serde(rename = "redirectedPaths")]
    pub redirected_paths: Vec<String>,

/// 
    #[serde(rename = "redirectingGroup")]
    pub redirecting_group: Option<String>,

/// 
    #[serde(rename = "redirectionFlags")]
    pub redirection_flags: Option<u32>,

/// 
    #[serde(rename = "resultantPath")]
    pub resultant_path: Option<String>,

/// 
    #[serde(rename = "securityGroups")]
    pub security_groups: Vec<String>,
}

impl RSOP_FolderRedirectionPolicySetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: RSOP_PolicySetting::new(),
            configuration_control: None,
            grant_type: None,
            installation_type: None,
            move_type: None,
            parent_folder_id: None,
            policy_removal: None,
            primary_computer_evaluation: None,
            redirected_paths: Vec::new(),
            redirecting_group: None,
            redirection_flags: None,
            resultant_path: None,
            security_groups: Vec::new(),
        }
    }


    /// Sets the value of configurationControl
    pub fn set_configuration_control(&mut self, value: u32) {
        self.configuration_control = Some(value);
    }

    /// Gets the value of configurationControl
    pub fn get_configuration_control(&self) -> Option<&u32> {
        self.configuration_control.as_ref()
    }

    /// Sets the value of grantType
    pub fn set_grant_type(&mut self, value: bool) {
        self.grant_type = Some(value);
    }

    /// Gets the value of grantType
    pub fn get_grant_type(&self) -> Option<&bool> {
        self.grant_type.as_ref()
    }

    /// Sets the value of installationType
    pub fn set_installation_type(&mut self, value: u32) {
        self.installation_type = Some(value);
    }

    /// Gets the value of installationType
    pub fn get_installation_type(&self) -> Option<&u32> {
        self.installation_type.as_ref()
    }

    /// Sets the value of moveType
    pub fn set_move_type(&mut self, value: bool) {
        self.move_type = Some(value);
    }

    /// Gets the value of moveType
    pub fn get_move_type(&self) -> Option<&bool> {
        self.move_type.as_ref()
    }

    /// Sets the value of parentFolderId
    pub fn set_parent_folder_id(&mut self, value: String) {
        self.parent_folder_id = Some(value);
    }

    /// Gets the value of parentFolderId
    pub fn get_parent_folder_id(&self) -> Option<&String> {
        self.parent_folder_id.as_ref()
    }

    /// Sets the value of policyRemoval
    pub fn set_policy_removal(&mut self, value: u32) {
        self.policy_removal = Some(value);
    }

    /// Gets the value of policyRemoval
    pub fn get_policy_removal(&self) -> Option<&u32> {
        self.policy_removal.as_ref()
    }

    /// Sets the value of primaryComputerEvaluation
    pub fn set_primary_computer_evaluation(&mut self, value: u32) {
        self.primary_computer_evaluation = Some(value);
    }

    /// Gets the value of primaryComputerEvaluation
    pub fn get_primary_computer_evaluation(&self) -> Option<&u32> {
        self.primary_computer_evaluation.as_ref()
    }

    /// Sets the value of redirectedPaths
    pub fn set_redirected_paths(&mut self, value: Vec<String>) {
        self.redirected_paths = value;
    }

    /// Gets the value of redirectedPaths
    pub fn get_redirected_paths(&self) -> &Vec<String> {
        &self.redirected_paths
    }

    /// Sets the value of redirectingGroup
    pub fn set_redirecting_group(&mut self, value: String) {
        self.redirecting_group = Some(value);
    }

    /// Gets the value of redirectingGroup
    pub fn get_redirecting_group(&self) -> Option<&String> {
        self.redirecting_group.as_ref()
    }

    /// Sets the value of redirectionFlags
    pub fn set_redirection_flags(&mut self, value: u32) {
        self.redirection_flags = Some(value);
    }

    /// Gets the value of redirectionFlags
    pub fn get_redirection_flags(&self) -> Option<&u32> {
        self.redirection_flags.as_ref()
    }

    /// Sets the value of resultantPath
    pub fn set_resultant_path(&mut self, value: String) {
        self.resultant_path = Some(value);
    }

    /// Gets the value of resultantPath
    pub fn get_resultant_path(&self) -> Option<&String> {
        self.resultant_path.as_ref()
    }

    /// Sets the value of securityGroups
    pub fn set_security_groups(&mut self, value: Vec<String>) {
        self.security_groups = value;
    }

    /// Gets the value of securityGroups
    pub fn get_security_groups(&self) -> &Vec<String> {
        &self.security_groups
    }
}

