// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.MSCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSCluster_Cluster struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSCluster_Cluster {
    #[serde(flatten)]
    pub base: CIM_Cluster,

/// 
    #[serde(rename = "AcceleratedNetworkingEnabled")]
    pub accelerated_networking_enabled: Option<u32>,

/// 
    #[serde(rename = "AcceleratedNetworkingNodeReserve")]
    pub accelerated_networking_node_reserve: Option<u32>,

/// 
    #[serde(rename = "AddEvictDelay")]
    pub add_evict_delay: Option<u32>,

/// 
    #[serde(rename = "AdminAccessPoint")]
    pub admin_access_point: Option<u32>,

/// 
    #[serde(rename = "AdminExtensions")]
    pub admin_extensions: Vec<String>,

/// 
    #[serde(rename = "AutoAssignNodeSite")]
    pub auto_assign_node_site: Option<u32>,

/// 
    #[serde(rename = "AutoBalancerLevel")]
    pub auto_balancer_level: Option<u32>,

/// 
    #[serde(rename = "AutoBalancerMode")]
    pub auto_balancer_mode: Option<u32>,

/// 
    #[serde(rename = "BackupInProgress")]
    pub backup_in_progress: Option<u32>,

/// 
    #[serde(rename = "BlockCacheSize")]
    pub block_cache_size: Option<u32>,

/// 
    #[serde(rename = "ClusSvcHangTimeout")]
    pub clus_svc_hang_timeout: Option<u32>,

/// 
    #[serde(rename = "ClusSvcRegroupOpeningTimeout")]
    pub clus_svc_regroup_opening_timeout: Option<u32>,

/// 
    #[serde(rename = "ClusSvcRegroupPruningTimeout")]
    pub clus_svc_regroup_pruning_timeout: Option<u32>,

/// 
    #[serde(rename = "ClusSvcRegroupStageTimeout")]
    pub clus_svc_regroup_stage_timeout: Option<u32>,

/// 
    #[serde(rename = "ClusSvcRegroupTickInMilliseconds")]
    pub clus_svc_regroup_tick_in_milliseconds: Option<u32>,

/// 
    #[serde(rename = "ClusterEnforcedAntiAffinity")]
    pub cluster_enforced_anti_affinity: Option<u32>,

/// 
    #[serde(rename = "ClusterFunctionalLevel")]
    pub cluster_functional_level: Option<u32>,

/// 
    #[serde(rename = "ClusterGroupWaitDelay")]
    pub cluster_group_wait_delay: Option<u32>,

/// 
    #[serde(rename = "ClusterLogLevel")]
    pub cluster_log_level: Option<u32>,

/// 
    #[serde(rename = "ClusterLogSize")]
    pub cluster_log_size: Option<u32>,

/// 
    #[serde(rename = "ClusterUpgradeVersion")]
    pub cluster_upgrade_version: Option<u32>,

/// 
    #[serde(rename = "CrossSiteDelay")]
    pub cross_site_delay: Option<u32>,

/// 
    #[serde(rename = "CrossSiteThreshold")]
    pub cross_site_threshold: Option<u32>,

/// 
    #[serde(rename = "CrossSubnetDelay")]
    pub cross_subnet_delay: Option<u32>,

/// 
    #[serde(rename = "CrossSubnetThreshold")]
    pub cross_subnet_threshold: Option<u32>,

/// 
    #[serde(rename = "CsvBalancer")]
    pub csv_balancer: Option<u32>,

/// 
    #[serde(rename = "CsvTimeToWait")]
    pub csv_time_to_wait: Option<u32>,

/// 
    #[serde(rename = "DatabaseReadWriteMode")]
    pub database_read_write_mode: Option<u32>,

/// 
    #[serde(rename = "DefaultNetworkRole")]
    pub default_network_role: Option<u32>,

/// 
    #[serde(rename = "DetectedCloudPlatform")]
    pub detected_cloud_platform: Option<u32>,

/// 
    #[serde(rename = "DetectManagedEvents")]
    pub detect_managed_events: Option<u32>,

/// 
    #[serde(rename = "DetectManagedEventsThreshold")]
    pub detect_managed_events_threshold: Option<u32>,

/// 
    #[serde(rename = "DisableGroupPreferredOwnerRandomization")]
    pub disable_group_preferred_owner_randomization: Option<u32>,

/// 
    #[serde(rename = "DpcWatchdogProfileCumulativeDpcThreshold")]
    pub dpc_watchdog_profile_cumulative_dpc_threshold: Option<u32>,

/// 
    #[serde(rename = "DpcWatchdogProfileSingleDpcThreshold")]
    pub dpc_watchdog_profile_single_dpc_threshold: Option<u32>,

/// 
    #[serde(rename = "DrainOnShutdown")]
    pub drain_on_shutdown: Option<u32>,

/// 
    #[serde(rename = "DumpPolicy")]
    pub dump_policy: Option<u64>,

/// 
    #[serde(rename = "DynamicQuorumEnabled")]
    pub dynamic_quorum_enabled: Option<u32>,

/// 
    #[serde(rename = "EnabledEventLogs")]
    pub enabled_event_logs: Vec<String>,

/// 
    #[serde(rename = "EnableSharedVolumes")]
    pub enable_shared_volumes: Option<u32>,

/// 
    #[serde(rename = "FixQuorum")]
    pub fix_quorum: Option<u32>,

/// 
    #[serde(rename = "Fqdn")]
    pub fqdn: Option<String>,

/// 
    #[serde(rename = "GlobalWprSessionConfig")]
    pub global_wpr_session_config: Vec<String>,

/// 
    #[serde(rename = "GracePeriodEnabled")]
    pub grace_period_enabled: Option<u32>,

/// 
    #[serde(rename = "GracePeriodTimeout")]
    pub grace_period_timeout: Option<u32>,

/// 
    #[serde(rename = "GroupAdminExtensions")]
    pub group_admin_extensions: Vec<String>,

/// 
    #[serde(rename = "GroupDependencyTimeout")]
    pub group_dependency_timeout: Option<u32>,

/// 
    #[serde(rename = "HangRecoveryAction")]
    pub hang_recovery_action: Option<u32>,

/// 
    #[serde(rename = "IgnorePersistentStateOnStartup")]
    pub ignore_persistent_state_on_startup: Option<u32>,

/// 
    #[serde(rename = "LogResourceControls")]
    pub log_resource_controls: Option<u32>,

/// 
    #[serde(rename = "LowerQuorumPriorityNodeId")]
    pub lower_quorum_priority_node_id: Option<u32>,

/// 
    #[serde(rename = "MaintenanceFile")]
    pub maintenance_file: Option<String>,

/// 
    #[serde(rename = "MaximumParallelMigrations")]
    pub maximum_parallel_migrations: Option<u32>,

/// 
    #[serde(rename = "MessageBufferLength")]
    pub message_buffer_length: Option<u32>,

/// 
    #[serde(rename = "MinimumNeverPreemptPriority")]
    pub minimum_never_preempt_priority: Option<u32>,

/// 
    #[serde(rename = "MinimumPreemptorPriority")]
    pub minimum_preemptor_priority: Option<u32>,

/// 
    #[serde(rename = "NetftIPSecEnabled")]
    pub netft_ipsec_enabled: Option<u32>,

/// 
    #[serde(rename = "NetworkAdminExtensions")]
    pub network_admin_extensions: Vec<String>,

/// 
    #[serde(rename = "NetworkInterfaceAdminExtensions")]
    pub network_interface_admin_extensions: Vec<String>,

/// 
    #[serde(rename = "NetworkPriorities")]
    pub network_priorities: Vec<String>,

/// 
    #[serde(rename = "NodeAdminExtensions")]
    pub node_admin_extensions: Vec<String>,

/// 
    #[serde(rename = "PlacementOptions")]
    pub placement_options: Option<u32>,

/// 
    #[serde(rename = "PlumbAllCrossSubnetRoutes")]
    pub plumb_all_cross_subnet_routes: Option<u32>,

/// 
    #[serde(rename = "PreferredSite")]
    pub preferred_site: Option<String>,

/// 
    #[serde(rename = "PreventQuorum")]
    pub prevent_quorum: Option<u32>,

/// 
    #[serde(rename = "PrivateProperties")]
    pub private_properties: Option<MSCluster_Property>,

/// 
    #[serde(rename = "QuarantineDuration")]
    pub quarantine_duration: Option<u32>,

/// 
    #[serde(rename = "QuarantineThreshold")]
    pub quarantine_threshold: Option<u32>,

/// 
    #[serde(rename = "QuorumArbitrationTimeMax")]
    pub quorum_arbitration_time_max: Option<u32>,

/// 
    #[serde(rename = "QuorumArbitrationTimeMin")]
    pub quorum_arbitration_time_min: Option<u32>,

/// 
    #[serde(rename = "QuorumLogFileSize")]
    pub quorum_log_file_size: Option<u32>,

