// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Config01_TimeLanguageSettings02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Config01_TimeLanguageSettings02 {

/// 
    #[serde(rename = "BlockCleanupOfUnusedPreinstalledLangPacks")]
    pub block_cleanup_of_unused_preinstalled_lang_packs: Option<i32>,

/// 
    #[serde(rename = "ConfigureTimeZone")]
    pub configure_time_zone: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "MachineUILanguageOverwrite")]
    pub machine_uilanguage_overwrite: Option<i32>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "RestrictLanguagePacksAndFeaturesInstall")]
    pub restrict_language_packs_and_features_install: Option<i32>,
}

impl MDM_Policy_Config01_TimeLanguageSettings02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            block_cleanup_of_unused_preinstalled_lang_packs: None,
            configure_time_zone: None,
            instance_id: None,
            machine_uilanguage_overwrite: None,
            parent_id: None,
            restrict_language_packs_and_features_install: None,
        }
    }


    /// Sets the value of BlockCleanupOfUnusedPreinstalledLangPacks
    pub fn set_block_cleanup_of_unused_preinstalled_lang_packs(&mut self, value: i32) {
        self.block_cleanup_of_unused_preinstalled_lang_packs = Some(value);
    }

    /// Gets the value of BlockCleanupOfUnusedPreinstalledLangPacks
    pub fn get_block_cleanup_of_unused_preinstalled_lang_packs(&self) -> Option<&i32> {
        self.block_cleanup_of_unused_preinstalled_lang_packs.as_ref()
    }

    /// Sets the value of ConfigureTimeZone
    pub fn set_configure_time_zone(&mut self, value: String) {
        self.configure_time_zone = Some(value);
    }

    /// Gets the value of ConfigureTimeZone
    pub fn get_configure_time_zone(&self) -> Option<&String> {
        self.configure_time_zone.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of MachineUILanguageOverwrite
    pub fn set_machine_uilanguage_overwrite(&mut self, value: i32) {
        self.machine_uilanguage_overwrite = Some(value);
    }

    /// Gets the value of MachineUILanguageOverwrite
    pub fn get_machine_uilanguage_overwrite(&self) -> Option<&i32> {
        self.machine_uilanguage_overwrite.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of RestrictLanguagePacksAndFeaturesInstall
    pub fn set_restrict_language_packs_and_features_install(&mut self, value: i32) {
        self.restrict_language_packs_and_features_install = Some(value);
    }

    /// Gets the value of RestrictLanguagePacksAndFeaturesInstall
    pub fn get_restrict_language_packs_and_features_install(&self) -> Option<&i32> {
        self.restrict_language_packs_and_features_install.as_ref()
    }
}

