use phf::{Map, phf_map};
type HashType<'a> = Map<&'a str, u32>;

/*
Integers values represent the minimum Rōblox API version (inclusive).
Empty string indicies are equivalent to the class itself.
*/

#[macro_export]
macro_rules! single_attr {
    ($v_min:expr) => {
        phf_map! { "" => $v_min }
    };
}
pub const VERSION_MIN: u32 = 47;
pub const VERSION_MAX: u32 = 678;

const TEXT_GUI_OBJECT: HashType = phf_map! {
    "OpenTypeFeatures" => 629,
    "LocalizationMatchIdentifier" => 603,
    "LocalizationMatchedSourceText" => 603,
    "TextDirection" => 581,
    "MaxVisibleGraphemes" => 469,
    "RichText" => 438,
    "TextSize" => 263,
    "TextWrapped" => 51,
    "TextScaled" => 50,
    "TextStrokeColor3" => 48,
    "TextStrokeTransparency" => 48,

    // Shared between all `GuiObject` types.
    "Interactable" => 589,
    "AutomaticSize" => 449,
    "BorderMode" => 397,
    "LayoutOrder" => 280,
    "AnchorPoint" => 271,
    "Selectable" => 201,
    "Rotation" => 131,
    "ClipsDescendants" => 48,
};

const IMAGE_GUI_OBJECT: HashType = phf_map! {
    "ResampleMode" => 490,
    "TileSize" => 290,
    "ScaleType" => 207,
    "SliceCenter" => 207,
    "ImageTransparency" => 148,
    "ImageRectOffset" => 131,

    // Shared between all `GuiObject` types.
    "Interactable" => 589,
    "AutomaticSize" => 449,
    "BorderMode" => 397,
    "LayoutOrder" => 280,
    "AnchorPoint" => 271,
    "Selectable" => 201,
    "Rotation" => 131,
    "ClipsDescendants" => 48,
};

const BASE_PART: HashType = phf_map! {
    "AudioCanCollide" => 652,
    "EnableFluidForces" => 581,
    "CanQuery" => 484,
    "PivotOffset" => 470,
    "CanTouch" => 460,
    // "CastShadow" => 380, TODO: add support for 2018 FiB builds
    "RootPriority" => 361,
    "CollisionGroupId" => 287,
    "CustomPhysicalProperties" => 220,
    // "Rotation" => 110,
};