/// 
    #[serde(rename = "QuorumPath")]
    pub quorum_path: Option<String>,

/// 
    #[serde(rename = "QuorumType")]
    pub quorum_type: Option<String>,

/// 
    #[serde(rename = "QuorumTypeValue")]
    pub quorum_type_value: Option<u32>,

/// 
    #[serde(rename = "RdmaConnectionsPerInterfaceForStorage")]
    pub rdma_connections_per_interface_for_storage: Option<u32>,

/// 
    #[serde(rename = "RecentEventsResetTime")]
    pub recent_events_reset_time: Option<String>,

/// 
    #[serde(rename = "RequestReplyTimeout")]
    pub request_reply_timeout: Option<u32>,

/// 
    #[serde(rename = "ResiliencyDefaultPeriod")]
    pub resiliency_default_period: Option<u32>,

/// 
    #[serde(rename = "ResiliencyLevel")]
    pub resiliency_level: Option<u32>,

/// 
    #[serde(rename = "ResourceAdminExtensions")]
    pub resource_admin_extensions: Vec<String>,

/// 
    #[serde(rename = "ResourceDllDeadlockPeriod")]
    pub resource_dll_deadlock_period: Option<u32>,

/// 
    #[serde(rename = "ResourceTypeAdminExtensions")]
    pub resource_type_admin_extensions: Vec<String>,

/// 
    #[serde(rename = "RolloutAudience")]
    pub rollout_audience: Option<u32>,

/// 
    #[serde(rename = "RootMemoryReserved")]
    pub root_memory_reserved: Option<u32>,

/// 
    #[serde(rename = "RouteHistoryLength")]
    pub route_history_length: Option<u32>,

/// 
    #[serde(rename = "S2DBusTypes")]
    pub s2_dbus_types: Option<u32>,

/// 
    #[serde(rename = "S2DCacheBehavior")]
    pub s2_dcache_behavior: Option<u64>,

/// 
    #[serde(rename = "S2DCacheDesiredState")]
    pub s2_dcache_desired_state: Option<u32>,

/// 
    #[serde(rename = "S2DCacheDeviceModel")]
    pub s2_dcache_device_model: Vec<String>,

/// 
    #[serde(rename = "S2DCacheFlashReservePercent")]
    pub s2_dcache_flash_reserve_percent: Option<u32>,

/// 
    #[serde(rename = "S2DCacheMetadataReserveBytes")]
    pub s2_dcache_metadata_reserve_bytes: Option<u64>,

/// 
    #[serde(rename = "S2DCachePageSizeKBytes")]
    pub s2_dcache_page_size_kbytes: Option<u32>,

/// 
    #[serde(rename = "S2DEnabled")]
    pub s2_denabled: Option<u32>,

/// 
    #[serde(rename = "S2DIOLatencyThreshold")]
    pub s2_diolatency_threshold: Option<u32>,

/// 
    #[serde(rename = "S2DOptimizations")]
    pub s2_doptimizations: Option<u32>,

/// 
    #[serde(rename = "SameSubnetDelay")]
    pub same_subnet_delay: Option<u32>,

/// 
    #[serde(rename = "SameSubnetThreshold")]
    pub same_subnet_threshold: Option<u32>,

/// 
    #[serde(rename = "Security")]
    pub security: Vec<u8>,

/// 
    #[serde(rename = "Security_Descriptor")]
    pub security__descriptor: Vec<u8>,

/// 
    #[serde(rename = "SecurityLevel")]
    pub security_level: Option<u32>,

/// 
    #[serde(rename = "SecurityLevelForStorage")]
    pub security_level_for_storage: Option<u32>,

/// 
    #[serde(rename = "SetSMBBandwidthLimit")]
    pub set_smbbandwidth_limit: Option<u32>,

/// 
    #[serde(rename = "SharedVolumeCompatibleFilters")]
    pub shared_volume_compatible_filters: Vec<String>,

/// 
    #[serde(rename = "SharedVolumeIncompatibleFilters")]
    pub shared_volume_incompatible_filters: Vec<String>,

/// 
    #[serde(rename = "SharedVolumeSecurityDescriptor")]
    pub shared_volume_security_descriptor: Vec<u8>,

/// 
    #[serde(rename = "SharedVolumesRoot")]
    pub shared_volumes_root: Option<String>,

/// 
    #[serde(rename = "SharedVolumeVssWriterOperationTimeout")]
    pub shared_volume_vss_writer_operation_timeout: Option<u32>,

/// 
    #[serde(rename = "ShutdownTimeoutInMinutes")]
    pub shutdown_timeout_in_minutes: Option<u32>,

/// 
    #[serde(rename = "SMBBandwidthLimitFactor")]
    pub smbbandwidth_limit_factor: Option<u32>,

/// 
    #[serde(rename = "UseClientAccessNetworksForSharedVolumes")]
    pub use_client_access_networks_for_shared_volumes: Option<u32>,

/// 
    #[serde(rename = "UseRdmaForStorage")]
    pub use_rdma_for_storage: Option<u32>,

/// 
    #[serde(rename = "WitnessDatabaseWriteTimeout")]
    pub witness_database_write_timeout: Option<u32>,

/// 
    #[serde(rename = "WitnessDynamicWeight")]
    pub witness_dynamic_weight: Option<u32>,

/// 
    #[serde(rename = "WitnessRestartInterval")]
    pub witness_restart_interval: Option<u32>,

/// 
    #[serde(rename = "WprSessionCoolOffTime")]
    pub wpr_session_cool_off_time: Option<u32>,

/// 
    #[serde(rename = "WprSessionCount")]
    pub wpr_session_count: Option<u32>,
}

