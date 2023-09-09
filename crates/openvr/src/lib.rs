#![allow(
	non_camel_case_types,
	dead_code,
	non_snake_case,
	non_upper_case_globals,
	clippy::not_unsafe_ptr_arg_deref
)]

use cppvtbl::{vtable, VtableRef};
use real_c_string::real_c_string;
use std::ffi::c_void;
use std::os::raw::c_char;

// Graphic api stubs
type VkDevice_T = ();
type VkInstance_T = ();
type VkQueue_T = ();
type VkPhysicalDevice_T = ();

#[derive(Clone, Copy)]
pub union Union0 {
	reserved: VREvent_Reserved_t,
	controller: VREvent_Controller_t,
	mouse: VREvent_Mouse_t,
	scroll: VREvent_Scroll_t,
	process: VREvent_Process_t,
	notification: VREvent_Notification_t,
	overlay: VREvent_Overlay_t,
	status: VREvent_Status_t,
	keyboard: VREvent_Keyboard_t,
	ipd: VREvent_Ipd_t,
	chaperone: VREvent_Chaperone_t,
	performanceTest: VREvent_PerformanceTest_t,
	touchPadMove: VREvent_TouchPadMove_t,
	seatedZeroPoseReset: VREvent_SeatedZeroPoseReset_t,
	screenshot: VREvent_Screenshot_t,
	screenshotProgress: VREvent_ScreenshotProgress_t,
	applicationLaunch: VREvent_ApplicationLaunch_t,
	cameraSurface: VREvent_EditingCameraSurface_t,
	messageOverlay: VREvent_MessageOverlay_t,
	property: VREvent_Property_t,
	hapticVibration: VREvent_HapticVibration_t,
	webConsole: VREvent_WebConsole_t,
	inputBinding: VREvent_InputBindingLoad_t,
	actionManifest: VREvent_InputActionManifestLoad_t,
	spatialAnchor: VREvent_SpatialAnchor_t,
	progressUpdate: VREvent_ProgressUpdate_t,
	showUi: VREvent_ShowUI_t,
	showDevTools: VREvent_ShowDevTools_t,
	hdcpError: VREvent_HDCPError_t,
}
pub type PropertyTypeTag_t = u32;
pub type vrshared_uint64_t = u64;
pub type vrshared_double = f64;
pub type SpatialAnchorHandle_t = u32;
pub type glSharedTextureHandle_t = *mut c_void;
pub type glInt_t = i32;
pub type glUInt_t = u32;
pub type SharedTextureHandle_t = u64;
pub type DriverId_t = u32;
pub type TrackedDeviceIndex_t = u32;
pub type WebConsoleHandle_t = u64;
pub type PropertyContainerHandle_t = u64;
pub type DriverHandle_t = PropertyContainerHandle_t;
pub type VRActionHandle_t = u64;
pub type VRActionSetHandle_t = u64;
pub type VRInputValueHandle_t = u64;
pub type VREvent_Data_t = Union0;
pub type VRComponentProperties = u32;
pub type VRControllerState_t = VRControllerState001_t;
pub type VROverlayHandle_t = u64;
pub type BoneIndex_t = i32;
pub type TrackedCameraHandle_t = u64;
pub type ScreenshotHandle_t = u32;
pub type VRInputComponentHandle_t = u64;
pub type IOBufferHandle_t = u64;
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum EVREye {
	Eye_Left = 0,
	Eye_Right = 1,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum ETextureType {
	TextureType_Invalid = -1,
	TextureType_DirectX = 0,
	TextureType_OpenGL = 1,
	TextureType_Vulkan = 2,
	TextureType_IOSurface = 3,
	TextureType_DirectX12 = 4,
	TextureType_DXGISharedHandle = 5,
	TextureType_Metal = 6,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum EColorSpace {
	ColorSpace_Auto = 0,
	ColorSpace_Gamma = 1,
	ColorSpace_Linear = 2,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum ETrackingResult {
	TrackingResult_Uninitialized = 1,
	TrackingResult_Calibrating_InProgress = 100,
	TrackingResult_Calibrating_OutOfRange = 101,
	TrackingResult_Running_OK = 200,
	TrackingResult_Running_OutOfRange = 201,
	TrackingResult_Fallback_RotationOnly = 300,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum ETrackedDeviceClass {
	TrackedDeviceClass_Invalid = 0,
	TrackedDeviceClass_HMD = 1,
	TrackedDeviceClass_Controller = 2,
	TrackedDeviceClass_GenericTracker = 3,
	TrackedDeviceClass_TrackingReference = 4,
	TrackedDeviceClass_DisplayRedirect = 5,
	TrackedDeviceClass_Max,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum ETrackedControllerRole {
	TrackedControllerRole_Invalid = 0,
	TrackedControllerRole_LeftHand = 1,
	TrackedControllerRole_RightHand = 2,
	TrackedControllerRole_OptOut = 3,
	TrackedControllerRole_Treadmill = 4,
	TrackedControllerRole_Stylus = 5,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum ETrackingUniverseOrigin {
	TrackingUniverseSeated = 0,
	TrackingUniverseStanding = 1,
	TrackingUniverseRawAndUncalibrated = 2,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum EAdditionalRadioFeatures {
	AdditionalRadioFeatures_None = 0,
	AdditionalRadioFeatures_HTCLinkBox = 1,
	AdditionalRadioFeatures_InternalDongle = 2,
	AdditionalRadioFeatures_ExternalDongle = 4,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum ETrackedDeviceProperty {
	Prop_Invalid = 0,
	Prop_TrackingSystemName_String = 1000,
	Prop_ModelNumber_String = 1001,
	Prop_SerialNumber_String = 1002,
	Prop_RenderModelName_String = 1003,
	Prop_WillDriftInYaw_Bool = 1004,
	Prop_ManufacturerName_String = 1005,
	Prop_TrackingFirmwareVersion_String = 1006,
	Prop_HardwareRevision_String = 1007,
	Prop_AllWirelessDongleDescriptions_String = 1008,
	Prop_ConnectedWirelessDongle_String = 1009,
	Prop_DeviceIsWireless_Bool = 1010,
	Prop_DeviceIsCharging_Bool = 1011,
	Prop_DeviceBatteryPercentage_Float = 1012,
	Prop_StatusDisplayTransform_Matrix34 = 1013,
	Prop_Firmware_UpdateAvailable_Bool = 1014,
	Prop_Firmware_ManualUpdate_Bool = 1015,
	Prop_Firmware_ManualUpdateURL_String = 1016,
	Prop_HardwareRevision_Uint64 = 1017,
	Prop_FirmwareVersion_Uint64 = 1018,
	Prop_FPGAVersion_Uint64 = 1019,
	Prop_VRCVersion_Uint64 = 1020,
	Prop_RadioVersion_Uint64 = 1021,
	Prop_DongleVersion_Uint64 = 1022,
	Prop_BlockServerShutdown_Bool = 1023,
	Prop_CanUnifyCoordinateSystemWithHmd_Bool = 1024,
	Prop_ContainsProximitySensor_Bool = 1025,
	Prop_DeviceProvidesBatteryStatus_Bool = 1026,
	Prop_DeviceCanPowerOff_Bool = 1027,
	Prop_Firmware_ProgrammingTarget_String = 1028,
	Prop_DeviceClass_Int32 = 1029,
	Prop_HasCamera_Bool = 1030,
	Prop_DriverVersion_String = 1031,
	Prop_Firmware_ForceUpdateRequired_Bool = 1032,
	Prop_ViveSystemButtonFixRequired_Bool = 1033,
	Prop_ParentDriver_Uint64 = 1034,
	Prop_ResourceRoot_String = 1035,
	Prop_RegisteredDeviceType_String = 1036,
	Prop_InputProfilePath_String = 1037,
	Prop_NeverTracked_Bool = 1038,
	Prop_NumCameras_Int32 = 1039,
	Prop_CameraFrameLayout_Int32 = 1040,
	Prop_CameraStreamFormat_Int32 = 1041,
	Prop_AdditionalDeviceSettingsPath_String = 1042,
	Prop_Identifiable_Bool = 1043,
	Prop_BootloaderVersion_Uint64 = 1044,
	Prop_AdditionalSystemReportData_String = 1045,
	Prop_CompositeFirmwareVersion_String = 1046,
	Prop_Firmware_RemindUpdate_Bool = 1047,
	Prop_PeripheralApplicationVersion_Uint64 = 1048,
	Prop_ManufacturerSerialNumber_String = 1049,
	Prop_ComputedSerialNumber_String = 1050,
	Prop_EstimatedDeviceFirstUseTime_Int32 = 1051,
	Prop_DevicePowerUsage_Float = 1052,
	Prop_IgnoreMotionForStandby_Bool = 1053,
	Prop_ReportsTimeSinceVSync_Bool = 2000,
	Prop_SecondsFromVsyncToPhotons_Float = 2001,
	Prop_DisplayFrequency_Float = 2002,
	Prop_UserIpdMeters_Float = 2003,
	Prop_CurrentUniverseId_Uint64 = 2004,
	Prop_PreviousUniverseId_Uint64 = 2005,
	Prop_DisplayFirmwareVersion_Uint64 = 2006,
	Prop_IsOnDesktop_Bool = 2007,
	Prop_DisplayMCType_Int32 = 2008,
	Prop_DisplayMCOffset_Float = 2009,
	Prop_DisplayMCScale_Float = 2010,
	Prop_EdidVendorID_Int32 = 2011,
	Prop_DisplayMCImageLeft_String = 2012,
	Prop_DisplayMCImageRight_String = 2013,
	Prop_DisplayGCBlackClamp_Float = 2014,
	Prop_EdidProductID_Int32 = 2015,
	Prop_CameraToHeadTransform_Matrix34 = 2016,
	Prop_DisplayGCType_Int32 = 2017,
	Prop_DisplayGCOffset_Float = 2018,
	Prop_DisplayGCScale_Float = 2019,
	Prop_DisplayGCPrescale_Float = 2020,
	Prop_DisplayGCImage_String = 2021,
	Prop_LensCenterLeftU_Float = 2022,
	Prop_LensCenterLeftV_Float = 2023,
	Prop_LensCenterRightU_Float = 2024,
	Prop_LensCenterRightV_Float = 2025,
	Prop_UserHeadToEyeDepthMeters_Float = 2026,
	Prop_CameraFirmwareVersion_Uint64 = 2027,
	Prop_CameraFirmwareDescription_String = 2028,
	Prop_DisplayFPGAVersion_Uint64 = 2029,
	Prop_DisplayBootloaderVersion_Uint64 = 2030,
	Prop_DisplayHardwareVersion_Uint64 = 2031,
	Prop_AudioFirmwareVersion_Uint64 = 2032,
	Prop_CameraCompatibilityMode_Int32 = 2033,
	Prop_ScreenshotHorizontalFieldOfViewDegrees_Float = 2034,
	Prop_ScreenshotVerticalFieldOfViewDegrees_Float = 2035,
	Prop_DisplaySuppressed_Bool = 2036,
	Prop_DisplayAllowNightMode_Bool = 2037,
	Prop_DisplayMCImageWidth_Int32 = 2038,
	Prop_DisplayMCImageHeight_Int32 = 2039,
	Prop_DisplayMCImageNumChannels_Int32 = 2040,
	Prop_DisplayMCImageData_Binary = 2041,
	Prop_SecondsFromPhotonsToVblank_Float = 2042,
	Prop_DriverDirectModeSendsVsyncEvents_Bool = 2043,
	Prop_DisplayDebugMode_Bool = 2044,
	Prop_GraphicsAdapterLuid_Uint64 = 2045,
	Prop_DriverProvidedChaperonePath_String = 2048,
	Prop_ExpectedTrackingReferenceCount_Int32 = 2049,
	Prop_ExpectedControllerCount_Int32 = 2050,
	Prop_NamedIconPathControllerLeftDeviceOff_String = 2051,
	Prop_NamedIconPathControllerRightDeviceOff_String = 2052,
	Prop_NamedIconPathTrackingReferenceDeviceOff_String = 2053,
	Prop_DoNotApplyPrediction_Bool = 2054,
	Prop_CameraToHeadTransforms_Matrix34_Array = 2055,
	Prop_DistortionMeshResolution_Int32 = 2056,
	Prop_DriverIsDrawingControllers_Bool = 2057,
	Prop_DriverRequestsApplicationPause_Bool = 2058,
	Prop_DriverRequestsReducedRendering_Bool = 2059,
	Prop_MinimumIpdStepMeters_Float = 2060,
	Prop_AudioBridgeFirmwareVersion_Uint64 = 2061,
	Prop_ImageBridgeFirmwareVersion_Uint64 = 2062,
	Prop_ImuToHeadTransform_Matrix34 = 2063,
	Prop_ImuFactoryGyroBias_Vector3 = 2064,
	Prop_ImuFactoryGyroScale_Vector3 = 2065,
	Prop_ImuFactoryAccelerometerBias_Vector3 = 2066,
	Prop_ImuFactoryAccelerometerScale_Vector3 = 2067,
	Prop_ConfigurationIncludesLighthouse20Features_Bool = 2069,
	Prop_AdditionalRadioFeatures_Uint64 = 2070,
	Prop_CameraWhiteBalance_Vector4_Array = 2071,
	Prop_CameraDistortionFunction_Int32_Array = 2072,
	Prop_CameraDistortionCoefficients_Float_Array = 2073,
	Prop_ExpectedControllerType_String = 2074,
	Prop_HmdTrackingStyle_Int32 = 2075,
	Prop_DriverProvidedChaperoneVisibility_Bool = 2076,
	Prop_HmdColumnCorrectionSettingPrefix_String = 2077,
	Prop_CameraSupportsCompatibilityModes_Bool = 2078,
	Prop_SupportsRoomViewDepthProjection_Bool = 2079,
	Prop_DisplayAvailableFrameRates_Float_Array = 2080,
	Prop_DisplaySupportsMultipleFramerates_Bool = 2081,
	Prop_DisplayColorMultLeft_Vector3 = 2082,
	Prop_DisplayColorMultRight_Vector3 = 2083,
	Prop_DisplaySupportsRuntimeFramerateChange_Bool = 2084,
	Prop_DisplaySupportsAnalogGain_Bool = 2085,
	Prop_DisplayMinAnalogGain_Float = 2086,
	Prop_DisplayMaxAnalogGain_Float = 2087,
	Prop_CameraExposureTime_Float = 2088,
	Prop_CameraGlobalGain_Float = 2089,
	Prop_DashboardScale_Float = 2091,
	Prop_PeerButtonInfo_String = 2092,
	Prop_Hmd_SupportsHDR10_Bool = 2093,
	Prop_Hmd_EnableParallelRenderCameras_Bool = 2094,
	Prop_DriverProvidedChaperoneJson_String = 2095,
	Prop_IpdUIRangeMinMeters_Float = 2100,
	Prop_IpdUIRangeMaxMeters_Float = 2101,
	Prop_Hmd_SupportsHDCP14LegacyCompat_Bool = 2102,
	Prop_Hmd_SupportsMicMonitoring_Bool = 2103,
	Prop_Hmd_SupportsDisplayPortTrainingMode_Bool = 2104,
	Prop_Hmd_SupportsRoomViewDirect_Bool = 2105,
	Prop_Hmd_SupportsAppThrottling_Bool = 2106,
	Prop_Hmd_SupportsGpuBusMonitoring_Bool = 2107,
	Prop_DSCVersion_Int32 = 2110,
	Prop_DSCSliceCount_Int32 = 2111,
	Prop_DSCBPPx16_Int32 = 2112,
	Prop_DriverRequestedMuraCorrectionMode_Int32 = 2200,
	Prop_DriverRequestedMuraFeather_InnerLeft_Int32 = 2201,
	Prop_DriverRequestedMuraFeather_InnerRight_Int32 = 2202,
	Prop_DriverRequestedMuraFeather_InnerTop_Int32 = 2203,
	Prop_DriverRequestedMuraFeather_InnerBottom_Int32 = 2204,
	Prop_DriverRequestedMuraFeather_OuterLeft_Int32 = 2205,
	Prop_DriverRequestedMuraFeather_OuterRight_Int32 = 2206,
	Prop_DriverRequestedMuraFeather_OuterTop_Int32 = 2207,
	Prop_DriverRequestedMuraFeather_OuterBottom_Int32 = 2208,
	Prop_Audio_DefaultPlaybackDeviceId_String = 2300,
	Prop_Audio_DefaultRecordingDeviceId_String = 2301,
	Prop_Audio_DefaultPlaybackDeviceVolume_Float = 2302,
	Prop_Audio_SupportsDualSpeakerAndJackOutput_Bool = 2303,
	Prop_AttachedDeviceId_String = 3000,
	Prop_SupportedButtons_Uint64 = 3001,
	Prop_Axis0Type_Int32 = 3002,
	Prop_Axis1Type_Int32 = 3003,
	Prop_Axis2Type_Int32 = 3004,
	Prop_Axis3Type_Int32 = 3005,
	Prop_Axis4Type_Int32 = 3006,
	Prop_ControllerRoleHint_Int32 = 3007,
	Prop_FieldOfViewLeftDegrees_Float = 4000,
	Prop_FieldOfViewRightDegrees_Float = 4001,
	Prop_FieldOfViewTopDegrees_Float = 4002,
	Prop_FieldOfViewBottomDegrees_Float = 4003,
	Prop_TrackingRangeMinimumMeters_Float = 4004,
	Prop_TrackingRangeMaximumMeters_Float = 4005,
	Prop_ModeLabel_String = 4006,
	Prop_CanWirelessIdentify_Bool = 4007,
	Prop_Nonce_Int32 = 4008,
	Prop_IconPathName_String = 5000,
	Prop_NamedIconPathDeviceOff_String = 5001,
	Prop_NamedIconPathDeviceSearching_String = 5002,
	Prop_NamedIconPathDeviceSearchingAlert_String = 5003,
	Prop_NamedIconPathDeviceReady_String = 5004,
	Prop_NamedIconPathDeviceReadyAlert_String = 5005,
	Prop_NamedIconPathDeviceNotReady_String = 5006,
	Prop_NamedIconPathDeviceStandby_String = 5007,
	Prop_NamedIconPathDeviceAlertLow_String = 5008,
	Prop_NamedIconPathDeviceStandbyAlert_String = 5009,
	Prop_DisplayHiddenArea_Binary_Start = 5100,
	Prop_DisplayHiddenArea_Binary_End = 5150,
	Prop_ParentContainer = 5151,
	Prop_OverrideContainer_Uint64 = 5152,
	Prop_UserConfigPath_String = 6000,
	Prop_InstallPath_String = 6001,
	Prop_HasDisplayComponent_Bool = 6002,
	Prop_HasControllerComponent_Bool = 6003,
	Prop_HasCameraComponent_Bool = 6004,
	Prop_HasDriverDirectModeComponent_Bool = 6005,
	Prop_HasVirtualDisplayComponent_Bool = 6006,
	Prop_HasSpatialAnchorsSupport_Bool = 6007,
	Prop_ControllerType_String = 7000,
	Prop_ControllerHandSelectionPriority_Int32 = 7002,
	Prop_VendorSpecific_Reserved_Start = 10000,
	Prop_VendorSpecific_Reserved_End = 10999,
	Prop_TrackedDeviceProperty_Max = 1000000,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum ETrackedPropertyError {
	TrackedProp_Success = 0,
	TrackedProp_WrongDataType = 1,
	TrackedProp_WrongDeviceClass = 2,
	TrackedProp_BufferTooSmall = 3,
	TrackedProp_UnknownProperty = 4,
	TrackedProp_InvalidDevice = 5,
	TrackedProp_CouldNotContactServer = 6,
	TrackedProp_ValueNotProvidedByDevice = 7,
	TrackedProp_StringExceedsMaximumLength = 8,
	TrackedProp_NotYetAvailable = 9,
	TrackedProp_PermissionDenied = 10,
	TrackedProp_InvalidOperation = 11,
	TrackedProp_CannotWriteToWildcards = 12,
	TrackedProp_IPCReadFailure = 13,
	TrackedProp_OutOfMemory = 14,
	TrackedProp_InvalidContainer = 15,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum EHmdTrackingStyle {
	HmdTrackingStyle_Unknown = 0,
	HmdTrackingStyle_Lighthouse = 1,
	HmdTrackingStyle_OutsideInCameras = 2,
	HmdTrackingStyle_InsideOutCameras = 3,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum EVRSubmitFlags {
	Submit_Default = 0,
	Submit_LensDistortionAlreadyApplied = 1,
	Submit_GlRenderBuffer = 2,
	Submit_Reserved = 4,
	Submit_TextureWithPose = 8,
	Submit_TextureWithDepth = 16,
	Submit_FrameDiscontinuty = 32,
	Submit_VulkanTextureWithArrayData = 64,
	Submit_GlArrayTexture = 128,
	Submit_Reserved2 = 32768,
	Submit_Reserved3 = 65536,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum EVRState {
	VRState_Undefined = -1,
	VRState_Off = 0,
	VRState_Searching = 1,
	VRState_Searching_Alert = 2,
	VRState_Ready = 3,
	VRState_Ready_Alert = 4,
	VRState_NotReady = 5,
	VRState_Standby = 6,
	VRState_Ready_Alert_Low = 7,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum EVREventType {
	VREvent_None = 0,
	VREvent_TrackedDeviceActivated = 100,
	VREvent_TrackedDeviceDeactivated = 101,
	VREvent_TrackedDeviceUpdated = 102,
	VREvent_TrackedDeviceUserInteractionStarted = 103,
	VREvent_TrackedDeviceUserInteractionEnded = 104,
	VREvent_IpdChanged = 105,
	VREvent_EnterStandbyMode = 106,
	VREvent_LeaveStandbyMode = 107,
	VREvent_TrackedDeviceRoleChanged = 108,
	VREvent_WatchdogWakeUpRequested = 109,
	VREvent_LensDistortionChanged = 110,
	VREvent_PropertyChanged = 111,
	VREvent_WirelessDisconnect = 112,
	VREvent_WirelessReconnect = 113,
	VREvent_ButtonPress = 200,
	VREvent_ButtonUnpress = 201,
	VREvent_ButtonTouch = 202,
	VREvent_ButtonUntouch = 203,
	VREvent_Modal_Cancel = 257,
	VREvent_MouseMove = 300,
	VREvent_MouseButtonDown = 301,
	VREvent_MouseButtonUp = 302,
	VREvent_FocusEnter = 303,
	VREvent_FocusLeave = 304,
	VREvent_ScrollDiscrete = 305,
	VREvent_TouchPadMove = 306,
	VREvent_OverlayFocusChanged = 307,
	VREvent_ReloadOverlays = 308,
	VREvent_ScrollSmooth = 309,
	VREvent_LockMousePosition = 310,
	VREvent_UnlockMousePosition = 311,
	VREvent_InputFocusCaptured = 400,
	VREvent_InputFocusReleased = 401,
	VREvent_SceneApplicationChanged = 404,
	VREvent_InputFocusChanged = 406,
	VREvent_SceneApplicationUsingWrongGraphicsAdapter = 408,
	VREvent_ActionBindingReloaded = 409,
	VREvent_HideRenderModels = 410,
	VREvent_ShowRenderModels = 411,
	VREvent_SceneApplicationStateChanged = 412,
	VREvent_SceneAppPipeDisconnected = 413,
	VREvent_ConsoleOpened = 420,
	VREvent_ConsoleClosed = 421,
	VREvent_OverlayShown = 500,
	VREvent_OverlayHidden = 501,
	VREvent_DashboardActivated = 502,
	VREvent_DashboardDeactivated = 503,
	VREvent_DashboardRequested = 505,
	VREvent_ResetDashboard = 506,
	VREvent_ImageLoaded = 508,
	VREvent_ShowKeyboard = 509,
	VREvent_HideKeyboard = 510,
	VREvent_OverlayGamepadFocusGained = 511,
	VREvent_OverlayGamepadFocusLost = 512,
	VREvent_OverlaySharedTextureChanged = 513,
	VREvent_ScreenshotTriggered = 516,
	VREvent_ImageFailed = 517,
	VREvent_DashboardOverlayCreated = 518,
	VREvent_SwitchGamepadFocus = 519,
	VREvent_RequestScreenshot = 520,
	VREvent_ScreenshotTaken = 521,
	VREvent_ScreenshotFailed = 522,
	VREvent_SubmitScreenshotToDashboard = 523,
	VREvent_ScreenshotProgressToDashboard = 524,
	VREvent_PrimaryDashboardDeviceChanged = 525,
	VREvent_RoomViewShown = 526,
	VREvent_RoomViewHidden = 527,
	VREvent_ShowUI = 528,
	VREvent_ShowDevTools = 529,
	VREvent_DesktopViewUpdating = 530,
	VREvent_DesktopViewReady = 531,
	VREvent_StartDashboard = 532,
	VREvent_ElevatePrism = 533,
	VREvent_OverlayClosed = 534,
	VREvent_Notification_Shown = 600,
	VREvent_Notification_Hidden = 601,
	VREvent_Notification_BeginInteraction = 602,
	VREvent_Notification_Destroyed = 603,
	VREvent_Quit = 700,
	VREvent_ProcessQuit = 701,
	VREvent_QuitAcknowledged = 703,
	VREvent_DriverRequestedQuit = 704,
	VREvent_RestartRequested = 705,
	VREvent_InvalidateSwapTextureSets = 706,
	VREvent_ChaperoneDataHasChanged = 800,
	VREvent_ChaperoneUniverseHasChanged = 801,
	VREvent_ChaperoneTempDataHasChanged = 802,
	VREvent_ChaperoneSettingsHaveChanged = 803,
	VREvent_SeatedZeroPoseReset = 804,
	VREvent_ChaperoneFlushCache = 805,
	VREvent_ChaperoneRoomSetupStarting = 806,
	VREvent_ChaperoneRoomSetupFinished = 807,
	VREvent_StandingZeroPoseReset = 808,
	VREvent_AudioSettingsHaveChanged = 820,
	VREvent_BackgroundSettingHasChanged = 850,
	VREvent_CameraSettingsHaveChanged = 851,
	VREvent_ReprojectionSettingHasChanged = 852,
	VREvent_ModelSkinSettingsHaveChanged = 853,
	VREvent_EnvironmentSettingsHaveChanged = 854,
	VREvent_PowerSettingsHaveChanged = 855,
	VREvent_EnableHomeAppSettingsHaveChanged = 856,
	VREvent_SteamVRSectionSettingChanged = 857,
	VREvent_LighthouseSectionSettingChanged = 858,
	VREvent_NullSectionSettingChanged = 859,
	VREvent_UserInterfaceSectionSettingChanged = 860,
	VREvent_NotificationsSectionSettingChanged = 861,
	VREvent_KeyboardSectionSettingChanged = 862,
	VREvent_PerfSectionSettingChanged = 863,
	VREvent_DashboardSectionSettingChanged = 864,
	VREvent_WebInterfaceSectionSettingChanged = 865,
	VREvent_TrackersSectionSettingChanged = 866,
	VREvent_LastKnownSectionSettingChanged = 867,
	VREvent_DismissedWarningsSectionSettingChanged = 868,
	VREvent_GpuSpeedSectionSettingChanged = 869,
	VREvent_WindowsMRSectionSettingChanged = 870,
	VREvent_OtherSectionSettingChanged = 871,
	VREvent_StatusUpdate = 900,
	VREvent_WebInterface_InstallDriverCompleted = 950,
	VREvent_MCImageUpdated = 1000,
	VREvent_FirmwareUpdateStarted = 1100,
	VREvent_FirmwareUpdateFinished = 1101,
	VREvent_KeyboardClosed = 1200,
	VREvent_KeyboardCharInput = 1201,
	VREvent_KeyboardDone = 1202,
	VREvent_ApplicationListUpdated = 1303,
	VREvent_ApplicationMimeTypeLoad = 1304,
	VREvent_ProcessConnected = 1306,
	VREvent_ProcessDisconnected = 1307,
	VREvent_Compositor_ChaperoneBoundsShown = 1410,
	VREvent_Compositor_ChaperoneBoundsHidden = 1411,
	VREvent_Compositor_DisplayDisconnected = 1412,
	VREvent_Compositor_DisplayReconnected = 1413,
	VREvent_Compositor_HDCPError = 1414,
	VREvent_Compositor_ApplicationNotResponding = 1415,
	VREvent_Compositor_ApplicationResumed = 1416,
	VREvent_Compositor_OutOfVideoMemory = 1417,
	VREvent_Compositor_DisplayModeNotSupported = 1418,
	VREvent_Compositor_StageOverrideReady = 1419,
	VREvent_Compositor_RequestDisconnectReconnect = 1420,
	VREvent_TrackedCamera_StartVideoStream = 1500,
	VREvent_TrackedCamera_StopVideoStream = 1501,
	VREvent_TrackedCamera_PauseVideoStream = 1502,
	VREvent_TrackedCamera_ResumeVideoStream = 1503,
	VREvent_TrackedCamera_EditingSurface = 1550,
	VREvent_PerformanceTest_EnableCapture = 1600,
	VREvent_PerformanceTest_DisableCapture = 1601,
	VREvent_PerformanceTest_FidelityLevel = 1602,
	VREvent_MessageOverlay_Closed = 1650,
	VREvent_MessageOverlayCloseRequested = 1651,
	VREvent_Input_HapticVibration = 1700,
	VREvent_Input_BindingLoadFailed = 1701,
	VREvent_Input_BindingLoadSuccessful = 1702,
	VREvent_Input_ActionManifestReloaded = 1703,
	VREvent_Input_ActionManifestLoadFailed = 1704,
	VREvent_Input_ProgressUpdate = 1705,
	VREvent_Input_TrackerActivated = 1706,
	VREvent_Input_BindingsUpdated = 1707,
	VREvent_Input_BindingSubscriptionChanged = 1708,
	VREvent_SpatialAnchors_PoseUpdated = 1800,
	VREvent_SpatialAnchors_DescriptorUpdated = 1801,
	VREvent_SpatialAnchors_RequestPoseUpdate = 1802,
	VREvent_SpatialAnchors_RequestDescriptorUpdate = 1803,
	VREvent_SystemReport_Started = 1900,
	VREvent_Monitor_ShowHeadsetView = 2000,
	VREvent_Monitor_HideHeadsetView = 2001,
	VREvent_VendorSpecific_Reserved_Start = 10000,
	VREvent_VendorSpecific_Reserved_End = 19999,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum EDeviceActivityLevel {
	k_EDeviceActivityLevel_Unknown = -1,
	k_EDeviceActivityLevel_Idle = 0,
	k_EDeviceActivityLevel_UserInteraction = 1,
	k_EDeviceActivityLevel_UserInteraction_Timeout = 2,
	k_EDeviceActivityLevel_Standby = 3,
	k_EDeviceActivityLevel_Idle_Timeout = 4,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum EVRButtonId {
	k_EButton_System = 0,
	k_EButton_ApplicationMenu = 1,
	k_EButton_Grip = 2,
	k_EButton_DPad_Left = 3,
	k_EButton_DPad_Up = 4,
	k_EButton_DPad_Right = 5,
	k_EButton_DPad_Down = 6,
	k_EButton_A = 7,
	k_EButton_ProximitySensor = 31,
	k_EButton_Axis0 = 32,
	k_EButton_Axis1 = 33,
	k_EButton_Axis2 = 34,
	k_EButton_Axis3 = 35,
	k_EButton_Axis4 = 36,
	k_EButton_Reserved0 = 50,
	k_EButton_Reserved1 = 51,
	k_EButton_Max = 64,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum EVRMouseButton {
	VRMouseButton_Left = 1,
	VRMouseButton_Right = 2,
	VRMouseButton_Middle = 4,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum EShowUIType {
	ShowUI_ControllerBinding = 0,
	ShowUI_ManageTrackers = 1,
	ShowUI_Pairing = 3,
	ShowUI_Settings = 4,
	ShowUI_DebugCommands = 5,
	ShowUI_FullControllerBinding = 6,
	ShowUI_ManageDrivers = 7,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum EHDCPError {
	HDCPError_None = 0,
	HDCPError_LinkLost = 1,
	HDCPError_Tampered = 2,
	HDCPError_DeviceRevoked = 3,
	HDCPError_Unknown = 4,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum EVRComponentProperty {
	VRComponentProperty_IsStatic = 1,
	VRComponentProperty_IsVisible = 2,
	VRComponentProperty_IsTouched = 4,
	VRComponentProperty_IsPressed = 8,
	VRComponentProperty_IsScrolled = 16,
	VRComponentProperty_IsHighlighted = 32,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum EVRInputError {
	VRInputError_None = 0,
	VRInputError_NameNotFound = 1,
	VRInputError_WrongType = 2,
	VRInputError_InvalidHandle = 3,
	VRInputError_InvalidParam = 4,
	VRInputError_NoSteam = 5,
	VRInputError_MaxCapacityReached = 6,
	VRInputError_IPCError = 7,
	VRInputError_NoActiveActionSet = 8,
	VRInputError_InvalidDevice = 9,
	VRInputError_InvalidSkeleton = 10,
	VRInputError_InvalidBoneCount = 11,
	VRInputError_InvalidCompressedData = 12,
	VRInputError_NoData = 13,
	VRInputError_BufferTooSmall = 14,
	VRInputError_MismatchedActionManifest = 15,
	VRInputError_MissingSkeletonData = 16,
	VRInputError_InvalidBoneIndex = 17,
	VRInputError_InvalidPriority = 18,
	VRInputError_PermissionDenied = 19,
	VRInputError_InvalidRenderModel = 20,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum EVRSpatialAnchorError {
	VRSpatialAnchorError_Success = 0,
	VRSpatialAnchorError_Internal = 1,
	VRSpatialAnchorError_UnknownHandle = 2,
	VRSpatialAnchorError_ArrayTooSmall = 3,
	VRSpatialAnchorError_InvalidDescriptorChar = 4,
	VRSpatialAnchorError_NotYetAvailable = 5,
	VRSpatialAnchorError_NotAvailableInThisUniverse = 6,
	VRSpatialAnchorError_PermanentlyUnavailable = 7,
	VRSpatialAnchorError_WrongDriver = 8,
	VRSpatialAnchorError_DescriptorTooLong = 9,
	VRSpatialAnchorError_Unknown = 10,
	VRSpatialAnchorError_NoRoomCalibration = 11,
	VRSpatialAnchorError_InvalidArgument = 12,
	VRSpatialAnchorError_UnknownDriver = 13,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum EHiddenAreaMeshType {
	k_eHiddenAreaMesh_Standard = 0,
	k_eHiddenAreaMesh_Inverse = 1,
	k_eHiddenAreaMesh_LineLoop = 2,
	k_eHiddenAreaMesh_Max = 3,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum EVRControllerAxisType {
	k_eControllerAxis_None = 0,
	k_eControllerAxis_TrackPad = 1,
	k_eControllerAxis_Joystick = 2,
	k_eControllerAxis_Trigger = 3,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum EVRControllerEventOutputType {
	ControllerEventOutput_OSEvents = 0,
	ControllerEventOutput_VREvents = 1,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum ECollisionBoundsStyle {
	COLLISION_BOUNDS_STYLE_BEGINNER = 0,
	COLLISION_BOUNDS_STYLE_INTERMEDIATE,
	COLLISION_BOUNDS_STYLE_SQUARES,
	COLLISION_BOUNDS_STYLE_ADVANCED,
	COLLISION_BOUNDS_STYLE_NONE,
	COLLISION_BOUNDS_STYLE_COUNT,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum EVROverlayError {
	VROverlayError_None = 0,
	VROverlayError_UnknownOverlay = 10,
	VROverlayError_InvalidHandle = 11,
	VROverlayError_PermissionDenied = 12,
	VROverlayError_OverlayLimitExceeded = 13,
	VROverlayError_WrongVisibilityType = 14,
	VROverlayError_KeyTooLong = 15,
	VROverlayError_NameTooLong = 16,
	VROverlayError_KeyInUse = 17,
	VROverlayError_WrongTransformType = 18,
	VROverlayError_InvalidTrackedDevice = 19,
	VROverlayError_InvalidParameter = 20,
	VROverlayError_ThumbnailCantBeDestroyed = 21,
	VROverlayError_ArrayTooSmall = 22,
	VROverlayError_RequestFailed = 23,
	VROverlayError_InvalidTexture = 24,
	VROverlayError_UnableToLoadFile = 25,
	VROverlayError_KeyboardAlreadyInUse = 26,
	VROverlayError_NoNeighbor = 27,
	VROverlayError_TooManyMaskPrimitives = 29,
	VROverlayError_BadMaskPrimitive = 30,
	VROverlayError_TextureAlreadyLocked = 31,
	VROverlayError_TextureLockCapacityReached = 32,
	VROverlayError_TextureNotLocked = 33,
	VROverlayError_TimedOut = 34,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum EVRApplicationType {
	VRApplication_Other = 0,
	VRApplication_Scene = 1,
	VRApplication_Overlay = 2,
	VRApplication_Background = 3,
	VRApplication_Utility = 4,
	VRApplication_VRMonitor = 5,
	VRApplication_SteamWatchdog = 6,
	VRApplication_Bootstrapper = 7,
	VRApplication_WebHelper = 8,
	VRApplication_OpenXRInstance = 9,
	VRApplication_OpenXRScene = 10,
	VRApplication_OpenXROverlay = 11,
	VRApplication_Prism = 12,
	VRApplication_RoomView = 13,
	VRApplication_Max,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum EVRFirmwareError {
	VRFirmwareError_None = 0,
	VRFirmwareError_Success = 1,
	VRFirmwareError_Fail = 2,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum EVRNotificationError {
	VRNotificationError_OK = 0,
	VRNotificationError_InvalidNotificationId = 100,
	VRNotificationError_NotificationQueueFull = 101,
	VRNotificationError_InvalidOverlayHandle = 102,
	VRNotificationError_SystemWithUserValueAlreadyExists = 103,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum EVRSkeletalMotionRange {
	VRSkeletalMotionRange_WithController = 0,
	VRSkeletalMotionRange_WithoutController = 1,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum EVRSkeletalTrackingLevel {
	VRSkeletalTracking_Estimated = 0,
	VRSkeletalTracking_Partial = 1,
	VRSkeletalTracking_Full = 2,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum EVRInitError {
	VRInitError_None = 0,
	VRInitError_Unknown = 1,
	VRInitError_Init_InstallationNotFound = 100,
	VRInitError_Init_InstallationCorrupt = 101,
	VRInitError_Init_VRClientDLLNotFound = 102,
	VRInitError_Init_FileNotFound = 103,
	VRInitError_Init_FactoryNotFound = 104,
	VRInitError_Init_InterfaceNotFound = 105,
	VRInitError_Init_InvalidInterface = 106,
	VRInitError_Init_UserConfigDirectoryInvalid = 107,
	VRInitError_Init_HmdNotFound = 108,
	VRInitError_Init_NotInitialized = 109,
	VRInitError_Init_PathRegistryNotFound = 110,
	VRInitError_Init_NoConfigPath = 111,
	VRInitError_Init_NoLogPath = 112,
	VRInitError_Init_PathRegistryNotWritable = 113,
	VRInitError_Init_AppInfoInitFailed = 114,
	VRInitError_Init_Retry = 115,
	VRInitError_Init_InitCanceledByUser = 116,
	VRInitError_Init_AnotherAppLaunching = 117,
	VRInitError_Init_SettingsInitFailed = 118,
	VRInitError_Init_ShuttingDown = 119,
	VRInitError_Init_TooManyObjects = 120,
	VRInitError_Init_NoServerForBackgroundApp = 121,
	VRInitError_Init_NotSupportedWithCompositor = 122,
	VRInitError_Init_NotAvailableToUtilityApps = 123,
	VRInitError_Init_Internal = 124,
	VRInitError_Init_HmdDriverIdIsNone = 125,
	VRInitError_Init_HmdNotFoundPresenceFailed = 126,
	VRInitError_Init_VRMonitorNotFound = 127,
	VRInitError_Init_VRMonitorStartupFailed = 128,
	VRInitError_Init_LowPowerWatchdogNotSupported = 129,
	VRInitError_Init_InvalidApplicationType = 130,
	VRInitError_Init_NotAvailableToWatchdogApps = 131,
	VRInitError_Init_WatchdogDisabledInSettings = 132,
	VRInitError_Init_VRDashboardNotFound = 133,
	VRInitError_Init_VRDashboardStartupFailed = 134,
	VRInitError_Init_VRHomeNotFound = 135,
	VRInitError_Init_VRHomeStartupFailed = 136,
	VRInitError_Init_RebootingBusy = 137,
	VRInitError_Init_FirmwareUpdateBusy = 138,
	VRInitError_Init_FirmwareRecoveryBusy = 139,
	VRInitError_Init_USBServiceBusy = 140,
	VRInitError_Init_VRWebHelperStartupFailed = 141,
	VRInitError_Init_TrackerManagerInitFailed = 142,
	VRInitError_Init_AlreadyRunning = 143,
	VRInitError_Init_FailedForVrMonitor = 144,
	VRInitError_Init_PropertyManagerInitFailed = 145,
	VRInitError_Init_WebServerFailed = 146,
	VRInitError_Init_IllegalTypeTransition = 147,
	VRInitError_Init_MismatchedRuntimes = 148,
	VRInitError_Init_InvalidProcessId = 149,
	VRInitError_Init_VRServiceStartupFailed = 150,
	VRInitError_Init_PrismNeedsNewDrivers = 151,
	VRInitError_Init_PrismStartupTimedOut = 152,
	VRInitError_Init_CouldNotStartPrism = 153,
	VRInitError_Init_PrismClientInitFailed = 154,
	VRInitError_Init_PrismClientStartFailed = 155,
	VRInitError_Init_PrismExitedUnexpectedly = 156,
	VRInitError_Init_BadLuid = 157,
	VRInitError_Init_NoServerForAppContainer = 158,
	VRInitError_Init_DuplicateBootstrapper = 159,
	VRInitError_Init_VRDashboardServicePending = 160,
	VRInitError_Init_VRDashboardServiceTimeout = 161,
	VRInitError_Init_VRDashboardServiceStopped = 162,
	VRInitError_Init_VRDashboardAlreadyStarted = 163,
	VRInitError_Init_VRDashboardCopyFailed = 164,
	VRInitError_Init_VRDashboardTokenFailure = 165,
	VRInitError_Init_VRDashboardEnvironmentFailure = 166,
	VRInitError_Init_VRDashboardPathFailure = 167,
	VRInitError_Driver_Failed = 200,
	VRInitError_Driver_Unknown = 201,
	VRInitError_Driver_HmdUnknown = 202,
	VRInitError_Driver_NotLoaded = 203,
	VRInitError_Driver_RuntimeOutOfDate = 204,
	VRInitError_Driver_HmdInUse = 205,
	VRInitError_Driver_NotCalibrated = 206,
	VRInitError_Driver_CalibrationInvalid = 207,
	VRInitError_Driver_HmdDisplayNotFound = 208,
	VRInitError_Driver_TrackedDeviceInterfaceUnknown = 209,
	VRInitError_Driver_HmdDriverIdOutOfBounds = 211,
	VRInitError_Driver_HmdDisplayMirrored = 212,
	VRInitError_Driver_HmdDisplayNotFoundLaptop = 213,
	VRInitError_Driver_PeerDriverNotInstalled = 214,
	VRInitError_Driver_WirelessHmdNotConnected = 215,
	VRInitError_IPC_ServerInitFailed = 300,
	VRInitError_IPC_ConnectFailed = 301,
	VRInitError_IPC_SharedStateInitFailed = 302,
	VRInitError_IPC_CompositorInitFailed = 303,
	VRInitError_IPC_MutexInitFailed = 304,
	VRInitError_IPC_Failed = 305,
	VRInitError_IPC_CompositorConnectFailed = 306,
	VRInitError_IPC_CompositorInvalidConnectResponse = 307,
	VRInitError_IPC_ConnectFailedAfterMultipleAttempts = 308,
	VRInitError_IPC_ConnectFailedAfterTargetExited = 309,
	VRInitError_IPC_NamespaceUnavailable = 310,
	VRInitError_Compositor_Failed = 400,
	VRInitError_Compositor_D3D11HardwareRequired = 401,
	VRInitError_Compositor_FirmwareRequiresUpdate = 402,
	VRInitError_Compositor_OverlayInitFailed = 403,
	VRInitError_Compositor_ScreenshotsInitFailed = 404,
	VRInitError_Compositor_UnableToCreateDevice = 405,
	VRInitError_Compositor_SharedStateIsNull = 406,
	VRInitError_Compositor_NotificationManagerIsNull = 407,
	VRInitError_Compositor_ResourceManagerClientIsNull = 408,
	VRInitError_Compositor_MessageOverlaySharedStateInitFailure = 409,
	VRInitError_Compositor_PropertiesInterfaceIsNull = 410,
	VRInitError_Compositor_CreateFullscreenWindowFailed = 411,
	VRInitError_Compositor_SettingsInterfaceIsNull = 412,
	VRInitError_Compositor_FailedToShowWindow = 413,
	VRInitError_Compositor_DistortInterfaceIsNull = 414,
	VRInitError_Compositor_DisplayFrequencyFailure = 415,
	VRInitError_Compositor_RendererInitializationFailed = 416,
	VRInitError_Compositor_DXGIFactoryInterfaceIsNull = 417,
	VRInitError_Compositor_DXGIFactoryCreateFailed = 418,
	VRInitError_Compositor_DXGIFactoryQueryFailed = 419,
	VRInitError_Compositor_InvalidAdapterDesktop = 420,
	VRInitError_Compositor_InvalidHmdAttachment = 421,
	VRInitError_Compositor_InvalidOutputDesktop = 422,
	VRInitError_Compositor_InvalidDeviceProvided = 423,
	VRInitError_Compositor_D3D11RendererInitializationFailed = 424,
	VRInitError_Compositor_FailedToFindDisplayMode = 425,
	VRInitError_Compositor_FailedToCreateSwapChain = 426,
	VRInitError_Compositor_FailedToGetBackBuffer = 427,
	VRInitError_Compositor_FailedToCreateRenderTarget = 428,
	VRInitError_Compositor_FailedToCreateDXGI2SwapChain = 429,
	VRInitError_Compositor_FailedtoGetDXGI2BackBuffer = 430,
	VRInitError_Compositor_FailedToCreateDXGI2RenderTarget = 431,
	VRInitError_Compositor_FailedToGetDXGIDeviceInterface = 432,
	VRInitError_Compositor_SelectDisplayMode = 433,
	VRInitError_Compositor_FailedToCreateNvAPIRenderTargets = 434,
	VRInitError_Compositor_NvAPISetDisplayMode = 435,
	VRInitError_Compositor_FailedToCreateDirectModeDisplay = 436,
	VRInitError_Compositor_InvalidHmdPropertyContainer = 437,
	VRInitError_Compositor_UpdateDisplayFrequency = 438,
	VRInitError_Compositor_CreateRasterizerState = 439,
	VRInitError_Compositor_CreateWireframeRasterizerState = 440,
	VRInitError_Compositor_CreateSamplerState = 441,
	VRInitError_Compositor_CreateClampToBorderSamplerState = 442,
	VRInitError_Compositor_CreateAnisoSamplerState = 443,
	VRInitError_Compositor_CreateOverlaySamplerState = 444,
	VRInitError_Compositor_CreatePanoramaSamplerState = 445,
	VRInitError_Compositor_CreateFontSamplerState = 446,
	VRInitError_Compositor_CreateNoBlendState = 447,
	VRInitError_Compositor_CreateBlendState = 448,
	VRInitError_Compositor_CreateAlphaBlendState = 449,
	VRInitError_Compositor_CreateBlendStateMaskR = 450,
	VRInitError_Compositor_CreateBlendStateMaskG = 451,
	VRInitError_Compositor_CreateBlendStateMaskB = 452,
	VRInitError_Compositor_CreateDepthStencilState = 453,
	VRInitError_Compositor_CreateDepthStencilStateNoWrite = 454,
	VRInitError_Compositor_CreateDepthStencilStateNoDepth = 455,
	VRInitError_Compositor_CreateFlushTexture = 456,
	VRInitError_Compositor_CreateDistortionSurfaces = 457,
	VRInitError_Compositor_CreateConstantBuffer = 458,
	VRInitError_Compositor_CreateHmdPoseConstantBuffer = 459,
	VRInitError_Compositor_CreateHmdPoseStagingConstantBuffer = 460,
	VRInitError_Compositor_CreateSharedFrameInfoConstantBuffer = 461,
	VRInitError_Compositor_CreateOverlayConstantBuffer = 462,
	VRInitError_Compositor_CreateSceneTextureIndexConstantBuffer = 463,
	VRInitError_Compositor_CreateReadableSceneTextureIndexConstantBuffer = 464,
	VRInitError_Compositor_CreateLayerGraphicsTextureIndexConstantBuffer = 465,
	VRInitError_Compositor_CreateLayerComputeTextureIndexConstantBuffer = 466,
	VRInitError_Compositor_CreateLayerComputeSceneTextureIndexConstantBuffer = 467,
	VRInitError_Compositor_CreateComputeHmdPoseConstantBuffer = 468,
	VRInitError_Compositor_CreateGeomConstantBuffer = 469,
	VRInitError_Compositor_CreatePanelMaskConstantBuffer = 470,
	VRInitError_Compositor_CreatePixelSimUBO = 471,
	VRInitError_Compositor_CreateMSAARenderTextures = 472,
	VRInitError_Compositor_CreateResolveRenderTextures = 473,
	VRInitError_Compositor_CreateComputeResolveRenderTextures = 474,
	VRInitError_Compositor_CreateDriverDirectModeResolveTextures = 475,
	VRInitError_Compositor_OpenDriverDirectModeResolveTextures = 476,
	VRInitError_Compositor_CreateFallbackSyncTexture = 477,
	VRInitError_Compositor_ShareFallbackSyncTexture = 478,
	VRInitError_Compositor_CreateOverlayIndexBuffer = 479,
	VRInitError_Compositor_CreateOverlayVertexBuffer = 480,
	VRInitError_Compositor_CreateTextVertexBuffer = 481,
	VRInitError_Compositor_CreateTextIndexBuffer = 482,
	VRInitError_Compositor_CreateMirrorTextures = 483,
	VRInitError_Compositor_CreateLastFrameRenderTexture = 484,
	VRInitError_Compositor_CreateMirrorOverlay = 485,
	VRInitError_Compositor_FailedToCreateVirtualDisplayBackbuffer = 486,
	VRInitError_Compositor_DisplayModeNotSupported = 487,
	VRInitError_Compositor_CreateOverlayInvalidCall = 488,
	VRInitError_Compositor_CreateOverlayAlreadyInitialized = 489,
	VRInitError_Compositor_FailedToCreateMailbox = 490,
	VRInitError_Compositor_WindowInterfaceIsNull = 491,
	VRInitError_Compositor_SystemLayerCreateInstance = 492,
	VRInitError_Compositor_SystemLayerCreateSession = 493,
	VRInitError_Compositor_CreateInverseDistortUVs = 494,
	VRInitError_Compositor_CreateBackbufferDepth = 495,
	VRInitError_VendorSpecific_UnableToConnectToOculusRuntime = 1000,
	VRInitError_VendorSpecific_WindowsNotInDevMode = 1001,
	VRInitError_VendorSpecific_OculusLinkNotEnabled = 1002,
	VRInitError_VendorSpecific_HmdFound_CantOpenDevice = 1101,
	VRInitError_VendorSpecific_HmdFound_UnableToRequestConfigStart = 1102,
	VRInitError_VendorSpecific_HmdFound_NoStoredConfig = 1103,
	VRInitError_VendorSpecific_HmdFound_ConfigTooBig = 1104,
	VRInitError_VendorSpecific_HmdFound_ConfigTooSmall = 1105,
	VRInitError_VendorSpecific_HmdFound_UnableToInitZLib = 1106,
	VRInitError_VendorSpecific_HmdFound_CantReadFirmwareVersion = 1107,
	VRInitError_VendorSpecific_HmdFound_UnableToSendUserDataStart = 1108,
	VRInitError_VendorSpecific_HmdFound_UnableToGetUserDataStart = 1109,
	VRInitError_VendorSpecific_HmdFound_UnableToGetUserDataNext = 1110,
	VRInitError_VendorSpecific_HmdFound_UserDataAddressRange = 1111,
	VRInitError_VendorSpecific_HmdFound_UserDataError = 1112,
	VRInitError_VendorSpecific_HmdFound_ConfigFailedSanityCheck = 1113,
	VRInitError_VendorSpecific_OculusRuntimeBadInstall = 1114,
	VRInitError_VendorSpecific_HmdFound_UnexpectedConfiguration_1 = 1115,
	VRInitError_Steam_SteamInstallationNotFound = 2000,
	VRInitError_LastError,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum EVRScreenshotType {
	VRScreenshotType_None = 0,
	VRScreenshotType_Mono = 1,
	VRScreenshotType_Stereo = 2,
	VRScreenshotType_Cubemap = 3,
	VRScreenshotType_MonoPanorama = 4,
	VRScreenshotType_StereoPanorama = 5,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum EVRScreenshotPropertyFilenames {
	VRScreenshotPropertyFilenames_Preview = 0,
	VRScreenshotPropertyFilenames_VR = 1,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum EVRTrackedCameraError {
	VRTrackedCameraError_None = 0,
	VRTrackedCameraError_OperationFailed = 100,
	VRTrackedCameraError_InvalidHandle = 101,
	VRTrackedCameraError_InvalidFrameHeaderVersion = 102,
	VRTrackedCameraError_OutOfHandles = 103,
	VRTrackedCameraError_IPCFailure = 104,
	VRTrackedCameraError_NotSupportedForThisDevice = 105,
	VRTrackedCameraError_SharedMemoryFailure = 106,
	VRTrackedCameraError_FrameBufferingFailure = 107,
	VRTrackedCameraError_StreamSetupFailure = 108,
	VRTrackedCameraError_InvalidGLTextureId = 109,
	VRTrackedCameraError_InvalidSharedTextureHandle = 110,
	VRTrackedCameraError_FailedToGetGLTextureId = 111,
	VRTrackedCameraError_SharedTextureFailure = 112,
	VRTrackedCameraError_NoFrameAvailable = 113,
	VRTrackedCameraError_InvalidArgument = 114,
	VRTrackedCameraError_InvalidFrameBufferSize = 115,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum EVRTrackedCameraFrameLayout {
	EVRTrackedCameraFrameLayout_Mono = 1,
	EVRTrackedCameraFrameLayout_Stereo = 2,
	EVRTrackedCameraFrameLayout_VerticalLayout = 16,
	EVRTrackedCameraFrameLayout_HorizontalLayout = 32,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum EVRTrackedCameraFrameType {
	VRTrackedCameraFrameType_Distorted = 0,
	VRTrackedCameraFrameType_Undistorted,
	VRTrackedCameraFrameType_MaximumUndistorted,
	MAX_CAMERA_FRAME_TYPES,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum EVRDistortionFunctionType {
	VRDistortionFunctionType_None,
	VRDistortionFunctionType_FTheta,
	VRDistortionFunctionType_Extended_FTheta,
	MAX_DISTORTION_FUNCTION_TYPES,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum EVSync {
	VSync_None,
	VSync_WaitRender,
	VSync_NoWaitRender,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum EVRMuraCorrectionMode {
	EVRMuraCorrectionMode_Default = 0,
	EVRMuraCorrectionMode_NoCorrection,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum Imu_OffScaleFlags {
	OffScale_AccelX = 1,
	OffScale_AccelY = 2,
	OffScale_AccelZ = 4,
	OffScale_GyroX = 8,
	OffScale_GyroY = 16,
	OffScale_GyroZ = 32,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum ECameraVideoStreamFormat {
	CVS_FORMAT_UNKNOWN = 0,
	CVS_FORMAT_RAW10 = 1,
	CVS_FORMAT_NV12 = 2,
	CVS_FORMAT_RGB24 = 3,
	CVS_FORMAT_NV12_2 = 4,
	CVS_FORMAT_YUYV16 = 5,
	CVS_FORMAT_BAYER16BG = 6,
	CVS_FORMAT_MJPEG = 7,
	CVS_FORMAT_RGBX32 = 8,
	CVS_MAX_FORMATS,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum ECameraCompatibilityMode {
	CAMERA_COMPAT_MODE_BULK_DEFAULT = 0,
	CAMERA_COMPAT_MODE_BULK_64K_DMA = 1,
	CAMERA_COMPAT_MODE_BULK_16K_DMA = 2,
	CAMERA_COMPAT_MODE_BULK_8K_DMA = 3,
	CAMERA_COMPAT_MODE_ISO_52FPS = 4,
	CAMERA_COMPAT_MODE_ISO_50FPS = 5,
	CAMERA_COMPAT_MODE_ISO_48FPS = 6,
	CAMERA_COMPAT_MODE_ISO_46FPS = 7,
	CAMERA_COMPAT_MODE_ISO_44FPS = 8,
	CAMERA_COMPAT_MODE_ISO_42FPS = 9,
	CAMERA_COMPAT_MODE_ISO_40FPS = 10,
	CAMERA_COMPAT_MODE_ISO_35FPS = 11,
	CAMERA_COMPAT_MODE_ISO_30FPS = 12,
	CAMERA_COMPAT_MODE_ISO_15FPS = 13,
	MAX_CAMERA_COMPAT_MODES,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum ECameraRoomViewStyle {
	CAMERA_ROOMVIEW_STYLE_DEFAULT = 0,
	CAMERA_ROOMVIEW_STYLE_EDGE_A = 1,
	CAMERA_ROOMVIEW_STYLE_EDGE_B = 2,
	CAMERA_ROOMVIEW_STYLE_VIDEO_TRANSLUSCENT = 3,
	CAMERA_ROOMVIEW_STYLE_VIDEO_OPAQUE = 4,
	CAMERA_ROOMVIEW_STYLE_COUNT = 5,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum EVRSettingsError {
	VRSettingsError_None = 0,
	VRSettingsError_IPCFailed = 1,
	VRSettingsError_WriteFailed = 2,
	VRSettingsError_ReadFailed = 3,
	VRSettingsError_JsonParseFailed = 4,
	VRSettingsError_UnsetSettingHasNoDefault = 5,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum VRSwapTextureFlag {
	VRSwapTextureFlag_Shared_NTHandle = 1,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum EPropertyWriteType {
	PropertyWrite_Set = 0,
	PropertyWrite_Erase = 1,
	PropertyWrite_SetError = 2,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum EVRScalarType {
	VRScalarType_Absolute = 0,
	VRScalarType_Relative = 1,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum EVRScalarUnits {
	VRScalarUnits_NormalizedOneSided = 0,
	VRScalarUnits_NormalizedTwoSided = 1,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum EIOBufferError {
	IOBuffer_Success = 0,
	IOBuffer_OperationFailed = 100,
	IOBuffer_InvalidHandle = 101,
	IOBuffer_InvalidArgument = 102,
	IOBuffer_PathExists = 103,
	IOBuffer_PathDoesNotExist = 104,
	IOBuffer_Permission = 105,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum EIOBufferMode {
	IOBufferMode_Read = 1,
	IOBufferMode_Write = 2,
	IOBufferMode_Create = 512,
}
pub const k_nSteamVRVersionMajor: u32 = 1;
pub const k_nSteamVRVersionMinor: u32 = 26;
pub const k_nSteamVRVersionBuild: u32 = 7;
pub const k_nDriverNone: u32 = 4294967295;
pub const k_unMaxDriverDebugResponseSize: u32 = 32768;
pub const k_unTrackedDeviceIndex_Hmd: u32 = 0;
pub const k_unMaxTrackedDeviceCount: u32 = 64;
pub const k_unTrackedDeviceIndexOther: u32 = 4294967294;
pub const k_unTrackedDeviceIndexInvalid: u32 = 4294967295;
pub const k_ulInvalidPropertyContainer: PropertyContainerHandle_t = 0;
pub const k_unInvalidPropertyTag: PropertyTypeTag_t = 0;
pub const k_ulInvalidDriverHandle: PropertyContainerHandle_t = 0;
pub const k_unFloatPropertyTag: PropertyTypeTag_t = 1;
pub const k_unInt32PropertyTag: PropertyTypeTag_t = 2;
pub const k_unUint64PropertyTag: PropertyTypeTag_t = 3;
pub const k_unBoolPropertyTag: PropertyTypeTag_t = 4;
pub const k_unStringPropertyTag: PropertyTypeTag_t = 5;
pub const k_unErrorPropertyTag: PropertyTypeTag_t = 6;
pub const k_unDoublePropertyTag: PropertyTypeTag_t = 7;
pub const k_unHmdMatrix34PropertyTag: PropertyTypeTag_t = 20;
pub const k_unHmdMatrix44PropertyTag: PropertyTypeTag_t = 21;
pub const k_unHmdVector3PropertyTag: PropertyTypeTag_t = 22;
pub const k_unHmdVector4PropertyTag: PropertyTypeTag_t = 23;
pub const k_unHmdVector2PropertyTag: PropertyTypeTag_t = 24;
pub const k_unHmdQuadPropertyTag: PropertyTypeTag_t = 25;
pub const k_unHiddenAreaPropertyTag: PropertyTypeTag_t = 30;
pub const k_unPathHandleInfoTag: PropertyTypeTag_t = 31;
pub const k_unActionPropertyTag: PropertyTypeTag_t = 32;
pub const k_unInputValuePropertyTag: PropertyTypeTag_t = 33;
pub const k_unWildcardPropertyTag: PropertyTypeTag_t = 34;
pub const k_unHapticVibrationPropertyTag: PropertyTypeTag_t = 35;
pub const k_unSkeletonPropertyTag: PropertyTypeTag_t = 36;
pub const k_unSpatialAnchorPosePropertyTag: PropertyTypeTag_t = 40;
pub const k_unJsonPropertyTag: PropertyTypeTag_t = 41;
pub const k_unActiveActionSetPropertyTag: PropertyTypeTag_t = 42;
pub const k_unOpenVRInternalReserved_Start: PropertyTypeTag_t = 1000;
pub const k_unOpenVRInternalReserved_End: PropertyTypeTag_t = 10000;
pub const k_unMaxPropertyStringSize: u32 = 32 * 1024;
pub const k_ulInvalidActionHandle: VRActionHandle_t = 0;
pub const k_ulInvalidActionSetHandle: VRActionSetHandle_t = 0;
pub const k_ulInvalidInputValueHandle: VRInputValueHandle_t = 0;
pub const k_unControllerStateAxisCount: u32 = 5;
pub const k_ulOverlayHandleInvalid: VROverlayHandle_t = 0;
pub const k_unInvalidBoneIndex: BoneIndex_t = -1;
pub const k_unMaxDistortionFunctionParameters: u32 = 8;
pub const k_unScreenshotHandleInvalid: u32 = 0;
pub const VRCompositor_ReprojectionReason_Cpu: u32 = 1;
pub const VRCompositor_ReprojectionReason_Gpu: u32 = 2;
pub const VRCompositor_ReprojectionAsync: u32 = 4;
pub const VRCompositor_ReprojectionMotion: u32 = 8;
pub const VRCompositor_PredictionMask: u32 = 240;
pub const VRCompositor_ThrottleMask: u32 = 3840;
pub const VRCompositor_ReprojectionMotion_Enabled: u32 = 256;
pub const VRCompositor_ReprojectionMotion_ForcedOn: u32 = 512;
pub const VRCompositor_ReprojectionMotion_AppThrottled: u32 = 1024;
pub const k_unMaxCameras: u32 = 4;
pub const k_unMaxCameraFrameSharedHandles: u32 = 4;
pub const k_unMaxSettingsKeyLength: u32 = 128;
pub const IVRSettings_Version: *const c_char = real_c_string!("IVRSettings_003");
pub const k_pch_SteamVR_Section: *const c_char = real_c_string!("steamvr");
pub const k_pch_SteamVR_RequireHmd_String: *const c_char = real_c_string!("requireHmd");
pub const k_pch_SteamVR_ForcedDriverKey_String: *const c_char = real_c_string!("forcedDriver");
pub const k_pch_SteamVR_ForcedHmdKey_String: *const c_char = real_c_string!("forcedHmd");
pub const k_pch_SteamVR_DisplayDebug_Bool: *const c_char = real_c_string!("displayDebug");
pub const k_pch_SteamVR_DebugProcessPipe_String: *const c_char = real_c_string!("debugProcessPipe");
pub const k_pch_SteamVR_DisplayDebugX_Int32: *const c_char = real_c_string!("displayDebugX");
pub const k_pch_SteamVR_DisplayDebugY_Int32: *const c_char = real_c_string!("displayDebugY");
pub const k_pch_SteamVR_SendSystemButtonToAllApps_Bool: *const c_char =
	real_c_string!("sendSystemButtonToAllApps");
pub const k_pch_SteamVR_LogLevel_Int32: *const c_char = real_c_string!("loglevel");
pub const k_pch_SteamVR_IPD_Float: *const c_char = real_c_string!("ipd");
pub const k_pch_SteamVR_Background_String: *const c_char = real_c_string!("background");
pub const k_pch_SteamVR_BackgroundUseDomeProjection_Bool: *const c_char =
	real_c_string!("backgroundUseDomeProjection");
pub const k_pch_SteamVR_BackgroundCameraHeight_Float: *const c_char =
	real_c_string!("backgroundCameraHeight");
pub const k_pch_SteamVR_BackgroundDomeRadius_Float: *const c_char =
	real_c_string!("backgroundDomeRadius");
pub const k_pch_SteamVR_GridColor_String: *const c_char = real_c_string!("gridColor");
pub const k_pch_SteamVR_PlayAreaColor_String: *const c_char = real_c_string!("playAreaColor");
pub const k_pch_SteamVR_TrackingLossColor_String: *const c_char =
	real_c_string!("trackingLossColor");
pub const k_pch_SteamVR_ShowStage_Bool: *const c_char = real_c_string!("showStage");
pub const k_pch_SteamVR_DrawTrackingReferences_Bool: *const c_char =
	real_c_string!("drawTrackingReferences");
pub const k_pch_SteamVR_ActivateMultipleDrivers_Bool: *const c_char =
	real_c_string!("activateMultipleDrivers");
pub const k_pch_SteamVR_UsingSpeakers_Bool: *const c_char = real_c_string!("usingSpeakers");
pub const k_pch_SteamVR_SpeakersForwardYawOffsetDegrees_Float: *const c_char =
	real_c_string!("speakersForwardYawOffsetDegrees");
pub const k_pch_SteamVR_BaseStationPowerManagement_Int32: *const c_char =
	real_c_string!("basestationPowerManagement");
pub const k_pch_SteamVR_ShowBaseStationPowerManagementTip_Int32: *const c_char =
	real_c_string!("ShowBaseStationPowerManagementTip");
pub const k_pch_SteamVR_NeverKillProcesses_Bool: *const c_char =
	real_c_string!("neverKillProcesses");
pub const k_pch_SteamVR_SupersampleScale_Float: *const c_char = real_c_string!("supersampleScale");
pub const k_pch_SteamVR_MaxRecommendedResolution_Int32: *const c_char =
	real_c_string!("maxRecommendedResolution");
pub const k_pch_SteamVR_MotionSmoothing_Bool: *const c_char = real_c_string!("motionSmoothing");
pub const k_pch_SteamVR_MotionSmoothingOverride_Int32: *const c_char =
	real_c_string!("motionSmoothingOverride");
pub const k_pch_SteamVR_FramesToThrottle_Int32: *const c_char = real_c_string!("framesToThrottle");
pub const k_pch_SteamVR_AdditionalFramesToPredict_Int32: *const c_char =
	real_c_string!("additionalFramesToPredict");
pub const k_pch_SteamVR_WorldScale_Float: *const c_char = real_c_string!("worldScale");
pub const k_pch_SteamVR_FovScale_Int32: *const c_char = real_c_string!("fovScale");
pub const k_pch_SteamVR_DisableAsyncReprojection_Bool: *const c_char =
	real_c_string!("disableAsync");
pub const k_pch_SteamVR_ForceFadeOnBadTracking_Bool: *const c_char =
	real_c_string!("forceFadeOnBadTracking");
pub const k_pch_SteamVR_DefaultMirrorView_Int32: *const c_char = real_c_string!("mirrorView");
pub const k_pch_SteamVR_ShowLegacyMirrorView_Bool: *const c_char =
	real_c_string!("showLegacyMirrorView");
pub const k_pch_SteamVR_MirrorViewVisibility_Bool: *const c_char = real_c_string!("showMirrorView");
pub const k_pch_SteamVR_MirrorViewDisplayMode_Int32: *const c_char =
	real_c_string!("mirrorViewDisplayMode");
pub const k_pch_SteamVR_MirrorViewEye_Int32: *const c_char = real_c_string!("mirrorViewEye");
pub const k_pch_SteamVR_MirrorViewGeometry_String: *const c_char =
	real_c_string!("mirrorViewGeometry");
pub const k_pch_SteamVR_MirrorViewGeometryMaximized_String: *const c_char =
	real_c_string!("mirrorViewGeometryMaximized");
pub const k_pch_SteamVR_PerfGraphVisibility_Bool: *const c_char = real_c_string!("showPerfGraph");
pub const k_pch_SteamVR_StartMonitorFromAppLaunch: *const c_char =
	real_c_string!("startMonitorFromAppLaunch");
pub const k_pch_SteamVR_StartCompositorFromAppLaunch_Bool: *const c_char =
	real_c_string!("startCompositorFromAppLaunch");
pub const k_pch_SteamVR_StartDashboardFromAppLaunch_Bool: *const c_char =
	real_c_string!("startDashboardFromAppLaunch");
pub const k_pch_SteamVR_StartOverlayAppsFromDashboard_Bool: *const c_char =
	real_c_string!("startOverlayAppsFromDashboard");
pub const k_pch_SteamVR_EnableHomeApp: *const c_char = real_c_string!("enableHomeApp");
pub const k_pch_SteamVR_CycleBackgroundImageTimeSec_Int32: *const c_char =
	real_c_string!("CycleBackgroundImageTimeSec");
pub const k_pch_SteamVR_RetailDemo_Bool: *const c_char = real_c_string!("retailDemo");
pub const k_pch_SteamVR_IpdOffset_Float: *const c_char = real_c_string!("ipdOffset");
pub const k_pch_SteamVR_AllowSupersampleFiltering_Bool: *const c_char =
	real_c_string!("allowSupersampleFiltering");
pub const k_pch_SteamVR_SupersampleManualOverride_Bool: *const c_char =
	real_c_string!("supersampleManualOverride");
pub const k_pch_SteamVR_EnableLinuxVulkanAsync_Bool: *const c_char =
	real_c_string!("enableLinuxVulkanAsync");
pub const k_pch_SteamVR_AllowDisplayLockedMode_Bool: *const c_char =
	real_c_string!("allowDisplayLockedMode");
pub const k_pch_SteamVR_HaveStartedTutorialForNativeChaperoneDriver_Bool: *const c_char =
	real_c_string!("haveStartedTutorialForNativeChaperoneDriver");
pub const k_pch_SteamVR_ForceWindows32bitVRMonitor: *const c_char =
	real_c_string!("forceWindows32BitVRMonitor");
pub const k_pch_SteamVR_DebugInputBinding: *const c_char = real_c_string!("debugInputBinding");
pub const k_pch_SteamVR_DoNotFadeToGrid: *const c_char = real_c_string!("doNotFadeToGrid");
pub const k_pch_SteamVR_EnableSharedResourceJournaling: *const c_char =
	real_c_string!("enableSharedResourceJournaling");
pub const k_pch_SteamVR_EnableSafeMode: *const c_char = real_c_string!("enableSafeMode");
pub const k_pch_SteamVR_PreferredRefreshRate: *const c_char =
	real_c_string!("preferredRefreshRate");
pub const k_pch_SteamVR_LastVersionNotice: *const c_char = real_c_string!("lastVersionNotice");
pub const k_pch_SteamVR_LastVersionNoticeDate: *const c_char =
	real_c_string!("lastVersionNoticeDate");
pub const k_pch_SteamVR_HmdDisplayColorGainR_Float: *const c_char =
	real_c_string!("hmdDisplayColorGainR");
pub const k_pch_SteamVR_HmdDisplayColorGainG_Float: *const c_char =
	real_c_string!("hmdDisplayColorGainG");
pub const k_pch_SteamVR_HmdDisplayColorGainB_Float: *const c_char =
	real_c_string!("hmdDisplayColorGainB");
pub const k_pch_SteamVR_CustomIconStyle_String: *const c_char = real_c_string!("customIconStyle");
pub const k_pch_SteamVR_CustomOffIconStyle_String: *const c_char =
	real_c_string!("customOffIconStyle");
pub const k_pch_SteamVR_CustomIconForceUpdate_String: *const c_char =
	real_c_string!("customIconForceUpdate");
pub const k_pch_SteamVR_AllowGlobalActionSetPriority: *const c_char =
	real_c_string!("globalActionSetPriority");
pub const k_pch_SteamVR_OverlayRenderQuality: *const c_char =
	real_c_string!("overlayRenderQuality_2");
pub const k_pch_SteamVR_BlockOculusSDKOnOpenVRLaunchOption_Bool: *const c_char =
	real_c_string!("blockOculusSDKOnOpenVRLaunchOption");
pub const k_pch_SteamVR_BlockOculusSDKOnAllLaunches_Bool: *const c_char =
	real_c_string!("blockOculusSDKOnAllLaunches");
pub const k_pch_SteamVR_HDCPLegacyCompatibility_Bool: *const c_char =
	real_c_string!("hdcp14legacyCompatibility");
pub const k_pch_SteamVR_DisplayPortTrainingMode_Int: *const c_char =
	real_c_string!("displayPortTrainingMode");
pub const k_pch_SteamVR_UsePrism_Bool: *const c_char = real_c_string!("usePrism");
pub const k_pch_DirectMode_Section: *const c_char = real_c_string!("direct_mode");
pub const k_pch_DirectMode_Enable_Bool: *const c_char = real_c_string!("enable");
pub const k_pch_DirectMode_Count_Int32: *const c_char = real_c_string!("count");
pub const k_pch_DirectMode_EdidVid_Int32: *const c_char = real_c_string!("edidVid");
pub const k_pch_DirectMode_EdidPid_Int32: *const c_char = real_c_string!("edidPid");
pub const k_pch_Lighthouse_Section: *const c_char = real_c_string!("driver_lighthouse");
pub const k_pch_Lighthouse_DisableIMU_Bool: *const c_char = real_c_string!("disableimu");
pub const k_pch_Lighthouse_DisableIMUExceptHMD_Bool: *const c_char =
	real_c_string!("disableimuexcepthmd");
pub const k_pch_Lighthouse_UseDisambiguation_String: *const c_char =
	real_c_string!("usedisambiguation");
pub const k_pch_Lighthouse_DisambiguationDebug_Int32: *const c_char =
	real_c_string!("disambiguationdebug");
pub const k_pch_Lighthouse_PrimaryBasestation_Int32: *const c_char =
	real_c_string!("primarybasestation");
pub const k_pch_Lighthouse_DBHistory_Bool: *const c_char = real_c_string!("dbhistory");
pub const k_pch_Lighthouse_EnableBluetooth_Bool: *const c_char = real_c_string!("enableBluetooth");
pub const k_pch_Lighthouse_PowerManagedBaseStations_String: *const c_char =
	real_c_string!("PowerManagedBaseStations");
pub const k_pch_Lighthouse_PowerManagedBaseStations2_String: *const c_char =
	real_c_string!("PowerManagedBaseStations2");
pub const k_pch_Lighthouse_InactivityTimeoutForBaseStations_Int32: *const c_char =
	real_c_string!("InactivityTimeoutForBaseStations");
pub const k_pch_Lighthouse_EnableImuFallback_Bool: *const c_char =
	real_c_string!("enableImuFallback");
pub const k_pch_Null_Section: *const c_char = real_c_string!("driver_null");
pub const k_pch_Null_SerialNumber_String: *const c_char = real_c_string!("serialNumber");
pub const k_pch_Null_ModelNumber_String: *const c_char = real_c_string!("modelNumber");
pub const k_pch_Null_WindowX_Int32: *const c_char = real_c_string!("windowX");
pub const k_pch_Null_WindowY_Int32: *const c_char = real_c_string!("windowY");
pub const k_pch_Null_WindowWidth_Int32: *const c_char = real_c_string!("windowWidth");
pub const k_pch_Null_WindowHeight_Int32: *const c_char = real_c_string!("windowHeight");
pub const k_pch_Null_RenderWidth_Int32: *const c_char = real_c_string!("renderWidth");
pub const k_pch_Null_RenderHeight_Int32: *const c_char = real_c_string!("renderHeight");
pub const k_pch_Null_SecondsFromVsyncToPhotons_Float: *const c_char =
	real_c_string!("secondsFromVsyncToPhotons");
pub const k_pch_Null_DisplayFrequency_Float: *const c_char = real_c_string!("displayFrequency");
pub const k_pch_WindowsMR_Section: *const c_char = real_c_string!("driver_holographic");
pub const k_pch_UserInterface_Section: *const c_char = real_c_string!("userinterface");
pub const k_pch_UserInterface_StatusAlwaysOnTop_Bool: *const c_char =
	real_c_string!("StatusAlwaysOnTop");
pub const k_pch_UserInterface_MinimizeToTray_Bool: *const c_char = real_c_string!("MinimizeToTray");
pub const k_pch_UserInterface_HidePopupsWhenStatusMinimized_Bool: *const c_char =
	real_c_string!("HidePopupsWhenStatusMinimized");
pub const k_pch_UserInterface_Screenshots_Bool: *const c_char = real_c_string!("screenshots");
pub const k_pch_UserInterface_ScreenshotType_Int: *const c_char = real_c_string!("screenshotType");
pub const k_pch_Notifications_Section: *const c_char = real_c_string!("notifications");
pub const k_pch_Notifications_DoNotDisturb_Bool: *const c_char = real_c_string!("DoNotDisturb");
pub const k_pch_Keyboard_Section: *const c_char = real_c_string!("keyboard");
pub const k_pch_Keyboard_TutorialCompletions: *const c_char = real_c_string!("TutorialCompletions");
pub const k_pch_Keyboard_ScaleX: *const c_char = real_c_string!("ScaleX");
pub const k_pch_Keyboard_ScaleY: *const c_char = real_c_string!("ScaleY");
pub const k_pch_Keyboard_OffsetLeftX: *const c_char = real_c_string!("OffsetLeftX");
pub const k_pch_Keyboard_OffsetRightX: *const c_char = real_c_string!("OffsetRightX");
pub const k_pch_Keyboard_OffsetY: *const c_char = real_c_string!("OffsetY");
pub const k_pch_Keyboard_Smoothing: *const c_char = real_c_string!("Smoothing");
pub const k_pch_Perf_Section: *const c_char = real_c_string!("perfcheck");
pub const k_pch_Perf_PerfGraphInHMD_Bool: *const c_char = real_c_string!("perfGraphInHMD");
pub const k_pch_Perf_AllowTimingStore_Bool: *const c_char = real_c_string!("allowTimingStore");
pub const k_pch_Perf_SaveTimingsOnExit_Bool: *const c_char = real_c_string!("saveTimingsOnExit");
pub const k_pch_Perf_TestData_Float: *const c_char = real_c_string!("perfTestData");
pub const k_pch_Perf_GPUProfiling_Bool: *const c_char = real_c_string!("GPUProfiling");
pub const k_pch_Perf_GpuBusMonitoring_Bool: *const c_char = real_c_string!("gpuBusMonitoring");
pub const k_pch_CollisionBounds_Section: *const c_char = real_c_string!("collisionBounds");
pub const k_pch_CollisionBounds_Style_Int32: *const c_char = real_c_string!("CollisionBoundsStyle");
pub const k_pch_CollisionBounds_GroundPerimeterOn_Bool: *const c_char =
	real_c_string!("CollisionBoundsGroundPerimeterOn");
pub const k_pch_CollisionBounds_CenterMarkerOn_Bool: *const c_char =
	real_c_string!("CollisionBoundsCenterMarkerOn");
pub const k_pch_CollisionBounds_PlaySpaceOn_Bool: *const c_char =
	real_c_string!("CollisionBoundsPlaySpaceOn");
pub const k_pch_CollisionBounds_FadeDistance_Float: *const c_char =
	real_c_string!("CollisionBoundsFadeDistance");
pub const k_pch_CollisionBounds_WallHeight_Float: *const c_char =
	real_c_string!("CollisionBoundsWallHeight");
pub const k_pch_CollisionBounds_ColorGammaR_Int32: *const c_char =
	real_c_string!("CollisionBoundsColorGammaR");
pub const k_pch_CollisionBounds_ColorGammaG_Int32: *const c_char =
	real_c_string!("CollisionBoundsColorGammaG");
pub const k_pch_CollisionBounds_ColorGammaB_Int32: *const c_char =
	real_c_string!("CollisionBoundsColorGammaB");
pub const k_pch_CollisionBounds_ColorGammaA_Int32: *const c_char =
	real_c_string!("CollisionBoundsColorGammaA");
pub const k_pch_CollisionBounds_EnableDriverImport: *const c_char =
	real_c_string!("enableDriverBoundsImport");
pub const k_pch_Camera_Section: *const c_char = real_c_string!("camera");
pub const k_pch_Camera_EnableCamera_Bool: *const c_char = real_c_string!("enableCamera");
pub const k_pch_Camera_ShowOnController_Bool: *const c_char = real_c_string!("showOnController");
pub const k_pch_Camera_EnableCameraForCollisionBounds_Bool: *const c_char =
	real_c_string!("enableCameraForCollisionBounds");
pub const k_pch_Camera_RoomView_Int32: *const c_char = real_c_string!("roomView");
pub const k_pch_Camera_BoundsColorGammaR_Int32: *const c_char =
	real_c_string!("cameraBoundsColorGammaR");
pub const k_pch_Camera_BoundsColorGammaG_Int32: *const c_char =
	real_c_string!("cameraBoundsColorGammaG");
pub const k_pch_Camera_BoundsColorGammaB_Int32: *const c_char =
	real_c_string!("cameraBoundsColorGammaB");
pub const k_pch_Camera_BoundsColorGammaA_Int32: *const c_char =
	real_c_string!("cameraBoundsColorGammaA");
pub const k_pch_Camera_BoundsStrength_Int32: *const c_char = real_c_string!("cameraBoundsStrength");
pub const k_pch_Camera_RoomViewStyle_Int32: *const c_char = real_c_string!("roomViewStyle");
pub const k_pch_audio_Section: *const c_char = real_c_string!("audio");
pub const k_pch_audio_SetOsDefaultPlaybackDevice_Bool: *const c_char =
	real_c_string!("setOsDefaultPlaybackDevice");
pub const k_pch_audio_EnablePlaybackDeviceOverride_Bool: *const c_char =
	real_c_string!("enablePlaybackDeviceOverride");
pub const k_pch_audio_PlaybackDeviceOverride_String: *const c_char =
	real_c_string!("playbackDeviceOverride");
pub const k_pch_audio_PlaybackDeviceOverrideName_String: *const c_char =
	real_c_string!("playbackDeviceOverrideName");
pub const k_pch_audio_SetOsDefaultRecordingDevice_Bool: *const c_char =
	real_c_string!("setOsDefaultRecordingDevice");
pub const k_pch_audio_EnableRecordingDeviceOverride_Bool: *const c_char =
	real_c_string!("enableRecordingDeviceOverride");
pub const k_pch_audio_RecordingDeviceOverride_String: *const c_char =
	real_c_string!("recordingDeviceOverride");
pub const k_pch_audio_RecordingDeviceOverrideName_String: *const c_char =
	real_c_string!("recordingDeviceOverrideName");
pub const k_pch_audio_EnablePlaybackMirror_Bool: *const c_char =
	real_c_string!("enablePlaybackMirror");
pub const k_pch_audio_PlaybackMirrorDevice_String: *const c_char =
	real_c_string!("playbackMirrorDevice");
pub const k_pch_audio_PlaybackMirrorDeviceName_String: *const c_char =
	real_c_string!("playbackMirrorDeviceName");
pub const k_pch_audio_OldPlaybackMirrorDevice_String: *const c_char =
	real_c_string!("onPlaybackMirrorDevice");
pub const k_pch_audio_ActiveMirrorDevice_String: *const c_char =
	real_c_string!("activePlaybackMirrorDevice");
pub const k_pch_audio_EnablePlaybackMirrorIndependentVolume_Bool: *const c_char =
	real_c_string!("enablePlaybackMirrorIndependentVolume");
pub const k_pch_audio_LastHmdPlaybackDeviceId_String: *const c_char =
	real_c_string!("lastHmdPlaybackDeviceId");
pub const k_pch_audio_VIVEHDMIGain: *const c_char = real_c_string!("viveHDMIGain");
pub const k_pch_audio_DualSpeakerAndJackOutput_Bool: *const c_char =
	real_c_string!("dualSpeakerAndJackOutput");
pub const k_pch_audio_MuteMicMonitor_Bool: *const c_char = real_c_string!("muteMicMonitor");
pub const k_pch_Power_Section: *const c_char = real_c_string!("power");
pub const k_pch_Power_PowerOffOnExit_Bool: *const c_char = real_c_string!("powerOffOnExit");
pub const k_pch_Power_TurnOffScreensTimeout_Float: *const c_char =
	real_c_string!("turnOffScreensTimeout");
pub const k_pch_Power_TurnOffControllersTimeout_Float: *const c_char =
	real_c_string!("turnOffControllersTimeout");
pub const k_pch_Power_ReturnToWatchdogTimeout_Float: *const c_char =
	real_c_string!("returnToWatchdogTimeout");
pub const k_pch_Power_AutoLaunchSteamVROnButtonPress: *const c_char =
	real_c_string!("autoLaunchSteamVROnButtonPress");
pub const k_pch_Power_PauseCompositorOnStandby_Bool: *const c_char =
	real_c_string!("pauseCompositorOnStandby");
pub const k_pch_Dashboard_Section: *const c_char = real_c_string!("dashboard");
pub const k_pch_Dashboard_EnableDashboard_Bool: *const c_char = real_c_string!("enableDashboard");
pub const k_pch_Dashboard_ArcadeMode_Bool: *const c_char = real_c_string!("arcadeMode");
pub const k_pch_Dashboard_Position: *const c_char = real_c_string!("position");
pub const k_pch_Dashboard_DesktopScale: *const c_char = real_c_string!("desktopScale");
pub const k_pch_Dashboard_DashboardScale: *const c_char = real_c_string!("dashboardScale");
pub const k_pch_Dashboard_UseStandaloneSystemLayer: *const c_char =
	real_c_string!("standaloneSystemLayer");
pub const k_pch_Dashboard_StickyDashboard: *const c_char = real_c_string!("stickyDashboard");
pub const k_pch_Dashboard_AllowSteamOverlays_Bool: *const c_char =
	real_c_string!("allowSteamOverlays");
pub const k_pch_modelskin_Section: *const c_char = real_c_string!("modelskins");
pub const k_pch_Driver_Enable_Bool: *const c_char = real_c_string!("enable");
pub const k_pch_Driver_BlockedBySafemode_Bool: *const c_char =
	real_c_string!("blocked_by_safe_mode");
pub const k_pch_Driver_LoadPriority_Int32: *const c_char = real_c_string!("loadPriority");
pub const k_pch_WebInterface_Section: *const c_char = real_c_string!("WebInterface");
pub const k_pch_VRWebHelper_Section: *const c_char = real_c_string!("VRWebHelper");
pub const k_pch_VRWebHelper_DebuggerEnabled_Bool: *const c_char = real_c_string!("DebuggerEnabled");
pub const k_pch_VRWebHelper_DebuggerPort_Int32: *const c_char = real_c_string!("DebuggerPort");
pub const k_pch_TrackingOverride_Section: *const c_char = real_c_string!("TrackingOverrides");
pub const k_pch_App_BindingAutosaveURLSuffix_String: *const c_char = real_c_string!("AutosaveURL");
pub const k_pch_App_BindingLegacyAPISuffix_String: *const c_char = real_c_string!("_legacy");
pub const k_pch_App_BindingSteamVRInputAPISuffix_String: *const c_char =
	real_c_string!("_steamvrinput");
pub const k_pch_App_BindingOpenXRAPISuffix_String: *const c_char = real_c_string!("_openxr");
pub const k_pch_App_BindingCurrentURLSuffix_String: *const c_char = real_c_string!("CurrentURL");
pub const k_pch_App_BindingPreviousURLSuffix_String: *const c_char = real_c_string!("PreviousURL");
pub const k_pch_App_NeedToUpdateAutosaveSuffix_Bool: *const c_char =
	real_c_string!("NeedToUpdateAutosave");
pub const k_pch_App_DominantHand_Int32: *const c_char = real_c_string!("DominantHand");
pub const k_pch_App_BlockOculusSDK_Bool: *const c_char = real_c_string!("blockOculusSDK");
pub const k_pch_Trackers_Section: *const c_char = real_c_string!("trackers");
pub const k_pch_DesktopUI_Section: *const c_char = real_c_string!("DesktopUI");
pub const k_pch_LastKnown_Section: *const c_char = real_c_string!("LastKnown");
pub const k_pch_LastKnown_HMDManufacturer_String: *const c_char = real_c_string!("HMDManufacturer");
pub const k_pch_LastKnown_HMDModel_String: *const c_char = real_c_string!("HMDModel");
pub const k_pch_DismissedWarnings_Section: *const c_char = real_c_string!("DismissedWarnings");
pub const k_pch_Input_Section: *const c_char = real_c_string!("input");
pub const k_pch_Input_LeftThumbstickRotation_Float: *const c_char =
	real_c_string!("leftThumbstickRotation");
pub const k_pch_Input_RightThumbstickRotation_Float: *const c_char =
	real_c_string!("rightThumbstickRotation");
pub const k_pch_Input_ThumbstickDeadzone_Float: *const c_char =
	real_c_string!("thumbstickDeadzone");
pub const k_pch_GpuSpeed_Section: *const c_char = real_c_string!("GpuSpeed");
pub const ITrackedDeviceServerDriver_Version: *const c_char =
	real_c_string!("ITrackedDeviceServerDriver_005");
pub const IVRDisplayComponent_Version: *const c_char = real_c_string!("IVRDisplayComponent_003");
pub const IVRDriverDirectModeComponent_Version: *const c_char =
	real_c_string!("IVRDriverDirectModeComponent_008");
pub const IVRCameraComponent_Version: *const c_char = real_c_string!("IVRCameraComponent_003");
pub const IServerTrackedDeviceProvider_Version: *const c_char =
	real_c_string!("IServerTrackedDeviceProvider_004");
pub const IVRWatchdogProvider_Version: *const c_char = real_c_string!("IVRWatchdogProvider_001");
pub const IVRCompositorPluginProvider_Version: *const c_char =
	real_c_string!("IVRCompositorPluginProvider_001");
pub const k_ulDisplayRedirectContainer: PropertyContainerHandle_t = 25769803779;
pub const IVRProperties_Version: *const c_char = real_c_string!("IVRProperties_001");
pub const k_ulInvalidInputComponentHandle: VRInputComponentHandle_t = 0;
pub const IVRDriverInput_Version: *const c_char = real_c_string!("IVRDriverInput_003");
pub const IVRDriverLog_Version: *const c_char = real_c_string!("IVRDriverLog_001");
pub const IVRServerDriverHost_Version: *const c_char = real_c_string!("IVRServerDriverHost_006");
pub const IVRCompositorDriverHost_Version: *const c_char =
	real_c_string!("IVRCompositorDriverHost_001");
pub const IVRWatchdogHost_Version: *const c_char = real_c_string!("IVRWatchdogHost_002");
pub const IVRVirtualDisplay_Version: *const c_char = real_c_string!("IVRVirtualDisplay_002");
pub const IVRResources_Version: *const c_char = real_c_string!("IVRResources_001");
pub const k_ulInvalidIOBufferHandle: u64 = 0;
pub const IVRIOBuffer_Version: *const c_char = real_c_string!("IVRIOBuffer_002");
pub const IVRDriverManager_Version: *const c_char = real_c_string!("IVRDriverManager_001");
pub const IVRDriverSpatialAnchors_Version: *const c_char =
	real_c_string!("IVRDriverSpatialAnchors_001");
pub const IVRExtendedDisplay_Version: *const c_char = real_c_string!("IVRExtendedDisplay_001");
// pub const k_InterfaceVersions: [*const c_char; 12] = {REF, REF, REF, REF, REF, REF, REF, REF, REF, REF, REF, NULL};
#[derive(Clone, Copy)]
#[repr(C)]
pub struct HmdMatrix34_t {
	pub m: [[f32; 3]; 4],
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct HmdMatrix33_t {
	pub m: [[f32; 3]; 3],
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct HmdMatrix44_t {
	pub m: [[f32; 4]; 4],
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct HmdVector3_t {
	pub v: [f32; 3],
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct HmdVector4_t {
	pub v: [f32; 4],
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct HmdVector3d_t {
	pub v: [f64; 3],
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct HmdVector2_t {
	pub v: [f32; 2],
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct HmdQuaternion_t {
	pub w: f64,
	pub x: f64,
	pub y: f64,
	pub z: f64,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct HmdQuaternionf_t {
	pub w: f32,
	pub x: f32,
	pub y: f32,
	pub z: f32,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct HmdColor_t {
	pub r: f32,
	pub g: f32,
	pub b: f32,
	pub a: f32,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct HmdQuad_t {
	pub vCorners: [HmdVector3_t; 4],
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct HmdRect2_t {
	pub vTopLeft: HmdVector2_t,
	pub vBottomRight: HmdVector2_t,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VRBoneTransform_t {
	pub position: HmdVector4_t,
	pub orientation: HmdQuaternionf_t,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DistortionCoordinates_t {
	pub rfRed: [f32; 2],
	pub rfGreen: [f32; 2],
	pub rfBlue: [f32; 2],
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Texture_t {
	pub handle: *mut c_void,
	pub eType: ETextureType,
	pub eColorSpace: EColorSpace,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VRTextureBounds_t {
	pub uMin: f32,
	pub vMin: f32,
	pub uMax: f32,
	pub vMax: f32,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VRTextureWithPose_t {
	pub mDeviceToAbsoluteTracking: HmdMatrix34_t,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VRTextureDepthInfo_t {
	pub handle: *mut c_void,
	pub mProjection: HmdMatrix44_t,
	pub vRange: HmdVector2_t,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VRTextureWithDepth_t {
	pub depth: VRTextureDepthInfo_t,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VRTextureWithPoseAndDepth_t {
	pub depth: VRTextureDepthInfo_t,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct TrackedDevicePose_t {
	pub mDeviceToAbsoluteTracking: HmdMatrix34_t,
	pub vVelocity: HmdVector3_t,
	pub vAngularVelocity: HmdVector3_t,
	pub eTrackingResult: ETrackingResult,
	pub bPoseIsValid: bool,
	pub bDeviceIsConnected: bool,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VRVulkanTextureData_t {
	pub m_nImage: u64,
	pub m_pDevice: *mut VkDevice_T,
	pub m_pPhysicalDevice: *mut VkPhysicalDevice_T,
	pub m_pInstance: *mut VkInstance_T,
	pub m_pQueue: *mut VkQueue_T,
	pub m_nQueueFamilyIndex: u32,
	pub m_nWidth: u32,
	pub m_nHeight: u32,
	pub m_nFormat: u32,
	pub m_nSampleCount: u32,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VRVulkanTextureArrayData_t {
	pub m_unArrayIndex: u32,
	pub m_unArraySize: u32,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct D3D12TextureData_t {
	pub m_pResource: *const c_void,
	pub m_pCommandQueue: *const c_void,
	pub m_nNodeMask: u32,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VREvent_Controller_t {
	pub button: u32,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VREvent_Mouse_t {
	pub x: f32,
	pub y: f32,
	pub button: u32,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VREvent_Scroll_t {
	pub xdelta: f32,
	pub ydelta: f32,
	pub unused: u32,
	pub viewportscale: f32,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VREvent_TouchPadMove_t {
	pub bFingerDown: bool,
	pub flSecondsFingerDown: f32,
	pub fValueXFirst: f32,
	pub fValueYFirst: f32,
	pub fValueXRaw: f32,
	pub fValueYRaw: f32,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VREvent_Notification_t {
	pub ulUserValue: u64,
	pub notificationId: u32,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VREvent_Process_t {
	pub pid: u32,
	pub oldPid: u32,
	pub bForced: bool,
	pub bConnectionLost: bool,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VREvent_Overlay_t {
	pub overlayHandle: u64,
	pub devicePath: u64,
	pub memoryBlockId: u64,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VREvent_Status_t {
	pub statusState: u32,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VREvent_Keyboard_t {
	pub cNewInput: [c_char; 8],
	pub uUserValue: u64,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VREvent_Ipd_t {
	pub ipdMeters: f32,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VREvent_Chaperone_t {
	pub m_nPreviousUniverse: u64,
	pub m_nCurrentUniverse: u64,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VREvent_Reserved_t {
	pub reserved0: u64,
	pub reserved1: u64,
	pub reserved2: u64,
	pub reserved3: u64,
	pub reserved4: u64,
	pub reserved5: u64,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VREvent_PerformanceTest_t {
	pub m_nFidelityLevel: u32,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VREvent_SeatedZeroPoseReset_t {
	pub bResetBySystemMenu: bool,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VREvent_Screenshot_t {
	pub handle: u32,
	pub typ: u32,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VREvent_ScreenshotProgress_t {
	pub progress: f32,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VREvent_ApplicationLaunch_t {
	pub pid: u32,
	pub unArgsHandle: u32,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VREvent_EditingCameraSurface_t {
	pub overlayHandle: u64,
	pub nVisualMode: u32,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VREvent_MessageOverlay_t {
	pub unVRMessageOverlayResponse: u32,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VREvent_Property_t {
	pub container: PropertyContainerHandle_t,
	pub prop: ETrackedDeviceProperty,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VREvent_HapticVibration_t {
	pub containerHandle: u64,
	pub componentHandle: u64,
	pub fDurationSeconds: f32,
	pub fFrequency: f32,
	pub fAmplitude: f32,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VREvent_WebConsole_t {
	pub webConsoleHandle: WebConsoleHandle_t,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VREvent_InputBindingLoad_t {
	pub ulAppContainer: PropertyContainerHandle_t,
	pub pathMessage: u64,
	pub pathUrl: u64,
	pub pathControllerType: u64,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VREvent_InputActionManifestLoad_t {
	pub pathAppKey: u64,
	pub pathMessage: u64,
	pub pathMessageParam: u64,
	pub pathManifestPath: u64,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VREvent_SpatialAnchor_t {
	pub unHandle: SpatialAnchorHandle_t,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VREvent_ProgressUpdate_t {
	pub ulApplicationPropertyContainer: u64,
	pub pathDevice: u64,
	pub pathInputSource: u64,
	pub pathProgressAction: u64,
	pub pathIcon: u64,
	pub fProgress: f32,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VREvent_ShowUI_t {
	pub eType: EShowUIType,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VREvent_ShowDevTools_t {
	pub nBrowserIdentifier: i32,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VREvent_HDCPError_t {
	pub eCode: EHDCPError,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VREvent_t {
	pub eventType: u32,
	pub trackedDeviceIndex: TrackedDeviceIndex_t,
	pub eventAgeSeconds: f32,
	pub data: VREvent_Data_t,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct RenderModel_ComponentState_t {
	pub mTrackingToComponentRenderModel: HmdMatrix34_t,
	pub mTrackingToComponentLocal: HmdMatrix34_t,
	pub uProperties: VRComponentProperties,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct HiddenAreaMesh_t {
	pub pVertexData: *const HmdVector2_t,
	pub unTriangleCount: u32,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VRControllerAxis_t {
	pub x: f32,
	pub y: f32,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VRControllerState001_t {
	pub unPacketNum: u32,
	pub ulButtonPressed: u64,
	pub ulButtonTouched: u64,
	pub rAxis: [VRControllerAxis_t; 5],
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct CameraVideoStreamFrameHeader_t {
	pub eFrameType: EVRTrackedCameraFrameType,
	pub nWidth: u32,
	pub nHeight: u32,
	pub nBytesPerPixel: u32,
	pub nFrameSequence: u32,
	pub trackedDevicePose: TrackedDevicePose_t,
	pub ulFrameExposureTime: u64,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Compositor_FrameTiming {
	pub m_nSize: u32,
	pub m_nFrameIndex: u32,
	pub m_nNumFramePresents: u32,
	pub m_nNumMisPresented: u32,
	pub m_nNumDroppedFrames: u32,
	pub m_nReprojectionFlags: u32,
	pub m_flSystemTimeInSeconds: f64,
	pub m_flPreSubmitGpuMs: f32,
	pub m_flPostSubmitGpuMs: f32,
	pub m_flTotalRenderGpuMs: f32,
	pub m_flCompositorRenderGpuMs: f32,
	pub m_flCompositorRenderCpuMs: f32,
	pub m_flCompositorIdleCpuMs: f32,
	pub m_flClientFrameIntervalMs: f32,
	pub m_flPresentCallCpuMs: f32,
	pub m_flWaitForPresentCpuMs: f32,
	pub m_flSubmitFrameMs: f32,
	pub m_flWaitGetPosesCalledMs: f32,
	pub m_flNewPosesReadyMs: f32,
	pub m_flNewFrameReadyMs: f32,
	pub m_flCompositorUpdateStartMs: f32,
	pub m_flCompositorUpdateEndMs: f32,
	pub m_flCompositorRenderStartMs: f32,
	pub m_HmdPose: TrackedDevicePose_t,
	pub m_nNumVSyncsReadyForUse: u32,
	pub m_nNumVSyncsToFirstView: u32,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Compositor_BenchmarkResults {
	pub m_flMegaPixelsPerSecond: f32,
	pub m_flHmdRecommendedMegaPixelsPerSecond: f32,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DriverDirectMode_FrameTiming {
	pub m_nSize: u32,
	pub m_nNumFramePresents: u32,
	pub m_nNumMisPresented: u32,
	pub m_nNumDroppedFrames: u32,
	pub m_nReprojectionFlags: u32,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImuSample_t {
	pub fSampleTime: f64,
	pub vAccel: HmdVector3d_t,
	pub vGyro: HmdVector3d_t,
	pub unOffScaleFlags: u32,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct CameraVideoStreamFrame_t {
	pub m_nStreamFormat: ECameraVideoStreamFormat,
	pub m_nWidth: u32,
	pub m_nHeight: u32,
	pub m_nImageDataSize: u32,
	pub m_nFrameSequence: u32,
	pub m_nBufferIndex: u32,
	pub m_nBufferCount: u32,
	pub m_nExposureTime: u32,
	pub m_nISPFrameTimeStamp: u32,
	pub m_nISPReferenceTimeStamp: u32,
	pub m_nSyncCounter: u32,
	pub m_nCamSyncEvents: u32,
	pub m_nISPSyncEvents: u32,
	pub m_flReferenceCamSyncTime: f64,
	pub m_flFrameElapsedTime: f64,
	pub m_flFrameDeliveryRate: f64,
	pub m_flFrameCaptureTime_DriverAbsolute: f64,
	pub m_flFrameCaptureTime_ServerRelative: f64,
	pub m_nFrameCaptureTicks_ServerAbsolute: u64,
	pub m_flFrameCaptureTime_ClientRelative: f64,
	pub m_flSyncMarkerError: f64,
	pub m_RawTrackedDevicePose: TrackedDevicePose_t,
	pub m_pImageData: u64,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DriverPoseQuaternion_t {
	pub w: f64,
	pub x: f64,
	pub y: f64,
	pub z: f64,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DriverPose_t {
	pub poseTimeOffset: f64,
	pub qWorldFromDriverRotation: HmdQuaternion_t,
	pub vecWorldFromDriverTranslation: [f64; 3],
	pub qDriverFromHeadRotation: HmdQuaternion_t,
	pub vecDriverFromHeadTranslation: [f64; 3],
	pub vecPosition: [f64; 3],
	pub vecVelocity: [f64; 3],
	pub vecAcceleration: [f64; 3],
	pub qRotation: HmdQuaternion_t,
	pub vecAngularVelocity: [f64; 3],
	pub vecAngularAcceleration: [f64; 3],
	pub result: ETrackingResult,
	pub poseIsValid: bool,
	pub willDriftInYaw: bool,
	pub shouldApplyHeadModel: bool,
	pub deviceIsConnected: bool,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PropertyWrite_t {
	pub prop: ETrackedDeviceProperty,
	pub writeType: EPropertyWriteType,
	pub eSetError: ETrackedPropertyError,
	pub pvBuffer: *mut c_void,
	pub unBufferSize: u32,
	pub unTag: PropertyTypeTag_t,
	pub eError: ETrackedPropertyError,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PropertyRead_t {
	pub prop: ETrackedDeviceProperty,
	pub pvBuffer: *mut c_void,
	pub unBufferSize: u32,
	pub unTag: PropertyTypeTag_t,
	pub unRequiredBufferSize: u32,
	pub eError: ETrackedPropertyError,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PresentInfo_t {
	pub backbufferTextureHandle: SharedTextureHandle_t,
	pub vsync: EVSync,
	pub nFrameId: u64,
	pub flVSyncTimeInSeconds: f64,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SpatialAnchorDriverPose_t {
	pub qWorldRotation: HmdQuaternion_t,
	pub vWorldTranslation: HmdVector3d_t,
	pub ulRequiredUniverseId: u64,
	pub fValidDuration: f64,
}
#[vtable]
pub trait ICameraVideoSinkCallback {
	fn OnCameraVideoSinkCallback(&self);
}
#[vtable]
pub trait IServerTrackedDeviceProvider {
	fn Init(&self, pDriverContext: *const VtableRef<IVRDriverContextVtable>) -> EVRInitError;
	fn Cleanup(&self);
	fn GetInterfaceVersions(&self) -> *const *const c_char;
	fn RunFrame(&self);
	fn ShouldBlockStandbyMode(&self) -> bool;
	fn EnterStandby(&self);
	fn LeaveStandby(&self);
}
#[vtable]
pub trait ITrackedDeviceServerDriver {
	fn Activate(&self, unObjectId: u32) -> EVRInitError;
	fn Deactivate(&self);
	fn EnterStandby(&self);
	fn GetComponent(&self, pchComponentNameAndVersion: *const c_char) -> *mut c_void;
	fn DebugRequest(
		&self,
		pchRequest: *const c_char,
		pchResponseBuffer: *mut c_char,
		unResponseBufferSize: u32,
	);
	fn GetPose(&self) -> DriverPose_t;
}
#[vtable]
pub trait IVRCameraComponent {
	fn GetCameraFrameDimensions(
		&self,
		nVideoStreamFormat: ECameraVideoStreamFormat,
		pWidth: *mut u32,
		pHeight: *mut u32,
	) -> bool;
	fn GetCameraFrameBufferingRequirements(
		&self,
		pDefaultFrameQueueSize: *mut i32,
		pFrameBufferDataSize: *mut u32,
	) -> bool;
	fn SetCameraFrameBuffering(
		&self,
		nFrameBufferCount: i32,
		ppFrameBuffers: *mut *mut c_void,
		nFrameBufferDataSize: u32,
	) -> bool;
	fn SetCameraVideoStreamFormat(&self, nVideoStreamFormat: ECameraVideoStreamFormat) -> bool;
	fn GetCameraVideoStreamFormat(&self) -> ECameraVideoStreamFormat;
	fn StartVideoStream(&self) -> bool;
	fn StopVideoStream(&self);
	fn IsVideoStreamActive(&self, pbPaused: *mut bool, pflElapsedTime: *mut f32) -> bool;
	fn GetVideoStreamFrame(&self) -> *const CameraVideoStreamFrame_t;
	fn ReleaseVideoStreamFrame(&self, pFrameImage: *const CameraVideoStreamFrame_t);
	fn SetAutoExposure(&self, bEnable: bool) -> bool;
	fn PauseVideoStream(&self) -> bool;
	fn ResumeVideoStream(&self) -> bool;
	fn GetCameraDistortion(
		&self,
		nCameraIndex: u32,
		flInputU: f32,
		flInputV: f32,
		pflOutputU: *mut f32,
		pflOutputV: *mut f32,
	) -> bool;
	fn GetCameraProjection(
		&self,
		nCameraIndex: u32,
		eFrameType: EVRTrackedCameraFrameType,
		flZNear: f32,
		flZFar: f32,
		pProjection: *mut HmdMatrix44_t,
	) -> bool;
	fn SetFrameRate(&self, nISPFrameRate: i32, nSensorFrameRate: i32) -> bool;
	fn SetCameraVideoSinkCallback(
		&self,
		pCameraVideoSinkCallback: *const VtableRef<ICameraVideoSinkCallbackVtable>,
	) -> bool;
	fn GetCameraCompatibilityMode(
		&self,
		pCameraCompatibilityMode: *mut ECameraCompatibilityMode,
	) -> bool;
	fn SetCameraCompatibilityMode(
		&self,
		nCameraCompatibilityMode: ECameraCompatibilityMode,
	) -> bool;
	fn GetCameraFrameBounds(
		&self,
		eFrameType: EVRTrackedCameraFrameType,
		pLeft: *mut u32,
		pTop: *mut u32,
		pWidth: *mut u32,
		pHeight: *mut u32,
	) -> bool;
	fn GetCameraIntrinsics(
		&self,
		nCameraIndex: u32,
		eFrameType: EVRTrackedCameraFrameType,
		pFocalLength: *mut HmdVector2_t,
		pCenter: *mut HmdVector2_t,
		peDistortionType: *mut EVRDistortionFunctionType,
		rCoefficients: *mut f64,
	) -> bool;
}
#[vtable]
pub trait IVRCompositorDriverHost {
	fn PollNextEvent(&self, pEvent: *mut VREvent_t, uncbVREvent: u32) -> bool;
}
#[vtable]
pub trait IVRCompositorPluginProvider {
	fn Init(&self, pDriverContext: *const VtableRef<IVRDriverContextVtable>) -> EVRInitError;
	fn Cleanup(&self);
	fn GetInterfaceVersions(&self) -> *const *const c_char;
	fn GetComponent(&self, pchComponentNameAndVersion: *const c_char) -> *mut c_void;
}
#[vtable]
pub trait IVRDisplayComponent {
	fn GetWindowBounds(&self, pnX: *mut i32, pnY: *mut i32, pnWidth: *mut u32, pnHeight: *mut u32);
	fn IsDisplayOnDesktop(&self) -> bool;
	fn IsDisplayRealDisplay(&self) -> bool;
	fn GetRecommendedRenderTargetSize(&self, pnWidth: *mut u32, pnHeight: *mut u32);
	fn GetEyeOutputViewport(
		&self,
		eEye: EVREye,
		pnX: *mut u32,
		pnY: *mut u32,
		pnWidth: *mut u32,
		pnHeight: *mut u32,
	);
	fn GetProjectionRaw(
		&self,
		eEye: EVREye,
		pfLeft: *mut f32,
		pfRight: *mut f32,
		pfTop: *mut f32,
		pfBottom: *mut f32,
	);
	fn ComputeDistortion(&self, eEye: EVREye, fU: f32, fV: f32) -> DistortionCoordinates_t;
	fn ComputeInverseDistortion(
		&self,
		idk1: *mut HmdVector2_t,
		eEye: EVREye,
		fU: f32,
		fV: f32,
	) -> i32;
}
#[vtable]
pub trait IVRDriverContext {
	fn GetGenericInterface(
		&self,
		pchInterfaceVersion: *const c_char,
		peError: *mut EVRInitError,
	) -> *mut c_void;
	fn GetDriverHandle(&self) -> DriverHandle_t;
}
#[vtable]
pub trait IVRDriverDirectModeComponent {
	fn CreateSwapTextureSet(
		&self,
		unPid: u32,
		pSwapTextureSetDesc: *const c_void,
		pOutSwapTextureSet: *mut c_void,
	);
	fn DestroySwapTextureSet(&self, sharedTextureHandle: SharedTextureHandle_t);
	fn DestroyAllSwapTextureSets(&self, unPid: u32);
	fn GetNextSwapTextureSetIndex(
		&self,
		sharedTextureHandles: *mut SharedTextureHandle_t,
		pIndices: *mut [u32; 2],
	);
	fn SubmitLayer(&self, perEye: *const c_void);
	fn Present(&self, syncTexture: SharedTextureHandle_t);
	fn PostPresent(&self, pThrottling: *const c_void);
	fn GetFrameTiming(&self, pFrameTiming: *mut DriverDirectMode_FrameTiming);
}
#[vtable]
pub trait IVRDriverInput {
	fn CreateBooleanComponent(
		&self,
		ulContainer: PropertyContainerHandle_t,
		pchName: *const c_char,
		pHandle: *mut VRInputComponentHandle_t,
	) -> EVRInputError;
	fn UpdateBooleanComponent(
		&self,
		ulComponent: VRInputComponentHandle_t,
		bNewValue: bool,
		fTimeOffset: f64,
	) -> EVRInputError;
	fn CreateScalarComponent(
		&self,
		ulContainer: PropertyContainerHandle_t,
		pchName: *const c_char,
		pHandle: *mut VRInputComponentHandle_t,
		eType: EVRScalarType,
		eUnits: EVRScalarUnits,
	) -> EVRInputError;
	fn UpdateScalarComponent(
		&self,
		ulComponent: VRInputComponentHandle_t,
		fNewValue: f32,
		fTimeOffset: f64,
	) -> EVRInputError;
	fn CreateHapticComponent(
		&self,
		ulContainer: PropertyContainerHandle_t,
		pchName: *const c_char,
		pHandle: *mut VRInputComponentHandle_t,
	) -> EVRInputError;
	fn CreateSkeletonComponent(
		&self,
		ulContainer: PropertyContainerHandle_t,
		pchName: *const c_char,
		pchSkeletonPath: *const c_char,
		pchBasePosePath: *const c_char,
		eSkeletalTrackingLevel: EVRSkeletalTrackingLevel,
		pGripLimitTransforms: *const VRBoneTransform_t,
		unGripLimitTransformCount: u32,
		pHandle: *mut VRInputComponentHandle_t,
	) -> EVRInputError;
	fn UpdateSkeletonComponent(
		&self,
		ulComponent: VRInputComponentHandle_t,
		eMotionRange: EVRSkeletalMotionRange,
		pTransforms: *const VRBoneTransform_t,
		unTransformCount: u32,
	) -> EVRInputError;
}
#[vtable]
pub trait IVRDriverLog {
	fn Log(&self, pchLogMessage: *const c_char);
}
#[vtable]
pub trait IVRDriverManager {
	fn GetDriverCount(&self) -> u32;
	fn GetDriverName(&self, nDriver: DriverId_t, pchValue: *mut c_char, unBufferSize: u32) -> u32;
	fn GetDriverHandle(&self, pchDriverName: *const c_char) -> DriverHandle_t;
	fn IsEnabled(&self, nDriver: DriverId_t) -> bool;
}
#[vtable]
pub trait IVRDriverSpatialAnchors {
	fn UpdateSpatialAnchorPose(
		&self,
		unHandle: SpatialAnchorHandle_t,
		pPose: *const SpatialAnchorDriverPose_t,
	) -> EVRSpatialAnchorError;
	fn SetSpatialAnchorPoseError(
		&self,
		unHandle: SpatialAnchorHandle_t,
		eError: EVRSpatialAnchorError,
		fValidDuration: f64,
	) -> EVRSpatialAnchorError;
	fn UpdateSpatialAnchorDescriptor(
		&self,
		unHandle: SpatialAnchorHandle_t,
		pchDescriptor: *const c_char,
	) -> EVRSpatialAnchorError;
	fn GetSpatialAnchorPose(
		&self,
		unHandle: SpatialAnchorHandle_t,
		pDriverPoseOut: *mut SpatialAnchorDriverPose_t,
	) -> EVRSpatialAnchorError;
	fn GetSpatialAnchorDescriptor(
		&self,
		unHandle: SpatialAnchorHandle_t,
		pchDescriptorOut: *mut c_char,
		punDescriptorBufferLenInOut: *mut u32,
		bDecorated: bool,
	) -> EVRSpatialAnchorError;
}
#[vtable]
pub trait IVRExtendedDisplay {
	fn GetWindowBounds(&self, pnX: *mut i32, pnY: *mut i32, pnWidth: *mut u32, pnHeight: *mut u32);
	fn GetEyeOutputViewport(
		&self,
		eEye: EVREye,
		pnX: *mut u32,
		pnY: *mut u32,
		pnWidth: *mut u32,
		pnHeight: *mut u32,
	);
	fn GetDXGIOutputInfo(&self, pnAdapterIndex: *mut i32, pnAdapterOutputIndex: *mut i32);
}
#[vtable]
pub trait IVRIOBuffer {
	fn Open(
		&self,
		pchPath: *const c_char,
		mode: EIOBufferMode,
		unElementSize: u32,
		unElements: u32,
		pulBuffer: *const c_void,
	) -> EIOBufferError;
	fn Close(&self, ulBuffer: IOBufferHandle_t) -> EIOBufferError;
	fn Read(
		&self,
		ulBuffer: IOBufferHandle_t,
		pDst: *mut c_void,
		unBytes: u32,
		punRead: *mut u32,
	) -> EIOBufferError;
	fn Write(&self, ulBuffer: IOBufferHandle_t, pSrc: *mut c_void, unBytes: u32) -> EIOBufferError;
	fn PropertyContainer(&self, ulBuffer: IOBufferHandle_t) -> PropertyContainerHandle_t;
	fn HasReaders(&self, ulBuffer: IOBufferHandle_t) -> bool;
}
#[vtable]
pub trait IVRProperties {
	fn ReadPropertyBatch(
		&self,
		ulContainerHandle: PropertyContainerHandle_t,
		pBatch: *mut PropertyRead_t,
		unBatchEntryCount: u32,
	) -> ETrackedPropertyError;
	fn WritePropertyBatch(
		&self,
		ulContainerHandle: PropertyContainerHandle_t,
		pBatch: *mut PropertyWrite_t,
		unBatchEntryCount: u32,
	) -> ETrackedPropertyError;
	fn GetPropErrorNameFromEnum(&self, error: ETrackedPropertyError) -> *const c_char;
	fn TrackedDeviceToPropertyContainer(
		&self,
		nDevice: TrackedDeviceIndex_t,
	) -> PropertyContainerHandle_t;
}
#[vtable]
pub trait IVRResources {
	fn LoadSharedResource(
		&self,
		pchResourceName: *const c_char,
		pchBuffer: *mut c_char,
		unBufferLen: u32,
	) -> u32;
	fn GetResourceFullPath(
		&self,
		pchResourceName: *const c_char,
		pchResourceTypeDirectory: *const c_char,
		pchPathBuffer: *mut c_char,
		unBufferLen: u32,
	) -> u32;
}
#[vtable]
pub trait IVRServerDriverHost {
	fn TrackedDeviceAdded(
		&self,
		pchDeviceSerialNumber: *const c_char,
		eDeviceClass: ETrackedDeviceClass,
		pDriver: *const VtableRef<ITrackedDeviceServerDriverVtable>,
	) -> bool;
	fn TrackedDevicePoseUpdated(
		&self,
		unWhichDevice: u32,
		newPose: *const DriverPose_t,
		unPoseStructSize: u32,
	);
	fn VsyncEvent(&self, vsyncTimeOffsetSeconds: f64);
	fn VendorSpecificEvent(
		&self,
		unWhichDevice: u32,
		eventType: EVREventType,
		eventData: *const VREvent_Data_t,
		eventTimeOffset: f64,
	);
	fn IsExiting(&self) -> bool;
	fn PollNextEvent(&self, pEvent: *mut VREvent_t, uncbVREvent: u32) -> bool;
	fn GetRawTrackedDevicePoses(
		&self,
		fPredictedSecondsFromNow: f32,
		pTrackedDevicePoseArray: *mut TrackedDevicePose_t,
		unTrackedDevicePoseArrayCount: u32,
	);
	fn RequestRestart(
		&self,
		pchLocalizedReason: *const c_char,
		pchExecutableToStart: *const c_char,
		pchArguments: *const c_char,
		pchWorkingDirectory: *const c_char,
	);
	fn GetFrameTimings(&self, pTiming: *mut Compositor_FrameTiming, nFrames: u32) -> u32;
	fn SetDisplayEyeToHead(
		&self,
		unWhichDevice: u32,
		eyeToHeadLeft: *const HmdMatrix34_t,
		eyeToHeadRight: *const HmdMatrix34_t,
	);
	fn SetDisplayProjectionRaw(
		&self,
		unWhichDevice: u32,
		eyeLeft: *const HmdRect2_t,
		eyeRight: *const HmdRect2_t,
	);
	fn SetRecommendedRenderTargetSize(&self, unWhichDevice: u32, nWidth: u32, nHeight: u32);
}
#[vtable]
pub trait IVRSettings {
	fn GetSettingsErrorNameFromEnum(&self, eError: EVRSettingsError) -> *const c_char;
	fn SetBool(
		&self,
		pchSection: *const c_char,
		pchSettingsKey: *const c_char,
		bValue: bool,
		peError: *mut EVRSettingsError,
	);
	fn SetInt32(
		&self,
		pchSection: *const c_char,
		pchSettingsKey: *const c_char,
		nValue: i32,
		peError: *mut EVRSettingsError,
	);
	fn SetFloat(
		&self,
		pchSection: *const c_char,
		pchSettingsKey: *const c_char,
		flValue: f32,
		peError: *mut EVRSettingsError,
	);
	fn SetString(
		&self,
		pchSection: *const c_char,
		pchSettingsKey: *const c_char,
		pchValue: *const c_char,
		peError: *mut EVRSettingsError,
	);
	fn GetBool(
		&self,
		pchSection: *const c_char,
		pchSettingsKey: *const c_char,
		peError: *mut EVRSettingsError,
	) -> bool;
	fn GetInt32(
		&self,
		pchSection: *const c_char,
		pchSettingsKey: *const c_char,
		peError: *mut EVRSettingsError,
	) -> i32;
	fn GetFloat(
		&self,
		pchSection: *const c_char,
		pchSettingsKey: *const c_char,
		peError: *mut EVRSettingsError,
	) -> f32;
	fn GetString(
		&self,
		pchSection: *const c_char,
		pchSettingsKey: *const c_char,
		pchValue: *mut c_char,
		unValueLen: u32,
		peError: *mut EVRSettingsError,
	);
	fn RemoveSection(&self, pchSection: *const c_char, peError: *mut EVRSettingsError);
	fn RemoveKeyInSection(
		&self,
		pchSection: *const c_char,
		pchSettingsKey: *const c_char,
		peError: *mut EVRSettingsError,
	);
}
#[vtable]
pub trait IVRVirtualDisplay {
	fn Present(&self, pPresentInfo: *const PresentInfo_t, unPresentInfoSize: u32);
	fn WaitForPresent(&self);
	fn GetTimeSinceLastVsync(
		&self,
		pfSecondsSinceLastVsync: *mut f32,
		pulFrameCounter: *mut u64,
	) -> bool;
}
#[vtable]
pub trait IVRWatchdogHost {
	fn WatchdogWakeUp(&self, eDeviceClass: ETrackedDeviceClass);
}
#[vtable]
pub trait IVRWatchdogProvider {
	fn Init(&self, pDriverContext: *const VtableRef<IVRDriverContextVtable>) -> EVRInitError;
	fn Cleanup(&self);
}
