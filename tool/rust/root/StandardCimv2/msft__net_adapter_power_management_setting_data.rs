// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetAdapterPowerManagementSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetAdapterPowerManagementSettingData {
    #[serde(flatten)]
    pub base: MSFT_NetAdapterSettingData,

/// 
    #[serde(rename = "AllowComputerToTurnOffDevice")]
    pub allow_computer_to_turn_off_device: Option<u32>,

/// 
    #[serde(rename = "ArpOffload")]
    pub arp_offload: Option<u32>,

/// 
    #[serde(rename = "D0PacketCoalescing")]
    pub d0_packet_coalescing: Option<u32>,

/// 
    #[serde(rename = "DeviceSleepOnDisconnect")]
    pub device_sleep_on_disconnect: Option<u32>,

/// 
    #[serde(rename = "NSOffload")]
    pub nsoffload: Option<u32>,

/// 
    #[serde(rename = "OffloadParameters")]
    pub offload_parameters: Vec<MSFT_NetAdapterPowerManagement_Offload>,

/// 
    #[serde(rename = "RsnRekeyOffload")]
    pub rsn_rekey_offload: Option<u32>,

/// 
    #[serde(rename = "SelectiveSuspend")]
    pub selective_suspend: Option<u32>,

/// 
    #[serde(rename = "WakeOnMagicPacket")]
    pub wake_on_magic_packet: Option<u32>,

/// 
    #[serde(rename = "WakeOnPattern")]
    pub wake_on_pattern: Option<u32>,

/// 
    #[serde(rename = "WakePatterns")]
    pub wake_patterns: Vec<MSFT_NetAdapterPowerManagement_WakePattern>,
}

impl MSFT_NetAdapterPowerManagementSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetAdapterSettingData::new(),
            allow_computer_to_turn_off_device: None,
            arp_offload: None,
            d0_packet_coalescing: None,
            device_sleep_on_disconnect: None,
            nsoffload: None,
            offload_parameters: Vec::new(),
            rsn_rekey_offload: None,
            selective_suspend: None,
            wake_on_magic_packet: None,
            wake_on_pattern: None,
            wake_patterns: Vec::new(),
        }
    }


    /// Sets the value of AllowComputerToTurnOffDevice
    pub fn set_allow_computer_to_turn_off_device(&mut self, value: u32) {
        self.allow_computer_to_turn_off_device = Some(value);
    }

    /// Gets the value of AllowComputerToTurnOffDevice
    pub fn get_allow_computer_to_turn_off_device(&self) -> Option<&u32> {
        self.allow_computer_to_turn_off_device.as_ref()
    }

    /// Sets the value of ArpOffload
    pub fn set_arp_offload(&mut self, value: u32) {
        self.arp_offload = Some(value);
    }

    /// Gets the value of ArpOffload
    pub fn get_arp_offload(&self) -> Option<&u32> {
        self.arp_offload.as_ref()
    }

    /// Sets the value of D0PacketCoalescing
    pub fn set_d0_packet_coalescing(&mut self, value: u32) {
        self.d0_packet_coalescing = Some(value);
    }

    /// Gets the value of D0PacketCoalescing
    pub fn get_d0_packet_coalescing(&self) -> Option<&u32> {
        self.d0_packet_coalescing.as_ref()
    }

    /// Sets the value of DeviceSleepOnDisconnect
    pub fn set_device_sleep_on_disconnect(&mut self, value: u32) {
        self.device_sleep_on_disconnect = Some(value);
    }

    /// Gets the value of DeviceSleepOnDisconnect
    pub fn get_device_sleep_on_disconnect(&self) -> Option<&u32> {
        self.device_sleep_on_disconnect.as_ref()
    }

    /// Sets the value of NSOffload
    pub fn set_nsoffload(&mut self, value: u32) {
        self.nsoffload = Some(value);
    }

    /// Gets the value of NSOffload
    pub fn get_nsoffload(&self) -> Option<&u32> {
        self.nsoffload.as_ref()
    }

    /// Sets the value of OffloadParameters
    pub fn set_offload_parameters(&mut self, value: Vec<MSFT_NetAdapterPowerManagement_Offload>) {
        self.offload_parameters = value;
    }

    /// Gets the value of OffloadParameters
    pub fn get_offload_parameters(&self) -> &Vec<MSFT_NetAdapterPowerManagement_Offload> {
        &self.offload_parameters
    }

    /// Sets the value of RsnRekeyOffload
    pub fn set_rsn_rekey_offload(&mut self, value: u32) {
        self.rsn_rekey_offload = Some(value);
    }

    /// Gets the value of RsnRekeyOffload
    pub fn get_rsn_rekey_offload(&self) -> Option<&u32> {
        self.rsn_rekey_offload.as_ref()
    }

    /// Sets the value of SelectiveSuspend
    pub fn set_selective_suspend(&mut self, value: u32) {
        self.selective_suspend = Some(value);
    }

    /// Gets the value of SelectiveSuspend
    pub fn get_selective_suspend(&self) -> Option<&u32> {
        self.selective_suspend.as_ref()
    }

    /// Sets the value of WakeOnMagicPacket
    pub fn set_wake_on_magic_packet(&mut self, value: u32) {
        self.wake_on_magic_packet = Some(value);
    }

    /// Gets the value of WakeOnMagicPacket
    pub fn get_wake_on_magic_packet(&self) -> Option<&u32> {
        self.wake_on_magic_packet.as_ref()
    }

    /// Sets the value of WakeOnPattern
    pub fn set_wake_on_pattern(&mut self, value: u32) {
        self.wake_on_pattern = Some(value);
    }

    /// Gets the value of WakeOnPattern
    pub fn get_wake_on_pattern(&self) -> Option<&u32> {
        self.wake_on_pattern.as_ref()
    }

    /// Sets the value of WakePatterns
    pub fn set_wake_patterns(&mut self, value: Vec<MSFT_NetAdapterPowerManagement_WakePattern>) {
        self.wake_patterns = value;
    }

    /// Gets the value of WakePatterns
    pub fn get_wake_patterns(&self) -> &Vec<MSFT_NetAdapterPowerManagement_WakePattern> {
        &self.wake_patterns
    }

