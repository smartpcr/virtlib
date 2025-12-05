// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Config01_TextInput02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Config01_TextInput02 {

/// 
    #[serde(rename = "AllowHardwareKeyboardTextSuggestions")]
    pub allow_hardware_keyboard_text_suggestions: Option<i32>,

/// 
    #[serde(rename = "AllowIMELogging")]
    pub allow_imelogging: Option<i32>,

/// 
    #[serde(rename = "AllowIMENetworkAccess")]
    pub allow_imenetwork_access: Option<i32>,

/// 
    #[serde(rename = "AllowInputPanel")]
    pub allow_input_panel: Option<i32>,

/// 
    #[serde(rename = "AllowJapaneseIMESurrogatePairCharacters")]
    pub allow_japanese_imesurrogate_pair_characters: Option<i32>,

/// 
    #[serde(rename = "AllowJapaneseIVSCharacters")]
    pub allow_japanese_ivscharacters: Option<i32>,

/// 
    #[serde(rename = "AllowJapaneseNonPublishingStandardGlyph")]
    pub allow_japanese_non_publishing_standard_glyph: Option<i32>,

/// 
    #[serde(rename = "AllowJapaneseUserDictionary")]
    pub allow_japanese_user_dictionary: Option<i32>,

/// 
    #[serde(rename = "AllowKeyboardTextSuggestions")]
    pub allow_keyboard_text_suggestions: Option<i32>,

/// 
    #[serde(rename = "AllowLanguageFeaturesUninstall")]
    pub allow_language_features_uninstall: Option<i32>,

/// 
    #[serde(rename = "AllowLinguisticDataCollection")]
    pub allow_linguistic_data_collection: Option<i32>,

/// 
    #[serde(rename = "AllowTextInputSuggestionUpdate")]
    pub allow_text_input_suggestion_update: Option<i32>,

/// 
    #[serde(rename = "ConfigureJapaneseIMEVersion")]
    pub configure_japanese_imeversion: Option<i32>,

/// 
    #[serde(rename = "ConfigureKoreanIMEVersion")]
    pub configure_korean_imeversion: Option<i32>,

/// 
    #[serde(rename = "ConfigureSimplifiedChineseIMEVersion")]
    pub configure_simplified_chinese_imeversion: Option<i32>,

/// 
    #[serde(rename = "ConfigureTraditionalChineseIMEVersion")]
    pub configure_traditional_chinese_imeversion: Option<i32>,

/// 
    #[serde(rename = "EnableTouchKeyboardAutoInvokeInDesktopMode")]
    pub enable_touch_keyboard_auto_invoke_in_desktop_mode: Option<i32>,

/// 
    #[serde(rename = "ExcludeJapaneseIMEExceptJIS0208")]
    pub exclude_japanese_imeexcept_jis0208: Option<i32>,

/// 
    #[serde(rename = "ExcludeJapaneseIMEExceptJIS0208andEUDC")]
    pub exclude_japanese_imeexcept_jis0208and_eudc: Option<i32>,

/// 
    #[serde(rename = "ExcludeJapaneseIMEExceptShiftJIS")]
    pub exclude_japanese_imeexcept_shift_jis: Option<i32>,

/// 
    #[serde(rename = "ForceTouchKeyboardDockedState")]
    pub force_touch_keyboard_docked_state: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "TouchKeyboardDictationButtonAvailability")]
    pub touch_keyboard_dictation_button_availability: Option<i32>,

/// 
    #[serde(rename = "TouchKeyboardEmojiButtonAvailability")]
    pub touch_keyboard_emoji_button_availability: Option<i32>,

/// 
    #[serde(rename = "TouchKeyboardFullModeAvailability")]
    pub touch_keyboard_full_mode_availability: Option<i32>,

/// 
    #[serde(rename = "TouchKeyboardHandwritingModeAvailability")]
    pub touch_keyboard_handwriting_mode_availability: Option<i32>,

/// 
    #[serde(rename = "TouchKeyboardNarrowModeAvailability")]
    pub touch_keyboard_narrow_mode_availability: Option<i32>,

/// 
    #[serde(rename = "TouchKeyboardSplitModeAvailability")]
    pub touch_keyboard_split_mode_availability: Option<i32>,

/// 
    #[serde(rename = "TouchKeyboardWideModeAvailability")]
    pub touch_keyboard_wide_mode_availability: Option<i32>,
}