impl MSCluster_Cluster {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Cluster::new(),
            accelerated_networking_enabled: None,
            accelerated_networking_node_reserve: None,
            add_evict_delay: None,
            admin_access_point: None,
            admin_extensions: Vec::new(),
            auto_assign_node_site: None,
            auto_balancer_level: None,
            auto_balancer_mode: None,
            backup_in_progress: None,
            block_cache_size: None,
            clus_svc_hang_timeout: None,
            clus_svc_regroup_opening_timeout: None,
            clus_svc_regroup_pruning_timeout: None,
            clus_svc_regroup_stage_timeout: None,
            clus_svc_regroup_tick_in_milliseconds: None,
            cluster_enforced_anti_affinity: None,
            cluster_functional_level: None,
            cluster_group_wait_delay: None,
            cluster_log_level: None,
            cluster_log_size: None,
            cluster_upgrade_version: None,
            cross_site_delay: None,
            cross_site_threshold: None,
            cross_subnet_delay: None,
            cross_subnet_threshold: None,
            csv_balancer: None,
            csv_time_to_wait: None,
            database_read_write_mode: None,
            default_network_role: None,
            detected_cloud_platform: None,
            detect_managed_events: None,
            detect_managed_events_threshold: None,
            disable_group_preferred_owner_randomization: None,
            dpc_watchdog_profile_cumulative_dpc_threshold: None,
            dpc_watchdog_profile_single_dpc_threshold: None,
            drain_on_shutdown: None,
            dump_policy: None,
            dynamic_quorum_enabled: None,
            enabled_event_logs: Vec::new(),
            enable_shared_volumes: None,
            fix_quorum: None,
            fqdn: None,
            global_wpr_session_config: Vec::new(),
            grace_period_enabled: None,
            grace_period_timeout: None,
            group_admin_extensions: Vec::new(),
            group_dependency_timeout: None,
            hang_recovery_action: None,
            ignore_persistent_state_on_startup: None,
            log_resource_controls: None,
            lower_quorum_priority_node_id: None,
            maintenance_file: None,
            maximum_parallel_migrations: None,
            message_buffer_length: None,
            minimum_never_preempt_priority: None,
            minimum_preemptor_priority: None,
            netft_ipsec_enabled: None,
            network_admin_extensions: Vec::new(),
            network_interface_admin_extensions: Vec::new(),
            network_priorities: Vec::new(),
            node_admin_extensions: Vec::new(),
            placement_options: None,
            plumb_all_cross_subnet_routes: None,
            preferred_site: None,
            prevent_quorum: None,
            private_properties: None,
            quarantine_duration: None,
            quarantine_threshold: None,
            quorum_arbitration_time_max: None,
            quorum_arbitration_time_min: None,
            quorum_log_file_size: None,
            quorum_path: None,
            quorum_type: None,
            quorum_type_value: None,
            rdma_connections_per_interface_for_storage: None,
            recent_events_reset_time: None,
            request_reply_timeout: None,
            resiliency_default_period: None,
            resiliency_level: None,
            resource_admin_extensions: Vec::new(),
            resource_dll_deadlock_period: None,
            resource_type_admin_extensions: Vec::new(),
            rollout_audience: None,
            root_memory_reserved: None,
            route_history_length: None,
            s2_dbus_types: None,
            s2_dcache_behavior: None,
            s2_dcache_desired_state: None,
            s2_dcache_device_model: Vec::new(),
            s2_dcache_flash_reserve_percent: None,
            s2_dcache_metadata_reserve_bytes: None,
            s2_dcache_page_size_kbytes: None,
            s2_denabled: None,
            s2_diolatency_threshold: None,
            s2_doptimizations: None,
            same_subnet_delay: None,
            same_subnet_threshold: None,
            security: Vec::new(),
            security__descriptor: Vec::new(),
            security_level: None,
            security_level_for_storage: None,
            set_smbbandwidth_limit: None,
            shared_volume_compatible_filters: Vec::new(),
            shared_volume_incompatible_filters: Vec::new(),
            shared_volume_security_descriptor: Vec::new(),
            shared_volumes_root: None,
            shared_volume_vss_writer_operation_timeout: None,
            shutdown_timeout_in_minutes: None,
            smbbandwidth_limit_factor: None,
            use_client_access_networks_for_shared_volumes: None,
            use_rdma_for_storage: None,
            witness_database_write_timeout: None,
            witness_dynamic_weight: None,
            witness_restart_interval: None,
            wpr_session_cool_off_time: None,
            wpr_session_count: None,
        }
    }


    /// Sets the value of AcceleratedNetworkingEnabled
    pub fn set_accelerated_networking_enabled(&mut self, value: u32) {
        self.accelerated_networking_enabled = Some(value);
    }

    /// Gets the value of AcceleratedNetworkingEnabled
    pub fn get_accelerated_networking_enabled(&self) -> Option<&u32> {
        self.accelerated_networking_enabled.as_ref()
    }

    /// Sets the value of AcceleratedNetworkingNodeReserve
    pub fn set_accelerated_networking_node_reserve(&mut self, value: u32) {
        self.accelerated_networking_node_reserve = Some(value);
    }

    /// Gets the value of AcceleratedNetworkingNodeReserve
    pub fn get_accelerated_networking_node_reserve(&self) -> Option<&u32> {
        self.accelerated_networking_node_reserve.as_ref()
    }

    /// Sets the value of AddEvictDelay
    pub fn set_add_evict_delay(&mut self, value: u32) {
        self.add_evict_delay = Some(value);
    }

    /// Gets the value of AddEvictDelay
    pub fn get_add_evict_delay(&self) -> Option<&u32> {
        self.add_evict_delay.as_ref()
    }

    /// Sets the value of AdminAccessPoint
    pub fn set_admin_access_point(&mut self, value: u32) {
        self.admin_access_point = Some(value);
    }

    /// Gets the value of AdminAccessPoint
    pub fn get_admin_access_point(&self) -> Option<&u32> {
        self.admin_access_point.as_ref()
    }

    /// Sets the value of AdminExtensions
    pub fn set_admin_extensions(&mut self, value: Vec<String>) {
        self.admin_extensions = value;
    }

    /// Gets the value of AdminExtensions
    pub fn get_admin_extensions(&self) -> &Vec<String> {
        &self.admin_extensions
    }

    /// Sets the value of AutoAssignNodeSite
    pub fn set_auto_assign_node_site(&mut self, value: u32) {
        self.auto_assign_node_site = Some(value);
    }

    /// Gets the value of AutoAssignNodeSite
    pub fn get_auto_assign_node_site(&self) -> Option<&u32> {
        self.auto_assign_node_site.as_ref()
    }

    /// Sets the value of AutoBalancerLevel
    pub fn set_auto_balancer_level(&mut self, value: u32) {
        self.auto_balancer_level = Some(value);
    }

    /// Gets the value of AutoBalancerLevel
    pub fn get_auto_balancer_level(&self) -> Option<&u32> {
        self.auto_balancer_level.as_ref()
    }

    /// Sets the value of AutoBalancerMode
    pub fn set_auto_balancer_mode(&mut self, value: u32) {
        self.auto_balancer_mode = Some(value);
    }

    /// Gets the value of AutoBalancerMode
    pub fn get_auto_balancer_mode(&self) -> Option<&u32> {
        self.auto_balancer_mode.as_ref()
    }

    /// Sets the value of BackupInProgress
    pub fn set_backup_in_progress(&mut self, value: u32) {
        self.backup_in_progress = Some(value);
    }

    /// Gets the value of BackupInProgress
    pub fn get_backup_in_progress(&self) -> Option<&u32> {
        self.backup_in_progress.as_ref()
    }

    /// Sets the value of BlockCacheSize
    pub fn set_block_cache_size(&mut self, value: u32) {
        self.block_cache_size = Some(value);
    }

    /// Gets the value of BlockCacheSize
    pub fn get_block_cache_size(&self) -> Option<&u32> {
        self.block_cache_size.as_ref()
    }

    /// Sets the value of ClusSvcHangTimeout
    pub fn set_clus_svc_hang_timeout(&mut self, value: u32) {
        self.clus_svc_hang_timeout = Some(value);
    }

    /// Gets the value of ClusSvcHangTimeout
    pub fn get_clus_svc_hang_timeout(&self) -> Option<&u32> {
        self.clus_svc_hang_timeout.as_ref()
    }

    /// Sets the value of ClusSvcRegroupOpeningTimeout
    pub fn set_clus_svc_regroup_opening_timeout(&mut self, value: u32) {
        self.clus_svc_regroup_opening_timeout = Some(value);
    }

    /// Gets the value of ClusSvcRegroupOpeningTimeout
    pub fn get_clus_svc_regroup_opening_timeout(&self) -> Option<&u32> {
        self.clus_svc_regroup_opening_timeout.as_ref()
    }

    /// Sets the value of ClusSvcRegroupPruningTimeout
    pub fn set_clus_svc_regroup_pruning_timeout(&mut self, value: u32) {
        self.clus_svc_regroup_pruning_timeout = Some(value);
    }

    /// Gets the value of ClusSvcRegroupPruningTimeout
    pub fn get_clus_svc_regroup_pruning_timeout(&self) -> Option<&u32> {
        self.clus_svc_regroup_pruning_timeout.as_ref()
    }

    /// Sets the value of ClusSvcRegroupStageTimeout
    pub fn set_clus_svc_regroup_stage_timeout(&mut self, value: u32) {
        self.clus_svc_regroup_stage_timeout = Some(value);
    }

    /// Gets the value of ClusSvcRegroupStageTimeout
    pub fn get_clus_svc_regroup_stage_timeout(&self) -> Option<&u32> {
        self.clus_svc_regroup_stage_timeout.as_ref()
    }

    /// Sets the value of ClusSvcRegroupTickInMilliseconds
    pub fn set_clus_svc_regroup_tick_in_milliseconds(&mut self, value: u32) {
        self.clus_svc_regroup_tick_in_milliseconds = Some(value);
    }

    /// Gets the value of ClusSvcRegroupTickInMilliseconds
    pub fn get_clus_svc_regroup_tick_in_milliseconds(&self) -> Option<&u32> {
        self.clus_svc_regroup_tick_in_milliseconds.as_ref()
    }

    /// Sets the value of ClusterEnforcedAntiAffinity
    pub fn set_cluster_enforced_anti_affinity(&mut self, value: u32) {
        self.cluster_enforced_anti_affinity = Some(value);
    }

    /// Gets the value of ClusterEnforcedAntiAffinity
    pub fn get_cluster_enforced_anti_affinity(&self) -> Option<&u32> {
        self.cluster_enforced_anti_affinity.as_ref()
    }

    /// Sets the value of ClusterFunctionalLevel
    pub fn set_cluster_functional_level(&mut self, value: u32) {
        self.cluster_functional_level = Some(value);
    }

    /// Gets the value of ClusterFunctionalLevel
    pub fn get_cluster_functional_level(&self) -> Option<&u32> {
        self.cluster_functional_level.as_ref()
    }

    /// Sets the value of ClusterGroupWaitDelay
    pub fn set_cluster_group_wait_delay(&mut self, value: u32) {
        self.cluster_group_wait_delay = Some(value);
    }

    /// Gets the value of ClusterGroupWaitDelay
    pub fn get_cluster_group_wait_delay(&self) -> Option<&u32> {
        self.cluster_group_wait_delay.as_ref()
    }

    /// Sets the value of ClusterLogLevel
    pub fn set_cluster_log_level(&mut self, value: u32) {
        self.cluster_log_level = Some(value);
    }

    /// Gets the value of ClusterLogLevel
    pub fn get_cluster_log_level(&self) -> Option<&u32> {
        self.cluster_log_level.as_ref()
    }

    /// Sets the value of ClusterLogSize
    pub fn set_cluster_log_size(&mut self, value: u32) {
        self.cluster_log_size = Some(value);
    }

    /// Gets the value of ClusterLogSize
    pub fn get_cluster_log_size(&self) -> Option<&u32> {
        self.cluster_log_size.as_ref()
    }

    /// Sets the value of ClusterUpgradeVersion
    pub fn set_cluster_upgrade_version(&mut self, value: u32) {
        self.cluster_upgrade_version = Some(value);
    }

    /// Gets the value of ClusterUpgradeVersion
    pub fn get_cluster_upgrade_version(&self) -> Option<&u32> {
        self.cluster_upgrade_version.as_ref()
    }

    /// Sets the value of CrossSiteDelay
    pub fn set_cross_site_delay(&mut self, value: u32) {
        self.cross_site_delay = Some(value);
    }

    /// Gets the value of CrossSiteDelay
    pub fn get_cross_site_delay(&self) -> Option<&u32> {
        self.cross_site_delay.as_ref()
    }

    /// Sets the value of CrossSiteThreshold
    pub fn set_cross_site_threshold(&mut self, value: u32) {
        self.cross_site_threshold = Some(value);
    }

    /// Gets the value of CrossSiteThreshold
    pub fn get_cross_site_threshold(&self) -> Option<&u32> {
        self.cross_site_threshold.as_ref()
    }

    /// Sets the value of CrossSubnetDelay
    pub fn set_cross_subnet_delay(&mut self, value: u32) {
        self.cross_subnet_delay = Some(value);
    }

    /// Gets the value of CrossSubnetDelay
    pub fn get_cross_subnet_delay(&self) -> Option<&u32> {
        self.cross_subnet_delay.as_ref()
    }

    /// Sets the value of CrossSubnetThreshold
    pub fn set_cross_subnet_threshold(&mut self, value: u32) {
        self.cross_subnet_threshold = Some(value);
    }

    /// Gets the value of CrossSubnetThreshold
    pub fn get_cross_subnet_threshold(&self) -> Option<&u32> {
        self.cross_subnet_threshold.as_ref()
    }

    /// Sets the value of CsvBalancer
    pub fn set_csv_balancer(&mut self, value: u32) {
        self.csv_balancer = Some(value);
    }

    /// Gets the value of CsvBalancer
    pub fn get_csv_balancer(&self) -> Option<&u32> {
        self.csv_balancer.as_ref()
    }

    /// Sets the value of CsvTimeToWait
    pub fn set_csv_time_to_wait(&mut self, value: u32) {
        self.csv_time_to_wait = Some(value);
    }

    /// Gets the value of CsvTimeToWait
    pub fn get_csv_time_to_wait(&self) -> Option<&u32> {
        self.csv_time_to_wait.as_ref()
    }

    /// Sets the value of DatabaseReadWriteMode
    pub fn set_database_read_write_mode(&mut self, value: u32) {
        self.database_read_write_mode = Some(value);
    }

    /// Gets the value of DatabaseReadWriteMode
    pub fn get_database_read_write_mode(&self) -> Option<&u32> {
        self.database_read_write_mode.as_ref()
    }

    /// Sets the value of DefaultNetworkRole
    pub fn set_default_network_role(&mut self, value: u32) {
        self.default_network_role = Some(value);
    }

    /// Gets the value of DefaultNetworkRole
    pub fn get_default_network_role(&self) -> Option<&u32> {
        self.default_network_role.as_ref()
    }

    /// Sets the value of DetectedCloudPlatform
    pub fn set_detected_cloud_platform(&mut self, value: u32) {
        self.detected_cloud_platform = Some(value);
    }

    /// Gets the value of DetectedCloudPlatform
    pub fn get_detected_cloud_platform(&self) -> Option<&u32> {
        self.detected_cloud_platform.as_ref()
    }

    /// Sets the value of DetectManagedEvents
    pub fn set_detect_managed_events(&mut self, value: u32) {
        self.detect_managed_events = Some(value);
    }

    /// Gets the value of DetectManagedEvents
    pub fn get_detect_managed_events(&self) -> Option<&u32> {
        self.detect_managed_events.as_ref()
    }

    /// Sets the value of DetectManagedEventsThreshold
    pub fn set_detect_managed_events_threshold(&mut self, value: u32) {
        self.detect_managed_events_threshold = Some(value);
    }

    /// Gets the value of DetectManagedEventsThreshold
    pub fn get_detect_managed_events_threshold(&self) -> Option<&u32> {
        self.detect_managed_events_threshold.as_ref()
    }

    /// Sets the value of DisableGroupPreferredOwnerRandomization
    pub fn set_disable_group_preferred_owner_randomization(&mut self, value: u32) {
        self.disable_group_preferred_owner_randomization = Some(value);
    }

    /// Gets the value of DisableGroupPreferredOwnerRandomization
    pub fn get_disable_group_preferred_owner_randomization(&self) -> Option<&u32> {
        self.disable_group_preferred_owner_randomization.as_ref()
    }

    /// Sets the value of DpcWatchdogProfileCumulativeDpcThreshold
    pub fn set_dpc_watchdog_profile_cumulative_dpc_threshold(&mut self, value: u32) {
        self.dpc_watchdog_profile_cumulative_dpc_threshold = Some(value);
    }

    /// Gets the value of DpcWatchdogProfileCumulativeDpcThreshold
    pub fn get_dpc_watchdog_profile_cumulative_dpc_threshold(&self) -> Option<&u32> {
        self.dpc_watchdog_profile_cumulative_dpc_threshold.as_ref()
    }

    /// Sets the value of DpcWatchdogProfileSingleDpcThreshold
    pub fn set_dpc_watchdog_profile_single_dpc_threshold(&mut self, value: u32) {
        self.dpc_watchdog_profile_single_dpc_threshold = Some(value);
    }

    /// Gets the value of DpcWatchdogProfileSingleDpcThreshold
    pub fn get_dpc_watchdog_profile_single_dpc_threshold(&self) -> Option<&u32> {
        self.dpc_watchdog_profile_single_dpc_threshold.as_ref()
    }

    /// Sets the value of DrainOnShutdown
    pub fn set_drain_on_shutdown(&mut self, value: u32) {
        self.drain_on_shutdown = Some(value);
    }

    /// Gets the value of DrainOnShutdown
    pub fn get_drain_on_shutdown(&self) -> Option<&u32> {
        self.drain_on_shutdown.as_ref()
    }

    /// Sets the value of DumpPolicy
    pub fn set_dump_policy(&mut self, value: u64) {
        self.dump_policy = Some(value);
    }

    /// Gets the value of DumpPolicy
    pub fn get_dump_policy(&self) -> Option<&u64> {
        self.dump_policy.as_ref()
    }

    /// Sets the value of DynamicQuorumEnabled
    pub fn set_dynamic_quorum_enabled(&mut self, value: u32) {
        self.dynamic_quorum_enabled = Some(value);
    }

    /// Gets the value of DynamicQuorumEnabled
    pub fn get_dynamic_quorum_enabled(&self) -> Option<&u32> {
        self.dynamic_quorum_enabled.as_ref()
    }

    /// Sets the value of EnabledEventLogs
    pub fn set_enabled_event_logs(&mut self, value: Vec<String>) {
        self.enabled_event_logs = value;
    }

    /// Gets the value of EnabledEventLogs
    pub fn get_enabled_event_logs(&self) -> &Vec<String> {
        &self.enabled_event_logs
    }

    /// Sets the value of EnableSharedVolumes
    pub fn set_enable_shared_volumes(&mut self, value: u32) {
        self.enable_shared_volumes = Some(value);
    }

    /// Gets the value of EnableSharedVolumes
    pub fn get_enable_shared_volumes(&self) -> Option<&u32> {
        self.enable_shared_volumes.as_ref()
    }

    /// Sets the value of FixQuorum
    pub fn set_fix_quorum(&mut self, value: u32) {
        self.fix_quorum = Some(value);
    }

    /// Gets the value of FixQuorum
    pub fn get_fix_quorum(&self) -> Option<&u32> {
        self.fix_quorum.as_ref()
    }

    /// Sets the value of Fqdn
    pub fn set_fqdn(&mut self, value: String) {
        self.fqdn = Some(value);
    }

    /// Gets the value of Fqdn
    pub fn get_fqdn(&self) -> Option<&String> {
        self.fqdn.as_ref()
    }

    /// Sets the value of GlobalWprSessionConfig
    pub fn set_global_wpr_session_config(&mut self, value: Vec<String>) {
        self.global_wpr_session_config = value;
    }

    /// Gets the value of GlobalWprSessionConfig
    pub fn get_global_wpr_session_config(&self) -> &Vec<String> {
        &self.global_wpr_session_config
    }

    /// Sets the value of GracePeriodEnabled
    pub fn set_grace_period_enabled(&mut self, value: u32) {
        self.grace_period_enabled = Some(value);
    }

    /// Gets the value of GracePeriodEnabled
    pub fn get_grace_period_enabled(&self) -> Option<&u32> {
        self.grace_period_enabled.as_ref()
    }

    /// Sets the value of GracePeriodTimeout
    pub fn set_grace_period_timeout(&mut self, value: u32) {
        self.grace_period_timeout = Some(value);
    }

    /// Gets the value of GracePeriodTimeout
    pub fn get_grace_period_timeout(&self) -> Option<&u32> {
        self.grace_period_timeout.as_ref()
    }

    /// Sets the value of GroupAdminExtensions
    pub fn set_group_admin_extensions(&mut self, value: Vec<String>) {
        self.group_admin_extensions = value;
    }

    /// Gets the value of GroupAdminExtensions
    pub fn get_group_admin_extensions(&self) -> &Vec<String> {
        &self.group_admin_extensions
    }

    /// Sets the value of GroupDependencyTimeout
    pub fn set_group_dependency_timeout(&mut self, value: u32) {
        self.group_dependency_timeout = Some(value);
    }

    /// Gets the value of GroupDependencyTimeout
    pub fn get_group_dependency_timeout(&self) -> Option<&u32> {
        self.group_dependency_timeout.as_ref()
    }

    /// Sets the value of HangRecoveryAction
    pub fn set_hang_recovery_action(&mut self, value: u32) {
        self.hang_recovery_action = Some(value);
    }

    /// Gets the value of HangRecoveryAction
    pub fn get_hang_recovery_action(&self) -> Option<&u32> {
        self.hang_recovery_action.as_ref()
    }

    /// Sets the value of IgnorePersistentStateOnStartup
    pub fn set_ignore_persistent_state_on_startup(&mut self, value: u32) {
        self.ignore_persistent_state_on_startup = Some(value);
    }

    /// Gets the value of IgnorePersistentStateOnStartup
    pub fn get_ignore_persistent_state_on_startup(&self) -> Option<&u32> {
        self.ignore_persistent_state_on_startup.as_ref()
    }

    /// Sets the value of LogResourceControls
    pub fn set_log_resource_controls(&mut self, value: u32) {
        self.log_resource_controls = Some(value);
    }

    /// Gets the value of LogResourceControls
    pub fn get_log_resource_controls(&self) -> Option<&u32> {
        self.log_resource_controls.as_ref()
    }

    /// Sets the value of LowerQuorumPriorityNodeId
    pub fn set_lower_quorum_priority_node_id(&mut self, value: u32) {
        self.lower_quorum_priority_node_id = Some(value);
    }

    /// Gets the value of LowerQuorumPriorityNodeId
    pub fn get_lower_quorum_priority_node_id(&self) -> Option<&u32> {
        self.lower_quorum_priority_node_id.as_ref()
    }

    /// Sets the value of MaintenanceFile
    pub fn set_maintenance_file(&mut self, value: String) {
        self.maintenance_file = Some(value);
    }

    /// Gets the value of MaintenanceFile
    pub fn get_maintenance_file(&self) -> Option<&String> {
        self.maintenance_file.as_ref()
    }

    /// Sets the value of MaximumParallelMigrations
    pub fn set_maximum_parallel_migrations(&mut self, value: u32) {
        self.maximum_parallel_migrations = Some(value);
    }

    /// Gets the value of MaximumParallelMigrations
    pub fn get_maximum_parallel_migrations(&self) -> Option<&u32> {
        self.maximum_parallel_migrations.as_ref()
    }

    /// Sets the value of MessageBufferLength
    pub fn set_message_buffer_length(&mut self, value: u32) {
        self.message_buffer_length = Some(value);
    }

    /// Gets the value of MessageBufferLength
    pub fn get_message_buffer_length(&self) -> Option<&u32> {
        self.message_buffer_length.as_ref()
    }

    /// Sets the value of MinimumNeverPreemptPriority
    pub fn set_minimum_never_preempt_priority(&mut self, value: u32) {
        self.minimum_never_preempt_priority = Some(value);
    }

    /// Gets the value of MinimumNeverPreemptPriority
    pub fn get_minimum_never_preempt_priority(&self) -> Option<&u32> {
        self.minimum_never_preempt_priority.as_ref()
    }

    /// Sets the value of MinimumPreemptorPriority
    pub fn set_minimum_preemptor_priority(&mut self, value: u32) {
        self.minimum_preemptor_priority = Some(value);
    }

    /// Gets the value of MinimumPreemptorPriority
    pub fn get_minimum_preemptor_priority(&self) -> Option<&u32> {
        self.minimum_preemptor_priority.as_ref()
    }

    /// Sets the value of NetftIPSecEnabled
    pub fn set_netft_ipsec_enabled(&mut self, value: u32) {
        self.netft_ipsec_enabled = Some(value);
    }

    /// Gets the value of NetftIPSecEnabled
    pub fn get_netft_ipsec_enabled(&self) -> Option<&u32> {
        self.netft_ipsec_enabled.as_ref()
    }

    /// Sets the value of NetworkAdminExtensions
    pub fn set_network_admin_extensions(&mut self, value: Vec<String>) {
        self.network_admin_extensions = value;
    }

    /// Gets the value of NetworkAdminExtensions
    pub fn get_network_admin_extensions(&self) -> &Vec<String> {
        &self.network_admin_extensions
    }

    /// Sets the value of NetworkInterfaceAdminExtensions
    pub fn set_network_interface_admin_extensions(&mut self, value: Vec<String>) {
        self.network_interface_admin_extensions = value;
    }

    /// Gets the value of NetworkInterfaceAdminExtensions
    pub fn get_network_interface_admin_extensions(&self) -> &Vec<String> {
        &self.network_interface_admin_extensions
    }

    /// Sets the value of NetworkPriorities
    pub fn set_network_priorities(&mut self, value: Vec<String>) {
        self.network_priorities = value;
    }

    /// Gets the value of NetworkPriorities
    pub fn get_network_priorities(&self) -> &Vec<String> {
        &self.network_priorities
    }

    /// Sets the value of NodeAdminExtensions
    pub fn set_node_admin_extensions(&mut self, value: Vec<String>) {
        self.node_admin_extensions = value;
    }

    /// Gets the value of NodeAdminExtensions
    pub fn get_node_admin_extensions(&self) -> &Vec<String> {
        &self.node_admin_extensions
    }

    /// Sets the value of PlacementOptions
    pub fn set_placement_options(&mut self, value: u32) {
        self.placement_options = Some(value);
    }

    /// Gets the value of PlacementOptions
    pub fn get_placement_options(&self) -> Option<&u32> {
        self.placement_options.as_ref()
    }

    /// Sets the value of PlumbAllCrossSubnetRoutes
    pub fn set_plumb_all_cross_subnet_routes(&mut self, value: u32) {
        self.plumb_all_cross_subnet_routes = Some(value);
    }

    /// Gets the value of PlumbAllCrossSubnetRoutes
    pub fn get_plumb_all_cross_subnet_routes(&self) -> Option<&u32> {
        self.plumb_all_cross_subnet_routes.as_ref()
    }

    /// Sets the value of PreferredSite
    pub fn set_preferred_site(&mut self, value: String) {
        self.preferred_site = Some(value);
    }

    /// Gets the value of PreferredSite
    pub fn get_preferred_site(&self) -> Option<&String> {
        self.preferred_site.as_ref()
    }

    /// Sets the value of PreventQuorum
    pub fn set_prevent_quorum(&mut self, value: u32) {
        self.prevent_quorum = Some(value);
    }

    /// Gets the value of PreventQuorum
    pub fn get_prevent_quorum(&self) -> Option<&u32> {
        self.prevent_quorum.as_ref()
    }

    /// Sets the value of PrivateProperties
    pub fn set_private_properties(&mut self, value: MSCluster_Property) {
        self.private_properties = Some(value);
    }

    /// Gets the value of PrivateProperties
    pub fn get_private_properties(&self) -> Option<&MSCluster_Property> {
        self.private_properties.as_ref()
    }

    /// Sets the value of QuarantineDuration
    pub fn set_quarantine_duration(&mut self, value: u32) {
        self.quarantine_duration = Some(value);
    }

    /// Gets the value of QuarantineDuration
    pub fn get_quarantine_duration(&self) -> Option<&u32> {
        self.quarantine_duration.as_ref()
    }

    /// Sets the value of QuarantineThreshold
    pub fn set_quarantine_threshold(&mut self, value: u32) {
        self.quarantine_threshold = Some(value);
    }

    /// Gets the value of QuarantineThreshold
    pub fn get_quarantine_threshold(&self) -> Option<&u32> {
        self.quarantine_threshold.as_ref()
    }

    /// Sets the value of QuorumArbitrationTimeMax
    pub fn set_quorum_arbitration_time_max(&mut self, value: u32) {
        self.quorum_arbitration_time_max = Some(value);
    }

    /// Gets the value of QuorumArbitrationTimeMax
    pub fn get_quorum_arbitration_time_max(&self) -> Option<&u32> {
        self.quorum_arbitration_time_max.as_ref()
    }

    /// Sets the value of QuorumArbitrationTimeMin
    pub fn set_quorum_arbitration_time_min(&mut self, value: u32) {
        self.quorum_arbitration_time_min = Some(value);
    }

    /// Gets the value of QuorumArbitrationTimeMin
    pub fn get_quorum_arbitration_time_min(&self) -> Option<&u32> {
        self.quorum_arbitration_time_min.as_ref()
    }

    /// Sets the value of QuorumLogFileSize
    pub fn set_quorum_log_file_size(&mut self, value: u32) {
        self.quorum_log_file_size = Some(value);
    }

    /// Gets the value of QuorumLogFileSize
    pub fn get_quorum_log_file_size(&self) -> Option<&u32> {
        self.quorum_log_file_size.as_ref()
    }

    /// Sets the value of QuorumPath
    pub fn set_quorum_path(&mut self, value: String) {
        self.quorum_path = Some(value);
    }

    /// Gets the value of QuorumPath
    pub fn get_quorum_path(&self) -> Option<&String> {
        self.quorum_path.as_ref()
    }

    /// Sets the value of QuorumType
    pub fn set_quorum_type(&mut self, value: String) {
        self.quorum_type = Some(value);
    }

    /// Gets the value of QuorumType
    pub fn get_quorum_type(&self) -> Option<&String> {
        self.quorum_type.as_ref()
    }

    /// Sets the value of QuorumTypeValue
    pub fn set_quorum_type_value(&mut self, value: u32) {
        self.quorum_type_value = Some(value);
    }

    /// Gets the value of QuorumTypeValue
    pub fn get_quorum_type_value(&self) -> Option<&u32> {
        self.quorum_type_value.as_ref()
    }

    /// Sets the value of RdmaConnectionsPerInterfaceForStorage
    pub fn set_rdma_connections_per_interface_for_storage(&mut self, value: u32) {
        self.rdma_connections_per_interface_for_storage = Some(value);
    }

    /// Gets the value of RdmaConnectionsPerInterfaceForStorage
    pub fn get_rdma_connections_per_interface_for_storage(&self) -> Option<&u32> {
        self.rdma_connections_per_interface_for_storage.as_ref()
    }

    /// Sets the value of RecentEventsResetTime
    pub fn set_recent_events_reset_time(&mut self, value: String) {
        self.recent_events_reset_time = Some(value);
    }

    /// Gets the value of RecentEventsResetTime
    pub fn get_recent_events_reset_time(&self) -> Option<&String> {
        self.recent_events_reset_time.as_ref()
    }

    /// Sets the value of RequestReplyTimeout
    pub fn set_request_reply_timeout(&mut self, value: u32) {
        self.request_reply_timeout = Some(value);
    }

    /// Gets the value of RequestReplyTimeout
    pub fn get_request_reply_timeout(&self) -> Option<&u32> {
        self.request_reply_timeout.as_ref()
    }

    /// Sets the value of ResiliencyDefaultPeriod
    pub fn set_resiliency_default_period(&mut self, value: u32) {
        self.resiliency_default_period = Some(value);
    }

    /// Gets the value of ResiliencyDefaultPeriod
    pub fn get_resiliency_default_period(&self) -> Option<&u32> {
        self.resiliency_default_period.as_ref()
    }

    /// Sets the value of ResiliencyLevel
    pub fn set_resiliency_level(&mut self, value: u32) {
        self.resiliency_level = Some(value);
    }

    /// Gets the value of ResiliencyLevel
    pub fn get_resiliency_level(&self) -> Option<&u32> {
        self.resiliency_level.as_ref()
    }

    /// Sets the value of ResourceAdminExtensions
    pub fn set_resource_admin_extensions(&mut self, value: Vec<String>) {
        self.resource_admin_extensions = value;
    }

    /// Gets the value of ResourceAdminExtensions
    pub fn get_resource_admin_extensions(&self) -> &Vec<String> {
        &self.resource_admin_extensions
    }

    /// Sets the value of ResourceDllDeadlockPeriod
    pub fn set_resource_dll_deadlock_period(&mut self, value: u32) {
        self.resource_dll_deadlock_period = Some(value);
    }

    /// Gets the value of ResourceDllDeadlockPeriod
    pub fn get_resource_dll_deadlock_period(&self) -> Option<&u32> {
        self.resource_dll_deadlock_period.as_ref()
    }

    /// Sets the value of ResourceTypeAdminExtensions
    pub fn set_resource_type_admin_extensions(&mut self, value: Vec<String>) {
        self.resource_type_admin_extensions = value;
    }

    /// Gets the value of ResourceTypeAdminExtensions
    pub fn get_resource_type_admin_extensions(&self) -> &Vec<String> {
        &self.resource_type_admin_extensions
    }

    /// Sets the value of RolloutAudience
    pub fn set_rollout_audience(&mut self, value: u32) {
        self.rollout_audience = Some(value);
    }

    /// Gets the value of RolloutAudience
    pub fn get_rollout_audience(&self) -> Option<&u32> {
        self.rollout_audience.as_ref()
    }

    /// Sets the value of RootMemoryReserved
    pub fn set_root_memory_reserved(&mut self, value: u32) {
        self.root_memory_reserved = Some(value);
    }

    /// Gets the value of RootMemoryReserved
    pub fn get_root_memory_reserved(&self) -> Option<&u32> {
        self.root_memory_reserved.as_ref()
    }

    /// Sets the value of RouteHistoryLength
    pub fn set_route_history_length(&mut self, value: u32) {
        self.route_history_length = Some(value);
    }

    /// Gets the value of RouteHistoryLength
    pub fn get_route_history_length(&self) -> Option<&u32> {
        self.route_history_length.as_ref()
    }

    /// Sets the value of S2DBusTypes
    pub fn set_s2_dbus_types(&mut self, value: u32) {
        self.s2_dbus_types = Some(value);
    }

    /// Gets the value of S2DBusTypes
    pub fn get_s2_dbus_types(&self) -> Option<&u32> {
        self.s2_dbus_types.as_ref()
    }

    /// Sets the value of S2DCacheBehavior
    pub fn set_s2_dcache_behavior(&mut self, value: u64) {
        self.s2_dcache_behavior = Some(value);
    }

    /// Gets the value of S2DCacheBehavior
    pub fn get_s2_dcache_behavior(&self) -> Option<&u64> {
        self.s2_dcache_behavior.as_ref()
    }

    /// Sets the value of S2DCacheDesiredState
    pub fn set_s2_dcache_desired_state(&mut self, value: u32) {
        self.s2_dcache_desired_state = Some(value);
    }

    /// Gets the value of S2DCacheDesiredState
    pub fn get_s2_dcache_desired_state(&self) -> Option<&u32> {
        self.s2_dcache_desired_state.as_ref()
    }

    /// Sets the value of S2DCacheDeviceModel
    pub fn set_s2_dcache_device_model(&mut self, value: Vec<String>) {
        self.s2_dcache_device_model = value;
    }

    /// Gets the value of S2DCacheDeviceModel
    pub fn get_s2_dcache_device_model(&self) -> &Vec<String> {
        &self.s2_dcache_device_model
    }

    /// Sets the value of S2DCacheFlashReservePercent
    pub fn set_s2_dcache_flash_reserve_percent(&mut self, value: u32) {
        self.s2_dcache_flash_reserve_percent = Some(value);
    }

    /// Gets the value of S2DCacheFlashReservePercent
    pub fn get_s2_dcache_flash_reserve_percent(&self) -> Option<&u32> {
        self.s2_dcache_flash_reserve_percent.as_ref()
    }

    /// Sets the value of S2DCacheMetadataReserveBytes
    pub fn set_s2_dcache_metadata_reserve_bytes(&mut self, value: u64) {
        self.s2_dcache_metadata_reserve_bytes = Some(value);
    }

    /// Gets the value of S2DCacheMetadataReserveBytes
    pub fn get_s2_dcache_metadata_reserve_bytes(&self) -> Option<&u64> {
        self.s2_dcache_metadata_reserve_bytes.as_ref()
    }

    /// Sets the value of S2DCachePageSizeKBytes
    pub fn set_s2_dcache_page_size_kbytes(&mut self, value: u32) {
        self.s2_dcache_page_size_kbytes = Some(value);
    }

    /// Gets the value of S2DCachePageSizeKBytes
    pub fn get_s2_dcache_page_size_kbytes(&self) -> Option<&u32> {
        self.s2_dcache_page_size_kbytes.as_ref()
    }

    /// Sets the value of S2DEnabled
    pub fn set_s2_denabled(&mut self, value: u32) {
        self.s2_denabled = Some(value);
    }

    /// Gets the value of S2DEnabled
    pub fn get_s2_denabled(&self) -> Option<&u32> {
        self.s2_denabled.as_ref()
    }

    /// Sets the value of S2DIOLatencyThreshold
    pub fn set_s2_diolatency_threshold(&mut self, value: u32) {
        self.s2_diolatency_threshold = Some(value);
    }

    /// Gets the value of S2DIOLatencyThreshold
    pub fn get_s2_diolatency_threshold(&self) -> Option<&u32> {
        self.s2_diolatency_threshold.as_ref()
    }

    /// Sets the value of S2DOptimizations
    pub fn set_s2_doptimizations(&mut self, value: u32) {
        self.s2_doptimizations = Some(value);
    }

    /// Gets the value of S2DOptimizations
    pub fn get_s2_doptimizations(&self) -> Option<&u32> {
        self.s2_doptimizations.as_ref()
    }

    /// Sets the value of SameSubnetDelay
    pub fn set_same_subnet_delay(&mut self, value: u32) {
        self.same_subnet_delay = Some(value);
    }

    /// Gets the value of SameSubnetDelay
    pub fn get_same_subnet_delay(&self) -> Option<&u32> {
        self.same_subnet_delay.as_ref()
    }

    /// Sets the value of SameSubnetThreshold
    pub fn set_same_subnet_threshold(&mut self, value: u32) {
        self.same_subnet_threshold = Some(value);
    }

    /// Gets the value of SameSubnetThreshold
    pub fn get_same_subnet_threshold(&self) -> Option<&u32> {
        self.same_subnet_threshold.as_ref()
    }

    /// Sets the value of Security
    pub fn set_security(&mut self, value: Vec<u8>) {
        self.security = value;
    }

    /// Gets the value of Security
    pub fn get_security(&self) -> &Vec<u8> {
        &self.security
    }

    /// Sets the value of Security_Descriptor
    pub fn set_security__descriptor(&mut self, value: Vec<u8>) {
        self.security__descriptor = value;
    }

    /// Gets the value of Security_Descriptor
    pub fn get_security__descriptor(&self) -> &Vec<u8> {
        &self.security__descriptor
    }

    /// Sets the value of SecurityLevel
    pub fn set_security_level(&mut self, value: u32) {
        self.security_level = Some(value);
    }

    /// Gets the value of SecurityLevel
    pub fn get_security_level(&self) -> Option<&u32> {
        self.security_level.as_ref()
    }

    /// Sets the value of SecurityLevelForStorage
    pub fn set_security_level_for_storage(&mut self, value: u32) {
        self.security_level_for_storage = Some(value);
    }

    /// Gets the value of SecurityLevelForStorage
    pub fn get_security_level_for_storage(&self) -> Option<&u32> {
        self.security_level_for_storage.as_ref()
    }

    /// Sets the value of SetSMBBandwidthLimit
    pub fn set_set_smbbandwidth_limit(&mut self, value: u32) {
        self.set_smbbandwidth_limit = Some(value);
    }

    /// Gets the value of SetSMBBandwidthLimit
    pub fn get_set_smbbandwidth_limit(&self) -> Option<&u32> {
        self.set_smbbandwidth_limit.as_ref()
    }

    /// Sets the value of SharedVolumeCompatibleFilters
    pub fn set_shared_volume_compatible_filters(&mut self, value: Vec<String>) {
        self.shared_volume_compatible_filters = value;
    }

    /// Gets the value of SharedVolumeCompatibleFilters
    pub fn get_shared_volume_compatible_filters(&self) -> &Vec<String> {
        &self.shared_volume_compatible_filters
    }

    /// Sets the value of SharedVolumeIncompatibleFilters
    pub fn set_shared_volume_incompatible_filters(&mut self, value: Vec<String>) {
        self.shared_volume_incompatible_filters = value;
    }

    /// Gets the value of SharedVolumeIncompatibleFilters
    pub fn get_shared_volume_incompatible_filters(&self) -> &Vec<String> {
        &self.shared_volume_incompatible_filters
    }

    /// Sets the value of SharedVolumeSecurityDescriptor
    pub fn set_shared_volume_security_descriptor(&mut self, value: Vec<u8>) {
        self.shared_volume_security_descriptor = value;
    }

    /// Gets the value of SharedVolumeSecurityDescriptor
    pub fn get_shared_volume_security_descriptor(&self) -> &Vec<u8> {
        &self.shared_volume_security_descriptor
    }

    /// Sets the value of SharedVolumesRoot
    pub fn set_shared_volumes_root(&mut self, value: String) {
        self.shared_volumes_root = Some(value);
    }

    /// Gets the value of SharedVolumesRoot
    pub fn get_shared_volumes_root(&self) -> Option<&String> {
        self.shared_volumes_root.as_ref()
    }

    /// Sets the value of SharedVolumeVssWriterOperationTimeout
    pub fn set_shared_volume_vss_writer_operation_timeout(&mut self, value: u32) {
        self.shared_volume_vss_writer_operation_timeout = Some(value);
    }

    /// Gets the value of SharedVolumeVssWriterOperationTimeout
    pub fn get_shared_volume_vss_writer_operation_timeout(&self) -> Option<&u32> {
        self.shared_volume_vss_writer_operation_timeout.as_ref()
    }

    /// Sets the value of ShutdownTimeoutInMinutes
    pub fn set_shutdown_timeout_in_minutes(&mut self, value: u32) {
        self.shutdown_timeout_in_minutes = Some(value);
    }

    /// Gets the value of ShutdownTimeoutInMinutes
    pub fn get_shutdown_timeout_in_minutes(&self) -> Option<&u32> {
        self.shutdown_timeout_in_minutes.as_ref()
    }

    /// Sets the value of SMBBandwidthLimitFactor
    pub fn set_smbbandwidth_limit_factor(&mut self, value: u32) {
        self.smbbandwidth_limit_factor = Some(value);
    }

    /// Gets the value of SMBBandwidthLimitFactor
    pub fn get_smbbandwidth_limit_factor(&self) -> Option<&u32> {
        self.smbbandwidth_limit_factor.as_ref()
    }

    /// Sets the value of UseClientAccessNetworksForSharedVolumes
    pub fn set_use_client_access_networks_for_shared_volumes(&mut self, value: u32) {
        self.use_client_access_networks_for_shared_volumes = Some(value);
    }

    /// Gets the value of UseClientAccessNetworksForSharedVolumes
    pub fn get_use_client_access_networks_for_shared_volumes(&self) -> Option<&u32> {
        self.use_client_access_networks_for_shared_volumes.as_ref()
    }

    /// Sets the value of UseRdmaForStorage
    pub fn set_use_rdma_for_storage(&mut self, value: u32) {
        self.use_rdma_for_storage = Some(value);
    }

    /// Gets the value of UseRdmaForStorage
    pub fn get_use_rdma_for_storage(&self) -> Option<&u32> {
        self.use_rdma_for_storage.as_ref()
    }

    /// Sets the value of WitnessDatabaseWriteTimeout
    pub fn set_witness_database_write_timeout(&mut self, value: u32) {
        self.witness_database_write_timeout = Some(value);
    }

    /// Gets the value of WitnessDatabaseWriteTimeout
    pub fn get_witness_database_write_timeout(&self) -> Option<&u32> {
        self.witness_database_write_timeout.as_ref()
    }

    /// Sets the value of WitnessDynamicWeight
    pub fn set_witness_dynamic_weight(&mut self, value: u32) {
        self.witness_dynamic_weight = Some(value);
    }

    /// Gets the value of WitnessDynamicWeight
    pub fn get_witness_dynamic_weight(&self) -> Option<&u32> {
        self.witness_dynamic_weight.as_ref()
    }

    /// Sets the value of WitnessRestartInterval
    pub fn set_witness_restart_interval(&mut self, value: u32) {
        self.witness_restart_interval = Some(value);
    }

    /// Gets the value of WitnessRestartInterval
    pub fn get_witness_restart_interval(&self) -> Option<&u32> {
        self.witness_restart_interval.as_ref()
    }

    /// Sets the value of WprSessionCoolOffTime
    pub fn set_wpr_session_cool_off_time(&mut self, value: u32) {
        self.wpr_session_cool_off_time = Some(value);
    }

    /// Gets the value of WprSessionCoolOffTime
    pub fn get_wpr_session_cool_off_time(&self) -> Option<&u32> {
        self.wpr_session_cool_off_time.as_ref()
    }

    /// Sets the value of WprSessionCount
    pub fn set_wpr_session_count(&mut self, value: u32) {
        self.wpr_session_count = Some(value);
    }

    /// Gets the value of WprSessionCount
    pub fn get_wpr_session_count(&self) -> Option<&u32> {
        self.wpr_session_count.as_ref()
    }

