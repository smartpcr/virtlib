// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_FolderRedirectionUserConfiguration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_FolderRedirectionUserConfiguration {

/// AppData\Roaming folder, GUID is {3EB685DB-65F9-4CF6-A03A-E3EF65729F3D}
    #[serde(rename = "AppDataRoaming")]
    pub app_data_roaming: Option<Win32_FolderRedirection>,

/// Contacts folder, GUID is {56784854-C6CB-462b-8169-88E350ACB882}
    #[serde(rename = "Contacts")]
    pub contacts: Option<Win32_FolderRedirection>,

/// Desktop folder, GUID is {B4BFCC3A-DB2C-424C-B029-7FE99A87C641}
    #[serde(rename = "Desktop")]
    pub desktop: Option<Win32_FolderRedirection>,

/// Documents folder, GUID is {FDD39AD0-238F-46AF-ADB4-6C85480369C7}
    #[serde(rename = "Documents")]
    pub documents: Option<Win32_FolderRedirection>,

/// Downloads folder, GUID is {374DE290-123F-4565-9164-39C4925E467B}
    #[serde(rename = "Downloads")]
    pub downloads: Option<Win32_FolderRedirection>,

/// Favorites folder, GUID is {1777F761-68AD-4D8A-87BD-30B759FA33DD}
    #[serde(rename = "Favorites")]
    pub favorites: Option<Win32_FolderRedirection>,

/// Indicates if the settings configured through this WMI class are taking affect.
    #[serde(rename = "IsConfiguredByWMI")]
    pub is_configured_by_wmi: Option<bool>,

/// Links folder, GUID is {BFB9D5E0-C6A9-404C-B2B2-AE6DB6AF4968}
    #[serde(rename = "Links")]
    pub links: Option<Win32_FolderRedirection>,

/// Music folder, GUID is {4BD8D571-6D19-48D3-BE97-422220080E43}
    #[serde(rename = "Music")]
    pub music: Option<Win32_FolderRedirection>,

/// Pictures folder, GUID is {33E28130-4E1E-4676-835A-98395C3BC3BB}
    #[serde(rename = "Pictures")]
    pub pictures: Option<Win32_FolderRedirection>,

/// The Primary Computer feature is enabled for this user
    #[serde(rename = "PrimaryComputerEnabled")]
    pub primary_computer_enabled: Option<bool>,

/// SavedGames folder, GUID is {4C5C32FF-BB9D-43b0-B5B4-2D72E54EAAA4}
    #[serde(rename = "SavedGames")]
    pub saved_games: Option<Win32_FolderRedirection>,

/// Searches folder, GUID is {7D1D3A04-DEBB-4115-95CF-2F29DA2920DA}
    #[serde(rename = "Searches")]
    pub searches: Option<Win32_FolderRedirection>,

/// Start Menu folder, GUID is {625B53C3-AB48-4EC1-BA1F-A1EF4146FC19}
    #[serde(rename = "StartMenu")]
    pub start_menu: Option<Win32_FolderRedirection>,

/// Videos folder, GUID is {18989B1D-99B5-455B-841C-AB7C74E4DDFC}
    #[serde(rename = "Videos")]
    pub videos: Option<Win32_FolderRedirection>,
}

