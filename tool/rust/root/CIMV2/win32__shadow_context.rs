// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_ShadowContext struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_ShadowContext {
    #[serde(flatten)]
    pub base: CIM_Setting,

/// 
    #[serde(rename = "ClientAccessible")]
    pub client_accessible: Option<bool>,

/// 
    #[serde(rename = "Differential")]
    pub differential: Option<bool>,

/// 
    #[serde(rename = "ExposedLocally")]
    pub exposed_locally: Option<bool>,

/// 
    #[serde(rename = "ExposedRemotely")]
    pub exposed_remotely: Option<bool>,

/// 
    #[serde(rename = "HardwareAssisted")]
    pub hardware_assisted: Option<bool>,

/// 
    #[serde(rename = "Imported")]
    pub imported: Option<bool>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "NoAutoRelease")]
    pub no_auto_release: Option<bool>,

/// 
    #[serde(rename = "NotSurfaced")]
    pub not_surfaced: Option<bool>,

/// 
    #[serde(rename = "NoWriters")]
    pub no_writers: Option<bool>,

/// 
    #[serde(rename = "Persistent")]
    pub persistent: Option<bool>,

/// 
    #[serde(rename = "Plex")]
    pub plex: Option<bool>,

/// 
    #[serde(rename = "Transportable")]
    pub transportable: Option<bool>,
}

impl Win32_ShadowContext {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Setting::new(),
            client_accessible: None,
            differential: None,
            exposed_locally: None,
            exposed_remotely: None,
            hardware_assisted: None,
            imported: None,
            name: None,
            no_auto_release: None,
            not_surfaced: None,
            no_writers: None,
            persistent: None,
            plex: None,
            transportable: None,
        }
    }


    /// Sets the value of ClientAccessible
    pub fn set_client_accessible(&mut self, value: bool) {
        self.client_accessible = Some(value);
    }

    /// Gets the value of ClientAccessible
    pub fn get_client_accessible(&self) -> Option<&bool> {
        self.client_accessible.as_ref()
    }

    /// Sets the value of Differential
    pub fn set_differential(&mut self, value: bool) {
        self.differential = Some(value);
    }

    /// Gets the value of Differential
    pub fn get_differential(&self) -> Option<&bool> {
        self.differential.as_ref()
    }

    /// Sets the value of ExposedLocally
    pub fn set_exposed_locally(&mut self, value: bool) {
        self.exposed_locally = Some(value);
    }

    /// Gets the value of ExposedLocally
    pub fn get_exposed_locally(&self) -> Option<&bool> {
        self.exposed_locally.as_ref()
    }

    /// Sets the value of ExposedRemotely
    pub fn set_exposed_remotely(&mut self, value: bool) {
        self.exposed_remotely = Some(value);
    }

    /// Gets the value of ExposedRemotely
    pub fn get_exposed_remotely(&self) -> Option<&bool> {
        self.exposed_remotely.as_ref()
    }

    /// Sets the value of HardwareAssisted
    pub fn set_hardware_assisted(&mut self, value: bool) {
        self.hardware_assisted = Some(value);
    }

    /// Gets the value of HardwareAssisted
    pub fn get_hardware_assisted(&self) -> Option<&bool> {
        self.hardware_assisted.as_ref()
    }

    /// Sets the value of Imported
    pub fn set_imported(&mut self, value: bool) {
        self.imported = Some(value);
    }

    /// Gets the value of Imported
    pub fn get_imported(&self) -> Option<&bool> {
        self.imported.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of NoAutoRelease
    pub fn set_no_auto_release(&mut self, value: bool) {
        self.no_auto_release = Some(value);
    }

    /// Gets the value of NoAutoRelease
    pub fn get_no_auto_release(&self) -> Option<&bool> {
        self.no_auto_release.as_ref()
    }

    /// Sets the value of NotSurfaced
    pub fn set_not_surfaced(&mut self, value: bool) {
        self.not_surfaced = Some(value);
    }

    /// Gets the value of NotSurfaced
    pub fn get_not_surfaced(&self) -> Option<&bool> {
        self.not_surfaced.as_ref()
    }

    /// Sets the value of NoWriters
    pub fn set_no_writers(&mut self, value: bool) {
        self.no_writers = Some(value);
    }

    /// Gets the value of NoWriters
    pub fn get_no_writers(&self) -> Option<&bool> {
        self.no_writers.as_ref()
    }

    /// Sets the value of Persistent
    pub fn set_persistent(&mut self, value: bool) {
        self.persistent = Some(value);
    }

    /// Gets the value of Persistent
    pub fn get_persistent(&self) -> Option<&bool> {
        self.persistent.as_ref()
    }

    /// Sets the value of Plex
    pub fn set_plex(&mut self, value: bool) {
        self.plex = Some(value);
    }

    /// Gets the value of Plex
    pub fn get_plex(&self) -> Option<&bool> {
        self.plex.as_ref()
    }

    /// Sets the value of Transportable
    pub fn set_transportable(&mut self, value: bool) {
        self.transportable = Some(value);
    }

    /// Gets the value of Transportable
    pub fn get_transportable(&self) -> Option<&bool> {
        self.transportable.as_ref()
    }
}

