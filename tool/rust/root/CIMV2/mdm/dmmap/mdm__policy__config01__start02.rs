// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Config01_Start02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Config01_Start02 {

/// 
    #[serde(rename = "AllowPinnedFolderDocuments")]
    pub allow_pinned_folder_documents: Option<i32>,

/// 
    #[serde(rename = "AllowPinnedFolderDownloads")]
    pub allow_pinned_folder_downloads: Option<i32>,

/// 
    #[serde(rename = "AllowPinnedFolderFileExplorer")]
    pub allow_pinned_folder_file_explorer: Option<i32>,

/// 
    #[serde(rename = "AllowPinnedFolderHomeGroup")]
    pub allow_pinned_folder_home_group: Option<i32>,

/// 
    #[serde(rename = "AllowPinnedFolderMusic")]
    pub allow_pinned_folder_music: Option<i32>,

/// 
    #[serde(rename = "AllowPinnedFolderNetwork")]
    pub allow_pinned_folder_network: Option<i32>,

/// 
    #[serde(rename = "AllowPinnedFolderPersonalFolder")]
    pub allow_pinned_folder_personal_folder: Option<i32>,

/// 
    #[serde(rename = "AllowPinnedFolderPictures")]
    pub allow_pinned_folder_pictures: Option<i32>,

/// 
    #[serde(rename = "AllowPinnedFolderSettings")]
    pub allow_pinned_folder_settings: Option<i32>,

/// 
    #[serde(rename = "AllowPinnedFolderVideos")]
    pub allow_pinned_folder_videos: Option<i32>,

/// 
    #[serde(rename = "DisableContextMenus")]
    pub disable_context_menus: Option<i32>,

/// 
    #[serde(rename = "ForceStartSize")]
    pub force_start_size: Option<i32>,

/// 
    #[serde(rename = "HideAppList")]
    pub hide_app_list: Option<i32>,

/// 
    #[serde(rename = "HideChangeAccountSettings")]
    pub hide_change_account_settings: Option<i32>,

/// 
    #[serde(rename = "HideFrequentlyUsedApps")]
    pub hide_frequently_used_apps: Option<i32>,

/// 
    #[serde(rename = "HideHibernate")]
    pub hide_hibernate: Option<i32>,

/// 
    #[serde(rename = "HideLock")]
    pub hide_lock: Option<i32>,

/// 
    #[serde(rename = "HidePowerButton")]
    pub hide_power_button: Option<i32>,

/// 
    #[serde(rename = "HideRecentJumplists")]
    pub hide_recent_jumplists: Option<i32>,

/// 
    #[serde(rename = "HideRecentlyAddedApps")]
    pub hide_recently_added_apps: Option<i32>,

/// 
    #[serde(rename = "HideRestart")]
    pub hide_restart: Option<i32>,

/// 
    #[serde(rename = "HideShutDown")]
    pub hide_shut_down: Option<i32>,

/// 
    #[serde(rename = "HideSignOut")]
    pub hide_sign_out: Option<i32>,

/// 
    #[serde(rename = "HideSleep")]
    pub hide_sleep: Option<i32>,

/// 
    #[serde(rename = "HideSwitchAccount")]
    pub hide_switch_account: Option<i32>,

/// 
    #[serde(rename = "HideUserTile")]
    pub hide_user_tile: Option<i32>,

/// 
    #[serde(rename = "ImportEdgeAssets")]
    pub import_edge_assets: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "NoPinningToTaskbar")]
    pub no_pinning_to_taskbar: Option<i32>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "ShowOrHideMostUsedApps")]
    pub show_or_hide_most_used_apps: Option<i32>,

/// 
    #[serde(rename = "StartLayout")]
    pub start_layout: Option<String>,
}