impl MDM_Policy_Config01_TextInput02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allow_hardware_keyboard_text_suggestions: None,
            allow_imelogging: None,
            allow_imenetwork_access: None,
            allow_input_panel: None,
            allow_japanese_imesurrogate_pair_characters: None,
            allow_japanese_ivscharacters: None,
            allow_japanese_non_publishing_standard_glyph: None,
            allow_japanese_user_dictionary: None,
            allow_keyboard_text_suggestions: None,
            allow_language_features_uninstall: None,
            allow_linguistic_data_collection: None,
            allow_text_input_suggestion_update: None,
            configure_japanese_imeversion: None,
            configure_korean_imeversion: None,
            configure_simplified_chinese_imeversion: None,
            configure_traditional_chinese_imeversion: None,
            enable_touch_keyboard_auto_invoke_in_desktop_mode: None,
            exclude_japanese_imeexcept_jis0208: None,
            exclude_japanese_imeexcept_jis0208and_eudc: None,
            exclude_japanese_imeexcept_shift_jis: None,
            force_touch_keyboard_docked_state: None,
            instance_id: None,
            parent_id: None,
            touch_keyboard_dictation_button_availability: None,
            touch_keyboard_emoji_button_availability: None,
            touch_keyboard_full_mode_availability: None,
            touch_keyboard_handwriting_mode_availability: None,
            touch_keyboard_narrow_mode_availability: None,
            touch_keyboard_split_mode_availability: None,
            touch_keyboard_wide_mode_availability: None,
        }
    }


    /// Sets the value of AllowHardwareKeyboardTextSuggestions
    pub fn set_allow_hardware_keyboard_text_suggestions(&mut self, value: i32) {
        self.allow_hardware_keyboard_text_suggestions = Some(value);
    }

    /// Gets the value of AllowHardwareKeyboardTextSuggestions
    pub fn get_allow_hardware_keyboard_text_suggestions(&self) -> Option<&i32> {
        self.allow_hardware_keyboard_text_suggestions.as_ref()
    }

    /// Sets the value of AllowIMELogging
    pub fn set_allow_imelogging(&mut self, value: i32) {
        self.allow_imelogging = Some(value);
    }

    /// Gets the value of AllowIMELogging
    pub fn get_allow_imelogging(&self) -> Option<&i32> {
        self.allow_imelogging.as_ref()
    }

    /// Sets the value of AllowIMENetworkAccess
    pub fn set_allow_imenetwork_access(&mut self, value: i32) {
        self.allow_imenetwork_access = Some(value);
    }

    /// Gets the value of AllowIMENetworkAccess
    pub fn get_allow_imenetwork_access(&self) -> Option<&i32> {
        self.allow_imenetwork_access.as_ref()
    }

    /// Sets the value of AllowInputPanel
    pub fn set_allow_input_panel(&mut self, value: i32) {
        self.allow_input_panel = Some(value);
    }

    /// Gets the value of AllowInputPanel
    pub fn get_allow_input_panel(&self) -> Option<&i32> {
        self.allow_input_panel.as_ref()
    }

    /// Sets the value of AllowJapaneseIMESurrogatePairCharacters
    pub fn set_allow_japanese_imesurrogate_pair_characters(&mut self, value: i32) {
        self.allow_japanese_imesurrogate_pair_characters = Some(value);
    }

    /// Gets the value of AllowJapaneseIMESurrogatePairCharacters
    pub fn get_allow_japanese_imesurrogate_pair_characters(&self) -> Option<&i32> {
        self.allow_japanese_imesurrogate_pair_characters.as_ref()
    }

    /// Sets the value of AllowJapaneseIVSCharacters
    pub fn set_allow_japanese_ivscharacters(&mut self, value: i32) {
        self.allow_japanese_ivscharacters = Some(value);
    }

    /// Gets the value of AllowJapaneseIVSCharacters
    pub fn get_allow_japanese_ivscharacters(&self) -> Option<&i32> {
        self.allow_japanese_ivscharacters.as_ref()
    }

    /// Sets the value of AllowJapaneseNonPublishingStandardGlyph
    pub fn set_allow_japanese_non_publishing_standard_glyph(&mut self, value: i32) {
        self.allow_japanese_non_publishing_standard_glyph = Some(value);
    }

    /// Gets the value of AllowJapaneseNonPublishingStandardGlyph
    pub fn get_allow_japanese_non_publishing_standard_glyph(&self) -> Option<&i32> {
        self.allow_japanese_non_publishing_standard_glyph.as_ref()
    }

    /// Sets the value of AllowJapaneseUserDictionary
    pub fn set_allow_japanese_user_dictionary(&mut self, value: i32) {
        self.allow_japanese_user_dictionary = Some(value);
    }

    /// Gets the value of AllowJapaneseUserDictionary
    pub fn get_allow_japanese_user_dictionary(&self) -> Option<&i32> {
        self.allow_japanese_user_dictionary.as_ref()
    }

    /// Sets the value of AllowKeyboardTextSuggestions
    pub fn set_allow_keyboard_text_suggestions(&mut self, value: i32) {
        self.allow_keyboard_text_suggestions = Some(value);
    }

    /// Gets the value of AllowKeyboardTextSuggestions
    pub fn get_allow_keyboard_text_suggestions(&self) -> Option<&i32> {
        self.allow_keyboard_text_suggestions.as_ref()
    }

    /// Sets the value of AllowLanguageFeaturesUninstall
    pub fn set_allow_language_features_uninstall(&mut self, value: i32) {
        self.allow_language_features_uninstall = Some(value);
    }

    /// Gets the value of AllowLanguageFeaturesUninstall
    pub fn get_allow_language_features_uninstall(&self) -> Option<&i32> {
        self.allow_language_features_uninstall.as_ref()
    }

    /// Sets the value of AllowLinguisticDataCollection
    pub fn set_allow_linguistic_data_collection(&mut self, value: i32) {
        self.allow_linguistic_data_collection = Some(value);
    }

    /// Gets the value of AllowLinguisticDataCollection
    pub fn get_allow_linguistic_data_collection(&self) -> Option<&i32> {
        self.allow_linguistic_data_collection.as_ref()
    }

    /// Sets the value of AllowTextInputSuggestionUpdate
    pub fn set_allow_text_input_suggestion_update(&mut self, value: i32) {
        self.allow_text_input_suggestion_update = Some(value);
    }

    /// Gets the value of AllowTextInputSuggestionUpdate
    pub fn get_allow_text_input_suggestion_update(&self) -> Option<&i32> {
        self.allow_text_input_suggestion_update.as_ref()
    }

    /// Sets the value of ConfigureJapaneseIMEVersion
    pub fn set_configure_japanese_imeversion(&mut self, value: i32) {
        self.configure_japanese_imeversion = Some(value);
    }

    /// Gets the value of ConfigureJapaneseIMEVersion
    pub fn get_configure_japanese_imeversion(&self) -> Option<&i32> {
        self.configure_japanese_imeversion.as_ref()
    }

    /// Sets the value of ConfigureKoreanIMEVersion
    pub fn set_configure_korean_imeversion(&mut self, value: i32) {
        self.configure_korean_imeversion = Some(value);
    }

    /// Gets the value of ConfigureKoreanIMEVersion
    pub fn get_configure_korean_imeversion(&self) -> Option<&i32> {
        self.configure_korean_imeversion.as_ref()
    }

    /// Sets the value of ConfigureSimplifiedChineseIMEVersion
    pub fn set_configure_simplified_chinese_imeversion(&mut self, value: i32) {
        self.configure_simplified_chinese_imeversion = Some(value);
    }

    /// Gets the value of ConfigureSimplifiedChineseIMEVersion
    pub fn get_configure_simplified_chinese_imeversion(&self) -> Option<&i32> {
        self.configure_simplified_chinese_imeversion.as_ref()
    }

    /// Sets the value of ConfigureTraditionalChineseIMEVersion
    pub fn set_configure_traditional_chinese_imeversion(&mut self, value: i32) {
        self.configure_traditional_chinese_imeversion = Some(value);
    }

    /// Gets the value of ConfigureTraditionalChineseIMEVersion
    pub fn get_configure_traditional_chinese_imeversion(&self) -> Option<&i32> {
        self.configure_traditional_chinese_imeversion.as_ref()
    }

    /// Sets the value of EnableTouchKeyboardAutoInvokeInDesktopMode
    pub fn set_enable_touch_keyboard_auto_invoke_in_desktop_mode(&mut self, value: i32) {
        self.enable_touch_keyboard_auto_invoke_in_desktop_mode = Some(value);
    }

    /// Gets the value of EnableTouchKeyboardAutoInvokeInDesktopMode
    pub fn get_enable_touch_keyboard_auto_invoke_in_desktop_mode(&self) -> Option<&i32> {
        self.enable_touch_keyboard_auto_invoke_in_desktop_mode.as_ref()
    }

    /// Sets the value of ExcludeJapaneseIMEExceptJIS0208
    pub fn set_exclude_japanese_imeexcept_jis0208(&mut self, value: i32) {
        self.exclude_japanese_imeexcept_jis0208 = Some(value);
    }

    /// Gets the value of ExcludeJapaneseIMEExceptJIS0208
    pub fn get_exclude_japanese_imeexcept_jis0208(&self) -> Option<&i32> {
        self.exclude_japanese_imeexcept_jis0208.as_ref()
    }

    /// Sets the value of ExcludeJapaneseIMEExceptJIS0208andEUDC
    pub fn set_exclude_japanese_imeexcept_jis0208and_eudc(&mut self, value: i32) {
        self.exclude_japanese_imeexcept_jis0208and_eudc = Some(value);
    }

    /// Gets the value of ExcludeJapaneseIMEExceptJIS0208andEUDC
    pub fn get_exclude_japanese_imeexcept_jis0208and_eudc(&self) -> Option<&i32> {
        self.exclude_japanese_imeexcept_jis0208and_eudc.as_ref()
    }

    /// Sets the value of ExcludeJapaneseIMEExceptShiftJIS
    pub fn set_exclude_japanese_imeexcept_shift_jis(&mut self, value: i32) {
        self.exclude_japanese_imeexcept_shift_jis = Some(value);
    }

    /// Gets the value of ExcludeJapaneseIMEExceptShiftJIS
    pub fn get_exclude_japanese_imeexcept_shift_jis(&self) -> Option<&i32> {
        self.exclude_japanese_imeexcept_shift_jis.as_ref()
    }

    /// Sets the value of ForceTouchKeyboardDockedState
    pub fn set_force_touch_keyboard_docked_state(&mut self, value: i32) {
        self.force_touch_keyboard_docked_state = Some(value);
    }

    /// Gets the value of ForceTouchKeyboardDockedState
    pub fn get_force_touch_keyboard_docked_state(&self) -> Option<&i32> {
        self.force_touch_keyboard_docked_state.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of TouchKeyboardDictationButtonAvailability
    pub fn set_touch_keyboard_dictation_button_availability(&mut self, value: i32) {
        self.touch_keyboard_dictation_button_availability = Some(value);
    }

    /// Gets the value of TouchKeyboardDictationButtonAvailability
    pub fn get_touch_keyboard_dictation_button_availability(&self) -> Option<&i32> {
        self.touch_keyboard_dictation_button_availability.as_ref()
    }

    /// Sets the value of TouchKeyboardEmojiButtonAvailability
    pub fn set_touch_keyboard_emoji_button_availability(&mut self, value: i32) {
        self.touch_keyboard_emoji_button_availability = Some(value);
    }

    /// Gets the value of TouchKeyboardEmojiButtonAvailability
    pub fn get_touch_keyboard_emoji_button_availability(&self) -> Option<&i32> {
        self.touch_keyboard_emoji_button_availability.as_ref()
    }

    /// Sets the value of TouchKeyboardFullModeAvailability
    pub fn set_touch_keyboard_full_mode_availability(&mut self, value: i32) {
        self.touch_keyboard_full_mode_availability = Some(value);
    }

    /// Gets the value of TouchKeyboardFullModeAvailability
    pub fn get_touch_keyboard_full_mode_availability(&self) -> Option<&i32> {
        self.touch_keyboard_full_mode_availability.as_ref()
    }

    /// Sets the value of TouchKeyboardHandwritingModeAvailability
    pub fn set_touch_keyboard_handwriting_mode_availability(&mut self, value: i32) {
        self.touch_keyboard_handwriting_mode_availability = Some(value);
    }

    /// Gets the value of TouchKeyboardHandwritingModeAvailability
    pub fn get_touch_keyboard_handwriting_mode_availability(&self) -> Option<&i32> {
        self.touch_keyboard_handwriting_mode_availability.as_ref()
    }

    /// Sets the value of TouchKeyboardNarrowModeAvailability
    pub fn set_touch_keyboard_narrow_mode_availability(&mut self, value: i32) {
        self.touch_keyboard_narrow_mode_availability = Some(value);
    }

    /// Gets the value of TouchKeyboardNarrowModeAvailability
    pub fn get_touch_keyboard_narrow_mode_availability(&self) -> Option<&i32> {
        self.touch_keyboard_narrow_mode_availability.as_ref()
    }

    /// Sets the value of TouchKeyboardSplitModeAvailability
    pub fn set_touch_keyboard_split_mode_availability(&mut self, value: i32) {
        self.touch_keyboard_split_mode_availability = Some(value);
    }

    /// Gets the value of TouchKeyboardSplitModeAvailability
    pub fn get_touch_keyboard_split_mode_availability(&self) -> Option<&i32> {
        self.touch_keyboard_split_mode_availability.as_ref()
    }

    /// Sets the value of TouchKeyboardWideModeAvailability
    pub fn set_touch_keyboard_wide_mode_availability(&mut self, value: i32) {
        self.touch_keyboard_wide_mode_availability = Some(value);
    }

    /// Gets the value of TouchKeyboardWideModeAvailability
    pub fn get_touch_keyboard_wide_mode_availability(&self) -> Option<&i32> {
        self.touch_keyboard_wide_mode_availability.as_ref()
    }
}