/// 

    /// * `new_name` -  (String)
    /// * `reason` -  (String)
    pub fn rename(&self, new_name: &String, reason: &Option<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "NewName".to_string(), value: new_name.into() });
        if let Some(val) = reason {
            args.push(MethodParameter { name: "Reason".to_string(), value: val.into() });
        }
        self.invoke_method("Rename", &args)

    }


/// 

    /// * `quorum_path` -  (String)
    /// * `reason` -  (String)
    /// * `resource` -  (String)
    pub fn set_majority_quorum(&self, resource: &String, quorum_path: &String, reason: &Option<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Resource".to_string(), value: resource.into() });
        args.push(MethodParameter { name: "QuorumPath".to_string(), value: quorum_path.into() });
        if let Some(val) = reason {
            args.push(MethodParameter { name: "Reason".to_string(), value: val.into() });
        }
        self.invoke_method("SetMajorityQuorum", &args)

    }


/// 

    /// * `quorum_path` -  (String)
    /// * `reason` -  (String)
    /// * `resource` -  (String)
    pub fn set_disk_quorum(&self, resource: &String, quorum_path: &String, reason: &Option<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Resource".to_string(), value: resource.into() });
        args.push(MethodParameter { name: "QuorumPath".to_string(), value: quorum_path.into() });
        if let Some(val) = reason {
            args.push(MethodParameter { name: "Reason".to_string(), value: val.into() });
        }
        self.invoke_method("SetDiskQuorum", &args)

    }