impl Win32_FolderRedirectionUserConfiguration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            app_data_roaming: None,
            contacts: None,
            desktop: None,
            documents: None,
            downloads: None,
            favorites: None,
            is_configured_by_wmi: None,
            links: None,
            music: None,
            pictures: None,
            primary_computer_enabled: None,
            saved_games: None,
            searches: None,
            start_menu: None,
            videos: None,
        }
    }


    /// Sets the value of AppDataRoaming
    pub fn set_app_data_roaming(&mut self, value: Win32_FolderRedirection) {
        self.app_data_roaming = Some(value);
    }

    /// Gets the value of AppDataRoaming
    pub fn get_app_data_roaming(&self) -> Option<&Win32_FolderRedirection> {
        self.app_data_roaming.as_ref()
    }

    /// Sets the value of Contacts
    pub fn set_contacts(&mut self, value: Win32_FolderRedirection) {
        self.contacts = Some(value);
    }

    /// Gets the value of Contacts
    pub fn get_contacts(&self) -> Option<&Win32_FolderRedirection> {
        self.contacts.as_ref()
    }

    /// Sets the value of Desktop
    pub fn set_desktop(&mut self, value: Win32_FolderRedirection) {
        self.desktop = Some(value);
    }

    /// Gets the value of Desktop
    pub fn get_desktop(&self) -> Option<&Win32_FolderRedirection> {
        self.desktop.as_ref()
    }

    /// Sets the value of Documents
    pub fn set_documents(&mut self, value: Win32_FolderRedirection) {
        self.documents = Some(value);
    }

    /// Gets the value of Documents
    pub fn get_documents(&self) -> Option<&Win32_FolderRedirection> {
        self.documents.as_ref()
    }

    /// Sets the value of Downloads
    pub fn set_downloads(&mut self, value: Win32_FolderRedirection) {
        self.downloads = Some(value);
    }

    /// Gets the value of Downloads
    pub fn get_downloads(&self) -> Option<&Win32_FolderRedirection> {
        self.downloads.as_ref()
    }

    /// Sets the value of Favorites
    pub fn set_favorites(&mut self, value: Win32_FolderRedirection) {
        self.favorites = Some(value);
    }

    /// Gets the value of Favorites
    pub fn get_favorites(&self) -> Option<&Win32_FolderRedirection> {
        self.favorites.as_ref()
    }

    /// Sets the value of IsConfiguredByWMI
    pub fn set_is_configured_by_wmi(&mut self, value: bool) {
        self.is_configured_by_wmi = Some(value);
    }

    /// Gets the value of IsConfiguredByWMI
    pub fn get_is_configured_by_wmi(&self) -> Option<&bool> {
        self.is_configured_by_wmi.as_ref()
    }

    /// Sets the value of Links
    pub fn set_links(&mut self, value: Win32_FolderRedirection) {
        self.links = Some(value);
    }

    /// Gets the value of Links
    pub fn get_links(&self) -> Option<&Win32_FolderRedirection> {
        self.links.as_ref()
    }

    /// Sets the value of Music
    pub fn set_music(&mut self, value: Win32_FolderRedirection) {
        self.music = Some(value);
    }

    /// Gets the value of Music
    pub fn get_music(&self) -> Option<&Win32_FolderRedirection> {
        self.music.as_ref()
    }

    /// Sets the value of Pictures
    pub fn set_pictures(&mut self, value: Win32_FolderRedirection) {
        self.pictures = Some(value);
    }

    /// Gets the value of Pictures
    pub fn get_pictures(&self) -> Option<&Win32_FolderRedirection> {
        self.pictures.as_ref()
    }

    /// Sets the value of PrimaryComputerEnabled
    pub fn set_primary_computer_enabled(&mut self, value: bool) {
        self.primary_computer_enabled = Some(value);
    }

    /// Gets the value of PrimaryComputerEnabled
    pub fn get_primary_computer_enabled(&self) -> Option<&bool> {
        self.primary_computer_enabled.as_ref()
    }

    /// Sets the value of SavedGames
    pub fn set_saved_games(&mut self, value: Win32_FolderRedirection) {
        self.saved_games = Some(value);
    }

    /// Gets the value of SavedGames
    pub fn get_saved_games(&self) -> Option<&Win32_FolderRedirection> {
        self.saved_games.as_ref()
    }

    /// Sets the value of Searches
    pub fn set_searches(&mut self, value: Win32_FolderRedirection) {
        self.searches = Some(value);
    }

    /// Gets the value of Searches
    pub fn get_searches(&self) -> Option<&Win32_FolderRedirection> {
        self.searches.as_ref()
    }

    /// Sets the value of StartMenu
    pub fn set_start_menu(&mut self, value: Win32_FolderRedirection) {
        self.start_menu = Some(value);
    }

    /// Gets the value of StartMenu
    pub fn get_start_menu(&self) -> Option<&Win32_FolderRedirection> {
        self.start_menu.as_ref()
    }

    /// Sets the value of Videos
    pub fn set_videos(&mut self, value: Win32_FolderRedirection) {
        self.videos = Some(value);
    }

    /// Gets the value of Videos
    pub fn get_videos(&self) -> Option<&Win32_FolderRedirection> {
        self.videos.as_ref()
    }
}