pub const TRAITS: Map<&str, HashType> = phf_map! {
    "Part" => BASE_PART,
    "WedgePart" => BASE_PART,
    "CornerWedgePart" => BASE_PART,
    "TextLabel" => TEXT_GUI_OBJECT,
    "TextButton" => TEXT_GUI_OBJECT,
    "ImageButton" => IMAGE_GUI_OBJECT,
    "ImageLabel" => IMAGE_GUI_OBJECT,

    "PointLight" => phf_map! {
        "Enabled" => 101,
        "Brightness" => 95,
    },
    "SpotLight" => phf_map! {
        "Enabled" => 101,
        "Brightness" => 95,
    },
    "SurfaceLight" => phf_map! {
        "Angle" => 184,
        "Enabled" => 101,
        "Brightness" => 95,
    },

    "UICorner" => single_attr! (435),
    "UIStroke" => single_attr! (466),
    "UIScale" => single_attr! (287),
    "UIFlexItem" => single_attr! (598),
    "UIGridLayout" => single_attr! (266),
    "UITableLayout" => phf_map! {
        "FillEmptySpaceColumns" => 290,
    },
    "UIListLayout" => phf_map! {
        "ItemLineAlignment" => 599,
        "HorizontalFlex" => 598,
        "Padding" => 274,
    },
    "UIPageLayout" => phf_map! {
        "GamepadInputEnabled" => 310,
        "Animated" => 284,
    },
    "UIGradient" => phf_map! {
        "Enabled" => 423,
        "Color" => 412,
    },
    "ViewportFrame" => phf_map! {
        "Ambient" => 384,
        "ImageColor3" => 367,
        "" => 361,
    },
    "ScrollingFrame" => phf_map! {
        "AutomaticCanvasSize" => 449,
        "ScrollBarImageColor3" => 348,
        "ElasticBehavior" => 329,
        "HorizontalScrollBarInset" => 299,
        "" => 149,
    },
    "VideoFrame" => single_attr! (414),

    "Model" => phf_map! {
        "ModelStreamingMode" => 548,
        "LevelOfDetail" => 442,
    },
    "BillboardGui" => phf_map! {
        "SelectionBehaviorDown" => 522,
        "Brightness" => 480,
        "DistanceLowerLimit" => 382,
        "MaxDistance" => 295,
        "LightInfluence" => 291,
        "ExtentsOffsetWorldSpace" => 281,
    },
    "SurfaceGui" => phf_map! {
        "MaxDistance" => 590,
        "SelectionBehaviorDown" => 522,
        "Brightness" => 480,
        "PixelsPerStud" => 383,
        "LightInfluence" => 291,
        "ZOffset" => 280,
        "AlwaysOnTop" => 241,
        "ToolPunchThroughDistance" => 152,
        "CanvasSize" => 131,
    },
    "Beam" => phf_map! {
        "Brightness" => 498,
        "LightInfluence" => 323,
        "" => 315,
    },
    "Terrain" => phf_map! {
        "GrassLength" => 595,
        "Decoration" => 410,
        "WaterColor" => 223,
    },
    "Players" => phf_map! {
        "BanningEnabled" => 651,
        "RespawnTime" => 371,
    },
    "StudioData" => phf_map! {
        "EnableScriptCollabByDefaultOnLoad" => 419,
        "" => 384,
    },
    "Workspace" => phf_map! {
        "UseImprovedModelLod" => 678,
        "UseNewLuauTypeSolver" => 677,
        "PhysicsImprovedSleep" => 662,
        "FallHeightEnabled" => 655,
        "TouchEventsUseCollisionGroups" => 650,
        "SandboxedInstanceMode" => 648,
        "PathfindingUseImprovedSearch" => 640,
        "MoverConstraintRootBehavior" => 628,
        "RenderingCacheOptimizations" => 623,
        "PlayerCharacterDestroyBehavior" => 603,
        "PrimalPhysicsSolver" => 600,
        "AirDensity" => 581,
        "GlobalWind" => 512,
        "Retargeting" => 494,
        "ClientAnimatorThrottling" => 475,
        "Gravity" => 241,
        "AllowThirdPartySales" => 221,
        "StreamingEnabled" => 132,
    },
    "Lighting" => phf_map! {
        "LightingStyle" => 653,
        "EnvironmentDiffuseScale" => 407,
        // "ShadowSoftness" => 380, TODO: add support for 2018 FiB builds
        "Outlines" => 114,
        "OutdoorAmbient" => 101,
        "GlobalShadows" => 98,
        "FogColor" => 49,
    },
    "Camera" => phf_map! {
        "VRTiltAndRollEnabled" => 577,
        "FieldOfViewMode" => 450,
        "HeadLocked" => 234,
        "FieldOfView" => 50,
    },
    "BubbleChatConfiguration" => phf_map! {
        "MaxBubbles" => 580,
        "BackgroundTransparency" => 553,
        "AdorneeName" => 543,
    },
    "AuroraService" => phf_map! {
        "HashRoundingPoint" => 673,
        "" => 654,
    },
    "InsertService" => phf_map! {
        "AllowInsertFreeModels" => 224,
    },
    "Decal" => phf_map! {
        "ZIndex" => 483,
        "Color3" => 274,
    },
    "Texture" => phf_map! {
        "OffsetStudsV" => 391,
    },
    "StarterGui" => phf_map! {
        "StudioDefaultStyleSheet" => 664,
        "StudioInsertWidgetLayerCollectorAutoLinkStyleSheet" => 661,
        "ScreenOrientation" => 290,
    },
    "ServiceVisibilityService" => phf_map! {
        "HiddenServices" => 587,
        "VisibleServices" => 570,
    },
    "TestService" => phf_map! {
        "ThrottlePhysicsToRealtime" => 660,
        "SimulateSecondsLag" => 178,
        "NumberOfPlayers" => 95,
        "AutoRuns" => 52,
        "Description" => 51,
    },
    "ParticleEmitter" => phf_map! {
        "WindAffectsDrag" => 576,
        "FlipbookFramerate" => 506,
        "Brightness" => 498,
        "TimeScale" => 464,
        "Orientation" => 463,
        "LightInfluence" => 293,
        "Drag" => 209,
        "Acceleration" => 189,
    },
    "ChannelTabsConfiguration" => phf_map! {
        "BackgroundColor3" => 635,
    },
    "ChatInputBarConfiguration" => phf_map! {
        "AutocompleteEnabled" => 588,
        "KeyboardKeyCode" => 574,
        "BackgroundColor3" => 554,
        "Enabled" => 514,
    },
    "ChatWindowConfiguration" => phf_map! {
        "BackgroundColor3" => 551,
        "Enabled" => 514,
    },
    "StarterPlayer" => phf_map! {
        "AvatarJointUpgrade_SerializedRollout" => 650,
        "LuaCharacterController" => 603,
        "EnableDynamicHeads" => 540,
        "UserEmotesEnabled" => 384,
        "GameSettingsAvatar" => 381,
        "CharacterJumpHeight" => 375,
        "GameSettingsAssetIDFace" => 373,
        "LoadCharacterAppearance" => 218,
        "AutoJumpEnabled" => 206,
        "DevCameraOcclusionMode" => 176,
        "CameraMaxZoomDistance" => 172,
    },
    "PlayerEmulatorService" => phf_map! {
        "TextElongationFactor" => 645,
        "PseudolocalizationEnabled" => 615,
        "CustomPoliciesEnabled" => 493,
        "EmulatedCountryCode" => 455,
    },
    "SoundService" => phf_map! {
        "AudioApiByDefault" => 660,
        "CharacterSoundsUseNewApi" => 659,
        "DefaultListenerLocation" => 645,
        "RespectFilteringEnabled" => 305,
    },
    "Sky" => phf_map! {
        "SkyboxOrientation" => 671,
    },
    "ModuleScript" => phf_map! {
        "Source" => 137,
        "" => 131,
    },
    "DepthOfFieldEffect" => phf_map! {
        "FarIntensity" => 427,
    },
    "BloomEffect" => phf_map! {
        "Intensity" => 243,
    },
    "ColorCorrectionEffect" => phf_map! {
        "Brightness" => 243,
    },
    "SunRaysEffect" => phf_map! {
        "Intensity" => 243,
    },
    "Atmosphere" => phf_map! {
        "Color" => 429,
    },
    "SpawnLocation" => phf_map! {
        "Enabled" => 204,
    },
    "DataStoreService" => phf_map! {
        "AutomaticRetry" => 306,
        "LegacyNamingScheme" => 155,
        "" => 132,
    },
    "Humanoid" => phf_map! {
        "DisplayName" => 425,
        "CollisionType" => 376,
        "JumpHeight" => 375,
        "BreakJointsOnDeath" => 369,
        "HealthDisplayType" => 271,
        "HipHeight" => 227,
        "JumpPower" => 210,
        "HealthDisplayDistance" => 187,
        "NameOcclusion" => 51,
    },
    "TextChatService" => phf_map! {
        "HasSeenDeprecationDialog" => 657,
        "ChatTranslationFTUXShown" => 599,
        "ChatTranslationToggleEnabled" => 595,
        "ChatVersion" => 514,
    },

    "MaterialService" => single_attr! (494),
    "LodDataService" => single_attr! (503),
    "ProcessInstancePhysicsService" => single_attr! (498),
    "ProximityPromptService" => single_attr! (454),
    "PermissionsService" => single_attr! (420),
    "LocalizationService" => single_attr! (276),
    "TouchInputService" => single_attr! (205),
    "StarterPlayerScripts" => single_attr! (189),
    "NonReplicatedCSGDictionaryService" => single_attr! (176),
    "Folder" => single_attr! (162),
    "ReplicatedFirst" => single_attr! (154),
    "HttpService" => single_attr! (122),
    "AssetService" => single_attr! (119),
    "ScriptService" => single_attr! (90),
    "GamePassService" => single_attr! (72),
};

pub const ERAS: [(&str, u32); 44] = [
    ("2025M", 671),
    ("2025E", 655),
    ("2024L", 641),
    ("2024M", 623),
    ("2024E", 607),
    ("2023L", 593),
    ("2023M", 574),
    ("2023E", 557),
    ("2022L", 542),
    ("2022M", 525),
    ("2022E", 508),
    ("2021L", 493),
    ("2021M", 477),
    ("2021E", 460),
    ("2020L", 447),
    ("2020M", 431),
    ("2020E", 414),
    ("2019L", 401),
    ("2019M", 384),
    ("2019E", 367),
    ("2018L", 353),
    ("2018M", 336),
    ("2018E", 319),
    ("2017L", 306),
    ("2017M", 289),
    ("2017E", 272),
    ("2016L", 257),
    ("2016M", 242),
    ("2016E", 225),
    ("2015L", 211),
    ("2015M", 195),
    ("2015E", 179),
    ("2014L", 166),
    ("2014M", 149),
    ("2014E", 133),
    ("2013L", 119),
    ("2013M", 101),
    ("2013E", 84),
    ("2012L", 71),
    ("2012M", 59),
    ("2012E", 49),
    ("2011L", 45),
    ("2011M", 39),
    ("2011E", 30),
];
