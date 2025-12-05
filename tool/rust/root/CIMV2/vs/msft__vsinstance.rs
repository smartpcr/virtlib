// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.vs
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_VSInstance struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_VSInstance {
    #[serde(flatten)]
    pub base: CIM_Product,

/// ID of the channel where the product was installed from.
    #[serde(rename = "ChannelId")]
    pub channel_id: Option<String>,

/// URI or path to the channel manifest to use for updates.
    #[serde(rename = "ChannelUri")]
    pub channel_uri: Option<String>,

/// Date and time the product was installed.
    #[serde(rename = "InstallDate")]
    pub install_date: Option<String>,

/// Location where the product is installed.
    #[serde(rename = "InstallLocation")]
    pub install_location: Option<String>,

/// Whether all components are successfully installed.
    #[serde(rename = "IsComplete")]
    pub is_complete: Option<bool>,

/// Whether the product is launchable, though some components may not be completely installed.
    #[serde(rename = "IsLaunchable")]
    pub is_launchable: Option<bool>,

/// Whether the product is a prerelease.
    #[serde(rename = "IsPrerelease")]
    pub is_prerelease: Option<bool>,

/// Path to the layout directory.
    #[serde(rename = "LayoutPath")]
    pub layout_path: Option<String>,

/// Id of the product.
    #[serde(rename = "ProductId")]
    pub product_id: Option<String>,

/// Location of the primary product application if any defined.
    #[serde(rename = "ProductLocation")]
    pub product_location: Option<String>,

/// State of the instance.
    #[serde(rename = "State")]
    pub state: Option<u32>,
}

impl MSFT_VSInstance {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Product::new(),
            channel_id: None,
            channel_uri: None,
            install_date: None,
            install_location: None,
            is_complete: None,
            is_launchable: None,
            is_prerelease: None,
            layout_path: None,
            product_id: None,
            product_location: None,
            state: None,
        }
    }


    /// Sets the value of ChannelId
    pub fn set_channel_id(&mut self, value: String) {
        self.channel_id = Some(value);
    }

    /// Gets the value of ChannelId
    pub fn get_channel_id(&self) -> Option<&String> {
        self.channel_id.as_ref()
    }

    /// Sets the value of ChannelUri
    pub fn set_channel_uri(&mut self, value: String) {
        self.channel_uri = Some(value);
    }

    /// Gets the value of ChannelUri
    pub fn get_channel_uri(&self) -> Option<&String> {
        self.channel_uri.as_ref()
    }

    /// Sets the value of InstallDate
    pub fn set_install_date(&mut self, value: String) {
        self.install_date = Some(value);
    }

    /// Gets the value of InstallDate
    pub fn get_install_date(&self) -> Option<&String> {
        self.install_date.as_ref()
    }

    /// Sets the value of InstallLocation
    pub fn set_install_location(&mut self, value: String) {
        self.install_location = Some(value);
    }

    /// Gets the value of InstallLocation
    pub fn get_install_location(&self) -> Option<&String> {
        self.install_location.as_ref()
    }

    /// Sets the value of IsComplete
    pub fn set_is_complete(&mut self, value: bool) {
        self.is_complete = Some(value);
    }

    /// Gets the value of IsComplete
    pub fn get_is_complete(&self) -> Option<&bool> {
        self.is_complete.as_ref()
    }

    /// Sets the value of IsLaunchable
    pub fn set_is_launchable(&mut self, value: bool) {
        self.is_launchable = Some(value);
    }

    /// Gets the value of IsLaunchable
    pub fn get_is_launchable(&self) -> Option<&bool> {
        self.is_launchable.as_ref()
    }

    /// Sets the value of IsPrerelease
    pub fn set_is_prerelease(&mut self, value: bool) {
        self.is_prerelease = Some(value);
    }

    /// Gets the value of IsPrerelease
    pub fn get_is_prerelease(&self) -> Option<&bool> {
        self.is_prerelease.as_ref()
    }

    /// Sets the value of LayoutPath
    pub fn set_layout_path(&mut self, value: String) {
        self.layout_path = Some(value);
    }

    /// Gets the value of LayoutPath
    pub fn get_layout_path(&self) -> Option<&String> {
        self.layout_path.as_ref()
    }

    /// Sets the value of ProductId
    pub fn set_product_id(&mut self, value: String) {
        self.product_id = Some(value);
    }

    /// Gets the value of ProductId
    pub fn get_product_id(&self) -> Option<&String> {
        self.product_id.as_ref()
    }

    /// Sets the value of ProductLocation
    pub fn set_product_location(&mut self, value: String) {
        self.product_location = Some(value);
    }

    /// Gets the value of ProductLocation
    pub fn get_product_location(&self) -> Option<&String> {
        self.product_location.as_ref()
    }

    /// Sets the value of State
    pub fn set_state(&mut self, value: u32) {
        self.state = Some(value);
    }

    /// Gets the value of State
    pub fn get_state(&self) -> Option<&u32> {
        self.state.as_ref()
    }
}

