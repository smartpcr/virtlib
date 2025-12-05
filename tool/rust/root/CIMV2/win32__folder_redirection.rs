// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_FolderRedirection struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_FolderRedirection {

/// Move the contents of local <folder> to the new location. This will copy the redirected folder data into the local UNC location. Then this content will be synced with the server share content. Effectively, moving the content from the local location to the share
    #[serde(rename = "ContentsMoved")]
    pub contents_moved: Option<bool>,

/// When the redirection policy is removed, the folder's content will be moved to the local profileIf true, the folder will be moved back to the local user profile location when policy is removed.If false, the folder will remain in the redirected location after the redirection policy is removed.
    #[serde(rename = "ContentsMovedOnPolicyRemoval")]
    pub contents_moved_on_policy_removal: Option<bool>,

/// Content is renamed from old to new location in Offline Files cache; assumes data on server is moved between names through other means
    #[serde(rename = "ContentsRenamedInLocalCache")]
    pub contents_renamed_in_local_cache: Option<bool>,

/// Grant the user exclusive rights to <folder>
    #[serde(rename = "ExclusiveRightsGranted")]
    pub exclusive_rights_granted: Option<bool>,

/// known folder unique id (guid)
    #[serde(rename = "FolderId")]
    pub folder_id: Option<String>,

/// Do not automatically make redirected folders available offline
    #[serde(rename = "MakeFolderAvailableOfflineDisabled")]
    pub make_folder_available_offline_disabled: Option<bool>,

/// Redirection Path [may be used when RedirectionType == {0,1}
    #[serde(rename = "RedirectionPath")]
    pub redirection_path: Option<String>,

/// The type of folder redirection to be performed.
    #[serde(rename = "RedirectionType")]
    pub redirection_type: Option<FolderRedirection_RedirectionType>,
}

impl Win32_FolderRedirection {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            contents_moved: None,
            contents_moved_on_policy_removal: None,
            contents_renamed_in_local_cache: None,
            exclusive_rights_granted: None,
            folder_id: None,
            make_folder_available_offline_disabled: None,
            redirection_path: None,
            redirection_type: None,
        }
    }


    /// Sets the value of ContentsMoved
    pub fn set_contents_moved(&mut self, value: bool) {
        self.contents_moved = Some(value);
    }

    /// Gets the value of ContentsMoved
    pub fn get_contents_moved(&self) -> Option<&bool> {
        self.contents_moved.as_ref()
    }

    /// Sets the value of ContentsMovedOnPolicyRemoval
    pub fn set_contents_moved_on_policy_removal(&mut self, value: bool) {
        self.contents_moved_on_policy_removal = Some(value);
    }

    /// Gets the value of ContentsMovedOnPolicyRemoval
    pub fn get_contents_moved_on_policy_removal(&self) -> Option<&bool> {
        self.contents_moved_on_policy_removal.as_ref()
    }

    /// Sets the value of ContentsRenamedInLocalCache
    pub fn set_contents_renamed_in_local_cache(&mut self, value: bool) {
        self.contents_renamed_in_local_cache = Some(value);
    }

    /// Gets the value of ContentsRenamedInLocalCache
    pub fn get_contents_renamed_in_local_cache(&self) -> Option<&bool> {
        self.contents_renamed_in_local_cache.as_ref()
    }

    /// Sets the value of ExclusiveRightsGranted
    pub fn set_exclusive_rights_granted(&mut self, value: bool) {
        self.exclusive_rights_granted = Some(value);
    }

    /// Gets the value of ExclusiveRightsGranted
    pub fn get_exclusive_rights_granted(&self) -> Option<&bool> {
        self.exclusive_rights_granted.as_ref()
    }

    /// Sets the value of FolderId
    pub fn set_folder_id(&mut self, value: String) {
        self.folder_id = Some(value);
    }

    /// Gets the value of FolderId
    pub fn get_folder_id(&self) -> Option<&String> {
        self.folder_id.as_ref()
    }

    /// Sets the value of MakeFolderAvailableOfflineDisabled
    pub fn set_make_folder_available_offline_disabled(&mut self, value: bool) {
        self.make_folder_available_offline_disabled = Some(value);
    }

    /// Gets the value of MakeFolderAvailableOfflineDisabled
    pub fn get_make_folder_available_offline_disabled(&self) -> Option<&bool> {
        self.make_folder_available_offline_disabled.as_ref()
    }

    /// Sets the value of RedirectionPath
    pub fn set_redirection_path(&mut self, value: String) {
        self.redirection_path = Some(value);
    }

    /// Gets the value of RedirectionPath
    pub fn get_redirection_path(&self) -> Option<&String> {
        self.redirection_path.as_ref()
    }

    /// Sets the value of RedirectionType
    pub fn set_redirection_type(&mut self, value: FolderRedirection_RedirectionType) {
        self.redirection_type = Some(value);
    }

    /// Gets the value of RedirectionType
    pub fn get_redirection_type(&self) -> Option<&FolderRedirection_RedirectionType> {
        self.redirection_type.as_ref()
    }
}