/// 

    /// * `reason` -  (String)
    pub fn set_node_majority_quorum(&self, reason: &Option<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        if let Some(val) = reason {
            args.push(MethodParameter { name: "Reason".to_string(), value: val.into() });
        }
        self.invoke_method("SetNodeMajorityQuorum", &args)

    }


/// 

    /// * `cluster_state` -  (i32)
    pub fn get_node_cluster_state(&self, cluster_state: &mut i32) -> Result<(), WmiError> {

        let result = self.invoke_method("GetNodeClusterState", &[])?;
        let cluster_state = result.get_value("ClusterState")?;
        Ok(result.return_value)

    }


/// 

    /// * `administrative_access_point` -  (u32)
    /// * `cluster_name` -  (String)
    /// * `ipaddresses` -  (String[])
    /// * `node_names` -  (String[])
    /// * `subnet_masks` -  (String[])
    pub fn create_cluster(&self, cluster_name: &String, node_names: &Vec<String>, ipaddresses: &Vec<String>, subnet_masks: &Vec<String>, administrative_access_point: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ClusterName".to_string(), value: cluster_name.into() });
        args.push(MethodParameter { name: "NodeNames".to_string(), value: node_names.into() });
        args.push(MethodParameter { name: "IPAddresses".to_string(), value: ipaddresses.into() });
        args.push(MethodParameter { name: "SubnetMasks".to_string(), value: subnet_masks.into() });
        args.push(MethodParameter { name: "AdministrativeAccessPoint".to_string(), value: administrative_access_point.into() });
        self.invoke_method("CreateCluster", &args)

    }


