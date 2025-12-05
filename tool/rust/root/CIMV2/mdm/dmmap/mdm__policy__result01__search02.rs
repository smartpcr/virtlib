// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Result01_Search02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Result01_Search02 {

/// 
    #[serde(rename = "AllowCloudSearch")]
    pub allow_cloud_search: Option<i32>,

/// 
    #[serde(rename = "AllowCortanaInAAD")]
    pub allow_cortana_in_aad: Option<i32>,

/// 
    #[serde(rename = "AllowFindMyFiles")]
    pub allow_find_my_files: Option<i32>,

/// 
    #[serde(rename = "AllowIndexingEncryptedStoresOrItems")]
    pub allow_indexing_encrypted_stores_or_items: Option<i32>,

/// 
    #[serde(rename = "AllowSearchToUseLocation")]
    pub allow_search_to_use_location: Option<i32>,

/// 
    #[serde(rename = "AllowStoringImagesFromVisionSearch")]
    pub allow_storing_images_from_vision_search: Option<i32>,

/// 
    #[serde(rename = "AllowUsingDiacritics")]
    pub allow_using_diacritics: Option<i32>,

/// 
    #[serde(rename = "AllowWindowsIndexer")]
    pub allow_windows_indexer: Option<i32>,

/// 
    #[serde(rename = "AlwaysUseAutoLangDetection")]
    pub always_use_auto_lang_detection: Option<i32>,

/// 
    #[serde(rename = "DisableBackoff")]
    pub disable_backoff: Option<i32>,

/// 
    #[serde(rename = "DisableRemovableDriveIndexing")]
    pub disable_removable_drive_indexing: Option<i32>,

/// 
    #[serde(rename = "DoNotUseWebResults")]
    pub do_not_use_web_results: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "PreventIndexingLowDiskSpaceMB")]
    pub prevent_indexing_low_disk_space_mb: Option<i32>,

/// 
    #[serde(rename = "PreventRemoteQueries")]
    pub prevent_remote_queries: Option<i32>,
}

