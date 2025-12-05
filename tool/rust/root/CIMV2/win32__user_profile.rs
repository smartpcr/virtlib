// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_UserProfile struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_UserProfile {

/// A Win32_FolderRedirectionHealth object that represents the health of the user's redirected AppData\Roaming folder.
    #[serde(rename = "AppDataRoaming")]
    pub app_data_roaming: Option<Win32_FolderRedirectionHealth>,

/// A Win32_FolderRedirectionHealth object that represents the health of the user's redirected Contacts folder.
    #[serde(rename = "Contacts")]
    pub contacts: Option<Win32_FolderRedirectionHealth>,

/// A Win32_FolderRedirectionHealth object that represents the health of the user's redirected Desktop folder.
    #[serde(rename = "Desktop")]
    pub desktop: Option<Win32_FolderRedirectionHealth>,

/// A Win32_FolderRedirectionHealth object that represents the health of the user's redirected Documents folder.
    #[serde(rename = "Documents")]
    pub documents: Option<Win32_FolderRedirectionHealth>,

/// A Win32_FolderRedirectionHealth object that represents the health of the user's redirected Downloads folder.
    #[serde(rename = "Downloads")]
    pub downloads: Option<Win32_FolderRedirectionHealth>,

/// A Win32_FolderRedirectionHealth object that represents the health of the user's redirected Favorites folder.
    #[serde(rename = "Favorites")]
    pub favorites: Option<Win32_FolderRedirectionHealth>,

/// The health status of this profile, based on the values that were set in the Win32_RoamingUserHealthConfiguration properties.
    #[serde(rename = "HealthStatus")]
    pub health_status: Option<UserProfile_HealthStatus>,

/// If the profile is a roaming profile, this property is a DATETIME value that indicates the last time an attempt was made to download the profile from the server, even if it was unsuccessful. If the profile is a local profile, this property is zero.
    #[serde(rename = "LastAttemptedProfileDownloadTime")]
    pub last_attempted_profile_download_time: Option<String>,

/// If the profile is a roaming profile, this property is a DATETIME value that indicates the last time an attempt was made to upload the profile to the server, even if it was unsuccessful.
    #[serde(rename = "LastAttemptedProfileUploadTime")]
    pub last_attempted_profile_upload_time: Option<String>,

/// If this profile is a roaming profile, this property is a DATETIME value that indicates the last time the profile's registry hive was uploaded to the server.
    #[serde(rename = "LastBackgroundRegistryUploadTime")]
    pub last_background_registry_upload_time: Option<String>,

/// 
    #[serde(rename = "LastDownloadTime")]
    pub last_download_time: Option<String>,

/// 
    #[serde(rename = "LastUploadTime")]
    pub last_upload_time: Option<String>,

/// 
    #[serde(rename = "LastUseTime")]
    pub last_use_time: Option<String>,

/// A Win32_FolderRedirectionHealth object that represents the health of the user's redirected Links folder.
    #[serde(rename = "Links")]
    pub links: Option<Win32_FolderRedirectionHealth>,

/// 
    #[serde(rename = "Loaded")]
    pub loaded: Option<bool>,

/// 
    #[serde(rename = "LocalPath")]
    pub local_path: Option<String>,

/// A Win32_FolderRedirectionHealth object that represents the health of the user's redirected Music folder.
    #[serde(rename = "Music")]
    pub music: Option<Win32_FolderRedirectionHealth>,

/// A Win32_FolderRedirectionHealth object that represents the health of the user's redirected Pictures folder.
    #[serde(rename = "Pictures")]
    pub pictures: Option<Win32_FolderRedirectionHealth>,

/// 
    #[serde(rename = "RefCount")]
    pub ref_count: Option<u32>,

/// 
    #[serde(rename = "RoamingConfigured")]
    pub roaming_configured: Option<bool>,

/// 
    #[serde(rename = "RoamingPath")]
    pub roaming_path: Option<String>,

/// 
    #[serde(rename = "RoamingPreference")]
    pub roaming_preference: Option<bool>,

/// A Win32_FolderRedirectionHealth object that represents the health of the user's redirected Saved Games folder.
    #[serde(rename = "SavedGames")]
    pub saved_games: Option<Win32_FolderRedirectionHealth>,

/// A Win32_FolderRedirectionHealth object that represents the health of the user's redirected Searches folder.
    #[serde(rename = "Searches")]
    pub searches: Option<Win32_FolderRedirectionHealth>,

/// 
    #[serde(rename = "SID")]
    pub sid: Option<String>,

/// 
    #[serde(rename = "Special")]
    pub special: Option<bool>,

/// A Win32_FolderRedirectionHealth object that represents the health of the user's redirected Start Menu folder.
    #[serde(rename = "StartMenu")]
    pub start_menu: Option<Win32_FolderRedirectionHealth>,

/// 
    #[serde(rename = "Status")]
    pub status: Option<u32>,

/// A Win32_FolderRedirectionHealth object that represents the health of the user's redirected Videos folder.
    #[serde(rename = "Videos")]
    pub videos: Option<Win32_FolderRedirectionHealth>,
}

