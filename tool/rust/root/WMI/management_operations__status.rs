// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ManagementOperations_Status
//////////////////////////////////////////////

/// ManagementOperations_Status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ManagementOperations_Status {
    /// Success
    #[serde(rename = "Success")]
    Success = 0,
    /// Non_Specific_Error
    #[serde(rename = "Non_Specific_Error")]
    NonSpecificError = 1,
    /// Login_Failed
    #[serde(rename = "Login_Failed")]
    LoginFailed = 2,
    /// Connection_Failed
    #[serde(rename = "Connection_Failed")]
    ConnectionFailed = 3,
    /// Initiator_Node_Already_Exists
    #[serde(rename = "Initiator_Node_Already_Exists")]
    InitiatorNodeAlreadyExists = 4,
    /// Initiator_Node_Does_Not_Exist
    #[serde(rename = "Initiator_Node_Does_Not_Exist")]
    InitiatorNodeDoesNotExist = 5,
    /// Target_Moved_Temporarily
    #[serde(rename = "Target_Moved_Temporarily")]
    TargetMovedTemporarily = 6,
    /// Target_Moved_Permamently
    #[serde(rename = "Target_Moved_Permamently")]
    TargetMovedPermamently = 7,
    /// Initiator_Error
    #[serde(rename = "Initiator_Error")]
    InitiatorError = 8,
    /// Authentication_Failure
    #[serde(rename = "Authentication_Failure")]
    AuthenticationFailure = 9,
    /// Authorization_Failure
    #[serde(rename = "Authorization_Failure")]
    AuthorizationFailure = 10,
    /// Not_Found
    #[serde(rename = "Not_Found")]
    NotFound = 11,
    /// Target_Removed
    #[serde(rename = "Target_Removed")]
    TargetRemoved = 12,
    /// Unsupported_Version
    #[serde(rename = "Unsupported_Version")]
    UnsupportedVersion = 13,
    /// Too_many_Connections
    #[serde(rename = "Too_many_Connections")]
    TooManyConnections = 14,
    /// Missing_Parameter
    #[serde(rename = "Missing_Parameter")]
    MissingParameter = 15,
    /// Can_not_include_in_session
    #[serde(rename = "Can_not_include_in_session")]
    CanNotIncludeInSession = 16,
    /// Session_type_not_supported
    #[serde(rename = "Session_type_not_supported")]
    SessionTypeNotSupported = 17,
    /// Target_Error
    #[serde(rename = "Target_Error")]
    TargetError = 18,
    /// Service_Unavailable
    #[serde(rename = "Service_Unavailable")]
    ServiceUnavailable = 19,
    /// Out_of_Resources
    #[serde(rename = "Out_of_Resources")]
    OutOfResources = 20,
    /// Connections_already_exist_on_initiator_node
    #[serde(rename = "Connections_already_exist_on_initiator_node")]
    ConnectionsAlreadyExistOnInitiatorNode = 21,
    /// Session_Already_Exists
    #[serde(rename = "Session_Already_Exists")]
    SessionAlreadyExists = 22,
    /// Initiator_Instance_Does_Not_Exist
    #[serde(rename = "Initiator_Instance_Does_Not_Exist")]
    InitiatorInstanceDoesNotExist = 23,
    /// Target_Already_Exists
    #[serde(rename = "Target_Already_Exists")]
    TargetAlreadyExists = 24,
    /// The_iscsi_driver_implementation_did_not_complete_an_operation_correctly
    #[serde(rename = "The_iscsi_driver_implementation_did_not_complete_an_operation_correctly")]
    TheIscsiDriverImplementationDidNotCompleteAnOperationCorrectly = 25,
    /// An_invalid_key_text_was_encountered
    #[serde(rename = "An_invalid_key_text_was_encountered")]
    AnInvalidKeyTextWasEncountered = 26,
    /// Invalid_SendTargets_response_text_was_encountered
    #[serde(rename = "Invalid_SendTargets_response_text_was_encountered")]
    InvalidSendTargetsResponseTextWasEncountered = 27,
    /// Invalid_Session_Id
    #[serde(rename = "Invalid_Session_Id")]
    InvalidSessionId = 28,
    /// The_scsi_request_failed
    #[serde(rename = "The_scsi_request_failed")]
    TheScsiRequestFailed = 29,
    /// Exceeded_max_sessions_for_this_initiator_
    #[serde(rename = "Exceeded_max_sessions_for_this_initiator_")]
    ExceededMaxSessionsForThisInitiator = 30,
    /// Session_is_busy_since_a_request_is_already_in_progress_
    #[serde(rename = "Session_is_busy_since_a_request_is_already_in_progress_")]
    SessionIsBusySinceARequestIsAlreadyInProgress = 31,
    /// The_target_mapping_is_not_available
    #[serde(rename = "The_target_mapping_is_not_available")]
    TheTargetMappingIsNotAvailable = 32,
    /// The_Target_Address_type_given_is_not_supported
    #[serde(rename = "The_Target_Address_type_given_is_not_supported")]
    TheTargetAddressTypeGivenIsNotSupported = 33,
    /// Logon_Failed
    #[serde(rename = "Logon_Failed")]
    LogonFailed = 34,
    /// TCP_Send_Failed
    #[serde(rename = "TCP_Send_Failed")]
    TCPSendFailed = 35,
    /// TCP_Transport_Error
    #[serde(rename = "TCP_Transport_Error")]
    TCPTransportError = 36,
    /// iSCSI_Version_Mismatch
    #[serde(rename = "iSCSI_Version_Mismatch")]
    ISCSIVersionMismatch = 37,
    /// The_Target_Mapping_Address_passed_is_out_of_range_for_the_adapter_configuration
    #[serde(rename = "The_Target_Mapping_Address_passed_is_out_of_range_for_the_adapter_configuration")]
    TheTargetMappingAddressPassedIsOutOfRangeForTheAdapterConfiguration = 38,
    /// The_preshared_key_for_the_target_or_IKE_identification_payload_is_not_available_
    #[serde(rename = "The_preshared_key_for_the_target_or_IKE_identification_payload_is_not_available_")]
    ThePresharedKeyForTheTargetOrIKEIdentificationPayloadIsNotAvailable = 39,
    /// The_authentication_information_for_the_target_is_not_available
    #[serde(rename = "The_authentication_information_for_the_target_is_not_available")]
    TheAuthenticationInformationForTheTargetIsNotAvailable = 40,
    /// The_target_name_is_not_found_or_is_marked_as_hidden_from_login_
    #[serde(rename = "The_target_name_is_not_found_or_is_marked_as_hidden_from_login_")]
    TheTargetNameIsNotFoundOrIsMarkedAsHiddenFromLogin = 41,
    /// One_or_more_parameters_specified_in_LoginTargetIN_structure_is_invalid_
    #[serde(rename = "One_or_more_parameters_specified_in_LoginTargetIN_structure_is_invalid_")]
    OneOrMoreParametersSpecifiedInLoginTargetINStructureIsInvalid = 42,
    /// Given_target_mapping_already_exists_
    #[serde(rename = "Given_target_mapping_already_exists_")]
    GivenTargetMappingAlreadyExists = 43,
    /// The_HBA_security_information_cache_is_full_
    #[serde(rename = "The_HBA_security_information_cache_is_full_")]
    TheHBASecurityInformationCacheIsFull = 44,
    /// The_port_number_passed_is_not_valid_for_the_initiator_
    #[serde(rename = "The_port_number_passed_is_not_valid_for_the_initiator_")]
    ThePortNumberPassedIsNotValidForTheInitiator = 45,
    /// Operation_was_not_successful_for_all_initiators_
    #[serde(rename = "Operation_was_not_successful_for_all_initiators_")]
    OperationWasNotSuccessfulForAllInitiators = 46,
    /// The_HBA_security_information_cache_is_not_supported_by_this_adapter_
    #[serde(rename = "The_HBA_security_information_cache_is_not_supported_by_this_adapter_")]
    TheHBASecurityInformationCacheIsNotSupportedByThisAdapter = 47,
    /// The_IKE_id_payload_type_specified_is_not_supported_
    #[serde(rename = "The_IKE_id_payload_type_specified_is_not_supported_")]
    TheIKEIdPayloadTypeSpecifiedIsNotSupported = 48,
    /// The_IKE_id_payload_size_specified_is_not_correct_
    #[serde(rename = "The_IKE_id_payload_size_specified_is_not_correct_")]
    TheIKEIdPayloadSizeSpecifiedIsNotCorrect = 49,
    /// Target_Portal_Structure_Already_Exists_
    #[serde(rename = "Target_Portal_Structure_Already_Exists_")]
    TargetPortalStructureAlreadyExists = 50,
    /// Target_Address_Structure_Already_Exists_
    #[serde(rename = "Target_Address_Structure_Already_Exists_")]
    TargetAddressStructureAlreadyExists = 51,
    /// There_is_no_IKE_authentication_information_available_
    #[serde(rename = "There_is_no_IKE_authentication_information_available_")]
    ThereIsNoIKEAuthenticationInformationAvailable = 52,
    /// There_is_no_tunnel_mode_outer_address_specified_
    #[serde(rename = "There_is_no_tunnel_mode_outer_address_specified_")]
    ThereIsNoTunnelModeOuterAddressSpecified = 53,
    /// Authentication_or_tunnel_address_cache_is_corrupted_
    #[serde(rename = "Authentication_or_tunnel_address_cache_is_corrupted_")]
    AuthenticationOrTunnelAddressCacheIsCorrupted = 54,
    /// The_request_or_operation_is_not_supported_
    #[serde(rename = "The_request_or_operation_is_not_supported_")]
    TheRequestOrOperationIsNotSupported = 55,
    /// The_target_does_not_have_enough_resources_to_process_the_given_request_
    #[serde(rename = "The_target_does_not_have_enough_resources_to_process_the_given_request_")]
    TheTargetDoesNotHaveEnoughResourcesToProcessTheGivenRequest = 56,
    /// The_initiator_service_did_not_respond_to_the_request_sent_by_the_driver_
    #[serde(rename = "The_initiator_service_did_not_respond_to_the_request_sent_by_the_driver_")]
    TheInitiatorServiceDidNotRespondToTheRequestSentByTheDriver = 57,
    /// The_iSNS_server_was_not_found_or_is_unavailable_
    #[serde(rename = "The_iSNS_server_was_not_found_or_is_unavailable_")]
    TheISNSServerWasNotFoundOrIsUnavailable = 58,
    /// The_operation_was_successful_but_requires_a_driver_reload_or_reboot_to_become_effective_
    #[serde(rename = "The_operation_was_successful_but_requires_a_driver_reload_or_reboot_to_become_effective_")]
    TheOperationWasSuccessfulButRequiresADriverReloadOrRebootToBecomeEffective = 59,
    /// There_is_no_target_portal_available_to_complete_the_login_
    #[serde(rename = "There_is_no_target_portal_available_to_complete_the_login_")]
    ThereIsNoTargetPortalAvailableToCompleteTheLogin = 60,
    /// Cannot_remove_the_last_connection_for_a_session_
    #[serde(rename = "Cannot_remove_the_last_connection_for_a_session_")]
    CannotRemoveTheLastConnectionForASession = 61,
    /// The_Microsoft_iSCSI_Initiator_Service_is_not_running__Please_start_the_service_and_retry_
    #[serde(rename = "The_Microsoft_iSCSI_Initiator_Service_is_not_running__Please_start_the_service_and_retry_")]
    TheMicrosoftISCSIInitiatorServiceIsNotRunningPleaseStartTheServiceAndRetry = 62,
    /// The_target_has_already_been_logged_in_via_an_iSCSI_session_
    #[serde(rename = "The_target_has_already_been_logged_in_via_an_iSCSI_session_")]
    TheTargetHasAlreadyBeenLoggedInViaAnISCSISession = 63,
    /// The_session_cannot_be_logged_out_since_a_device_on_that_session_is_currently_being_used_
    #[serde(rename = "The_session_cannot_be_logged_out_since_a_device_on_that_session_is_currently_being_used_")]
    TheSessionCannotBeLoggedOutSinceADeviceOnThatSessionIsCurrentlyBeingUsed = 64,
    /// Failed_to_save_persistent_login_information_
    #[serde(rename = "Failed_to_save_persistent_login_information_")]
    FailedToSavePersistentLoginInformation = 65,
    /// Failed_to_remove_persistent_login_information_
    #[serde(rename = "Failed_to_remove_persistent_login_information_")]
    FailedToRemovePersistentLoginInformation = 66,
    /// The_specified_initiator_name_was_not_found_
    #[serde(rename = "The_specified_initiator_name_was_not_found_")]
    TheSpecifiedInitiatorNameWasNotFound = 67,
    /// The_specified_portal_was_not_found_
    #[serde(rename = "The_specified_portal_was_not_found_")]
    TheSpecifiedPortalWasNotFound = 68,
    /// The_specified_discovery_mechanism_was_not_found_
    #[serde(rename = "The_specified_discovery_mechanism_was_not_found_")]
    TheSpecifiedDiscoveryMechanismWasNotFound = 69,
    /// iSCSI_does_not_support_IPSEC_for_this_version_of_the_OS_
    #[serde(rename = "iSCSI_does_not_support_IPSEC_for_this_version_of_the_OS_")]
    ISCSIDoesNotSupportIPSECForThisVersionOfTheOS = 70,
    /// The_iSCSI_service_timed_out_waiting_for_all_persistent_logins_to_complete_
    #[serde(rename = "The_iSCSI_service_timed_out_waiting_for_all_persistent_logins_to_complete_")]
    TheISCSIServiceTimedOutWaitingForAllPersistentLoginsToComplete = 71,
    /// The_specified_CHAP_secret_is_less_than_96_bits_and_will_not_be_usable_for_authenticating_over_non_ipsec_connections_
    #[serde(rename = "The_specified_CHAP_secret_is_less_than_96_bits_and_will_not_be_usable_for_authenticating_over_non_ipsec_connections_")]
    TheSpecifiedCHAPSecretIsLessThan96BitsAndWillNotBeUsableForAuthenticatingOverNonIpsecConnections = 72,
    /// The_evaluation_period_for_the_iSCSI_initiator_service_has_expired_
    #[serde(rename = "The_evaluation_period_for_the_iSCSI_initiator_service_has_expired_")]
    TheEvaluationPeriodForTheISCSIInitiatorServiceHasExpired = 73,
    /// CHAP_secret_given_does_not_conform_to_the_standard__Please_see_system_event_log_for_more_information_
    #[serde(rename = "CHAP_secret_given_does_not_conform_to_the_standard__Please_see_system_event_log_for_more_information_")]
    CHAPSecretGivenDoesNotConformToTheStandardPleaseSeeSystemEventLogForMoreInformation = 74,
    /// Target_CHAP_secret_given_is_invalid_
    #[serde(rename = "Target_CHAP_secret_given_is_invalid_")]
    TargetCHAPSecretGivenIsInvalid = 75,
    /// Initiator_CHAP_secret_given_is_invalid_
    #[serde(rename = "Initiator_CHAP_secret_given_is_invalid_")]
    InitiatorCHAPSecretGivenIsInvalid = 76,
    /// CHAP_Username_given_is_invalid_
    #[serde(rename = "CHAP_Username_given_is_invalid_")]
    CHAPUsernameGivenIsInvalid = 77,
    /// Logon_Authentication_type_given_is_invalid_
    #[serde(rename = "Logon_Authentication_type_given_is_invalid_")]
    LogonAuthenticationTypeGivenIsInvalid = 78,
    /// Target_Mapping_information_given_is_invalid_
    #[serde(rename = "Target_Mapping_information_given_is_invalid_")]
    TargetMappingInformationGivenIsInvalid = 79,
    /// Target_Id_given_in_Target_Mapping_is_invalid_
    #[serde(rename = "Target_Id_given_in_Target_Mapping_is_invalid_")]
    TargetIdGivenInTargetMappingIsInvalid = 80,
    /// The_iSCSI_name_specified_contains_invalid_characters_or_is_too_long_
    #[serde(rename = "The_iSCSI_name_specified_contains_invalid_characters_or_is_too_long_")]
    TheISCSINameSpecifiedContainsInvalidCharactersOrIsTooLong = 81,
    /// The_iSNS_version_number_returned_from_the_iSNS_server_is_not_compatible_with_this_version_of_the_iSNS_client_
    #[serde(rename = "The_iSNS_version_number_returned_from_the_iSNS_server_is_not_compatible_with_this_version_of_the_iSNS_client_")]
    TheISNSVersionNumberReturnedFromTheISNSServerIsNotCompatibleWithThisVersionOfTheISNSClient = 82,
    /// Initiator_failed_to_configure_IPSec_for_the_given_connection__This_could_be_because_of_low_resources_
    #[serde(rename = "Initiator_failed_to_configure_IPSec_for_the_given_connection__This_could_be_because_of_low_resources_")]
    InitiatorFailedToConfigureIPSecForTheGivenConnectionThisCouldBeBecauseOfLowResources = 83,
    /// The_buffer_given_for_processing_the_request_is_too_small_
    #[serde(rename = "The_buffer_given_for_processing_the_request_is_too_small_")]
    TheBufferGivenForProcessingTheRequestIsTooSmall = 84,
    /// The_given_Load_Balance_policy_is_not_recognized_by_iScsi_initiator_
    #[serde(rename = "The_given_Load_Balance_policy_is_not_recognized_by_iScsi_initiator_")]
    TheGivenLoadBalancePolicyIsNotRecognizedByIScsiInitiator = 85,
    /// One_or_more_paramaters_specified_is_not_valid_
    #[serde(rename = "One_or_more_paramaters_specified_is_not_valid_")]
    OneOrMoreParamatersSpecifiedIsNotValid = 86,
    /// Duplicate_PathIds_were_specified_in_the_call_to_set_Load_Balance_Policy_
    #[serde(rename = "Duplicate_PathIds_were_specified_in_the_call_to_set_Load_Balance_Policy_")]
    DuplicatePathIdsWereSpecifiedInTheCallToSetLoadBalancePolicy = 87,
    /// Number_of_paths_specified_in_Set_Load_Balance_Policy_does_not_match_the_number_of_paths_to_the_target_
    #[serde(rename = "Number_of_paths_specified_in_Set_Load_Balance_Policy_does_not_match_the_number_of_paths_to_the_target_")]
    NumberOfPathsSpecifiedInSetLoadBalancePolicyDoesNotMatchTheNumberOfPathsToTheTarget = 88,
    /// Path_Id_specified_in_the_call_to_set_Load_Balance_Policy_is_not_valid_
    #[serde(rename = "Path_Id_specified_in_the_call_to_set_Load_Balance_Policy_is_not_valid_")]
    PathIdSpecifiedInTheCallToSetLoadBalancePolicyIsNotValid = 89,
    /// Multiple_primary_paths_specified_when_only_one_primary_path_is_expected_
    #[serde(rename = "Multiple_primary_paths_specified_when_only_one_primary_path_is_expected_")]
    MultiplePrimaryPathsSpecifiedWhenOnlyOnePrimaryPathIsExpected = 90,
    /// No_primary_path_specified_when_at_least_one_is_expected_
    #[serde(rename = "No_primary_path_specified_when_at_least_one_is_expected_")]
    NoPrimaryPathSpecifiedWhenAtLeastOneIsExpected = 91,
    /// Volume_is_already_a_persistently_bound_volume_
    #[serde(rename = "Volume_is_already_a_persistently_bound_volume_")]
    VolumeIsAlreadyAPersistentlyBoundVolume = 92,
    /// Volume_was_not_found_
    #[serde(rename = "Volume_was_not_found_")]
    VolumeWasNotFound = 93,
    /// The_volume_specified_does_not_originate_from_an_iSCSI_disk_
    #[serde(rename = "The_volume_specified_does_not_originate_from_an_iSCSI_disk_")]
    TheVolumeSpecifiedDoesNotOriginateFromAnISCSIDisk = 94,
    /// The_DNS_name_specified_was_not_resolved_
    #[serde(rename = "The_DNS_name_specified_was_not_resolved_")]
    TheDNSNameSpecifiedWasNotResolved = 95,
    /// There_is_no_connection_available_in_the_iSCSI_session_to_process_the_request_
    #[serde(rename = "There_is_no_connection_available_in_the_iSCSI_session_to_process_the_request_")]
    ThereIsNoConnectionAvailableInTheISCSISessionToProcessTheRequest = 96,
    /// The_given_Load_Balance_policy_is_not_supported_
    #[serde(rename = "The_given_Load_Balance_policy_is_not_supported_")]
    TheGivenLoadBalancePolicyIsNotSupported = 97,
    /// A_remove_connection_request_is_already_in_progress_for_this_session_
    #[serde(rename = "A_remove_connection_request_is_already_in_progress_for_this_session_")]
    ARemoveConnectionRequestIsAlreadyInProgressForThisSession = 98,
    /// Given_connection_was_not_found_in_the_session_
    #[serde(rename = "Given_connection_was_not_found_in_the_session_")]
    GivenConnectionWasNotFoundInTheSession = 99,
    /// The_leading_connection_in_the_session_cannot_be_removed_
    #[serde(rename = "The_leading_connection_in_the_session_cannot_be_removed_")]
    TheLeadingConnectionInTheSessionCannotBeRemoved = 100,
}

impl Default for ManagementOperations_Status {
    fn default() -> Self {
        Self::Success
    }
}

