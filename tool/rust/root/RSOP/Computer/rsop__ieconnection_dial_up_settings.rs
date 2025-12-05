// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_IEConnectionDialUpSettings struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_IEConnectionDialUpSettings {

/// 
    #[serde(rename = "alternateOffset")]
    pub alternate_offset: Option<u32>,

/// 
    #[serde(rename = "alternatePhoneNumbers")]
    pub alternate_phone_numbers: Option<String>,

/// 
    #[serde(rename = "areaCode")]
    pub area_code: Option<String>,

/// 
    #[serde(rename = "autoDialDll")]
    pub auto_dial_dll: Option<String>,

/// 
    #[serde(rename = "autoDialFunction")]
    pub auto_dial_function: Option<String>,

/// 
    #[serde(rename = "channels")]
    pub channels: Option<u32>,

/// 
    #[serde(rename = "connectionName")]
    pub connection_name: Option<String>,

/// 
    #[serde(rename = "countryCode")]
    pub country_code: Option<u32>,

/// 
    #[serde(rename = "countryID")]
    pub country_id: Option<u32>,

/// 
    #[serde(rename = "customAuthenticationKey")]
    pub custom_authentication_key: Option<u32>,

/// 
    #[serde(rename = "customDialDll")]
    pub custom_dial_dll: Option<String>,

/// 
    #[serde(rename = "deviceName")]
    pub device_name: Option<String>,

/// 
    #[serde(rename = "deviceType")]
    pub device_type: Option<String>,

/// 
    #[serde(rename = "dialExtraPercent")]
    pub dial_extra_percent: Option<u32>,

/// 
    #[serde(rename = "dialExtraSampleSeconds")]
    pub dial_extra_sample_seconds: Option<u32>,

/// 
    #[serde(rename = "dialMode")]
    pub dial_mode: Option<u32>,

/// 
    #[serde(rename = "encryptionType")]
    pub encryption_type: Option<u32>,

/// 
    #[serde(rename = "frameSize")]
    pub frame_size: Option<u32>,

/// 
    #[serde(rename = "framingProtocol")]
    pub framing_protocol: Option<u32>,

/// 
    #[serde(rename = "guidID")]
    pub guid_id: Option<String>,

/// 
    #[serde(rename = "hangUpExtraPercent")]
    pub hang_up_extra_percent: Option<u32>,

/// 
    #[serde(rename = "hangUpExtraSampleSeconds")]
    pub hang_up_extra_sample_seconds: Option<u32>,

/// 
    #[serde(rename = "idleDisconnectSeconds")]
    pub idle_disconnect_seconds: Option<u32>,

/// 
    #[serde(rename = "ipAddress")]
    pub ip_address: Option<String>,

/// 
    #[serde(rename = "ipDNSAddress")]
    pub ip_dnsaddress: Option<String>,

/// 
    #[serde(rename = "ipDNSAddressAlternate")]
    pub ip_dnsaddress_alternate: Option<String>,

/// 
    #[serde(rename = "ipWINSAddress")]
    pub ip_winsaddress: Option<String>,

/// 
    #[serde(rename = "ipWINSAddressAlternate")]
    pub ip_winsaddress_alternate: Option<String>,

/// 
    #[serde(rename = "localPhoneNumber")]
    pub local_phone_number: Option<String>,

/// 
    #[serde(rename = "netProtocols")]
    pub net_protocols: Option<u32>,

/// 
    #[serde(rename = "options")]
    pub options: Option<u32>,

/// 
    #[serde(rename = "options2")]
    pub options2: Option<u32>,

/// 
    #[serde(rename = "options3")]
    pub options3: Option<u32>,

/// 
    #[serde(rename = "rasEntryData")]
    pub ras_entry_data: Vec<u8>,

/// 
    #[serde(rename = "rasEntryDataSize")]
    pub ras_entry_data_size: Option<u32>,

/// 
    #[serde(rename = "reserved1")]
    pub reserved1: Option<u32>,

/// 
    #[serde(rename = "reserved2")]
    pub reserved2: Option<u32>,

/// 
    #[serde(rename = "rsopID")]
    pub rsop_id: Option<String>,

