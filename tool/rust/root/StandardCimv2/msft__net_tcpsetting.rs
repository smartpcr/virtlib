// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetTCPSetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetTCPSetting {
    #[serde(flatten)]
    pub base: CIM_PolicyAction,

/// 
    #[serde(rename = "AutomaticUseCustom")]
    pub automatic_use_custom: Option<u8>,

/// 
    #[serde(rename = "AutoReusePortRangeNumberOfPorts")]
    pub auto_reuse_port_range_number_of_ports: Option<u16>,

/// 
    #[serde(rename = "AutoReusePortRangeStartPort")]
    pub auto_reuse_port_range_start_port: Option<u16>,

/// 
    #[serde(rename = "AutoTuningLevelEffective")]
    pub auto_tuning_level_effective: Option<u8>,

/// 
    #[serde(rename = "AutoTuningLevelGroupPolicy")]
    pub auto_tuning_level_group_policy: Option<u8>,

/// 
    #[serde(rename = "AutoTuningLevelLocal")]
    pub auto_tuning_level_local: Option<u8>,

/// 
    #[serde(rename = "CongestionProvider")]
    pub congestion_provider: Option<u8>,

/// 
    #[serde(rename = "CwndRestart")]
    pub cwnd_restart: Option<u8>,

/// 
    #[serde(rename = "DelayedAckFrequency")]
    pub delayed_ack_frequency: Option<u8>,

/// 
    #[serde(rename = "DelayedAckTimeout")]
    pub delayed_ack_timeout: Option<u32>,

/// 
    #[serde(rename = "DynamicPortRangeNumberOfPorts")]
    pub dynamic_port_range_number_of_ports: Option<u16>,

/// 
    #[serde(rename = "DynamicPortRangeStartPort")]
    pub dynamic_port_range_start_port: Option<u16>,

/// 
    #[serde(rename = "EcnCapability")]
    pub ecn_capability: Option<u8>,

/// 
    #[serde(rename = "ForceWS")]
    pub force_ws: Option<u8>,

/// 
    #[serde(rename = "InitialCongestionWindow")]
    pub initial_congestion_window: Option<u32>,

/// 
    #[serde(rename = "InitialRto")]
    pub initial_rto: Option<u32>,

/// 
    #[serde(rename = "MaxSynRetransmissions")]
    pub max_syn_retransmissions: Option<u8>,

/// 
    #[serde(rename = "MemoryPressureProtection")]
    pub memory_pressure_protection: Option<u8>,

/// 
    #[serde(rename = "MinRto")]
    pub min_rto: Option<u32>,

/// 
    #[serde(rename = "NonSackRttResiliency")]
    pub non_sack_rtt_resiliency: Option<u8>,

/// 
    #[serde(rename = "ScalingHeuristics")]
    pub scaling_heuristics: Option<u8>,

/// 
    #[serde(rename = "SettingName")]
    pub setting_name: Option<String>,

/// 
    #[serde(rename = "Timestamps")]
    pub timestamps: Option<u8>,
}

