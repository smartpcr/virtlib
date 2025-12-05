// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_Printer struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_Printer {
    #[serde(flatten)]
    pub base: CIM_LogicalDevice,

/// 
    #[serde(rename = "AvailableJobSheets")]
    pub available_job_sheets: Vec<String>,

/// 
    #[serde(rename = "Capabilities")]
    pub capabilities: Vec<u16>,

/// 
    #[serde(rename = "CapabilityDescriptions")]
    pub capability_descriptions: Vec<String>,

/// 
    #[serde(rename = "CharSetsSupported")]
    pub char_sets_supported: Vec<String>,

/// 
    #[serde(rename = "CurrentCapabilities")]
    pub current_capabilities: Vec<u16>,

/// 
    #[serde(rename = "CurrentCharSet")]
    pub current_char_set: Option<String>,

/// 
    #[serde(rename = "CurrentLanguage")]
    pub current_language: Option<u16>,

/// 
    #[serde(rename = "CurrentMimeType")]
    pub current_mime_type: Option<String>,

/// 
    #[serde(rename = "CurrentNaturalLanguage")]
    pub current_natural_language: Option<String>,

/// 
    #[serde(rename = "CurrentPaperType")]
    pub current_paper_type: Option<String>,

/// 
    #[serde(rename = "DefaultCapabilities")]
    pub default_capabilities: Vec<u16>,

/// 
    #[serde(rename = "DefaultCopies")]
    pub default_copies: Option<u32>,

/// 
    #[serde(rename = "DefaultLanguage")]
    pub default_language: Option<u16>,

/// 
    #[serde(rename = "DefaultMimeType")]
    pub default_mime_type: Option<String>,

/// 
    #[serde(rename = "DefaultNumberUp")]
    pub default_number_up: Option<u32>,

/// 
    #[serde(rename = "DefaultPaperType")]
    pub default_paper_type: Option<String>,

/// 
    #[serde(rename = "DetectedErrorState")]
    pub detected_error_state: Option<u16>,

/// 
    #[serde(rename = "ErrorInformation")]
    pub error_information: Vec<String>,

/// 
    #[serde(rename = "HorizontalResolution")]
    pub horizontal_resolution: Option<u32>,

/// 
    #[serde(rename = "JobCountSinceLastReset")]
    pub job_count_since_last_reset: Option<u32>,

/// 
    #[serde(rename = "LanguagesSupported")]
    pub languages_supported: Vec<u16>,

/// 
    #[serde(rename = "MarkingTechnology")]
    pub marking_technology: Option<u16>,

/// 
    #[serde(rename = "MaxCopies")]
    pub max_copies: Option<u32>,

/// 
    #[serde(rename = "MaxNumberUp")]
    pub max_number_up: Option<u32>,

/// 
    #[serde(rename = "MaxSizeSupported")]
    pub max_size_supported: Option<u32>,

/// 
    #[serde(rename = "MimeTypesSupported")]
    pub mime_types_supported: Vec<String>,

/// 
    #[serde(rename = "NaturalLanguagesSupported")]
    pub natural_languages_supported: Vec<String>,

/// 
    #[serde(rename = "PaperSizesSupported")]
    pub paper_sizes_supported: Vec<u16>,

/// 
    #[serde(rename = "PaperTypesAvailable")]
    pub paper_types_available: Vec<String>,

/// 
    #[serde(rename = "PrinterStatus")]
    pub printer_status: Option<u16>,

/// 
    #[serde(rename = "TimeOfLastReset")]
    pub time_of_last_reset: Option<String>,

/// 
    #[serde(rename = "VerticalResolution")]
    pub vertical_resolution: Option<u32>,
}

