// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_IESecurityContentRatings struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_IESecurityContentRatings {

/// 
    #[serde(rename = "alwaysViewableSites")]
    pub always_viewable_sites: Vec<String>,

/// 
    #[serde(rename = "neverViewableSites")]
    pub never_viewable_sites: Vec<String>,

/// 
    #[serde(rename = "passwordOverrideEnabled")]
    pub password_override_enabled: Option<bool>,

/// 
    #[serde(rename = "ratingSystemFileNames")]
    pub rating_system_file_names: Vec<String>,

/// 
    #[serde(rename = "ratingSystems")]
    pub rating_systems: Vec<String>,

/// 
    #[serde(rename = "rsopID")]
    pub rsop_id: Option<String>,

/// 
    #[serde(rename = "rsopPrecedence")]
    pub rsop_precedence: Option<i32>,

/// 
    #[serde(rename = "selectedRatingsBureau")]
    pub selected_ratings_bureau: Option<String>,

/// 
    #[serde(rename = "viewUnknownRatedSites")]
    pub view_unknown_rated_sites: Option<bool>,
}

impl RSOP_IESecurityContentRatings {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            always_viewable_sites: Vec::new(),
            never_viewable_sites: Vec::new(),
            password_override_enabled: None,
            rating_system_file_names: Vec::new(),
            rating_systems: Vec::new(),
            rsop_id: None,
            rsop_precedence: None,
            selected_ratings_bureau: None,
            view_unknown_rated_sites: None,
        }
    }


    /// Sets the value of alwaysViewableSites
    pub fn set_always_viewable_sites(&mut self, value: Vec<String>) {
        self.always_viewable_sites = value;
    }

    /// Gets the value of alwaysViewableSites
    pub fn get_always_viewable_sites(&self) -> &Vec<String> {
        &self.always_viewable_sites
    }

    /// Sets the value of neverViewableSites
    pub fn set_never_viewable_sites(&mut self, value: Vec<String>) {
        self.never_viewable_sites = value;
    }

    /// Gets the value of neverViewableSites
    pub fn get_never_viewable_sites(&self) -> &Vec<String> {
        &self.never_viewable_sites
    }

    /// Sets the value of passwordOverrideEnabled
    pub fn set_password_override_enabled(&mut self, value: bool) {
        self.password_override_enabled = Some(value);
    }

    /// Gets the value of passwordOverrideEnabled
    pub fn get_password_override_enabled(&self) -> Option<&bool> {
        self.password_override_enabled.as_ref()
    }

    /// Sets the value of ratingSystemFileNames
    pub fn set_rating_system_file_names(&mut self, value: Vec<String>) {
        self.rating_system_file_names = value;
    }

    /// Gets the value of ratingSystemFileNames
    pub fn get_rating_system_file_names(&self) -> &Vec<String> {
        &self.rating_system_file_names
    }

    /// Sets the value of ratingSystems
    pub fn set_rating_systems(&mut self, value: Vec<String>) {
        self.rating_systems = value;
    }

    /// Gets the value of ratingSystems
    pub fn get_rating_systems(&self) -> &Vec<String> {
        &self.rating_systems
    }

    /// Sets the value of rsopID
    pub fn set_rsop_id(&mut self, value: String) {
        self.rsop_id = Some(value);
    }

    /// Gets the value of rsopID
    pub fn get_rsop_id(&self) -> Option<&String> {
        self.rsop_id.as_ref()
    }

    /// Sets the value of rsopPrecedence
    pub fn set_rsop_precedence(&mut self, value: i32) {
        self.rsop_precedence = Some(value);
    }

    /// Gets the value of rsopPrecedence
    pub fn get_rsop_precedence(&self) -> Option<&i32> {
        self.rsop_precedence.as_ref()
    }

    /// Sets the value of selectedRatingsBureau
    pub fn set_selected_ratings_bureau(&mut self, value: String) {
        self.selected_ratings_bureau = Some(value);
    }

    /// Gets the value of selectedRatingsBureau
    pub fn get_selected_ratings_bureau(&self) -> Option<&String> {
        self.selected_ratings_bureau.as_ref()
    }

    /// Sets the value of viewUnknownRatedSites
    pub fn set_view_unknown_rated_sites(&mut self, value: bool) {
        self.view_unknown_rated_sites = Some(value);
    }

    /// Gets the value of viewUnknownRatedSites
    pub fn get_view_unknown_rated_sites(&self) -> Option<&bool> {
        self.view_unknown_rated_sites.as_ref()
    }
}

