// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_ProcessStartup struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_ProcessStartup {
    #[serde(flatten)]
    pub base: Win32_MethodParameterClass,

/// 
    #[serde(rename = "CreateFlags")]
    pub create_flags: Option<u32>,

/// 
    #[serde(rename = "EnvironmentVariables")]
    pub environment_variables: Vec<String>,

/// 
    #[serde(rename = "ErrorMode")]
    pub error_mode: Option<u16>,

/// 
    #[serde(rename = "FillAttribute")]
    pub fill_attribute: Option<u32>,

/// 
    #[serde(rename = "PriorityClass")]
    pub priority_class: Option<u32>,

/// 
    #[serde(rename = "ShowWindow")]
    pub show_window: Option<u16>,

/// 
    #[serde(rename = "Title")]
    pub title: Option<String>,

/// 
    #[serde(rename = "WinstationDesktop")]
    pub winstation_desktop: Option<String>,

/// 
    #[serde(rename = "X")]
    pub x: Option<u32>,

/// 
    #[serde(rename = "XCountChars")]
    pub xcount_chars: Option<u32>,

/// 
    #[serde(rename = "XSize")]
    pub xsize: Option<u32>,

/// 
    #[serde(rename = "Y")]
    pub y: Option<u32>,

/// 
    #[serde(rename = "YCountChars")]
    pub ycount_chars: Option<u32>,

/// 
    #[serde(rename = "YSize")]
    pub ysize: Option<u32>,
}

impl Win32_ProcessStartup {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_MethodParameterClass::new(),
            create_flags: None,
            environment_variables: Vec::new(),
            error_mode: None,
            fill_attribute: None,
            priority_class: None,
            show_window: None,
            title: None,
            winstation_desktop: None,
            x: None,
            xcount_chars: None,
            xsize: None,
            y: None,
            ycount_chars: None,
            ysize: None,
        }
    }


    /// Sets the value of CreateFlags
    pub fn set_create_flags(&mut self, value: u32) {
        self.create_flags = Some(value);
    }

    /// Gets the value of CreateFlags
    pub fn get_create_flags(&self) -> Option<&u32> {
        self.create_flags.as_ref()
    }

    /// Sets the value of EnvironmentVariables
    pub fn set_environment_variables(&mut self, value: Vec<String>) {
        self.environment_variables = value;
    }

    /// Gets the value of EnvironmentVariables
    pub fn get_environment_variables(&self) -> &Vec<String> {
        &self.environment_variables
    }

    /// Sets the value of ErrorMode
    pub fn set_error_mode(&mut self, value: u16) {
        self.error_mode = Some(value);
    }

    /// Gets the value of ErrorMode
    pub fn get_error_mode(&self) -> Option<&u16> {
        self.error_mode.as_ref()
    }

    /// Sets the value of FillAttribute
    pub fn set_fill_attribute(&mut self, value: u32) {
        self.fill_attribute = Some(value);
    }

    /// Gets the value of FillAttribute
    pub fn get_fill_attribute(&self) -> Option<&u32> {
        self.fill_attribute.as_ref()
    }

    /// Sets the value of PriorityClass
    pub fn set_priority_class(&mut self, value: u32) {
        self.priority_class = Some(value);
    }

    /// Gets the value of PriorityClass
    pub fn get_priority_class(&self) -> Option<&u32> {
        self.priority_class.as_ref()
    }

    /// Sets the value of ShowWindow
    pub fn set_show_window(&mut self, value: u16) {
        self.show_window = Some(value);
    }

    /// Gets the value of ShowWindow
    pub fn get_show_window(&self) -> Option<&u16> {
        self.show_window.as_ref()
    }

    /// Sets the value of Title
    pub fn set_title(&mut self, value: String) {
        self.title = Some(value);
    }

    /// Gets the value of Title
    pub fn get_title(&self) -> Option<&String> {
        self.title.as_ref()
    }

    /// Sets the value of WinstationDesktop
    pub fn set_winstation_desktop(&mut self, value: String) {
        self.winstation_desktop = Some(value);
    }

    /// Gets the value of WinstationDesktop
    pub fn get_winstation_desktop(&self) -> Option<&String> {
        self.winstation_desktop.as_ref()
    }

    /// Sets the value of X
    pub fn set_x(&mut self, value: u32) {
        self.x = Some(value);
    }

    /// Gets the value of X
    pub fn get_x(&self) -> Option<&u32> {
        self.x.as_ref()
    }

    /// Sets the value of XCountChars
    pub fn set_xcount_chars(&mut self, value: u32) {
        self.xcount_chars = Some(value);
    }

    /// Gets the value of XCountChars
    pub fn get_xcount_chars(&self) -> Option<&u32> {
        self.xcount_chars.as_ref()
    }

    /// Sets the value of XSize
    pub fn set_xsize(&mut self, value: u32) {
        self.xsize = Some(value);
    }

    /// Gets the value of XSize
    pub fn get_xsize(&self) -> Option<&u32> {
        self.xsize.as_ref()
    }

    /// Sets the value of Y
    pub fn set_y(&mut self, value: u32) {
        self.y = Some(value);
    }

    /// Gets the value of Y
    pub fn get_y(&self) -> Option<&u32> {
        self.y.as_ref()
    }

    /// Sets the value of YCountChars
    pub fn set_ycount_chars(&mut self, value: u32) {
        self.ycount_chars = Some(value);
    }

    /// Gets the value of YCountChars
    pub fn get_ycount_chars(&self) -> Option<&u32> {
        self.ycount_chars.as_ref()
    }

    /// Sets the value of YSize
    pub fn set_ysize(&mut self, value: u32) {
        self.ysize = Some(value);
    }

    /// Gets the value of YSize
    pub fn get_ysize(&self) -> Option<&u32> {
        self.ysize.as_ref()
    }
}