impl MSFT_NetTCPSetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_PolicyAction::new(),
            automatic_use_custom: None,
            auto_reuse_port_range_number_of_ports: None,
            auto_reuse_port_range_start_port: None,
            auto_tuning_level_effective: None,
            auto_tuning_level_group_policy: None,
            auto_tuning_level_local: None,
            congestion_provider: None,
            cwnd_restart: None,
            delayed_ack_frequency: None,
            delayed_ack_timeout: None,
            dynamic_port_range_number_of_ports: None,
            dynamic_port_range_start_port: None,
            ecn_capability: None,
            force_ws: None,
            initial_congestion_window: None,
            initial_rto: None,
            max_syn_retransmissions: None,
            memory_pressure_protection: None,
            min_rto: None,
            non_sack_rtt_resiliency: None,
            scaling_heuristics: None,
            setting_name: None,
            timestamps: None,
        }
    }


    /// Sets the value of AutomaticUseCustom
    pub fn set_automatic_use_custom(&mut self, value: u8) {
        self.automatic_use_custom = Some(value);
    }

    /// Gets the value of AutomaticUseCustom
    pub fn get_automatic_use_custom(&self) -> Option<&u8> {
        self.automatic_use_custom.as_ref()
    }

    /// Sets the value of AutoReusePortRangeNumberOfPorts
    pub fn set_auto_reuse_port_range_number_of_ports(&mut self, value: u16) {
        self.auto_reuse_port_range_number_of_ports = Some(value);
    }

    /// Gets the value of AutoReusePortRangeNumberOfPorts
    pub fn get_auto_reuse_port_range_number_of_ports(&self) -> Option<&u16> {
        self.auto_reuse_port_range_number_of_ports.as_ref()
    }

    /// Sets the value of AutoReusePortRangeStartPort
    pub fn set_auto_reuse_port_range_start_port(&mut self, value: u16) {
        self.auto_reuse_port_range_start_port = Some(value);
    }

    /// Gets the value of AutoReusePortRangeStartPort
    pub fn get_auto_reuse_port_range_start_port(&self) -> Option<&u16> {
        self.auto_reuse_port_range_start_port.as_ref()
    }

    /// Sets the value of AutoTuningLevelEffective
    pub fn set_auto_tuning_level_effective(&mut self, value: u8) {
        self.auto_tuning_level_effective = Some(value);
    }

    /// Gets the value of AutoTuningLevelEffective
    pub fn get_auto_tuning_level_effective(&self) -> Option<&u8> {
        self.auto_tuning_level_effective.as_ref()
    }

    /// Sets the value of AutoTuningLevelGroupPolicy
    pub fn set_auto_tuning_level_group_policy(&mut self, value: u8) {
        self.auto_tuning_level_group_policy = Some(value);
    }

    /// Gets the value of AutoTuningLevelGroupPolicy
    pub fn get_auto_tuning_level_group_policy(&self) -> Option<&u8> {
        self.auto_tuning_level_group_policy.as_ref()
    }

    /// Sets the value of AutoTuningLevelLocal
    pub fn set_auto_tuning_level_local(&mut self, value: u8) {
        self.auto_tuning_level_local = Some(value);
    }

    /// Gets the value of AutoTuningLevelLocal
    pub fn get_auto_tuning_level_local(&self) -> Option<&u8> {
        self.auto_tuning_level_local.as_ref()
    }

    /// Sets the value of CongestionProvider
    pub fn set_congestion_provider(&mut self, value: u8) {
        self.congestion_provider = Some(value);
    }

    /// Gets the value of CongestionProvider
    pub fn get_congestion_provider(&self) -> Option<&u8> {
        self.congestion_provider.as_ref()
    }

    /// Sets the value of CwndRestart
    pub fn set_cwnd_restart(&mut self, value: u8) {
        self.cwnd_restart = Some(value);
    }

    /// Gets the value of CwndRestart
    pub fn get_cwnd_restart(&self) -> Option<&u8> {
        self.cwnd_restart.as_ref()
    }

    /// Sets the value of DelayedAckFrequency
    pub fn set_delayed_ack_frequency(&mut self, value: u8) {
        self.delayed_ack_frequency = Some(value);
    }

    /// Gets the value of DelayedAckFrequency
    pub fn get_delayed_ack_frequency(&self) -> Option<&u8> {
        self.delayed_ack_frequency.as_ref()
    }

    /// Sets the value of DelayedAckTimeout
    pub fn set_delayed_ack_timeout(&mut self, value: u32) {
        self.delayed_ack_timeout = Some(value);
    }

    /// Gets the value of DelayedAckTimeout
    pub fn get_delayed_ack_timeout(&self) -> Option<&u32> {
        self.delayed_ack_timeout.as_ref()
    }

    /// Sets the value of DynamicPortRangeNumberOfPorts
    pub fn set_dynamic_port_range_number_of_ports(&mut self, value: u16) {
        self.dynamic_port_range_number_of_ports = Some(value);
    }

    /// Gets the value of DynamicPortRangeNumberOfPorts
    pub fn get_dynamic_port_range_number_of_ports(&self) -> Option<&u16> {
        self.dynamic_port_range_number_of_ports.as_ref()
    }

    /// Sets the value of DynamicPortRangeStartPort
    pub fn set_dynamic_port_range_start_port(&mut self, value: u16) {
        self.dynamic_port_range_start_port = Some(value);
    }

    /// Gets the value of DynamicPortRangeStartPort
    pub fn get_dynamic_port_range_start_port(&self) -> Option<&u16> {
        self.dynamic_port_range_start_port.as_ref()
    }

    /// Sets the value of EcnCapability
    pub fn set_ecn_capability(&mut self, value: u8) {
        self.ecn_capability = Some(value);
    }

    /// Gets the value of EcnCapability
    pub fn get_ecn_capability(&self) -> Option<&u8> {
        self.ecn_capability.as_ref()
    }

    /// Sets the value of ForceWS
    pub fn set_force_ws(&mut self, value: u8) {
        self.force_ws = Some(value);
    }

    /// Gets the value of ForceWS
    pub fn get_force_ws(&self) -> Option<&u8> {
        self.force_ws.as_ref()
    }

    /// Sets the value of InitialCongestionWindow
    pub fn set_initial_congestion_window(&mut self, value: u32) {
        self.initial_congestion_window = Some(value);
    }

    /// Gets the value of InitialCongestionWindow
    pub fn get_initial_congestion_window(&self) -> Option<&u32> {
        self.initial_congestion_window.as_ref()
    }

    /// Sets the value of InitialRto
    pub fn set_initial_rto(&mut self, value: u32) {
        self.initial_rto = Some(value);
    }

    /// Gets the value of InitialRto
    pub fn get_initial_rto(&self) -> Option<&u32> {
        self.initial_rto.as_ref()
    }

    /// Sets the value of MaxSynRetransmissions
    pub fn set_max_syn_retransmissions(&mut self, value: u8) {
        self.max_syn_retransmissions = Some(value);
    }

    /// Gets the value of MaxSynRetransmissions
    pub fn get_max_syn_retransmissions(&self) -> Option<&u8> {
        self.max_syn_retransmissions.as_ref()
    }

    /// Sets the value of MemoryPressureProtection
    pub fn set_memory_pressure_protection(&mut self, value: u8) {
        self.memory_pressure_protection = Some(value);
    }

    /// Gets the value of MemoryPressureProtection
    pub fn get_memory_pressure_protection(&self) -> Option<&u8> {
        self.memory_pressure_protection.as_ref()
    }

    /// Sets the value of MinRto
    pub fn set_min_rto(&mut self, value: u32) {
        self.min_rto = Some(value);
    }

    /// Gets the value of MinRto
    pub fn get_min_rto(&self) -> Option<&u32> {
        self.min_rto.as_ref()
    }

    /// Sets the value of NonSackRttResiliency
    pub fn set_non_sack_rtt_resiliency(&mut self, value: u8) {
        self.non_sack_rtt_resiliency = Some(value);
    }

    /// Gets the value of NonSackRttResiliency
    pub fn get_non_sack_rtt_resiliency(&self) -> Option<&u8> {
        self.non_sack_rtt_resiliency.as_ref()
    }

    /// Sets the value of ScalingHeuristics
    pub fn set_scaling_heuristics(&mut self, value: u8) {
        self.scaling_heuristics = Some(value);
    }

    /// Gets the value of ScalingHeuristics
    pub fn get_scaling_heuristics(&self) -> Option<&u8> {
        self.scaling_heuristics.as_ref()
    }

    /// Sets the value of SettingName
    pub fn set_setting_name(&mut self, value: String) {
        self.setting_name = Some(value);
    }

    /// Gets the value of SettingName
    pub fn get_setting_name(&self) -> Option<&String> {
        self.setting_name.as_ref()
    }

    /// Sets the value of Timestamps
    pub fn set_timestamps(&mut self, value: u8) {
        self.timestamps = Some(value);
    }

    /// Gets the value of Timestamps
    pub fn get_timestamps(&self) -> Option<&u8> {
        self.timestamps.as_ref()
    }
}

