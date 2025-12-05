// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_IEAKPolicySetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_IEAKPolicySetting {
    #[serde(flatten)]
    pub base: RSOP_PolicySetting,

/// 
    #[serde(rename = "categories")]
    pub categories: Option<i32>,

/// 
    #[serde(rename = "channels")]
    pub channels: Option<i32>,

/// 
    #[serde(rename = "customFavorites")]
    pub custom_favorites: Option<i32>,

/// 
    #[serde(rename = "customizeAnimatedBitmaps")]
    pub customize_animated_bitmaps: Option<bool>,

/// 
    #[serde(rename = "customizeLogoBitmaps")]
    pub customize_logo_bitmaps: Option<bool>,

/// 
    #[serde(rename = "customLinks")]
    pub custom_links: Option<i32>,

/// 
    #[serde(rename = "deleteAdminCreatedFavoritesOnly")]
    pub delete_admin_created_favorites_only: Option<bool>,

/// 
    #[serde(rename = "deleteExistingChannels")]
    pub delete_existing_channels: Option<bool>,

/// 
    #[serde(rename = "deleteExistingFavorites")]
    pub delete_existing_favorites: Option<bool>,

/// 
    #[serde(rename = "deleteExistingToolbarButtons")]
    pub delete_existing_toolbar_buttons: Option<bool>,

/// 
    #[serde(rename = "enableDesktopChannelBarByDefault")]
    pub enable_desktop_channel_bar_by_default: Option<bool>,

/// 
    #[serde(rename = "enableTrustedPublisherLockdown")]
    pub enable_trusted_publisher_lockdown: Option<bool>,

/// 
    #[serde(rename = "homePageURL")]
    pub home_page_url: Option<String>,

/// 
    #[serde(rename = "importAuthenticodeSecurityInfo")]
    pub import_authenticode_security_info: Option<bool>,

/// 
    #[serde(rename = "importContentRatingsSettings")]
    pub import_content_ratings_settings: Option<bool>,

/// 
    #[serde(rename = "importedZoneCount")]
    pub imported_zone_count: Option<u32>,

/// 
    #[serde(rename = "importProgramSettings")]
    pub import_program_settings: Option<bool>,

/// 
    #[serde(rename = "importSecurityZoneSettings")]
    pub import_security_zone_settings: Option<bool>,

/// 
    #[serde(rename = "largeAnimatedBitmapName")]
    pub large_animated_bitmap_name: Option<String>,

/// 
    #[serde(rename = "largeAnimatedBitmapPath")]
    pub large_animated_bitmap_path: Option<String>,

/// 
    #[serde(rename = "largeCustomLogoBitmapName")]
    pub large_custom_logo_bitmap_name: Option<String>,

/// 
    #[serde(rename = "largeCustomLogoBitmapPath")]
    pub large_custom_logo_bitmap_path: Option<String>,

/// 
    #[serde(rename = "onlineHelpPageURL")]
    pub online_help_page_url: Option<String>,

/// 
    #[serde(rename = "placeFavoritesAtTopOfList")]
    pub place_favorites_at_top_of_list: Option<bool>,

/// 
    #[serde(rename = "preferenceMode")]
    pub preference_mode: Option<bool>,

/// 
    #[serde(rename = "searchBarURL")]
    pub search_bar_url: Option<String>,

/// 
    #[serde(rename = "smallAnimatedBitmapName")]
    pub small_animated_bitmap_name: Option<String>,

/// 
    #[serde(rename = "smallAnimatedBitmapPath")]
    pub small_animated_bitmap_path: Option<String>,

/// 
    #[serde(rename = "smallCustomLogoBitmapName")]
    pub small_custom_logo_bitmap_name: Option<String>,

/// 
    #[serde(rename = "smallCustomLogoBitmapPath")]
    pub small_custom_logo_bitmap_path: Option<String>,

/// 
    #[serde(rename = "titleBarCustomText")]
    pub title_bar_custom_text: Option<String>,

/// 
    #[serde(rename = "titleBarText")]
    pub title_bar_text: Option<String>,

/// 
    #[serde(rename = "toolbarBackgroundBitmapPath")]
    pub toolbar_background_bitmap_path: Option<String>,

/// 
    #[serde(rename = "toolbarButtons")]
    pub toolbar_buttons: Option<i32>,

/// 
    #[serde(rename = "userAgentText")]
    pub user_agent_text: Option<String>,
}

