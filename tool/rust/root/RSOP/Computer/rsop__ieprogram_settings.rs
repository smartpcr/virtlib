// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_IEProgramSettings struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_IEProgramSettings {

/// 
    #[serde(rename = "calendarProgram")]
    pub calendar_program: Option<String>,

/// 
    #[serde(rename = "checkIfIEIsDefaultBrowser")]
    pub check_if_ieis_default_browser: Option<bool>,

/// 
    #[serde(rename = "contactListProgram")]
    pub contact_list_program: Option<String>,

/// 
    #[serde(rename = "emailProgram")]
    pub email_program: Option<String>,

/// 
    #[serde(rename = "htmlEditorHKCURegData")]
    pub html_editor_hkcureg_data: Option<String>,

/// 
    #[serde(rename = "htmlEditorHKLMRegData")]
    pub html_editor_hklmreg_data: Option<String>,

/// 
    #[serde(rename = "htmlEditorProgram")]
    pub html_editor_program: Option<String>,

/// 
    #[serde(rename = "internetCallProgram")]
    pub internet_call_program: Option<String>,

/// 
    #[serde(rename = "newsgroupsProgram")]
    pub newsgroups_program: Option<String>,

/// 
    #[serde(rename = "rsopID")]
    pub rsop_id: Option<String>,

/// 
    #[serde(rename = "rsopPrecedence")]
    pub rsop_precedence: Option<u32>,

/// 
    #[serde(rename = "useIEForFTP")]
    pub use_iefor_ftp: Option<bool>,
}

impl RSOP_IEProgramSettings {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            calendar_program: None,
            check_if_ieis_default_browser: None,
            contact_list_program: None,
            email_program: None,
            html_editor_hkcureg_data: None,
            html_editor_hklmreg_data: None,
            html_editor_program: None,
            internet_call_program: None,
            newsgroups_program: None,
            rsop_id: None,
            rsop_precedence: None,
            use_iefor_ftp: None,
        }
    }


    /// Sets the value of calendarProgram
    pub fn set_calendar_program(&mut self, value: String) {
        self.calendar_program = Some(value);
    }

    /// Gets the value of calendarProgram
    pub fn get_calendar_program(&self) -> Option<&String> {
        self.calendar_program.as_ref()
    }

    /// Sets the value of checkIfIEIsDefaultBrowser
    pub fn set_check_if_ieis_default_browser(&mut self, value: bool) {
        self.check_if_ieis_default_browser = Some(value);
    }

    /// Gets the value of checkIfIEIsDefaultBrowser
    pub fn get_check_if_ieis_default_browser(&self) -> Option<&bool> {
        self.check_if_ieis_default_browser.as_ref()
    }

    /// Sets the value of contactListProgram
    pub fn set_contact_list_program(&mut self, value: String) {
        self.contact_list_program = Some(value);
    }

    /// Gets the value of contactListProgram
    pub fn get_contact_list_program(&self) -> Option<&String> {
        self.contact_list_program.as_ref()
    }

    /// Sets the value of emailProgram
    pub fn set_email_program(&mut self, value: String) {
        self.email_program = Some(value);
    }

    /// Gets the value of emailProgram
    pub fn get_email_program(&self) -> Option<&String> {
        self.email_program.as_ref()
    }

    /// Sets the value of htmlEditorHKCURegData
    pub fn set_html_editor_hkcureg_data(&mut self, value: String) {
        self.html_editor_hkcureg_data = Some(value);
    }

    /// Gets the value of htmlEditorHKCURegData
    pub fn get_html_editor_hkcureg_data(&self) -> Option<&String> {
        self.html_editor_hkcureg_data.as_ref()
    }

    /// Sets the value of htmlEditorHKLMRegData
    pub fn set_html_editor_hklmreg_data(&mut self, value: String) {
        self.html_editor_hklmreg_data = Some(value);
    }

    /// Gets the value of htmlEditorHKLMRegData
    pub fn get_html_editor_hklmreg_data(&self) -> Option<&String> {
        self.html_editor_hklmreg_data.as_ref()
    }

    /// Sets the value of htmlEditorProgram
    pub fn set_html_editor_program(&mut self, value: String) {
        self.html_editor_program = Some(value);
    }

    /// Gets the value of htmlEditorProgram
    pub fn get_html_editor_program(&self) -> Option<&String> {
        self.html_editor_program.as_ref()
    }

    /// Sets the value of internetCallProgram
    pub fn set_internet_call_program(&mut self, value: String) {
        self.internet_call_program = Some(value);
    }

    /// Gets the value of internetCallProgram
    pub fn get_internet_call_program(&self) -> Option<&String> {
        self.internet_call_program.as_ref()
    }

    /// Sets the value of newsgroupsProgram
    pub fn set_newsgroups_program(&mut self, value: String) {
        self.newsgroups_program = Some(value);
    }

    /// Gets the value of newsgroupsProgram
    pub fn get_newsgroups_program(&self) -> Option<&String> {
        self.newsgroups_program.as_ref()
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
    pub fn set_rsop_precedence(&mut self, value: u32) {
        self.rsop_precedence = Some(value);
    }

    /// Gets the value of rsopPrecedence
    pub fn get_rsop_precedence(&self) -> Option<&u32> {
        self.rsop_precedence.as_ref()
    }

    /// Sets the value of useIEForFTP
    pub fn set_use_iefor_ftp(&mut self, value: bool) {
        self.use_iefor_ftp = Some(value);
    }

    /// Gets the value of useIEForFTP
    pub fn get_use_iefor_ftp(&self) -> Option<&bool> {
        self.use_iefor_ftp.as_ref()
    }
}