/// 

    /// * `arp_offload` -  (bool)
    /// * `d0_packet_coalescing` -  (bool)
    /// * `device_sleep_on_disconnect` -  (bool)
    /// * `nsoffload` -  (bool)
    /// * `rsn_rekey_offload` -  (bool)
    /// * `selective_suspend` -  (bool)
    /// * `wake_on_magic_packet` -  (bool)
    /// * `wake_on_pattern` -  (bool)

    /// * `cmdlet_output` -  (MSFT_NetAdapterPowerManagementSettingData)
    /// * `return_value` -  (u32)
    pub fn enable(&self, cmdlet_output: &mut MSFT_NetAdapterPowerManagementSettingData, arp_offload: Option<bool>, d0_packet_coalescing: Option<bool>, device_sleep_on_disconnect: Option<bool>, nsoffload: Option<bool>, rsn_rekey_offload: Option<bool>, selective_suspend: Option<bool>, wake_on_magic_packet: Option<bool>, wake_on_pattern: Option<bool>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        if let Some(val) = arp_offload {
            args.push(MethodParameter { name: "ArpOffload".to_string(), value: val.into() });
        }
        if let Some(val) = d0_packet_coalescing {
            args.push(MethodParameter { name: "D0PacketCoalescing".to_string(), value: val.into() });
        }
        if let Some(val) = device_sleep_on_disconnect {
            args.push(MethodParameter { name: "DeviceSleepOnDisconnect".to_string(), value: val.into() });
        }
        if let Some(val) = nsoffload {
            args.push(MethodParameter { name: "NSOffload".to_string(), value: val.into() });
        }
        if let Some(val) = rsn_rekey_offload {
            args.push(MethodParameter { name: "RsnRekeyOffload".to_string(), value: val.into() });
        }
        if let Some(val) = selective_suspend {
            args.push(MethodParameter { name: "SelectiveSuspend".to_string(), value: val.into() });
        }
        if let Some(val) = wake_on_magic_packet {
            args.push(MethodParameter { name: "WakeOnMagicPacket".to_string(), value: val.into() });
        }
        if let Some(val) = wake_on_pattern {
            args.push(MethodParameter { name: "WakeOnPattern".to_string(), value: val.into() });
        }

        let result = self.invoke_method("Enable", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `arp_offload` -  (bool)
    /// * `d0_packet_coalescing` -  (bool)
    /// * `device_sleep_on_disconnect` -  (bool)
    /// * `nsoffload` -  (bool)
    /// * `rsn_rekey_offload` -  (bool)
    /// * `selective_suspend` -  (bool)
    /// * `wake_on_magic_packet` -  (bool)
    /// * `wake_on_pattern` -  (bool)

    /// * `cmdlet_output` -  (MSFT_NetAdapterPowerManagementSettingData)
    /// * `return_value` -  (u32)
    pub fn disable(&self, cmdlet_output: &mut MSFT_NetAdapterPowerManagementSettingData, arp_offload: Option<bool>, d0_packet_coalescing: Option<bool>, device_sleep_on_disconnect: Option<bool>, nsoffload: Option<bool>, rsn_rekey_offload: Option<bool>, selective_suspend: Option<bool>, wake_on_magic_packet: Option<bool>, wake_on_pattern: Option<bool>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        if let Some(val) = arp_offload {
            args.push(MethodParameter { name: "ArpOffload".to_string(), value: val.into() });
        }
        if let Some(val) = d0_packet_coalescing {
            args.push(MethodParameter { name: "D0PacketCoalescing".to_string(), value: val.into() });
        }
        if let Some(val) = device_sleep_on_disconnect {
            args.push(MethodParameter { name: "DeviceSleepOnDisconnect".to_string(), value: val.into() });
        }
        if let Some(val) = nsoffload {
            args.push(MethodParameter { name: "NSOffload".to_string(), value: val.into() });
        }
        if let Some(val) = rsn_rekey_offload {
            args.push(MethodParameter { name: "RsnRekeyOffload".to_string(), value: val.into() });
        }
        if let Some(val) = selective_suspend {
            args.push(MethodParameter { name: "SelectiveSuspend".to_string(), value: val.into() });
        }
        if let Some(val) = wake_on_magic_packet {
            args.push(MethodParameter { name: "WakeOnMagicPacket".to_string(), value: val.into() });
        }
        if let Some(val) = wake_on_pattern {
            args.push(MethodParameter { name: "WakeOnPattern".to_string(), value: val.into() });
        }

        let result = self.invoke_method("Disable", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }

}