impl CIM_Printer {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalDevice::new(),
            available_job_sheets: Vec::new(),
            capabilities: Vec::new(),
            capability_descriptions: Vec::new(),
            char_sets_supported: Vec::new(),
            current_capabilities: Vec::new(),
            current_char_set: None,
            current_language: None,
            current_mime_type: None,
            current_natural_language: None,
            current_paper_type: None,
            default_capabilities: Vec::new(),
            default_copies: None,
            default_language: None,
            default_mime_type: None,
            default_number_up: None,
            default_paper_type: None,
            detected_error_state: None,
            error_information: Vec::new(),
            horizontal_resolution: None,
            job_count_since_last_reset: None,
            languages_supported: Vec::new(),
            marking_technology: None,
            max_copies: None,
            max_number_up: None,
            max_size_supported: None,
            mime_types_supported: Vec::new(),
            natural_languages_supported: Vec::new(),
            paper_sizes_supported: Vec::new(),
            paper_types_available: Vec::new(),
            printer_status: None,
            time_of_last_reset: None,
            vertical_resolution: None,
        }
    }


    /// Sets the value of AvailableJobSheets
    pub fn set_available_job_sheets(&mut self, value: Vec<String>) {
        self.available_job_sheets = value;
    }

    /// Gets the value of AvailableJobSheets
    pub fn get_available_job_sheets(&self) -> &Vec<String> {
        &self.available_job_sheets
    }

    /// Sets the value of Capabilities
    pub fn set_capabilities(&mut self, value: Vec<u16>) {
        self.capabilities = value;
    }

    /// Gets the value of Capabilities
    pub fn get_capabilities(&self) -> &Vec<u16> {
        &self.capabilities
    }

    /// Sets the value of CapabilityDescriptions
    pub fn set_capability_descriptions(&mut self, value: Vec<String>) {
        self.capability_descriptions = value;
    }

    /// Gets the value of CapabilityDescriptions
    pub fn get_capability_descriptions(&self) -> &Vec<String> {
        &self.capability_descriptions
    }

    /// Sets the value of CharSetsSupported
    pub fn set_char_sets_supported(&mut self, value: Vec<String>) {
        self.char_sets_supported = value;
    }

    /// Gets the value of CharSetsSupported
    pub fn get_char_sets_supported(&self) -> &Vec<String> {
        &self.char_sets_supported
    }

    /// Sets the value of CurrentCapabilities
    pub fn set_current_capabilities(&mut self, value: Vec<u16>) {
        self.current_capabilities = value;
    }

    /// Gets the value of CurrentCapabilities
    pub fn get_current_capabilities(&self) -> &Vec<u16> {
        &self.current_capabilities
    }

    /// Sets the value of CurrentCharSet
    pub fn set_current_char_set(&mut self, value: String) {
        self.current_char_set = Some(value);
    }

    /// Gets the value of CurrentCharSet
    pub fn get_current_char_set(&self) -> Option<&String> {
        self.current_char_set.as_ref()
    }

    /// Sets the value of CurrentLanguage
    pub fn set_current_language(&mut self, value: u16) {
        self.current_language = Some(value);
    }

    /// Gets the value of CurrentLanguage
    pub fn get_current_language(&self) -> Option<&u16> {
        self.current_language.as_ref()
    }

    /// Sets the value of CurrentMimeType
    pub fn set_current_mime_type(&mut self, value: String) {
        self.current_mime_type = Some(value);
    }

    /// Gets the value of CurrentMimeType
    pub fn get_current_mime_type(&self) -> Option<&String> {
        self.current_mime_type.as_ref()
    }

    /// Sets the value of CurrentNaturalLanguage
    pub fn set_current_natural_language(&mut self, value: String) {
        self.current_natural_language = Some(value);
    }

    /// Gets the value of CurrentNaturalLanguage
    pub fn get_current_natural_language(&self) -> Option<&String> {
        self.current_natural_language.as_ref()
    }

    /// Sets the value of CurrentPaperType
    pub fn set_current_paper_type(&mut self, value: String) {
        self.current_paper_type = Some(value);
    }

    /// Gets the value of CurrentPaperType
    pub fn get_current_paper_type(&self) -> Option<&String> {
        self.current_paper_type.as_ref()
    }

    /// Sets the value of DefaultCapabilities
    pub fn set_default_capabilities(&mut self, value: Vec<u16>) {
        self.default_capabilities = value;
    }

    /// Gets the value of DefaultCapabilities
    pub fn get_default_capabilities(&self) -> &Vec<u16> {
        &self.default_capabilities
    }

    /// Sets the value of DefaultCopies
    pub fn set_default_copies(&mut self, value: u32) {
        self.default_copies = Some(value);
    }

    /// Gets the value of DefaultCopies
    pub fn get_default_copies(&self) -> Option<&u32> {
        self.default_copies.as_ref()
    }

    /// Sets the value of DefaultLanguage
    pub fn set_default_language(&mut self, value: u16) {
        self.default_language = Some(value);
    }

    /// Gets the value of DefaultLanguage
    pub fn get_default_language(&self) -> Option<&u16> {
        self.default_language.as_ref()
    }

    /// Sets the value of DefaultMimeType
    pub fn set_default_mime_type(&mut self, value: String) {
        self.default_mime_type = Some(value);
    }

    /// Gets the value of DefaultMimeType
    pub fn get_default_mime_type(&self) -> Option<&String> {
        self.default_mime_type.as_ref()
    }

    /// Sets the value of DefaultNumberUp
    pub fn set_default_number_up(&mut self, value: u32) {
        self.default_number_up = Some(value);
    }

    /// Gets the value of DefaultNumberUp
    pub fn get_default_number_up(&self) -> Option<&u32> {
        self.default_number_up.as_ref()
    }

    /// Sets the value of DefaultPaperType
    pub fn set_default_paper_type(&mut self, value: String) {
        self.default_paper_type = Some(value);
    }

    /// Gets the value of DefaultPaperType
    pub fn get_default_paper_type(&self) -> Option<&String> {
        self.default_paper_type.as_ref()
    }

    /// Sets the value of DetectedErrorState
    pub fn set_detected_error_state(&mut self, value: u16) {
        self.detected_error_state = Some(value);
    }

    /// Gets the value of DetectedErrorState
    pub fn get_detected_error_state(&self) -> Option<&u16> {
        self.detected_error_state.as_ref()
    }

    /// Sets the value of ErrorInformation
    pub fn set_error_information(&mut self, value: Vec<String>) {
        self.error_information = value;
    }

    /// Gets the value of ErrorInformation
    pub fn get_error_information(&self) -> &Vec<String> {
        &self.error_information
    }

    /// Sets the value of HorizontalResolution
    pub fn set_horizontal_resolution(&mut self, value: u32) {
        self.horizontal_resolution = Some(value);
    }

    /// Gets the value of HorizontalResolution
    pub fn get_horizontal_resolution(&self) -> Option<&u32> {
        self.horizontal_resolution.as_ref()
    }

    /// Sets the value of JobCountSinceLastReset
    pub fn set_job_count_since_last_reset(&mut self, value: u32) {
        self.job_count_since_last_reset = Some(value);
    }

    /// Gets the value of JobCountSinceLastReset
    pub fn get_job_count_since_last_reset(&self) -> Option<&u32> {
        self.job_count_since_last_reset.as_ref()
    }

    /// Sets the value of LanguagesSupported
    pub fn set_languages_supported(&mut self, value: Vec<u16>) {
        self.languages_supported = value;
    }

    /// Gets the value of LanguagesSupported
    pub fn get_languages_supported(&self) -> &Vec<u16> {
        &self.languages_supported
    }

    /// Sets the value of MarkingTechnology
    pub fn set_marking_technology(&mut self, value: u16) {
        self.marking_technology = Some(value);
    }

    /// Gets the value of MarkingTechnology
    pub fn get_marking_technology(&self) -> Option<&u16> {
        self.marking_technology.as_ref()
    }

    /// Sets the value of MaxCopies
    pub fn set_max_copies(&mut self, value: u32) {
        self.max_copies = Some(value);
    }

    /// Gets the value of MaxCopies
    pub fn get_max_copies(&self) -> Option<&u32> {
        self.max_copies.as_ref()
    }

    /// Sets the value of MaxNumberUp
    pub fn set_max_number_up(&mut self, value: u32) {
        self.max_number_up = Some(value);
    }

    /// Gets the value of MaxNumberUp
    pub fn get_max_number_up(&self) -> Option<&u32> {
        self.max_number_up.as_ref()
    }

    /// Sets the value of MaxSizeSupported
    pub fn set_max_size_supported(&mut self, value: u32) {
        self.max_size_supported = Some(value);
    }

    /// Gets the value of MaxSizeSupported
    pub fn get_max_size_supported(&self) -> Option<&u32> {
        self.max_size_supported.as_ref()
    }

    /// Sets the value of MimeTypesSupported
    pub fn set_mime_types_supported(&mut self, value: Vec<String>) {
        self.mime_types_supported = value;
    }

    /// Gets the value of MimeTypesSupported
    pub fn get_mime_types_supported(&self) -> &Vec<String> {
        &self.mime_types_supported
    }

    /// Sets the value of NaturalLanguagesSupported
    pub fn set_natural_languages_supported(&mut self, value: Vec<String>) {
        self.natural_languages_supported = value;
    }

    /// Gets the value of NaturalLanguagesSupported
    pub fn get_natural_languages_supported(&self) -> &Vec<String> {
        &self.natural_languages_supported
    }

    /// Sets the value of PaperSizesSupported
    pub fn set_paper_sizes_supported(&mut self, value: Vec<u16>) {
        self.paper_sizes_supported = value;
    }

    /// Gets the value of PaperSizesSupported
    pub fn get_paper_sizes_supported(&self) -> &Vec<u16> {
        &self.paper_sizes_supported
    }

    /// Sets the value of PaperTypesAvailable
    pub fn set_paper_types_available(&mut self, value: Vec<String>) {
        self.paper_types_available = value;
    }

    /// Gets the value of PaperTypesAvailable
    pub fn get_paper_types_available(&self) -> &Vec<String> {
        &self.paper_types_available
    }

    /// Sets the value of PrinterStatus
    pub fn set_printer_status(&mut self, value: u16) {
        self.printer_status = Some(value);
    }

    /// Gets the value of PrinterStatus
    pub fn get_printer_status(&self) -> Option<&u16> {
        self.printer_status.as_ref()
    }

    /// Sets the value of TimeOfLastReset
    pub fn set_time_of_last_reset(&mut self, value: String) {
        self.time_of_last_reset = Some(value);
    }

    /// Gets the value of TimeOfLastReset
    pub fn get_time_of_last_reset(&self) -> Option<&String> {
        self.time_of_last_reset.as_ref()
    }

    /// Sets the value of VerticalResolution
    pub fn set_vertical_resolution(&mut self, value: u32) {
        self.vertical_resolution = Some(value);
    }

    /// Gets the value of VerticalResolution
    pub fn get_vertical_resolution(&self) -> Option<&u32> {
        self.vertical_resolution.as_ref()
    }
}