/// 

    /// * `administrative_access_point` -  (u32)
    /// * `administrative_access_point_resource_type` -  (u32)
    /// * `cluster_name` -  (String)
    /// * `ipaddresses` -  (String[])
    /// * `node_names` -  (String[])
    /// * `subnet_masks` -  (String[])
    pub fn create_cluster_ex(&self, cluster_name: &String, node_names: &Vec<String>, ipaddresses: &Vec<String>, subnet_masks: &Vec<String>, administrative_access_point: u32, administrative_access_point_resource_type: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ClusterName".to_string(), value: cluster_name.into() });
        args.push(MethodParameter { name: "NodeNames".to_string(), value: node_names.into() });
        args.push(MethodParameter { name: "IPAddresses".to_string(), value: ipaddresses.into() });
        args.push(MethodParameter { name: "SubnetMasks".to_string(), value: subnet_masks.into() });
        args.push(MethodParameter { name: "AdministrativeAccessPoint".to_string(), value: administrative_access_point.into() });
        args.push(MethodParameter { name: "AdministrativeAccessPointResourceType".to_string(), value: administrative_access_point_resource_type.into() });
        self.invoke_method("CreateClusterEx", &args)

    }


/// 

    /// * `node_name` -  (String)
    pub fn add_node(&self, node_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "NodeName".to_string(), value: node_name.into() });
        self.invoke_method("AddNode", &args)

    }


