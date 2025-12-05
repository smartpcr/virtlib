// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.DEFAULT
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CommandLineEventConsumer struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CommandLineEventConsumer {
    #[serde(flatten)]
    pub base: __EventConsumer,

/// 
    #[serde(rename = "CommandLineTemplate")]
    pub command_line_template: Option<String>,

/// 
    #[serde(rename = "CreateNewConsole")]
    pub create_new_console: Option<bool>,

/// 
    #[serde(rename = "CreateNewProcessGroup")]
    pub create_new_process_group: Option<bool>,

/// 
    #[serde(rename = "CreateSeparateWowVdm")]
    pub create_separate_wow_vdm: Option<bool>,

/// 
    #[serde(rename = "CreateSharedWowVdm")]
    pub create_shared_wow_vdm: Option<bool>,

/// 
    #[serde(rename = "DesktopName")]
    pub desktop_name: Option<String>,

/// 
    #[serde(rename = "ExecutablePath")]
    pub executable_path: Option<String>,

/// 
    #[serde(rename = "FillAttribute")]
    pub fill_attribute: Option<u32>,

/// 
    #[serde(rename = "ForceOffFeedback")]
    pub force_off_feedback: Option<bool>,

/// 
    #[serde(rename = "ForceOnFeedback")]
    pub force_on_feedback: Option<bool>,

/// 
    #[serde(rename = "KillTimeout")]
    pub kill_timeout: Option<u32>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "Priority")]
    pub priority: Option<i32>,

/// 
    #[serde(rename = "RunInteractively")]
    pub run_interactively: Option<bool>,

/// 
    #[serde(rename = "ShowWindowCommand")]
    pub show_window_command: Option<u32>,

/// 
    #[serde(rename = "UseDefaultErrorMode")]
    pub use_default_error_mode: Option<bool>,

/// 
    #[serde(rename = "WindowTitle")]
    pub window_title: Option<String>,

/// 
    #[serde(rename = "WorkingDirectory")]
    pub working_directory: Option<String>,

/// 
    #[serde(rename = "XCoordinate")]
    pub xcoordinate: Option<u32>,

/// 
    #[serde(rename = "XNumCharacters")]
    pub xnum_characters: Option<u32>,

/// 
    #[serde(rename = "XSize")]
    pub xsize: Option<u32>,

/// 
    #[serde(rename = "YCoordinate")]
    pub ycoordinate: Option<u32>,

/// 
    #[serde(rename = "YNumCharacters")]
    pub ynum_characters: Option<u32>,

/// 
    #[serde(rename = "YSize")]
    pub ysize: Option<u32>,
}