/// 
    #[serde(rename = "rsopPrecedence")]
    pub rsop_precedence: Option<u32>,

/// 
    #[serde(rename = "scriptFile")]
    pub script_file: Option<String>,

/// 
    #[serde(rename = "subEntries")]
    pub sub_entries: Option<u32>,

/// 
    #[serde(rename = "type")]
    pub type: Option<u32>,

/// 
    #[serde(rename = "vpnStrategy")]
    pub vpn_strategy: Option<i32>,

/// 
    #[serde(rename = "windowsVersion")]
    pub windows_version: Option<u32>,

/// 
    #[serde(rename = "x25Address")]
    pub x25_address: Option<String>,

/// 
    #[serde(rename = "x25Facilities")]
    pub x25_facilities: Option<String>,

/// 
    #[serde(rename = "x25PadType")]
    pub x25_pad_type: Option<String>,

/// 
    #[serde(rename = "x25UserData")]
    pub x25_user_data: Option<String>,
}

impl RSOP_IEConnectionDialUpSettings {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            alternate_offset: None,
            alternate_phone_numbers: None,
            area_code: None,
            auto_dial_dll: None,
            auto_dial_function: None,
            channels: None,
            connection_name: None,
            country_code: None,
            country_id: None,
            custom_authentication_key: None,
            custom_dial_dll: None,
            device_name: None,
            device_type: None,
            dial_extra_percent: None,
            dial_extra_sample_seconds: None,
            dial_mode: None,
            encryption_type: None,
            frame_size: None,
            framing_protocol: None,
            guid_id: None,
            hang_up_extra_percent: None,
            hang_up_extra_sample_seconds: None,
            idle_disconnect_seconds: None,
            ip_address: None,
            ip_dnsaddress: None,
            ip_dnsaddress_alternate: None,
            ip_winsaddress: None,
            ip_winsaddress_alternate: None,
            local_phone_number: None,
            net_protocols: None,
            options: None,
            options2: None,
            options3: None,
            ras_entry_data: Vec::new(),
            ras_entry_data_size: None,
            reserved1: None,
            reserved2: None,
            rsop_id: None,
            rsop_precedence: None,
            script_file: None,
            sub_entries: None,
            type: None,
            vpn_strategy: None,
            windows_version: None,
            x25_address: None,
            x25_facilities: None,
            x25_pad_type: None,
            x25_user_data: None,
        }
    }


    /// Sets the value of alternateOffset
    pub fn set_alternate_offset(&mut self, value: u32) {
        self.alternate_offset = Some(value);
    }

    /// Gets the value of alternateOffset
    pub fn get_alternate_offset(&self) -> Option<&u32> {
        self.alternate_offset.as_ref()
    }

    /// Sets the value of alternatePhoneNumbers
    pub fn set_alternate_phone_numbers(&mut self, value: String) {
        self.alternate_phone_numbers = Some(value);
    }

    /// Gets the value of alternatePhoneNumbers
    pub fn get_alternate_phone_numbers(&self) -> Option<&String> {
        self.alternate_phone_numbers.as_ref()
    }

    /// Sets the value of areaCode
    pub fn set_area_code(&mut self, value: String) {
        self.area_code = Some(value);
    }

    /// Gets the value of areaCode
    pub fn get_area_code(&self) -> Option<&String> {
        self.area_code.as_ref()
    }

    /// Sets the value of autoDialDll
    pub fn set_auto_dial_dll(&mut self, value: String) {
        self.auto_dial_dll = Some(value);
    }

    /// Gets the value of autoDialDll
    pub fn get_auto_dial_dll(&self) -> Option<&String> {
        self.auto_dial_dll.as_ref()
    }

    /// Sets the value of autoDialFunction
    pub fn set_auto_dial_function(&mut self, value: String) {
        self.auto_dial_function = Some(value);
    }

    /// Gets the value of autoDialFunction
    pub fn get_auto_dial_function(&self) -> Option<&String> {
        self.auto_dial_function.as_ref()
    }

    /// Sets the value of channels
    pub fn set_channels(&mut self, value: u32) {
        self.channels = Some(value);
    }

    /// Gets the value of channels
    pub fn get_channels(&self) -> Option<&u32> {
        self.channels.as_ref()
    }

    /// Sets the value of connectionName
    pub fn set_connection_name(&mut self, value: String) {
        self.connection_name = Some(value);
    }

    /// Gets the value of connectionName
    pub fn get_connection_name(&self) -> Option<&String> {
        self.connection_name.as_ref()
    }

    /// Sets the value of countryCode
    pub fn set_country_code(&mut self, value: u32) {
        self.country_code = Some(value);
    }

    /// Gets the value of countryCode
    pub fn get_country_code(&self) -> Option<&u32> {
        self.country_code.as_ref()
    }

    /// Sets the value of countryID
    pub fn set_country_id(&mut self, value: u32) {
        self.country_id = Some(value);
    }

    /// Gets the value of countryID
    pub fn get_country_id(&self) -> Option<&u32> {
        self.country_id.as_ref()
    }

    /// Sets the value of customAuthenticationKey
    pub fn set_custom_authentication_key(&mut self, value: u32) {
        self.custom_authentication_key = Some(value);
    }

    /// Gets the value of customAuthenticationKey
    pub fn get_custom_authentication_key(&self) -> Option<&u32> {
        self.custom_authentication_key.as_ref()
    }

    /// Sets the value of customDialDll
    pub fn set_custom_dial_dll(&mut self, value: String) {
        self.custom_dial_dll = Some(value);
    }

    /// Gets the value of customDialDll
    pub fn get_custom_dial_dll(&self) -> Option<&String> {
        self.custom_dial_dll.as_ref()
    }

    /// Sets the value of deviceName
    pub fn set_device_name(&mut self, value: String) {
        self.device_name = Some(value);
    }

    /// Gets the value of deviceName
    pub fn get_device_name(&self) -> Option<&String> {
        self.device_name.as_ref()
    }

    /// Sets the value of deviceType
    pub fn set_device_type(&mut self, value: String) {
        self.device_type = Some(value);
    }

    /// Gets the value of deviceType
    pub fn get_device_type(&self) -> Option<&String> {
        self.device_type.as_ref()
    }

    /// Sets the value of dialExtraPercent
    pub fn set_dial_extra_percent(&mut self, value: u32) {
        self.dial_extra_percent = Some(value);
    }

    /// Gets the value of dialExtraPercent
    pub fn get_dial_extra_percent(&self) -> Option<&u32> {
        self.dial_extra_percent.as_ref()
    }

    /// Sets the value of dialExtraSampleSeconds
    pub fn set_dial_extra_sample_seconds(&mut self, value: u32) {
        self.dial_extra_sample_seconds = Some(value);
    }

    /// Gets the value of dialExtraSampleSeconds
    pub fn get_dial_extra_sample_seconds(&self) -> Option<&u32> {
        self.dial_extra_sample_seconds.as_ref()
    }

    /// Sets the value of dialMode
    pub fn set_dial_mode(&mut self, value: u32) {
        self.dial_mode = Some(value);
    }

    /// Gets the value of dialMode
    pub fn get_dial_mode(&self) -> Option<&u32> {
        self.dial_mode.as_ref()
    }

    /// Sets the value of encryptionType
    pub fn set_encryption_type(&mut self, value: u32) {
        self.encryption_type = Some(value);
    }

    /// Gets the value of encryptionType
    pub fn get_encryption_type(&self) -> Option<&u32> {
        self.encryption_type.as_ref()
    }

    /// Sets the value of frameSize
    pub fn set_frame_size(&mut self, value: u32) {
        self.frame_size = Some(value);
    }

    /// Gets the value of frameSize
    pub fn get_frame_size(&self) -> Option<&u32> {
        self.frame_size.as_ref()
    }

    /// Sets the value of framingProtocol
    pub fn set_framing_protocol(&mut self, value: u32) {
        self.framing_protocol = Some(value);
    }

    /// Gets the value of framingProtocol
    pub fn get_framing_protocol(&self) -> Option<&u32> {
        self.framing_protocol.as_ref()
    }

    /// Sets the value of guidID
    pub fn set_guid_id(&mut self, value: String) {
        self.guid_id = Some(value);
    }

    /// Gets the value of guidID
    pub fn get_guid_id(&self) -> Option<&String> {
        self.guid_id.as_ref()
    }

    /// Sets the value of hangUpExtraPercent
    pub fn set_hang_up_extra_percent(&mut self, value: u32) {
        self.hang_up_extra_percent = Some(value);
    }

    /// Gets the value of hangUpExtraPercent
    pub fn get_hang_up_extra_percent(&self) -> Option<&u32> {
        self.hang_up_extra_percent.as_ref()
    }

    /// Sets the value of hangUpExtraSampleSeconds
    pub fn set_hang_up_extra_sample_seconds(&mut self, value: u32) {
        self.hang_up_extra_sample_seconds = Some(value);
    }

    /// Gets the value of hangUpExtraSampleSeconds
    pub fn get_hang_up_extra_sample_seconds(&self) -> Option<&u32> {
        self.hang_up_extra_sample_seconds.as_ref()
    }

    /// Sets the value of idleDisconnectSeconds
    pub fn set_idle_disconnect_seconds(&mut self, value: u32) {
        self.idle_disconnect_seconds = Some(value);
    }

    /// Gets the value of idleDisconnectSeconds
    pub fn get_idle_disconnect_seconds(&self) -> Option<&u32> {
        self.idle_disconnect_seconds.as_ref()
    }

    /// Sets the value of ipAddress
    pub fn set_ip_address(&mut self, value: String) {
        self.ip_address = Some(value);
    }

    /// Gets the value of ipAddress
    pub fn get_ip_address(&self) -> Option<&String> {
        self.ip_address.as_ref()
    }

    /// Sets the value of ipDNSAddress
    pub fn set_ip_dnsaddress(&mut self, value: String) {
        self.ip_dnsaddress = Some(value);
    }

    /// Gets the value of ipDNSAddress
    pub fn get_ip_dnsaddress(&self) -> Option<&String> {
        self.ip_dnsaddress.as_ref()
    }

    /// Sets the value of ipDNSAddressAlternate
    pub fn set_ip_dnsaddress_alternate(&mut self, value: String) {
        self.ip_dnsaddress_alternate = Some(value);
    }

    /// Gets the value of ipDNSAddressAlternate
    pub fn get_ip_dnsaddress_alternate(&self) -> Option<&String> {
        self.ip_dnsaddress_alternate.as_ref()
    }

    /// Sets the value of ipWINSAddress
    pub fn set_ip_winsaddress(&mut self, value: String) {
        self.ip_winsaddress = Some(value);
    }

    /// Gets the value of ipWINSAddress
    pub fn get_ip_winsaddress(&self) -> Option<&String> {
        self.ip_winsaddress.as_ref()
    }

    /// Sets the value of ipWINSAddressAlternate
    pub fn set_ip_winsaddress_alternate(&mut self, value: String) {
        self.ip_winsaddress_alternate = Some(value);
    }

    /// Gets the value of ipWINSAddressAlternate
    pub fn get_ip_winsaddress_alternate(&self) -> Option<&String> {
        self.ip_winsaddress_alternate.as_ref()
    }

    /// Sets the value of localPhoneNumber
    pub fn set_local_phone_number(&mut self, value: String) {
        self.local_phone_number = Some(value);
    }

    /// Gets the value of localPhoneNumber
    pub fn get_local_phone_number(&self) -> Option<&String> {
        self.local_phone_number.as_ref()
    }

    /// Sets the value of netProtocols
    pub fn set_net_protocols(&mut self, value: u32) {
        self.net_protocols = Some(value);
    }

    /// Gets the value of netProtocols
    pub fn get_net_protocols(&self) -> Option<&u32> {
        self.net_protocols.as_ref()
    }

    /// Sets the value of options
    pub fn set_options(&mut self, value: u32) {
        self.options = Some(value);
    }

    /// Gets the value of options
    pub fn get_options(&self) -> Option<&u32> {
        self.options.as_ref()
    }

    /// Sets the value of options2
    pub fn set_options2(&mut self, value: u32) {
        self.options2 = Some(value);
    }

    /// Gets the value of options2
    pub fn get_options2(&self) -> Option<&u32> {
        self.options2.as_ref()
    }

    /// Sets the value of options3
    pub fn set_options3(&mut self, value: u32) {
        self.options3 = Some(value);
    }

    /// Gets the value of options3
    pub fn get_options3(&self) -> Option<&u32> {
        self.options3.as_ref()
    }

    /// Sets the value of rasEntryData
    pub fn set_ras_entry_data(&mut self, value: Vec<u8>) {
        self.ras_entry_data = value;
    }

    /// Gets the value of rasEntryData
    pub fn get_ras_entry_data(&self) -> &Vec<u8> {
        &self.ras_entry_data
    }

    /// Sets the value of rasEntryDataSize
    pub fn set_ras_entry_data_size(&mut self, value: u32) {
        self.ras_entry_data_size = Some(value);
    }

    /// Gets the value of rasEntryDataSize
    pub fn get_ras_entry_data_size(&self) -> Option<&u32> {
        self.ras_entry_data_size.as_ref()
    }

    /// Sets the value of reserved1
    pub fn set_reserved1(&mut self, value: u32) {
        self.reserved1 = Some(value);
    }

    /// Gets the value of reserved1
    pub fn get_reserved1(&self) -> Option<&u32> {
        self.reserved1.as_ref()
    }

    /// Sets the value of reserved2
    pub fn set_reserved2(&mut self, value: u32) {
        self.reserved2 = Some(value);
    }

    /// Gets the value of reserved2
    pub fn get_reserved2(&self) -> Option<&u32> {
        self.reserved2.as_ref()
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

    /// Sets the value of scriptFile
    pub fn set_script_file(&mut self, value: String) {
        self.script_file = Some(value);
    }

    /// Gets the value of scriptFile
    pub fn get_script_file(&self) -> Option<&String> {
        self.script_file.as_ref()
    }

    /// Sets the value of subEntries
    pub fn set_sub_entries(&mut self, value: u32) {
        self.sub_entries = Some(value);
    }

    /// Gets the value of subEntries
    pub fn get_sub_entries(&self) -> Option<&u32> {
        self.sub_entries.as_ref()
    }

    /// Sets the value of type
    pub fn set_type(&mut self, value: u32) {
        self.type = Some(value);
    }

    /// Gets the value of type
    pub fn get_type(&self) -> Option<&u32> {
        self.type.as_ref()
    }

    /// Sets the value of vpnStrategy
    pub fn set_vpn_strategy(&mut self, value: i32) {
        self.vpn_strategy = Some(value);
    }

    /// Gets the value of vpnStrategy
    pub fn get_vpn_strategy(&self) -> Option<&i32> {
        self.vpn_strategy.as_ref()
    }

    /// Sets the value of windowsVersion
    pub fn set_windows_version(&mut self, value: u32) {
        self.windows_version = Some(value);
    }

    /// Gets the value of windowsVersion
    pub fn get_windows_version(&self) -> Option<&u32> {
        self.windows_version.as_ref()
    }

    /// Sets the value of x25Address
    pub fn set_x25_address(&mut self, value: String) {
        self.x25_address = Some(value);
    }

    /// Gets the value of x25Address
    pub fn get_x25_address(&self) -> Option<&String> {
        self.x25_address.as_ref()
    }

    /// Sets the value of x25Facilities
    pub fn set_x25_facilities(&mut self, value: String) {
        self.x25_facilities = Some(value);
    }

    /// Gets the value of x25Facilities
    pub fn get_x25_facilities(&self) -> Option<&String> {
        self.x25_facilities.as_ref()
    }

    /// Sets the value of x25PadType
    pub fn set_x25_pad_type(&mut self, value: String) {
        self.x25_pad_type = Some(value);
    }

    /// Gets the value of x25PadType
    pub fn get_x25_pad_type(&self) -> Option<&String> {
        self.x25_pad_type.as_ref()
    }

    /// Sets the value of x25UserData
    pub fn set_x25_user_data(&mut self, value: String) {
        self.x25_user_data = Some(value);
    }

    /// Gets the value of x25UserData
    pub fn get_x25_user_data(&self) -> Option<&String> {
        self.x25_user_data.as_ref()
    }
}