impl MDM_Policy_Result01_Search02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allow_cloud_search: None,
            allow_cortana_in_aad: None,
            allow_find_my_files: None,
            allow_indexing_encrypted_stores_or_items: None,
            allow_search_to_use_location: None,
            allow_storing_images_from_vision_search: None,
            allow_using_diacritics: None,
            allow_windows_indexer: None,
            always_use_auto_lang_detection: None,
            disable_backoff: None,
            disable_removable_drive_indexing: None,
            do_not_use_web_results: None,
            instance_id: None,
            parent_id: None,
            prevent_indexing_low_disk_space_mb: None,
            prevent_remote_queries: None,
        }
    }


    /// Sets the value of AllowCloudSearch
    pub fn set_allow_cloud_search(&mut self, value: i32) {
        self.allow_cloud_search = Some(value);
    }

    /// Gets the value of AllowCloudSearch
    pub fn get_allow_cloud_search(&self) -> Option<&i32> {
        self.allow_cloud_search.as_ref()
    }

    /// Sets the value of AllowCortanaInAAD
    pub fn set_allow_cortana_in_aad(&mut self, value: i32) {
        self.allow_cortana_in_aad = Some(value);
    }

    /// Gets the value of AllowCortanaInAAD
    pub fn get_allow_cortana_in_aad(&self) -> Option<&i32> {
        self.allow_cortana_in_aad.as_ref()
    }

    /// Sets the value of AllowFindMyFiles
    pub fn set_allow_find_my_files(&mut self, value: i32) {
        self.allow_find_my_files = Some(value);
    }

    /// Gets the value of AllowFindMyFiles
    pub fn get_allow_find_my_files(&self) -> Option<&i32> {
        self.allow_find_my_files.as_ref()
    }

    /// Sets the value of AllowIndexingEncryptedStoresOrItems
    pub fn set_allow_indexing_encrypted_stores_or_items(&mut self, value: i32) {
        self.allow_indexing_encrypted_stores_or_items = Some(value);
    }

    /// Gets the value of AllowIndexingEncryptedStoresOrItems
    pub fn get_allow_indexing_encrypted_stores_or_items(&self) -> Option<&i32> {
        self.allow_indexing_encrypted_stores_or_items.as_ref()
    }

    /// Sets the value of AllowSearchToUseLocation
    pub fn set_allow_search_to_use_location(&mut self, value: i32) {
        self.allow_search_to_use_location = Some(value);
    }

    /// Gets the value of AllowSearchToUseLocation
    pub fn get_allow_search_to_use_location(&self) -> Option<&i32> {
        self.allow_search_to_use_location.as_ref()
    }

    /// Sets the value of AllowStoringImagesFromVisionSearch
    pub fn set_allow_storing_images_from_vision_search(&mut self, value: i32) {
        self.allow_storing_images_from_vision_search = Some(value);
    }

    /// Gets the value of AllowStoringImagesFromVisionSearch
    pub fn get_allow_storing_images_from_vision_search(&self) -> Option<&i32> {
        self.allow_storing_images_from_vision_search.as_ref()
    }

    /// Sets the value of AllowUsingDiacritics
    pub fn set_allow_using_diacritics(&mut self, value: i32) {
        self.allow_using_diacritics = Some(value);
    }

    /// Gets the value of AllowUsingDiacritics
    pub fn get_allow_using_diacritics(&self) -> Option<&i32> {
        self.allow_using_diacritics.as_ref()
    }

    /// Sets the value of AllowWindowsIndexer
    pub fn set_allow_windows_indexer(&mut self, value: i32) {
        self.allow_windows_indexer = Some(value);
    }

    /// Gets the value of AllowWindowsIndexer
    pub fn get_allow_windows_indexer(&self) -> Option<&i32> {
        self.allow_windows_indexer.as_ref()
    }

    /// Sets the value of AlwaysUseAutoLangDetection
    pub fn set_always_use_auto_lang_detection(&mut self, value: i32) {
        self.always_use_auto_lang_detection = Some(value);
    }

    /// Gets the value of AlwaysUseAutoLangDetection
    pub fn get_always_use_auto_lang_detection(&self) -> Option<&i32> {
        self.always_use_auto_lang_detection.as_ref()
    }

    /// Sets the value of DisableBackoff
    pub fn set_disable_backoff(&mut self, value: i32) {
        self.disable_backoff = Some(value);
    }

    /// Gets the value of DisableBackoff
    pub fn get_disable_backoff(&self) -> Option<&i32> {
        self.disable_backoff.as_ref()
    }

    /// Sets the value of DisableRemovableDriveIndexing
    pub fn set_disable_removable_drive_indexing(&mut self, value: i32) {
        self.disable_removable_drive_indexing = Some(value);
    }

    /// Gets the value of DisableRemovableDriveIndexing
    pub fn get_disable_removable_drive_indexing(&self) -> Option<&i32> {
        self.disable_removable_drive_indexing.as_ref()
    }

    /// Sets the value of DoNotUseWebResults
    pub fn set_do_not_use_web_results(&mut self, value: i32) {
        self.do_not_use_web_results = Some(value);
    }

    /// Gets the value of DoNotUseWebResults
    pub fn get_do_not_use_web_results(&self) -> Option<&i32> {
        self.do_not_use_web_results.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of PreventIndexingLowDiskSpaceMB
    pub fn set_prevent_indexing_low_disk_space_mb(&mut self, value: i32) {
        self.prevent_indexing_low_disk_space_mb = Some(value);
    }

    /// Gets the value of PreventIndexingLowDiskSpaceMB
    pub fn get_prevent_indexing_low_disk_space_mb(&self) -> Option<&i32> {
        self.prevent_indexing_low_disk_space_mb.as_ref()
    }

    /// Sets the value of PreventRemoteQueries
    pub fn set_prevent_remote_queries(&mut self, value: i32) {
        self.prevent_remote_queries = Some(value);
    }

    /// Gets the value of PreventRemoteQueries
    pub fn get_prevent_remote_queries(&self) -> Option<&i32> {
        self.prevent_remote_queries.as_ref()
    }
}