impl RSOP_IEAKPolicySetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: RSOP_PolicySetting::new(),
            categories: None,
            channels: None,
            custom_favorites: None,
            customize_animated_bitmaps: None,
            customize_logo_bitmaps: None,
            custom_links: None,
            delete_admin_created_favorites_only: None,
            delete_existing_channels: None,
            delete_existing_favorites: None,
            delete_existing_toolbar_buttons: None,
            enable_desktop_channel_bar_by_default: None,
            enable_trusted_publisher_lockdown: None,
            home_page_url: None,
            import_authenticode_security_info: None,
            import_content_ratings_settings: None,
            imported_zone_count: None,
            import_program_settings: None,
            import_security_zone_settings: None,
            large_animated_bitmap_name: None,
            large_animated_bitmap_path: None,
            large_custom_logo_bitmap_name: None,
            large_custom_logo_bitmap_path: None,
            online_help_page_url: None,
            place_favorites_at_top_of_list: None,
            preference_mode: None,
            search_bar_url: None,
            small_animated_bitmap_name: None,
            small_animated_bitmap_path: None,
            small_custom_logo_bitmap_name: None,
            small_custom_logo_bitmap_path: None,
            title_bar_custom_text: None,
            title_bar_text: None,
            toolbar_background_bitmap_path: None,
            toolbar_buttons: None,
            user_agent_text: None,
        }
    }


    /// Sets the value of categories
    pub fn set_categories(&mut self, value: i32) {
        self.categories = Some(value);
    }

    /// Gets the value of categories
    pub fn get_categories(&self) -> Option<&i32> {
        self.categories.as_ref()
    }

    /// Sets the value of channels
    pub fn set_channels(&mut self, value: i32) {
        self.channels = Some(value);
    }

    /// Gets the value of channels
    pub fn get_channels(&self) -> Option<&i32> {
        self.channels.as_ref()
    }

    /// Sets the value of customFavorites
    pub fn set_custom_favorites(&mut self, value: i32) {
        self.custom_favorites = Some(value);
    }

    /// Gets the value of customFavorites
    pub fn get_custom_favorites(&self) -> Option<&i32> {
        self.custom_favorites.as_ref()
    }

    /// Sets the value of customizeAnimatedBitmaps
    pub fn set_customize_animated_bitmaps(&mut self, value: bool) {
        self.customize_animated_bitmaps = Some(value);
    }

    /// Gets the value of customizeAnimatedBitmaps
    pub fn get_customize_animated_bitmaps(&self) -> Option<&bool> {
        self.customize_animated_bitmaps.as_ref()
    }

    /// Sets the value of customizeLogoBitmaps
    pub fn set_customize_logo_bitmaps(&mut self, value: bool) {
        self.customize_logo_bitmaps = Some(value);
    }

    /// Gets the value of customizeLogoBitmaps
    pub fn get_customize_logo_bitmaps(&self) -> Option<&bool> {
        self.customize_logo_bitmaps.as_ref()
    }

    /// Sets the value of customLinks
    pub fn set_custom_links(&mut self, value: i32) {
        self.custom_links = Some(value);
    }

    /// Gets the value of customLinks
    pub fn get_custom_links(&self) -> Option<&i32> {
        self.custom_links.as_ref()
    }

    /// Sets the value of deleteAdminCreatedFavoritesOnly
    pub fn set_delete_admin_created_favorites_only(&mut self, value: bool) {
        self.delete_admin_created_favorites_only = Some(value);
    }

    /// Gets the value of deleteAdminCreatedFavoritesOnly
    pub fn get_delete_admin_created_favorites_only(&self) -> Option<&bool> {
        self.delete_admin_created_favorites_only.as_ref()
    }

    /// Sets the value of deleteExistingChannels
    pub fn set_delete_existing_channels(&mut self, value: bool) {
        self.delete_existing_channels = Some(value);
    }

    /// Gets the value of deleteExistingChannels
    pub fn get_delete_existing_channels(&self) -> Option<&bool> {
        self.delete_existing_channels.as_ref()
    }

    /// Sets the value of deleteExistingFavorites
    pub fn set_delete_existing_favorites(&mut self, value: bool) {
        self.delete_existing_favorites = Some(value);
    }

    /// Gets the value of deleteExistingFavorites
    pub fn get_delete_existing_favorites(&self) -> Option<&bool> {
        self.delete_existing_favorites.as_ref()
    }

    /// Sets the value of deleteExistingToolbarButtons
    pub fn set_delete_existing_toolbar_buttons(&mut self, value: bool) {
        self.delete_existing_toolbar_buttons = Some(value);
    }

    /// Gets the value of deleteExistingToolbarButtons
    pub fn get_delete_existing_toolbar_buttons(&self) -> Option<&bool> {
        self.delete_existing_toolbar_buttons.as_ref()
    }

    /// Sets the value of enableDesktopChannelBarByDefault
    pub fn set_enable_desktop_channel_bar_by_default(&mut self, value: bool) {
        self.enable_desktop_channel_bar_by_default = Some(value);
    }

    /// Gets the value of enableDesktopChannelBarByDefault
    pub fn get_enable_desktop_channel_bar_by_default(&self) -> Option<&bool> {
        self.enable_desktop_channel_bar_by_default.as_ref()
    }

    /// Sets the value of enableTrustedPublisherLockdown
    pub fn set_enable_trusted_publisher_lockdown(&mut self, value: bool) {
        self.enable_trusted_publisher_lockdown = Some(value);
    }

    /// Gets the value of enableTrustedPublisherLockdown
    pub fn get_enable_trusted_publisher_lockdown(&self) -> Option<&bool> {
        self.enable_trusted_publisher_lockdown.as_ref()
    }

    /// Sets the value of homePageURL
    pub fn set_home_page_url(&mut self, value: String) {
        self.home_page_url = Some(value);
    }

    /// Gets the value of homePageURL
    pub fn get_home_page_url(&self) -> Option<&String> {
        self.home_page_url.as_ref()
    }

    /// Sets the value of importAuthenticodeSecurityInfo
    pub fn set_import_authenticode_security_info(&mut self, value: bool) {
        self.import_authenticode_security_info = Some(value);
    }

    /// Gets the value of importAuthenticodeSecurityInfo
    pub fn get_import_authenticode_security_info(&self) -> Option<&bool> {
        self.import_authenticode_security_info.as_ref()
    }

    /// Sets the value of importContentRatingsSettings
    pub fn set_import_content_ratings_settings(&mut self, value: bool) {
        self.import_content_ratings_settings = Some(value);
    }

    /// Gets the value of importContentRatingsSettings
    pub fn get_import_content_ratings_settings(&self) -> Option<&bool> {
        self.import_content_ratings_settings.as_ref()
    }

    /// Sets the value of importedZoneCount
    pub fn set_imported_zone_count(&mut self, value: u32) {
        self.imported_zone_count = Some(value);
    }

    /// Gets the value of importedZoneCount
    pub fn get_imported_zone_count(&self) -> Option<&u32> {
        self.imported_zone_count.as_ref()
    }

    /// Sets the value of importProgramSettings
    pub fn set_import_program_settings(&mut self, value: bool) {
        self.import_program_settings = Some(value);
    }

    /// Gets the value of importProgramSettings
    pub fn get_import_program_settings(&self) -> Option<&bool> {
        self.import_program_settings.as_ref()
    }

    /// Sets the value of importSecurityZoneSettings
    pub fn set_import_security_zone_settings(&mut self, value: bool) {
        self.import_security_zone_settings = Some(value);
    }

    /// Gets the value of importSecurityZoneSettings
    pub fn get_import_security_zone_settings(&self) -> Option<&bool> {
        self.import_security_zone_settings.as_ref()
    }

    /// Sets the value of largeAnimatedBitmapName
    pub fn set_large_animated_bitmap_name(&mut self, value: String) {
        self.large_animated_bitmap_name = Some(value);
    }

    /// Gets the value of largeAnimatedBitmapName
    pub fn get_large_animated_bitmap_name(&self) -> Option<&String> {
        self.large_animated_bitmap_name.as_ref()
    }

    /// Sets the value of largeAnimatedBitmapPath
    pub fn set_large_animated_bitmap_path(&mut self, value: String) {
        self.large_animated_bitmap_path = Some(value);
    }

    /// Gets the value of largeAnimatedBitmapPath
    pub fn get_large_animated_bitmap_path(&self) -> Option<&String> {
        self.large_animated_bitmap_path.as_ref()
    }

    /// Sets the value of largeCustomLogoBitmapName
    pub fn set_large_custom_logo_bitmap_name(&mut self, value: String) {
        self.large_custom_logo_bitmap_name = Some(value);
    }

    /// Gets the value of largeCustomLogoBitmapName
    pub fn get_large_custom_logo_bitmap_name(&self) -> Option<&String> {
        self.large_custom_logo_bitmap_name.as_ref()
    }

    /// Sets the value of largeCustomLogoBitmapPath
    pub fn set_large_custom_logo_bitmap_path(&mut self, value: String) {
        self.large_custom_logo_bitmap_path = Some(value);
    }

    /// Gets the value of largeCustomLogoBitmapPath
    pub fn get_large_custom_logo_bitmap_path(&self) -> Option<&String> {
        self.large_custom_logo_bitmap_path.as_ref()
    }

    /// Sets the value of onlineHelpPageURL
    pub fn set_online_help_page_url(&mut self, value: String) {
        self.online_help_page_url = Some(value);
    }

    /// Gets the value of onlineHelpPageURL
    pub fn get_online_help_page_url(&self) -> Option<&String> {
        self.online_help_page_url.as_ref()
    }

    /// Sets the value of placeFavoritesAtTopOfList
    pub fn set_place_favorites_at_top_of_list(&mut self, value: bool) {
        self.place_favorites_at_top_of_list = Some(value);
    }

    /// Gets the value of placeFavoritesAtTopOfList
    pub fn get_place_favorites_at_top_of_list(&self) -> Option<&bool> {
        self.place_favorites_at_top_of_list.as_ref()
    }

    /// Sets the value of preferenceMode
    pub fn set_preference_mode(&mut self, value: bool) {
        self.preference_mode = Some(value);
    }

    /// Gets the value of preferenceMode
    pub fn get_preference_mode(&self) -> Option<&bool> {
        self.preference_mode.as_ref()
    }

    /// Sets the value of searchBarURL
    pub fn set_search_bar_url(&mut self, value: String) {
        self.search_bar_url = Some(value);
    }

    /// Gets the value of searchBarURL
    pub fn get_search_bar_url(&self) -> Option<&String> {
        self.search_bar_url.as_ref()
    }

    /// Sets the value of smallAnimatedBitmapName
    pub fn set_small_animated_bitmap_name(&mut self, value: String) {
        self.small_animated_bitmap_name = Some(value);
    }

    /// Gets the value of smallAnimatedBitmapName
    pub fn get_small_animated_bitmap_name(&self) -> Option<&String> {
        self.small_animated_bitmap_name.as_ref()
    }

    /// Sets the value of smallAnimatedBitmapPath
    pub fn set_small_animated_bitmap_path(&mut self, value: String) {
        self.small_animated_bitmap_path = Some(value);
    }

    /// Gets the value of smallAnimatedBitmapPath
    pub fn get_small_animated_bitmap_path(&self) -> Option<&String> {
        self.small_animated_bitmap_path.as_ref()
    }

    /// Sets the value of smallCustomLogoBitmapName
    pub fn set_small_custom_logo_bitmap_name(&mut self, value: String) {
        self.small_custom_logo_bitmap_name = Some(value);
    }

    /// Gets the value of smallCustomLogoBitmapName
    pub fn get_small_custom_logo_bitmap_name(&self) -> Option<&String> {
        self.small_custom_logo_bitmap_name.as_ref()
    }

    /// Sets the value of smallCustomLogoBitmapPath
    pub fn set_small_custom_logo_bitmap_path(&mut self, value: String) {
        self.small_custom_logo_bitmap_path = Some(value);
    }

    /// Gets the value of smallCustomLogoBitmapPath
    pub fn get_small_custom_logo_bitmap_path(&self) -> Option<&String> {
        self.small_custom_logo_bitmap_path.as_ref()
    }

    /// Sets the value of titleBarCustomText
    pub fn set_title_bar_custom_text(&mut self, value: String) {
        self.title_bar_custom_text = Some(value);
    }

    /// Gets the value of titleBarCustomText
    pub fn get_title_bar_custom_text(&self) -> Option<&String> {
        self.title_bar_custom_text.as_ref()
    }

    /// Sets the value of titleBarText
    pub fn set_title_bar_text(&mut self, value: String) {
        self.title_bar_text = Some(value);
    }

    /// Gets the value of titleBarText
    pub fn get_title_bar_text(&self) -> Option<&String> {
        self.title_bar_text.as_ref()
    }

    /// Sets the value of toolbarBackgroundBitmapPath
    pub fn set_toolbar_background_bitmap_path(&mut self, value: String) {
        self.toolbar_background_bitmap_path = Some(value);
    }

    /// Gets the value of toolbarBackgroundBitmapPath
    pub fn get_toolbar_background_bitmap_path(&self) -> Option<&String> {
        self.toolbar_background_bitmap_path.as_ref()
    }

    /// Sets the value of toolbarButtons
    pub fn set_toolbar_buttons(&mut self, value: i32) {
        self.toolbar_buttons = Some(value);
    }

    /// Gets the value of toolbarButtons
    pub fn get_toolbar_buttons(&self) -> Option<&i32> {
        self.toolbar_buttons.as_ref()
    }

    /// Sets the value of userAgentText
    pub fn set_user_agent_text(&mut self, value: String) {
        self.user_agent_text = Some(value);
    }

    /// Gets the value of userAgentText
    pub fn get_user_agent_text(&self) -> Option<&String> {
        self.user_agent_text.as_ref()
    }
}