/// 

    /// * `node_name` -  (String)
    /// * `reason` -  (String)
    pub fn evict_node(&self, node_name: &String, reason: &Option<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "NodeName".to_string(), value: node_name.into() });
        if let Some(val) = reason {
            args.push(MethodParameter { name: "Reason".to_string(), value: val.into() });
        }
        self.invoke_method("EvictNode", &args)

    }


/// 

    /// * `cleanup_ad` -  (bool)
    pub fn destroy_cluster(&self, cleanup_ad: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "CleanupAD".to_string(), value: cleanup_ad.into() });
        self.invoke_method("DestroyCluster", &args)

    }


/// 

    /// * `node_name` -  (String)
    /// * `timeout` -  (u32)
    pub fn force_cleanup(&self, node_name: &String, timeout: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "NodeName".to_string(), value: node_name.into() });
        args.push(MethodParameter { name: "Timeout".to_string(), value: timeout.into() });
        self.invoke_method("ForceCleanup", &args)

    }


/// 

    /// * `control_code` -  (i32)
    /// * `input_buffer` -  (u8[])
    /// * `reason` -  (String)

    /// * `output_buffer` -  (u8[])
    /// * `output_buffer_size` -  (i32)
    pub fn execute_cluster_control(&self, control_code: i32, input_buffer: &Vec<u8>, output_buffer: &mut Vec<u8>, output_buffer_size: &mut i32, reason: &Option<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ControlCode".to_string(), value: control_code.into() });
        args.push(MethodParameter { name: "InputBuffer".to_string(), value: input_buffer.into() });
        if let Some(val) = reason {
            args.push(MethodParameter { name: "Reason".to_string(), value: val.into() });
        }

        let result = self.invoke_method("ExecuteClusterControl", &args)?;
        let output_buffer = result.get_value("OutputBuffer")?;
        let output_buffer_size = result.get_value("OutputBufferSize")?;
        Ok(result.return_value)

    }