impl MDM_Policy_Config01_Start02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allow_pinned_folder_documents: None,
            allow_pinned_folder_downloads: None,
            allow_pinned_folder_file_explorer: None,
            allow_pinned_folder_home_group: None,
            allow_pinned_folder_music: None,
            allow_pinned_folder_network: None,
            allow_pinned_folder_personal_folder: None,
            allow_pinned_folder_pictures: None,
            allow_pinned_folder_settings: None,
            allow_pinned_folder_videos: None,
            disable_context_menus: None,
            force_start_size: None,
            hide_app_list: None,
            hide_change_account_settings: None,
            hide_frequently_used_apps: None,
            hide_hibernate: None,
            hide_lock: None,
            hide_power_button: None,
            hide_recent_jumplists: None,
            hide_recently_added_apps: None,
            hide_restart: None,
            hide_shut_down: None,
            hide_sign_out: None,
            hide_sleep: None,
            hide_switch_account: None,
            hide_user_tile: None,
            import_edge_assets: None,
            instance_id: None,
            no_pinning_to_taskbar: None,
            parent_id: None,
            show_or_hide_most_used_apps: None,
            start_layout: None,
        }
    }


    /// Sets the value of AllowPinnedFolderDocuments
    pub fn set_allow_pinned_folder_documents(&mut self, value: i32) {
        self.allow_pinned_folder_documents = Some(value);
    }

    /// Gets the value of AllowPinnedFolderDocuments
    pub fn get_allow_pinned_folder_documents(&self) -> Option<&i32> {
        self.allow_pinned_folder_documents.as_ref()
    }

    /// Sets the value of AllowPinnedFolderDownloads
    pub fn set_allow_pinned_folder_downloads(&mut self, value: i32) {
        self.allow_pinned_folder_downloads = Some(value);
    }

    /// Gets the value of AllowPinnedFolderDownloads
    pub fn get_allow_pinned_folder_downloads(&self) -> Option<&i32> {
        self.allow_pinned_folder_downloads.as_ref()
    }

    /// Sets the value of AllowPinnedFolderFileExplorer
    pub fn set_allow_pinned_folder_file_explorer(&mut self, value: i32) {
        self.allow_pinned_folder_file_explorer = Some(value);
    }

    /// Gets the value of AllowPinnedFolderFileExplorer
    pub fn get_allow_pinned_folder_file_explorer(&self) -> Option<&i32> {
        self.allow_pinned_folder_file_explorer.as_ref()
    }

    /// Sets the value of AllowPinnedFolderHomeGroup
    pub fn set_allow_pinned_folder_home_group(&mut self, value: i32) {
        self.allow_pinned_folder_home_group = Some(value);
    }

    /// Gets the value of AllowPinnedFolderHomeGroup
    pub fn get_allow_pinned_folder_home_group(&self) -> Option<&i32> {
        self.allow_pinned_folder_home_group.as_ref()
    }

    /// Sets the value of AllowPinnedFolderMusic
    pub fn set_allow_pinned_folder_music(&mut self, value: i32) {
        self.allow_pinned_folder_music = Some(value);
    }

    /// Gets the value of AllowPinnedFolderMusic
    pub fn get_allow_pinned_folder_music(&self) -> Option<&i32> {
        self.allow_pinned_folder_music.as_ref()
    }

    /// Sets the value of AllowPinnedFolderNetwork
    pub fn set_allow_pinned_folder_network(&mut self, value: i32) {
        self.allow_pinned_folder_network = Some(value);
    }

    /// Gets the value of AllowPinnedFolderNetwork
    pub fn get_allow_pinned_folder_network(&self) -> Option<&i32> {
        self.allow_pinned_folder_network.as_ref()
    }

    /// Sets the value of AllowPinnedFolderPersonalFolder
    pub fn set_allow_pinned_folder_personal_folder(&mut self, value: i32) {
        self.allow_pinned_folder_personal_folder = Some(value);
    }

    /// Gets the value of AllowPinnedFolderPersonalFolder
    pub fn get_allow_pinned_folder_personal_folder(&self) -> Option<&i32> {
        self.allow_pinned_folder_personal_folder.as_ref()
    }

    /// Sets the value of AllowPinnedFolderPictures
    pub fn set_allow_pinned_folder_pictures(&mut self, value: i32) {
        self.allow_pinned_folder_pictures = Some(value);
    }

    /// Gets the value of AllowPinnedFolderPictures
    pub fn get_allow_pinned_folder_pictures(&self) -> Option<&i32> {
        self.allow_pinned_folder_pictures.as_ref()
    }

    /// Sets the value of AllowPinnedFolderSettings
    pub fn set_allow_pinned_folder_settings(&mut self, value: i32) {
        self.allow_pinned_folder_settings = Some(value);
    }

    /// Gets the value of AllowPinnedFolderSettings
    pub fn get_allow_pinned_folder_settings(&self) -> Option<&i32> {
        self.allow_pinned_folder_settings.as_ref()
    }

    /// Sets the value of AllowPinnedFolderVideos
    pub fn set_allow_pinned_folder_videos(&mut self, value: i32) {
        self.allow_pinned_folder_videos = Some(value);
    }

    /// Gets the value of AllowPinnedFolderVideos
    pub fn get_allow_pinned_folder_videos(&self) -> Option<&i32> {
        self.allow_pinned_folder_videos.as_ref()
    }

    /// Sets the value of DisableContextMenus
    pub fn set_disable_context_menus(&mut self, value: i32) {
        self.disable_context_menus = Some(value);
    }

    /// Gets the value of DisableContextMenus
    pub fn get_disable_context_menus(&self) -> Option<&i32> {
        self.disable_context_menus.as_ref()
    }

    /// Sets the value of ForceStartSize
    pub fn set_force_start_size(&mut self, value: i32) {
        self.force_start_size = Some(value);
    }

    /// Gets the value of ForceStartSize
    pub fn get_force_start_size(&self) -> Option<&i32> {
        self.force_start_size.as_ref()
    }

    /// Sets the value of HideAppList
    pub fn set_hide_app_list(&mut self, value: i32) {
        self.hide_app_list = Some(value);
    }

    /// Gets the value of HideAppList
    pub fn get_hide_app_list(&self) -> Option<&i32> {
        self.hide_app_list.as_ref()
    }

    /// Sets the value of HideChangeAccountSettings
    pub fn set_hide_change_account_settings(&mut self, value: i32) {
        self.hide_change_account_settings = Some(value);
    }

    /// Gets the value of HideChangeAccountSettings
    pub fn get_hide_change_account_settings(&self) -> Option<&i32> {
        self.hide_change_account_settings.as_ref()
    }

    /// Sets the value of HideFrequentlyUsedApps
    pub fn set_hide_frequently_used_apps(&mut self, value: i32) {
        self.hide_frequently_used_apps = Some(value);
    }

    /// Gets the value of HideFrequentlyUsedApps
    pub fn get_hide_frequently_used_apps(&self) -> Option<&i32> {
        self.hide_frequently_used_apps.as_ref()
    }

    /// Sets the value of HideHibernate
    pub fn set_hide_hibernate(&mut self, value: i32) {
        self.hide_hibernate = Some(value);
    }

    /// Gets the value of HideHibernate
    pub fn get_hide_hibernate(&self) -> Option<&i32> {
        self.hide_hibernate.as_ref()
    }

    /// Sets the value of HideLock
    pub fn set_hide_lock(&mut self, value: i32) {
        self.hide_lock = Some(value);
    }

    /// Gets the value of HideLock
    pub fn get_hide_lock(&self) -> Option<&i32> {
        self.hide_lock.as_ref()
    }

    /// Sets the value of HidePowerButton
    pub fn set_hide_power_button(&mut self, value: i32) {
        self.hide_power_button = Some(value);
    }

    /// Gets the value of HidePowerButton
    pub fn get_hide_power_button(&self) -> Option<&i32> {
        self.hide_power_button.as_ref()
    }

    /// Sets the value of HideRecentJumplists
    pub fn set_hide_recent_jumplists(&mut self, value: i32) {
        self.hide_recent_jumplists = Some(value);
    }

    /// Gets the value of HideRecentJumplists
    pub fn get_hide_recent_jumplists(&self) -> Option<&i32> {
        self.hide_recent_jumplists.as_ref()
    }

    /// Sets the value of HideRecentlyAddedApps
    pub fn set_hide_recently_added_apps(&mut self, value: i32) {
        self.hide_recently_added_apps = Some(value);
    }

    /// Gets the value of HideRecentlyAddedApps
    pub fn get_hide_recently_added_apps(&self) -> Option<&i32> {
        self.hide_recently_added_apps.as_ref()
    }

    /// Sets the value of HideRestart
    pub fn set_hide_restart(&mut self, value: i32) {
        self.hide_restart = Some(value);
    }

    /// Gets the value of HideRestart
    pub fn get_hide_restart(&self) -> Option<&i32> {
        self.hide_restart.as_ref()
    }

    /// Sets the value of HideShutDown
    pub fn set_hide_shut_down(&mut self, value: i32) {
        self.hide_shut_down = Some(value);
    }

    /// Gets the value of HideShutDown
    pub fn get_hide_shut_down(&self) -> Option<&i32> {
        self.hide_shut_down.as_ref()
    }

    /// Sets the value of HideSignOut
    pub fn set_hide_sign_out(&mut self, value: i32) {
        self.hide_sign_out = Some(value);
    }

    /// Gets the value of HideSignOut
    pub fn get_hide_sign_out(&self) -> Option<&i32> {
        self.hide_sign_out.as_ref()
    }

    /// Sets the value of HideSleep
    pub fn set_hide_sleep(&mut self, value: i32) {
        self.hide_sleep = Some(value);
    }

    /// Gets the value of HideSleep
    pub fn get_hide_sleep(&self) -> Option<&i32> {
        self.hide_sleep.as_ref()
    }

    /// Sets the value of HideSwitchAccount
    pub fn set_hide_switch_account(&mut self, value: i32) {
        self.hide_switch_account = Some(value);
    }

    /// Gets the value of HideSwitchAccount
    pub fn get_hide_switch_account(&self) -> Option<&i32> {
        self.hide_switch_account.as_ref()
    }

    /// Sets the value of HideUserTile
    pub fn set_hide_user_tile(&mut self, value: i32) {
        self.hide_user_tile = Some(value);
    }

    /// Gets the value of HideUserTile
    pub fn get_hide_user_tile(&self) -> Option<&i32> {
        self.hide_user_tile.as_ref()
    }

    /// Sets the value of ImportEdgeAssets
    pub fn set_import_edge_assets(&mut self, value: String) {
        self.import_edge_assets = Some(value);
    }

    /// Gets the value of ImportEdgeAssets
    pub fn get_import_edge_assets(&self) -> Option<&String> {
        self.import_edge_assets.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of NoPinningToTaskbar
    pub fn set_no_pinning_to_taskbar(&mut self, value: i32) {
        self.no_pinning_to_taskbar = Some(value);
    }

    /// Gets the value of NoPinningToTaskbar
    pub fn get_no_pinning_to_taskbar(&self) -> Option<&i32> {
        self.no_pinning_to_taskbar.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of ShowOrHideMostUsedApps
    pub fn set_show_or_hide_most_used_apps(&mut self, value: i32) {
        self.show_or_hide_most_used_apps = Some(value);
    }

    /// Gets the value of ShowOrHideMostUsedApps
    pub fn get_show_or_hide_most_used_apps(&self) -> Option<&i32> {
        self.show_or_hide_most_used_apps.as_ref()
    }

    /// Sets the value of StartLayout
    pub fn set_start_layout(&mut self, value: String) {
        self.start_layout = Some(value);
    }

    /// Gets the value of StartLayout
    pub fn get_start_layout(&self) -> Option<&String> {
        self.start_layout.as_ref()
    }
}