impl CommandLineEventConsumer {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __EventConsumer::new(),
            command_line_template: None,
            create_new_console: None,
            create_new_process_group: None,
            create_separate_wow_vdm: None,
            create_shared_wow_vdm: None,
            desktop_name: None,
            executable_path: None,
            fill_attribute: None,
            force_off_feedback: None,
            force_on_feedback: None,
            kill_timeout: None,
            name: None,
            priority: None,
            run_interactively: None,
            show_window_command: None,
            use_default_error_mode: None,
            window_title: None,
            working_directory: None,
            xcoordinate: None,
            xnum_characters: None,
            xsize: None,
            ycoordinate: None,
            ynum_characters: None,
            ysize: None,
        }
    }


    /// Sets the value of CommandLineTemplate
    pub fn set_command_line_template(&mut self, value: String) {
        self.command_line_template = Some(value);
    }

    /// Gets the value of CommandLineTemplate
    pub fn get_command_line_template(&self) -> Option<&String> {
        self.command_line_template.as_ref()
    }

    /// Sets the value of CreateNewConsole
    pub fn set_create_new_console(&mut self, value: bool) {
        self.create_new_console = Some(value);
    }

    /// Gets the value of CreateNewConsole
    pub fn get_create_new_console(&self) -> Option<&bool> {
        self.create_new_console.as_ref()
    }

    /// Sets the value of CreateNewProcessGroup
    pub fn set_create_new_process_group(&mut self, value: bool) {
        self.create_new_process_group = Some(value);
    }

    /// Gets the value of CreateNewProcessGroup
    pub fn get_create_new_process_group(&self) -> Option<&bool> {
        self.create_new_process_group.as_ref()
    }

    /// Sets the value of CreateSeparateWowVdm
    pub fn set_create_separate_wow_vdm(&mut self, value: bool) {
        self.create_separate_wow_vdm = Some(value);
    }

    /// Gets the value of CreateSeparateWowVdm
    pub fn get_create_separate_wow_vdm(&self) -> Option<&bool> {
        self.create_separate_wow_vdm.as_ref()
    }

    /// Sets the value of CreateSharedWowVdm
    pub fn set_create_shared_wow_vdm(&mut self, value: bool) {
        self.create_shared_wow_vdm = Some(value);
    }

    /// Gets the value of CreateSharedWowVdm
    pub fn get_create_shared_wow_vdm(&self) -> Option<&bool> {
        self.create_shared_wow_vdm.as_ref()
    }

    /// Sets the value of DesktopName
    pub fn set_desktop_name(&mut self, value: String) {
        self.desktop_name = Some(value);
    }

    /// Gets the value of DesktopName
    pub fn get_desktop_name(&self) -> Option<&String> {
        self.desktop_name.as_ref()
    }

    /// Sets the value of ExecutablePath
    pub fn set_executable_path(&mut self, value: String) {
        self.executable_path = Some(value);
    }

    /// Gets the value of ExecutablePath
    pub fn get_executable_path(&self) -> Option<&String> {
        self.executable_path.as_ref()
    }

    /// Sets the value of FillAttribute
    pub fn set_fill_attribute(&mut self, value: u32) {
        self.fill_attribute = Some(value);
    }

    /// Gets the value of FillAttribute
    pub fn get_fill_attribute(&self) -> Option<&u32> {
        self.fill_attribute.as_ref()
    }

    /// Sets the value of ForceOffFeedback
    pub fn set_force_off_feedback(&mut self, value: bool) {
        self.force_off_feedback = Some(value);
    }

    /// Gets the value of ForceOffFeedback
    pub fn get_force_off_feedback(&self) -> Option<&bool> {
        self.force_off_feedback.as_ref()
    }

    /// Sets the value of ForceOnFeedback
    pub fn set_force_on_feedback(&mut self, value: bool) {
        self.force_on_feedback = Some(value);
    }

    /// Gets the value of ForceOnFeedback
    pub fn get_force_on_feedback(&self) -> Option<&bool> {
        self.force_on_feedback.as_ref()
    }

    /// Sets the value of KillTimeout
    pub fn set_kill_timeout(&mut self, value: u32) {
        self.kill_timeout = Some(value);
    }

    /// Gets the value of KillTimeout
    pub fn get_kill_timeout(&self) -> Option<&u32> {
        self.kill_timeout.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of Priority
    pub fn set_priority(&mut self, value: i32) {
        self.priority = Some(value);
    }

    /// Gets the value of Priority
    pub fn get_priority(&self) -> Option<&i32> {
        self.priority.as_ref()
    }

    /// Sets the value of RunInteractively
    pub fn set_run_interactively(&mut self, value: bool) {
        self.run_interactively = Some(value);
    }

    /// Gets the value of RunInteractively
    pub fn get_run_interactively(&self) -> Option<&bool> {
        self.run_interactively.as_ref()
    }

    /// Sets the value of ShowWindowCommand
    pub fn set_show_window_command(&mut self, value: u32) {
        self.show_window_command = Some(value);
    }

    /// Gets the value of ShowWindowCommand
    pub fn get_show_window_command(&self) -> Option<&u32> {
        self.show_window_command.as_ref()
    }

    /// Sets the value of UseDefaultErrorMode
    pub fn set_use_default_error_mode(&mut self, value: bool) {
        self.use_default_error_mode = Some(value);
    }

    /// Gets the value of UseDefaultErrorMode
    pub fn get_use_default_error_mode(&self) -> Option<&bool> {
        self.use_default_error_mode.as_ref()
    }

    /// Sets the value of WindowTitle
    pub fn set_window_title(&mut self, value: String) {
        self.window_title = Some(value);
    }

    /// Gets the value of WindowTitle
    pub fn get_window_title(&self) -> Option<&String> {
        self.window_title.as_ref()
    }

    /// Sets the value of WorkingDirectory
    pub fn set_working_directory(&mut self, value: String) {
        self.working_directory = Some(value);
    }

    /// Gets the value of WorkingDirectory
    pub fn get_working_directory(&self) -> Option<&String> {
        self.working_directory.as_ref()
    }

    /// Sets the value of XCoordinate
    pub fn set_xcoordinate(&mut self, value: u32) {
        self.xcoordinate = Some(value);
    }

    /// Gets the value of XCoordinate
    pub fn get_xcoordinate(&self) -> Option<&u32> {
        self.xcoordinate.as_ref()
    }

    /// Sets the value of XNumCharacters
    pub fn set_xnum_characters(&mut self, value: u32) {
        self.xnum_characters = Some(value);
    }

    /// Gets the value of XNumCharacters
    pub fn get_xnum_characters(&self) -> Option<&u32> {
        self.xnum_characters.as_ref()
    }

    /// Sets the value of XSize
    pub fn set_xsize(&mut self, value: u32) {
        self.xsize = Some(value);
    }

    /// Gets the value of XSize
    pub fn get_xsize(&self) -> Option<&u32> {
        self.xsize.as_ref()
    }

    /// Sets the value of YCoordinate
    pub fn set_ycoordinate(&mut self, value: u32) {
        self.ycoordinate = Some(value);
    }

    /// Gets the value of YCoordinate
    pub fn get_ycoordinate(&self) -> Option<&u32> {
        self.ycoordinate.as_ref()
    }

    /// Sets the value of YNumCharacters
    pub fn set_ynum_characters(&mut self, value: u32) {
        self.ynum_characters = Some(value);
    }

    /// Gets the value of YNumCharacters
    pub fn get_ynum_characters(&self) -> Option<&u32> {
        self.ynum_characters.as_ref()
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