/// 

    /// * `resource` -  (String)
    pub fn add_resource_to_cluster_shared_volumes(&self, resource: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Resource".to_string(), value: resource.into() });
        self.invoke_method("AddResourceToClusterSharedVolumes", &args)

    }


/// 

    /// * `resource` -  (String)
    pub fn remove_resource_from_cluster_shared_volumes(&self, resource: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Resource".to_string(), value: resource.into() });
        self.invoke_method("RemoveResourceFromClusterSharedVolumes", &args)

    }


/// 

    /// * `status` -  (MSCluster_ValidationStatus)
    pub fn generate_validation_status(&self, status: &mut MSCluster_ValidationStatus) -> Result<(), WmiError> {

        let result = self.invoke_method("GenerateValidationStatus", &[])?;
        let status = result.get_value("Status")?;
        Ok(result.return_value)

    }


/// 

    /// * `virtual_machine` -  (String)
    pub fn add_virtual_machine(&self, virtual_machine: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VirtualMachine".to_string(), value: virtual_machine.into() });
        self.invoke_method("AddVirtualMachine", &args)

    }


/// 

    /// * `group` -  (String)
    /// * `path` -  (String)

    /// * `result` -  (u32)
    pub fn verify_path(&self, path: &String, group: &String, result: &mut u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Path".to_string(), value: path.into() });
        args.push(MethodParameter { name: "Group".to_string(), value: group.into() });

        let result = self.invoke_method("VerifyPath", &args)?;
        let result = result.get_value("result")?;
        Ok(result.return_value)

    }


/// 

    /// * `administrative_access_point` -  (u32)
    /// * `domain_name` -  (String)
    /// * `name` -  (String)
    /// * `password` -  (String)
    /// * `user_name` -  (String)
    pub fn add_cluster_name_account(&self, name: &String, domain_name: &String, user_name: &String, password: &String, administrative_access_point: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "DomainName".to_string(), value: domain_name.into() });
        args.push(MethodParameter { name: "UserName".to_string(), value: user_name.into() });
        args.push(MethodParameter { name: "Password".to_string(), value: password.into() });
        args.push(MethodParameter { name: "AdministrativeAccessPoint".to_string(), value: administrative_access_point.into() });
        self.invoke_method("AddClusterNameAccount", &args)

    }


/// 

    /// * `administrative_access_point` -  (u32)
    /// * `administrative_access_point_resource_type` -  (u32)
    /// * `domain_name` -  (String)
    /// * `name` -  (String)
    /// * `password` -  (String)
    /// * `upgrade_vcos` -  (u32)
    /// * `user_name` -  (String)
    pub fn add_cluster_name_account_ex(&self, name: &String, domain_name: &String, user_name: &String, password: &String, administrative_access_point: u32, administrative_access_point_resource_type: u32, upgrade_vcos: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "DomainName".to_string(), value: domain_name.into() });
        args.push(MethodParameter { name: "UserName".to_string(), value: user_name.into() });
        args.push(MethodParameter { name: "Password".to_string(), value: password.into() });
        args.push(MethodParameter { name: "AdministrativeAccessPoint".to_string(), value: administrative_access_point.into() });
        args.push(MethodParameter { name: "AdministrativeAccessPointResourceType".to_string(), value: administrative_access_point_resource_type.into() });
        args.push(MethodParameter { name: "upgradeVCOs".to_string(), value: upgrade_vcos.into() });
        self.invoke_method("AddClusterNameAccountEx", &args)

    }

}