impl Win32_UserProfile {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            app_data_roaming: None,
            contacts: None,
            desktop: None,
            documents: None,
            downloads: None,
            favorites: None,
            health_status: None,
            last_attempted_profile_download_time: None,
            last_attempted_profile_upload_time: None,
            last_background_registry_upload_time: None,
            last_download_time: None,
            last_upload_time: None,
            last_use_time: None,
            links: None,
            loaded: None,
            local_path: None,
            music: None,
            pictures: None,
            ref_count: None,
            roaming_configured: None,
            roaming_path: None,
            roaming_preference: None,
            saved_games: None,
            searches: None,
            sid: None,
            special: None,
            start_menu: None,
            status: None,
            videos: None,
        }
    }


    /// Sets the value of AppDataRoaming
    pub fn set_app_data_roaming(&mut self, value: Win32_FolderRedirectionHealth) {
        self.app_data_roaming = Some(value);
    }

    /// Gets the value of AppDataRoaming
    pub fn get_app_data_roaming(&self) -> Option<&Win32_FolderRedirectionHealth> {
        self.app_data_roaming.as_ref()
    }

    /// Sets the value of Contacts
    pub fn set_contacts(&mut self, value: Win32_FolderRedirectionHealth) {
        self.contacts = Some(value);
    }

    /// Gets the value of Contacts
    pub fn get_contacts(&self) -> Option<&Win32_FolderRedirectionHealth> {
        self.contacts.as_ref()
    }

    /// Sets the value of Desktop
    pub fn set_desktop(&mut self, value: Win32_FolderRedirectionHealth) {
        self.desktop = Some(value);
    }

    /// Gets the value of Desktop
    pub fn get_desktop(&self) -> Option<&Win32_FolderRedirectionHealth> {
        self.desktop.as_ref()
    }

    /// Sets the value of Documents
    pub fn set_documents(&mut self, value: Win32_FolderRedirectionHealth) {
        self.documents = Some(value);
    }

    /// Gets the value of Documents
    pub fn get_documents(&self) -> Option<&Win32_FolderRedirectionHealth> {
        self.documents.as_ref()
    }

    /// Sets the value of Downloads
    pub fn set_downloads(&mut self, value: Win32_FolderRedirectionHealth) {
        self.downloads = Some(value);
    }

    /// Gets the value of Downloads
    pub fn get_downloads(&self) -> Option<&Win32_FolderRedirectionHealth> {
        self.downloads.as_ref()
    }

    /// Sets the value of Favorites
    pub fn set_favorites(&mut self, value: Win32_FolderRedirectionHealth) {
        self.favorites = Some(value);
    }

    /// Gets the value of Favorites
    pub fn get_favorites(&self) -> Option<&Win32_FolderRedirectionHealth> {
        self.favorites.as_ref()
    }

    /// Sets the value of HealthStatus
    pub fn set_health_status(&mut self, value: UserProfile_HealthStatus) {
        self.health_status = Some(value);
    }

    /// Gets the value of HealthStatus
    pub fn get_health_status(&self) -> Option<&UserProfile_HealthStatus> {
        self.health_status.as_ref()
    }

    /// Sets the value of LastAttemptedProfileDownloadTime
    pub fn set_last_attempted_profile_download_time(&mut self, value: String) {
        self.last_attempted_profile_download_time = Some(value);
    }

    /// Gets the value of LastAttemptedProfileDownloadTime
    pub fn get_last_attempted_profile_download_time(&self) -> Option<&String> {
        self.last_attempted_profile_download_time.as_ref()
    }

    /// Sets the value of LastAttemptedProfileUploadTime
    pub fn set_last_attempted_profile_upload_time(&mut self, value: String) {
        self.last_attempted_profile_upload_time = Some(value);
    }

    /// Gets the value of LastAttemptedProfileUploadTime
    pub fn get_last_attempted_profile_upload_time(&self) -> Option<&String> {
        self.last_attempted_profile_upload_time.as_ref()
    }

    /// Sets the value of LastBackgroundRegistryUploadTime
    pub fn set_last_background_registry_upload_time(&mut self, value: String) {
        self.last_background_registry_upload_time = Some(value);
    }

    /// Gets the value of LastBackgroundRegistryUploadTime
    pub fn get_last_background_registry_upload_time(&self) -> Option<&String> {
        self.last_background_registry_upload_time.as_ref()
    }

    /// Sets the value of LastDownloadTime
    pub fn set_last_download_time(&mut self, value: String) {
        self.last_download_time = Some(value);
    }

    /// Gets the value of LastDownloadTime
    pub fn get_last_download_time(&self) -> Option<&String> {
        self.last_download_time.as_ref()
    }

    /// Sets the value of LastUploadTime
    pub fn set_last_upload_time(&mut self, value: String) {
        self.last_upload_time = Some(value);
    }

    /// Gets the value of LastUploadTime
    pub fn get_last_upload_time(&self) -> Option<&String> {
        self.last_upload_time.as_ref()
    }

    /// Sets the value of LastUseTime
    pub fn set_last_use_time(&mut self, value: String) {
        self.last_use_time = Some(value);
    }

    /// Gets the value of LastUseTime
    pub fn get_last_use_time(&self) -> Option<&String> {
        self.last_use_time.as_ref()
    }

    /// Sets the value of Links
    pub fn set_links(&mut self, value: Win32_FolderRedirectionHealth) {
        self.links = Some(value);
    }

    /// Gets the value of Links
    pub fn get_links(&self) -> Option<&Win32_FolderRedirectionHealth> {
        self.links.as_ref()
    }

    /// Sets the value of Loaded
    pub fn set_loaded(&mut self, value: bool) {
        self.loaded = Some(value);
    }

    /// Gets the value of Loaded
    pub fn get_loaded(&self) -> Option<&bool> {
        self.loaded.as_ref()
    }

    /// Sets the value of LocalPath
    pub fn set_local_path(&mut self, value: String) {
        self.local_path = Some(value);
    }

    /// Gets the value of LocalPath
    pub fn get_local_path(&self) -> Option<&String> {
        self.local_path.as_ref()
    }

    /// Sets the value of Music
    pub fn set_music(&mut self, value: Win32_FolderRedirectionHealth) {
        self.music = Some(value);
    }

    /// Gets the value of Music
    pub fn get_music(&self) -> Option<&Win32_FolderRedirectionHealth> {
        self.music.as_ref()
    }

    /// Sets the value of Pictures
    pub fn set_pictures(&mut self, value: Win32_FolderRedirectionHealth) {
        self.pictures = Some(value);
    }

    /// Gets the value of Pictures
    pub fn get_pictures(&self) -> Option<&Win32_FolderRedirectionHealth> {
        self.pictures.as_ref()
    }

    /// Sets the value of RefCount
    pub fn set_ref_count(&mut self, value: u32) {
        self.ref_count = Some(value);
    }

    /// Gets the value of RefCount
    pub fn get_ref_count(&self) -> Option<&u32> {
        self.ref_count.as_ref()
    }

    /// Sets the value of RoamingConfigured
    pub fn set_roaming_configured(&mut self, value: bool) {
        self.roaming_configured = Some(value);
    }

    /// Gets the value of RoamingConfigured
    pub fn get_roaming_configured(&self) -> Option<&bool> {
        self.roaming_configured.as_ref()
    }

    /// Sets the value of RoamingPath
    pub fn set_roaming_path(&mut self, value: String) {
        self.roaming_path = Some(value);
    }

    /// Gets the value of RoamingPath
    pub fn get_roaming_path(&self) -> Option<&String> {
        self.roaming_path.as_ref()
    }

    /// Sets the value of RoamingPreference
    pub fn set_roaming_preference(&mut self, value: bool) {
        self.roaming_preference = Some(value);
    }

    /// Gets the value of RoamingPreference
    pub fn get_roaming_preference(&self) -> Option<&bool> {
        self.roaming_preference.as_ref()
    }

    /// Sets the value of SavedGames
    pub fn set_saved_games(&mut self, value: Win32_FolderRedirectionHealth) {
        self.saved_games = Some(value);
    }

    /// Gets the value of SavedGames
    pub fn get_saved_games(&self) -> Option<&Win32_FolderRedirectionHealth> {
        self.saved_games.as_ref()
    }

    /// Sets the value of Searches
    pub fn set_searches(&mut self, value: Win32_FolderRedirectionHealth) {
        self.searches = Some(value);
    }

    /// Gets the value of Searches
    pub fn get_searches(&self) -> Option<&Win32_FolderRedirectionHealth> {
        self.searches.as_ref()
    }

    /// Sets the value of SID
    pub fn set_sid(&mut self, value: String) {
        self.sid = Some(value);
    }

    /// Gets the value of SID
    pub fn get_sid(&self) -> Option<&String> {
        self.sid.as_ref()
    }

    /// Sets the value of Special
    pub fn set_special(&mut self, value: bool) {
        self.special = Some(value);
    }

    /// Gets the value of Special
    pub fn get_special(&self) -> Option<&bool> {
        self.special.as_ref()
    }

    /// Sets the value of StartMenu
    pub fn set_start_menu(&mut self, value: Win32_FolderRedirectionHealth) {
        self.start_menu = Some(value);
    }

    /// Gets the value of StartMenu
    pub fn get_start_menu(&self) -> Option<&Win32_FolderRedirectionHealth> {
        self.start_menu.as_ref()
    }

    /// Sets the value of Status
    pub fn set_status(&mut self, value: u32) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&u32> {
        self.status.as_ref()
    }

    /// Sets the value of Videos
    pub fn set_videos(&mut self, value: Win32_FolderRedirectionHealth) {
        self.videos = Some(value);
    }

    /// Gets the value of Videos
    pub fn get_videos(&self) -> Option<&Win32_FolderRedirectionHealth> {
        self.videos.as_ref()
    }

/// 

    /// * `flags` -  (u32)
    /// * `new_owner_sid` -  (String)

    /// * `return_value` -  (u32)
    pub fn change_owner(&self, new_owner_sid: &String, flags: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "NewOwnerSID".to_string(), value: new_owner_sid.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        self.invoke_method("ChangeOwner", &args)

    }

}

