use crate::core::ig_core_platform::IG_CORE_PLATFORM;
use crate::core::ig_lists::igObjectList;
use paste::paste;

use byteorder::ReadBytesExt;
use std::any::Any;
use std::io::Cursor;
use std::str::FromStr;
use std::sync::{Arc, LazyLock, RwLock};

/// I'm the least proud of how this works honestly. The idea of the logic here was taken from igCauldron, but the usage of Option<arc<RwLock<dyn Any>>>> I personally find disgusting and I will do anything in my power to make sure the user doesn't get here
pub trait MetaFieldImpl: Send + Sync {
    fn is_array(&self) -> bool;
    fn attributes(&self) -> &igObjectList;
    fn deserialize_default_value(&mut self, value: &str);
    fn value(&self) -> Option<Arc<RwLock<dyn Any + Send + Sync>>>;
    fn default_value(&self) -> Arc<RwLock<dyn Any + Send + Sync>>;
    fn supports_platform(&self) -> bool;
    fn deserialize_igz_field(&mut self, handle: &mut Cursor<Vec<u8>>) -> Result<(), String>;
    // TODO: fn serialize_igz_field(&self, saver: igIGZSaver, section: igIGZSaverSection) -> Result<(), String>;
    fn size(&self, platform: IG_CORE_PLATFORM) -> u32;
    fn alignment(&self, platform: IG_CORE_PLATFORM) -> u32;
    fn is_null(&self) -> bool;
}

/// Most metafields can get away with using this type to make this file not balloon into a million lines
macro_rules! define_simple_meta_field {
    (
      $Name:ident, $size:expr, $alignment:expr, $Inner:ty, $default_expr:expr,
      parse_default = |$val:ident : &str| $parse_default:expr,
      read_igz     = |$h:ident : &mut Cursor<Vec<u8>>| $read_igz:expr
    ) => {
        paste! {
            pub struct $Name {
                value: Option<Arc<RwLock<dyn Any + Send + Sync>>>,
                attribs: crate::core::ig_lists::igObjectList,
            }

            static [<DEFAULT_ $Name:upper>]: LazyLock<Arc<RwLock<$Inner>>> =
                LazyLock::new(|| Arc::new(RwLock::new($default_expr)));

            impl $Name {
                pub fn new() -> Self {
                    $Name {
                        value: None,
                        attribs: crate::core::ig_lists::igObjectList::new(),
                    }
                }
            }

            impl MetaFieldImpl for $Name {
                fn deserialize_default_value(&mut self, s: &str) {
                    let $val = s;
                    let parsed: $Inner = $parse_default;
                    self.value = Some(Arc::new(RwLock::new(parsed)));
                }

                fn default_value(&self) -> Arc<RwLock<dyn Any + Send + Sync>> {
                    [<DEFAULT_ $Name:upper>].clone() as _
                }

                fn deserialize_igz_field(
                    &mut self,
                    $h: &mut Cursor<Vec<u8>>
                ) -> Result<(), String> {
                    let v: $Inner = $read_igz;
                    self.value = Some(Arc::new(RwLock::new(v)));
                    Ok(())
                }

                fn value(&self) -> Option<Arc<RwLock<dyn Any + Send + Sync>>> {
                    self.value.clone()
                }

                fn attributes(&self) -> &crate::core::ig_lists::igObjectList {
                    &self.attribs
                }

                fn is_array(&self) -> bool { false }
                fn supports_platform(&self) -> bool { true }

                fn size(&self, _platform: IG_CORE_PLATFORM) -> u32 {
                    $size
                }

                fn alignment(&self, _platform: IG_CORE_PLATFORM) -> u32 {
                    $alignment
                }

                fn is_null(&self) -> bool { self.value.is_none() }
            }
        }
    };
}

define_simple_meta_field!(
    igBoolMetaField,
    1,
    1,
    bool,
    false,
    parse_default = |val: &str| bool::from_str(val).unwrap(),
    read_igz = |h: &mut Cursor<Vec<u8>>| { h.read_u8().unwrap() != 0 }
);

pub(crate) struct igObjectRefMetaField;

pub(crate) struct igIntMetaField;
pub(crate) struct igRawRefMetaField;
pub(crate) struct igSizeTypeMetaField;
pub(crate) struct igUnsignedIntMetaField;
pub(crate) struct igStringMetaField;
pub(crate) struct igMemoryRefMetaField;
pub(crate) struct igUnsignedCharMetaField;
pub(crate) struct igUnsignedShortMetaField;
pub(crate) struct igStructMetaField;
pub(crate) struct igEnumMetaField;
pub(crate) struct igVectorMetaField;
pub(crate) struct igStaticMetaField;
pub(crate) struct igCharMetaField;
pub(crate) struct igFloatMetaField;
pub(crate) struct igLongMetaField;
pub(crate) struct igShortMetaField;
pub(crate) struct igPropertyFieldMetaField;
pub(crate) struct igSpinLockMetaField;
pub(crate) struct igUnsignedLongMetaField;
pub(crate) struct igBitFieldMetaField;
pub(crate) struct HeaderMetaField;
pub(crate) struct igStatHandleMetaField;
pub(crate) struct igNameMetaField;
pub(crate) struct igHandleMetaField;
pub(crate) struct igReadParamsMetaField;
pub(crate) struct igIGXUnresolvedEntryMetaField;
pub(crate) struct igVec3fMetaField;
pub(crate) struct igVec4fMetaField;
pub(crate) struct igMatrix44fMetaField;
pub(crate) struct igTraversalNodePropertiesMetaField;
pub(crate) struct igTimeMetaField;
pub(crate) struct igVec4iMetaField;
pub(crate) struct igVertexElementMetaField;
pub(crate) struct igVertexStreamMetaField;
pub(crate) struct igMemoryRefHandleMetaField;
pub(crate) struct igUnsignedIntPtrMetaField;
pub(crate) struct igQuaternionfMetaField;
pub(crate) struct igPickablePointLightDataMetaField;
pub(crate) struct igVec2fMetaField;
pub(crate) struct ExternalDirectoryInfoMetaField;
pub(crate) struct igCopyTextureParametersMetaField;
pub(crate) struct igCommandCopyTextureParametersMetaField;
pub(crate) struct igPS3LabelMetaField;
pub(crate) struct TileRegionMetaField;
pub(crate) struct igPS3ZcullRegionDescMetaField;
pub(crate) struct ZcullRegionMetaField;
pub(crate) struct igPS3ViewportMetaField;
pub(crate) struct QueueDataMetaField;
pub(crate) struct igDeferredDynamicBufferSwapMetaField;
pub(crate) struct igSortKeyMetaField;
pub(crate) struct igSortKeyValuePairMetaField;
pub(crate) struct igAtomicSortKeyValueListMetaField;
pub(crate) struct igVec4ucMetaField;
pub(crate) struct InstanceDataMetaField;
pub(crate) struct igFrustumCullerMetaField;
pub(crate) struct CameraMetaField;
pub(crate) struct PassFilterMetaField;
pub(crate) struct igShaderParameterMetaField;
pub(crate) struct igHandleNameMetaField;
pub(crate) struct DotNetTypeMetaField;
pub(crate) struct DotNetDataMetaField;
pub(crate) struct DotNetFieldDefinitionMetaField;
pub(crate) struct DelegateFunctionMetaField;
pub(crate) struct DelegateComparerMetaField;
pub(crate) struct DotNetDataComparerMetaField;
pub(crate) struct igDotNetEnumMetaField;
pub(crate) struct DotNetHashTraitsMetaField;
pub(crate) struct DotNetObjectMetaField;
pub(crate) struct DotNetThreadMetaField;
pub(crate) struct igRandomMetaField;
pub(crate) struct PacketMetaField;
pub(crate) struct NamedFunctionMetaField;
pub(crate) struct igNetReplayReplicaMetaField;
pub(crate) struct igNetReplayPacketMetaField;
pub(crate) struct igNetReplayFrameMetaField;
pub(crate) struct igIntPtrMetaField;
pub(crate) struct FieldInfoMetaField;
pub(crate) struct igScheduledMessageMetaField;
pub(crate) struct igPS3TileRegionDescMetaField;
pub(crate) struct igPS3TextureResourceMetaField;
pub(crate) struct igPS3RasterizerStateMetaField;
pub(crate) struct igPS3ShaderResourceMetaField;
pub(crate) struct igPS3ResourceMetaField;
pub(crate) struct igPS3RenderTargetViewResourceMetaField;
pub(crate) struct igRenderTargetViewDescMetaField;
pub(crate) struct igPS3QueryResourceMetaField;
pub(crate) struct igPS3GraphicsRingBufferBlockMetaField;
pub(crate) struct igPS3BufferResourceMetaField;
pub(crate) struct BlockMetaField;
pub(crate) struct igBlendStateBundleDescMetaField;
pub(crate) struct igAlphaTestStateBundleDescMetaField;
pub(crate) struct igDepthStateBundleDescMetaField;
pub(crate) struct igStencilStateBundleDescMetaField;
pub(crate) struct igRasterizerStateBundleDescMetaField;
pub(crate) struct igSortKeyPostPassCommandMetaField;
pub(crate) struct igSortKeyCommandMetaField;
pub(crate) struct igSortKeyByMaterialMetaField;
pub(crate) struct igSortKeyByDistanceMetaField;
pub(crate) struct igSamplerStateBundleDescMetaField;
pub(crate) struct igRenderTargetMaskDescMetaField;
pub(crate) struct igBaseRenderTargetViewDescMetaField;
pub(crate) struct igCommandStreamUtilitiesMetaField;
pub(crate) struct igCommandDrawEdgeGeometryParametersMetaField;
pub(crate) struct igCommandSetDitherStateParametersMetaField;
pub(crate) struct igCommandSetCommonRenderStateParametersMetaField;
pub(crate) struct igCommandComputeAndSetInstanceConstantsParametersMetaField;
pub(crate) struct igCommandComputeAndSetInstancesMatricesParametersMetaField;
pub(crate) struct igCommandSetCameraMatricesParametersMetaField;
pub(crate) struct igCommandExecuteCallbackParametersMetaField;
pub(crate) struct igCommandUpdateTextureParametersMetaField;
pub(crate) struct igCommandDecodeMemoryCommandStreamParametersMetaField;
pub(crate) struct igCommandIssueBufferedGpuTimestampParametersMetaField;
pub(crate) struct igCommandEndNamedEventParametersMetaField;
pub(crate) struct igCommandBeginNamedEventParametersMetaField;
pub(crate) struct igCommandDrawPrimitivesParametersMetaField;
pub(crate) struct igCommandClearRenderTargetParametersMetaField;
pub(crate) struct igCommandSetVertexShaderTextureSizeConstantParametersMetaField;
pub(crate) struct igCommandSetPixelShaderTextureSizeConstantParametersMetaField;
pub(crate) struct igCommandSetVertexShaderTextureEnabledConstantParametersMetaField;
pub(crate) struct igCommandSetPixelShaderTextureEnabledConstantParametersMetaField;
pub(crate) struct igCommandApplyConstantValueListParametersMetaField;
pub(crate) struct igCommandApplyConstantBundleParametersMetaField;
pub(crate) struct igCommandSetConstantArrayMatrix44fParametersMetaField;
pub(crate) struct igCommandSetConstantArrayVec4fParametersMetaField;
pub(crate) struct igCommandSetConstantArrayFloatParametersMetaField;
pub(crate) struct igCommandSetConstantArrayIntParametersMetaField;
pub(crate) struct igCommandSetConstantMatrix44fParametersMetaField;
pub(crate) struct igCommandSetConstantVec4fParametersMetaField;
pub(crate) struct igCommandSetConstantFloatParametersMetaField;
pub(crate) struct igCommandSetConstantIntParametersMetaField;
pub(crate) struct igCommandSetConstantBoolParametersMetaField;
pub(crate) struct igCommandSetRenderTargetsParametersMetaField;
pub(crate) struct igCommandPS3SetSCullParametersMetaField;
pub(crate) struct igCommandXenonSetGprCountsParametersMetaField;
pub(crate) struct igCommandXenonFlushHiZStencilParametersMetaField;
pub(crate) struct igCommandXenonSetHiStencilParametersMetaField;
pub(crate) struct igCommandSetStencilRefParametersMetaField;
pub(crate) struct igCommandSetStencilStateBundleParametersMetaField;
pub(crate) struct igCommandSetDepthStateBundleParametersMetaField;
pub(crate) struct igCommandSetRenderTargetMaskParametersMetaField;
pub(crate) struct igCommandSetBlendStateBundleParametersMetaField;
pub(crate) struct igCommandSetAlphaTestStateBundleParametersMetaField;
pub(crate) struct igCommandSetPixelShaderSamplerParametersMetaField;
pub(crate) struct igCommandSetPixelShaderTextureParametersMetaField;
pub(crate) struct igCommandSetPixelShaderVariantParametersMetaField;
pub(crate) struct igCommandSetPixelShaderParametersMetaField;
pub(crate) struct igCommandSetRasterizerStateBundleParametersMetaField;
pub(crate) struct igCommandSetScissorEnabledParametersMetaField;
pub(crate) struct igCommandSetScissorParametersMetaField;
pub(crate) struct igCommandSetViewportParametersMetaField;
pub(crate) struct igCommandSetVertexShaderSamplerParametersMetaField;
pub(crate) struct igCommandSetVertexShaderTextureParametersMetaField;
pub(crate) struct igCommandSetVertexShaderVariantParametersMetaField;
pub(crate) struct igCommandSetVertexShaderParametersMetaField;
pub(crate) struct igCommandSetVertexBufferParametersMetaField;
pub(crate) struct igCommandSetIndexBufferParametersMetaField;
pub(crate) struct igCommandSetPrimitiveTypeParametersMetaField;
pub(crate) struct ContainsPredicateMetaField;
pub(crate) struct igVariantMetaField;
pub(crate) struct igQuaternionFramefMetaField;
pub(crate) struct igVec3fAlignedMetaField;
pub(crate) struct igVfxManagerCameraDataMetaField;
pub(crate) struct igRangedFloatMetaField;
pub(crate) struct igGuiParticleCloudInstanceMetaField;
pub(crate) struct igVfxRangedCurveMetaField;
pub(crate) struct igGuiAnimationPlayDefinitionMetaField;
pub(crate) struct igGuiAnimationTagHashTraitsMetaField;
pub(crate) struct igGuiPlaceableUpdateStateMetaField;
pub(crate) struct igGuiInstanceHelperMetaField;
pub(crate) struct VisitorDataMetaField;
pub(crate) struct RequestMetaField;
pub(crate) struct ScopedSoundBankLoadQueueSectionMetaField;
pub(crate) struct ScopedSoundBankLoadContextSectionMetaField;
pub(crate) struct CSoundPitchInterpolatorMetaField;
pub(crate) struct CSoundPlayDelayInterpolatorMetaField;
pub(crate) struct CSoundInterleavedVolumeInterpolatorMetaField;
pub(crate) struct CSoundVolumeInterpolatorMetaField;
pub(crate) struct CSoundBaseInterpolatorMetaField;
pub(crate) struct CPriorityDspOverrideStateMetaField;
pub(crate) struct CDspInterpolatorMetaField;
pub(crate) struct CChannelGroupPitchInterpolatorMetaField;
pub(crate) struct CChannelGroupVolumeInterpolatorMetaField;
pub(crate) struct igVfxRgbCurveMetaField;
pub(crate) struct igVfxRangedRampMetaField;
pub(crate) struct igVfxCurveKeyframeMetaField;
pub(crate) struct igVfxModulationHelperMetaField;
pub(crate) struct igVfxAnimatedFrameInstanceMetaField;
pub(crate) struct igRangedVectorMetaField;
pub(crate) struct igRangedQuadraticMetaField;
pub(crate) struct igQuadraticMetaField;
pub(crate) struct igVfxVelocityInstantaneousOperatorInstanceMetaField;
pub(crate) struct igVfxPoseSpawnLocationOperatorPrimitiveMetaField;
pub(crate) struct igVfxPoseSpawnLocationOperatorInstanceMetaField;
pub(crate) struct FreeNodeMetaField;
pub(crate) struct igVfxOperatorContextMetaField;
pub(crate) struct igVfxOperatorCompilerMetaField;
pub(crate) struct igVfxDrawTrailOperatorInstanceMetaField;
pub(crate) struct igVec4fUnalignedMetaField;
pub(crate) struct igVfxDrawTrailControlPointMetaField;
pub(crate) struct igVfxDrawSubEffectOperatorPrimitiveMetaField;
pub(crate) struct igVfxDrawSubEffectOperatorInstanceMetaField;
pub(crate) struct igVfxDrawModelCommandMetaField;
pub(crate) struct igVfxDrawModelOperatorInstanceMetaField;
pub(crate) struct igVfxDrawIntervalPrimitiveOperatorInstanceMetaField;
pub(crate) struct igVfxDrawDecalOperatorCommandMetaField;
pub(crate) struct igVfxDrawDecalOperatorInstanceMetaField;
pub(crate) struct igVfxDrawDebugFrameColorCommandMetaField;
pub(crate) struct igVfxDrawDebugFrameVelocityCommandMetaField;
pub(crate) struct igVfxDrawDebugFrameCommandMetaField;
pub(crate) struct igVfxDrawDeathEffectOperatorPrimitiveMetaField;
pub(crate) struct igVfxPrimitiveInstanceMetaField;
pub(crate) struct igVfxOperatorPrimitiveInstanceMetaField;
pub(crate) struct TrackStateMetaField;
pub(crate) struct SPUVTableMetaField;
pub(crate) struct iteratorMetaField;
pub(crate) struct CMaterialHelpersMetaField;
pub(crate) struct CVfxRaycastOperatorInstanceMetaField;
pub(crate) struct CVfxLoadEntitySizeOperatorInstanceMetaField;
pub(crate) struct CVfxDrawVisualDataCommandMetaField;
pub(crate) struct CVfxDrawVisualDataOperatorInstanceMetaField;
pub(crate) struct CVfxDrawTintSphereOperatorInstanceMetaField;
pub(crate) struct CVfxDrawTintSphereCommandMetaField;
pub(crate) struct CEntityIDMetaField;
pub(crate) struct CBoltedModelMetaField;
pub(crate) struct CVfxDrawSoundOperatorPrimitiveMetaField;
pub(crate) struct CVfxDrawSoundCommandMetaField;
pub(crate) struct CVfxDrawPointLightOperatorInstanceMetaField;
pub(crate) struct CVfxDrawPointLightCommandMetaField;
pub(crate) struct CVfxDrawModelOperatorInstanceMetaField;
pub(crate) struct CVfxDrawModelAnimationCommandMetaField;
pub(crate) struct CVfxDrawForceOperatorInstanceMetaField;
pub(crate) struct CVfxDrawForceCommandMetaField;
pub(crate) struct CVfxDrawEntityTransformOperatorPrimitiveMetaField;
pub(crate) struct CVfxDrawEntityTransformCommandMetaField;
pub(crate) struct CVfxDrawEntityTintOperatorPrimitiveMetaField;
pub(crate) struct CVfxDrawEntityTintOperatorInstanceMetaField;
pub(crate) struct CVfxDrawEntityTintCommandMetaField;
pub(crate) struct CVfxDrawEntityFadeOperatorPrimitiveMetaField;
pub(crate) struct CVfxDrawEntityFadeCommandMetaField;
pub(crate) struct CVfxDrawDebrisOperatorInstanceMetaField;
pub(crate) struct CVfxDrawDebrisCommandMetaField;
pub(crate) struct CVfxDrawCameraShakeOperatorInstanceMetaField;
pub(crate) struct CVfxDrawBoxLightOperatorInstanceMetaField;
pub(crate) struct CVfxDrawBoxLightCommandMetaField;
pub(crate) struct igHashTraitsSDebugDrawBoltLookupPairMetaField;
pub(crate) struct CCameraShakeSystemMetaField;
pub(crate) struct CTransformMetaField;
pub(crate) struct CConstraintMetaField;
pub(crate) struct CCameraModelMetaField;
pub(crate) struct CDeactivateAllOverrideCameraListenerMetaField;
pub(crate) struct SFakeModelTurnSnapBackDataMetaField;
pub(crate) struct CHavokRigidBodyMetaField;
pub(crate) struct SModelRotationOverrideDataMetaField;
pub(crate) struct SOverrideSteeringDataMetaField;
pub(crate) struct igWideCharMetaField;
pub(crate) struct igTimeOfDayMetaField;
pub(crate) struct BehaviorDataMetaFieldPairMetaField;
pub(crate) struct igOrderedMapMetaField;
pub(crate) struct igSplineSegmentMetaField;
pub(crate) struct igSplineFloatKeyframeTrackSegmentMetaField;
pub(crate) struct igSplineVec3fKeyframeTrackSegmentMetaField;
pub(crate) struct igSplineRotationKeyframeTrackSegmentMetaField;
pub(crate) struct CEntityClosestToPointSorterFunctorMetaField;
pub(crate) struct igDoubleMetaField;
pub(crate) struct SWheelHitInfoMetaField;
pub(crate) struct igHashTraitsSVfxBoltSharingKeyMetaField;
pub(crate) struct CTriggerVolumeComponentQueryFlagsMetaField;
pub(crate) struct CEntitySoundValueMetaField;
pub(crate) struct CEntitySoundKeyMetaField;
pub(crate) struct SBranchLevelMetaField;
pub(crate) struct SLeafGroupMetaField;
pub(crate) struct SAmbientAtmosphereEffectDataMetaField;
pub(crate) struct SColorDataMetaField;
pub(crate) struct CModelBoltInfoMetaField;
pub(crate) struct CDeformerHelpersMetaField;
pub(crate) struct CMarketplaceInventoryItemComparerMetaField;
pub(crate) struct CNetworkPatcherScopedLoadMetaField;
pub(crate) struct CNetworkErrorStatusMetaField;
pub(crate) struct CCollectionVaultMetaField;
pub(crate) struct CCloudToyDataMetaField;
pub(crate) struct igVec3dMetaField;
pub(crate) struct CTimeFormatHelperMetaField;
pub(crate) struct CTagHashTraitsMetaField;
pub(crate) struct CScopedPrecacheMemoryTrackerMetaField;
pub(crate) struct CResourcePrecacherCurrentlyLoadingZoneScopeMetaField;
pub(crate) struct CResourcePrecacherDestinationPoolScopeMetaField;
pub(crate) struct COrgAnglesMetaField;
pub(crate) struct PrintAreaMetaField;
pub(crate) struct SHashTraitsEPlayerModeMetaField;
pub(crate) struct SPlayerInfoMetaField;
pub(crate) struct CGuiRacingConnectionListenerMetaField;
pub(crate) struct UIParamsMetaField;
pub(crate) struct CGuiConnectionListenerMetaField;
pub(crate) struct CButtonMapMetaField;
pub(crate) struct CHavokConstraintMetaField;
pub(crate) struct CHavokRigidBodyIteratorMetaField;
pub(crate) struct SQueryDumpHelperMetaField;
pub(crate) struct SPlacementBufferMetaField;
pub(crate) struct CHavokDeferredForceMetaField;
pub(crate) struct igHashTraitsSEntityContactPairMetaField;
pub(crate) struct CHavokBlockStreamAnalyzerMetaField;
pub(crate) struct SHavokInPlaceAllocationMetaField;
pub(crate) struct RigConstructionUtilsMetaField;
pub(crate) struct CHavokAnimationUpdateBatchMetaField;
pub(crate) struct CMinigameGlobalStateMetaField;
pub(crate) struct CMinigamePlayerStateMetaField;
pub(crate) struct SSupsensionSurfaceInfoMetaField;
pub(crate) struct SSupsensionImpulseMetaField;
pub(crate) struct SCollisionExtraResponseInfoMetaField;
pub(crate) struct SVehicleCollisionSubRecordMetaField;
pub(crate) struct SVehiclePostCollisionRecordMetaField;
pub(crate) struct SVehicleCollisionPhysicalResponsesMetaField;
pub(crate) struct CDotNetArgsMetaField;
pub(crate) struct SUniqueToyFinderMetaField;
pub(crate) struct SRequiredToyFinderMetaField;
pub(crate) struct SPlayerHandleCharacterFinderMetaField;
pub(crate) struct SPlayerIdCharacterFinderMetaField;
pub(crate) struct SHashTraitsElementTypeMetaField;
pub(crate) struct kTfbSpyroTag_ToyTypeHashTraitsMetaField;
pub(crate) struct CToyDataSorterMetaField;
pub(crate) struct CTfbSpyroTagMetaField;
pub(crate) struct CQueryMetaField;
pub(crate) struct SEntCostMetaField;
pub(crate) struct CStaticOrderMetaField;
pub(crate) struct CQueryMemoryMetaField;
pub(crate) struct CSortByHealthMetaField;
pub(crate) struct CSortByWeightedVisionMetaField;
pub(crate) struct CSortByLeftToRightPositionMetaField;
pub(crate) struct CSortByCenterDistanceMetaField;
pub(crate) struct CSortByOriginDistanceMetaField;
pub(crate) struct CSortByDotProductMetaField;
pub(crate) struct CSortOrderMetaField;
pub(crate) struct CQueryHMetaField;
pub(crate) struct CCollisionMetaField;
pub(crate) struct igVec2ucMetaField;
pub(crate) struct SHashTraitsTfbSpyroTag_ToyTypeMetaField;
pub(crate) struct SClocktibleCompareMetaField;
pub(crate) struct HistoryEntryMetaField;
pub(crate) struct PlatformHistoryEntryMetaField;
pub(crate) struct CNetworkDisableFieldReplicationComponentFinderMetaField;
pub(crate) struct SSplashInstanceInformationMetaField;
pub(crate) struct CBoltOnMetaField;
pub(crate) struct SEnabledComponentFinderMetaField;
pub(crate) struct CEntityComponentDataSortByInitializationOrderMetaField;
pub(crate) struct SBlockedLinkMetaField;
pub(crate) struct IEntityFactoryMetaField;
pub(crate) struct CTetherMetaField;
pub(crate) struct CTeleportInfoMetaField;
pub(crate) struct CSaveSlotDataMetaField;
pub(crate) struct CSaveLoadMetaField;
pub(crate) struct SRaceParticipantNintendoExclusiveCompareMetaField;
pub(crate) struct SRaceParticipantDriverChallengeVehicleDataNullCompareMetaField;
pub(crate) struct SRaceParticipantVehicleDataNullCompareMetaField;
pub(crate) struct SRaceParticipantMapCompareMetaField;
pub(crate) struct SRaceParticipantVehicleToyIdCompareMetaField;
pub(crate) struct SRaceParticipantCharacterToyIdCompareMetaField;
pub(crate) struct CCollectibleWaypointMetaField;
pub(crate) struct PreMagicMomentDataMetaField;
pub(crate) struct CMagicMomentRegisteredCharacterMetaField;
pub(crate) struct CEntityTagHashTraitsMetaField;
pub(crate) struct CEntityFrameCallbackSortMetaField;
pub(crate) struct CGameStateMetaField;
pub(crate) struct MeshVertexSpawnParamsMetaField;
pub(crate) struct DisplayOptionsMetaField;
pub(crate) struct ScopedIgnorePrecacheErrorsMetaField;
pub(crate) struct BodyInfoMetaField;
pub(crate) struct SFindDistanceLevelMetaField;
pub(crate) struct CHasFinishedPredicateMetaField;
pub(crate) struct CCupSorterMetaField;
pub(crate) struct CPositionSorterByPositionMetaField;
pub(crate) struct CPositionSorterByProgressionMetaField;
pub(crate) struct CPathDifficultyFunctorMetaField;
pub(crate) struct CBurnSegmentComparerMetaField;
pub(crate) struct SDispatchMessageByRecipientPredicateMetaField;
pub(crate) struct SDispatchMessageByMemoryPoolPredicateMetaField;
pub(crate) struct CDotNetEntityMessageComparerMetaField;
pub(crate) struct CCameraSystemManagerCameraDataMetaField;
pub(crate) struct CCameraShakeMetaField;
pub(crate) struct CCameraBlendMetaField;
pub(crate) struct SFindVillainByVehicleMetaField;
pub(crate) struct SFindVillainByDriverMetaField;
pub(crate) struct SFindVillainMetaField;
pub(crate) struct SNetProjectileSpawnInfoCollectorMetaField;
pub(crate) struct igVec3ucMetaField;

macro_rules! array_impl {
    ($type:ty) => {};
}

// Arrays
array_impl!(igStructArrayMetaField);
array_impl!(igVectorArrayMetaField);
array_impl!(igCharArrayMetaField);
array_impl!(igRawRefArrayMetaField);
array_impl!(igIntArrayMetaField);
array_impl!(igObjectRefArrayMetaField);
array_impl!(igBoolArrayMetaField);
array_impl!(igMemoryRefArrayMetaField);
array_impl!(igMatrix44fArrayMetaField);
array_impl!(igFloatArrayMetaField);
array_impl!(igUnsignedShortArrayMetaField);
array_impl!(igEnumArrayMetaField);
array_impl!(igUnsignedIntArrayMetaField);
array_impl!(igCachedBlendedVertexArrayMetaField);
array_impl!(igSizeTypeArrayMetaField);
array_impl!(igUnsignedCharArrayMetaField);
array_impl!(igStringArrayMetaField);
array_impl!(igVec4fArrayMetaField);
array_impl!(TileRegionArrayMetaField);
array_impl!(ZcullRegionArrayMetaField);
array_impl!(igDeferredDynamicBufferSwapArrayMetaField);
array_impl!(igVec2fArrayMetaField);
array_impl!(igVec4ucArrayMetaField);
array_impl!(DelegateComparerArrayMetaField);
array_impl!(DotNetDataComparerArrayMetaField);
array_impl!(DotNetFieldDefinitionArrayMetaField);
array_impl!(igLongArrayMetaField);
array_impl!(igUnsignedLongArrayMetaField);
array_impl!(igShortArrayMetaField);
array_impl!(DotNetHashTraitsArrayMetaField);
array_impl!(DotNetObjectArrayMetaField);
array_impl!(DotNetThreadArrayMetaField);
array_impl!(igNetReplayReplicaArrayMetaField);
array_impl!(igNetReplayPacketArrayMetaField);
array_impl!(igNetReplayFrameArrayMetaField);
array_impl!(igVec3fArrayMetaField);
array_impl!(igQuaternionfArrayMetaField);
array_impl!(igTimeArrayMetaField);
array_impl!(PacketArrayMetaField);
array_impl!(FieldInfoArrayMetaField);
array_impl!(NamedFunctionArrayMetaField);
array_impl!(igScheduledMessageArrayMetaField);
array_impl!(igPS3ZcullRegionDescArrayMetaField);
array_impl!(igPS3TileRegionDescArrayMetaField);
array_impl!(igPS3TextureResourceArrayMetaField);
array_impl!(igPS3RasterizerStateArrayMetaField);
array_impl!(igPS3ShaderResourceArrayMetaField);
array_impl!(igPS3ResourceArrayMetaField);
array_impl!(igPS3RenderTargetViewResourceArrayMetaField);
array_impl!(igRenderTargetViewDescArrayMetaField);
array_impl!(igPS3QueryResourceArrayMetaField);
array_impl!(igPS3LabelArrayMetaField);
array_impl!(igPS3GraphicsRingBufferBlockArrayMetaField);
array_impl!(igPS3BufferResourceArrayMetaField);
array_impl!(igPickablePointLightDataArrayMetaField);
array_impl!(igBlendStateBundleDescArrayMetaField);
array_impl!(igAlphaTestStateBundleDescArrayMetaField);
array_impl!(igDepthStateBundleDescArrayMetaField);
array_impl!(igStencilStateBundleDescArrayMetaField);
array_impl!(igRasterizerStateBundleDescArrayMetaField);
array_impl!(igSortKeyValuePairArrayMetaField);
array_impl!(igSortKeyPostPassCommandArrayMetaField);
array_impl!(igSortKeyCommandArrayMetaField);
array_impl!(igSortKeyByMaterialArrayMetaField);
array_impl!(igSortKeyByDistanceArrayMetaField);
array_impl!(igSortKeyArrayMetaField);
array_impl!(igAtomicSortKeyValueListArrayMetaField);
array_impl!(igSamplerStateBundleDescArrayMetaField);
array_impl!(igRenderTargetMaskDescArrayMetaField);
array_impl!(igBaseRenderTargetViewDescArrayMetaField);
array_impl!(igCopyTextureParametersArrayMetaField);
array_impl!(igCommandStreamUtilitiesArrayMetaField);
array_impl!(igCommandDrawEdgeGeometryParametersArrayMetaField);
array_impl!(igCommandSetDitherStateParametersArrayMetaField);
array_impl!(igCommandSetCommonRenderStateParametersArrayMetaField);
array_impl!(igCommandComputeAndSetInstanceConstantsParametersArrayMetaField);
array_impl!(igCommandComputeAndSetInstancesMatricesParametersArrayMetaField);
array_impl!(igCommandSetCameraMatricesParametersArrayMetaField);
array_impl!(igCommandExecuteCallbackParametersArrayMetaField);
array_impl!(igCommandUpdateTextureParametersArrayMetaField);
array_impl!(igCommandCopyTextureParametersArrayMetaField);
array_impl!(igCommandDecodeMemoryCommandStreamParametersArrayMetaField);
array_impl!(igCommandIssueBufferedGpuTimestampParametersArrayMetaField);
array_impl!(igCommandEndNamedEventParametersArrayMetaField);
array_impl!(igCommandBeginNamedEventParametersArrayMetaField);
array_impl!(igCommandDrawPrimitivesParametersArrayMetaField);
array_impl!(igCommandClearRenderTargetParametersArrayMetaField);
array_impl!(igCommandSetVertexShaderTextureSizeConstantParametersArrayMetaField);
array_impl!(igCommandSetPixelShaderTextureSizeConstantParametersArrayMetaField);
array_impl!(igCommandSetVertexShaderTextureEnabledConstantParametersArrayMetaField);
array_impl!(igCommandSetPixelShaderTextureEnabledConstantParametersArrayMetaField);
array_impl!(igCommandApplyConstantValueListParametersArrayMetaField);
array_impl!(igCommandApplyConstantBundleParametersArrayMetaField);
array_impl!(igCommandSetConstantArrayMatrix44fParametersArrayMetaField);
array_impl!(igCommandSetConstantArrayVec4fParametersArrayMetaField);
array_impl!(igCommandSetConstantArrayFloatParametersArrayMetaField);
array_impl!(igCommandSetConstantArrayIntParametersArrayMetaField);
array_impl!(igCommandSetConstantMatrix44fParametersArrayMetaField);
array_impl!(igCommandSetConstantVec4fParametersArrayMetaField);
array_impl!(igCommandSetConstantFloatParametersArrayMetaField);
array_impl!(igCommandSetConstantIntParametersArrayMetaField);
array_impl!(igCommandSetConstantBoolParametersArrayMetaField);
array_impl!(igCommandSetRenderTargetsParametersArrayMetaField);
array_impl!(igCommandPS3SetSCullParametersArrayMetaField);
array_impl!(igCommandXenonSetGprCountsParametersArrayMetaField);
array_impl!(igCommandXenonFlushHiZStencilParametersArrayMetaField);
array_impl!(igCommandXenonSetHiStencilParametersArrayMetaField);
array_impl!(igCommandSetStencilRefParametersArrayMetaField);
array_impl!(igCommandSetStencilStateBundleParametersArrayMetaField);
array_impl!(igCommandSetDepthStateBundleParametersArrayMetaField);
array_impl!(igCommandSetRenderTargetMaskParametersArrayMetaField);
array_impl!(igCommandSetBlendStateBundleParametersArrayMetaField);
array_impl!(igCommandSetAlphaTestStateBundleParametersArrayMetaField);
array_impl!(igCommandSetPixelShaderSamplerParametersArrayMetaField);
array_impl!(igCommandSetPixelShaderTextureParametersArrayMetaField);
array_impl!(igCommandSetPixelShaderVariantParametersArrayMetaField);
array_impl!(igCommandSetPixelShaderParametersArrayMetaField);
array_impl!(igCommandSetRasterizerStateBundleParametersArrayMetaField);
array_impl!(igCommandSetScissorEnabledParametersArrayMetaField);
array_impl!(igCommandSetScissorParametersArrayMetaField);
array_impl!(igCommandSetViewportParametersArrayMetaField);
array_impl!(igCommandSetVertexShaderSamplerParametersArrayMetaField);
array_impl!(igCommandSetVertexShaderTextureParametersArrayMetaField);
array_impl!(igCommandSetVertexShaderVariantParametersArrayMetaField);
array_impl!(igCommandSetVertexShaderParametersArrayMetaField);
array_impl!(igCommandSetVertexBufferParametersArrayMetaField);
array_impl!(igCommandSetIndexBufferParametersArrayMetaField);
array_impl!(igCommandSetPrimitiveTypeParametersArrayMetaField);
array_impl!(igPS3ViewportArrayMetaField);
array_impl!(ContainsPredicateArrayMetaField);
array_impl!(BlockArrayMetaField);
array_impl!(QueueDataArrayMetaField);
array_impl!(igHandleArrayMetaField);
array_impl!(igVfxManagerCameraDataArrayMetaField);
array_impl!(igGuiParticleCloudInstanceArrayMetaField);
array_impl!(igGuiAnimationPlayDefinitionArrayMetaField);
array_impl!(igGuiAnimationTagHashTraitsArrayMetaField);
array_impl!(igGuiPlaceableUpdateStateArrayMetaField);
array_impl!(igGuiInstanceHelperArrayMetaField);
array_impl!(VisitorDataArrayMetaField);
array_impl!(RequestArrayMetaField);
array_impl!(ScopedSoundBankLoadQueueSectionArrayMetaField);
array_impl!(ScopedSoundBankLoadContextSectionArrayMetaField);
array_impl!(CSoundPitchInterpolatorArrayMetaField);
array_impl!(CSoundPlayDelayInterpolatorArrayMetaField);
array_impl!(CSoundInterleavedVolumeInterpolatorArrayMetaField);
array_impl!(CSoundVolumeInterpolatorArrayMetaField);
array_impl!(CSoundBaseInterpolatorArrayMetaField);
array_impl!(CPriorityDspOverrideStateArrayMetaField);
array_impl!(CDspInterpolatorArrayMetaField);
array_impl!(CChannelGroupPitchInterpolatorArrayMetaField);
array_impl!(CChannelGroupVolumeInterpolatorArrayMetaField);
array_impl!(igVec4iArrayMetaField);
array_impl!(igVfxCurveKeyframeArrayMetaField);
array_impl!(igVfxModulationHelperArrayMetaField);
array_impl!(igVfxAnimatedFrameInstanceArrayMetaField);
array_impl!(igVfxVelocityInstantaneousOperatorInstanceArrayMetaField);
array_impl!(igVfxPoseSpawnLocationOperatorPrimitiveArrayMetaField);
array_impl!(igVfxPoseSpawnLocationOperatorInstanceArrayMetaField);
array_impl!(igVfxOperatorContextArrayMetaField);
array_impl!(igVfxOperatorCompilerArrayMetaField);
array_impl!(igVfxDrawTrailOperatorInstanceArrayMetaField);
array_impl!(igVfxDrawTrailControlPointArrayMetaField);
array_impl!(igVfxDrawSubEffectOperatorPrimitiveArrayMetaField);
array_impl!(igVfxDrawSubEffectOperatorInstanceArrayMetaField);
array_impl!(igVfxDrawModelCommandArrayMetaField);
array_impl!(igVfxDrawModelOperatorInstanceArrayMetaField);
array_impl!(igVfxDrawIntervalPrimitiveOperatorInstanceArrayMetaField);
array_impl!(igVfxDrawDecalOperatorCommandArrayMetaField);
array_impl!(igVfxDrawDecalOperatorInstanceArrayMetaField);
array_impl!(igVfxDrawDebugFrameColorCommandArrayMetaField);
array_impl!(igVfxDrawDebugFrameVelocityCommandArrayMetaField);
array_impl!(igVfxDrawDebugFrameCommandArrayMetaField);
array_impl!(igVfxDrawDeathEffectOperatorPrimitiveArrayMetaField);
array_impl!(igVfxPrimitiveInstanceArrayMetaField);
array_impl!(igVfxOperatorPrimitiveInstanceArrayMetaField);
array_impl!(FreeNodeArrayMetaField);
array_impl!(TrackStateArrayMetaField);
array_impl!(SPUVTableArrayMetaField);
array_impl!(iteratorArrayMetaField);
array_impl!(CMaterialHelpersArrayMetaField);
array_impl!(CVfxRaycastOperatorInstanceArrayMetaField);
array_impl!(CVfxLoadEntitySizeOperatorInstanceArrayMetaField);
array_impl!(CVfxDrawVisualDataCommandArrayMetaField);
array_impl!(CVfxDrawVisualDataOperatorInstanceArrayMetaField);
array_impl!(CVfxDrawTintSphereOperatorInstanceArrayMetaField);
array_impl!(CVfxDrawTintSphereCommandArrayMetaField);
array_impl!(CVfxDrawSoundOperatorPrimitiveArrayMetaField);
array_impl!(CVfxDrawSoundCommandArrayMetaField);
array_impl!(CVfxDrawPointLightOperatorInstanceArrayMetaField);
array_impl!(CVfxDrawPointLightCommandArrayMetaField);
array_impl!(CVfxDrawModelOperatorInstanceArrayMetaField);
array_impl!(CVfxDrawModelAnimationCommandArrayMetaField);
array_impl!(CVfxDrawForceOperatorInstanceArrayMetaField);
array_impl!(CVfxDrawForceCommandArrayMetaField);
array_impl!(CVfxDrawEntityTransformOperatorPrimitiveArrayMetaField);
array_impl!(CVfxDrawEntityTransformCommandArrayMetaField);
array_impl!(CVfxDrawEntityTintOperatorPrimitiveArrayMetaField);
array_impl!(CVfxDrawEntityTintOperatorInstanceArrayMetaField);
array_impl!(CVfxDrawEntityTintCommandArrayMetaField);
array_impl!(CVfxDrawEntityFadeOperatorPrimitiveArrayMetaField);
array_impl!(CVfxDrawEntityFadeCommandArrayMetaField);
array_impl!(CVfxDrawDebrisOperatorInstanceArrayMetaField);
array_impl!(CVfxDrawDebrisCommandArrayMetaField);
array_impl!(CVfxDrawCameraShakeOperatorInstanceArrayMetaField);
array_impl!(CVfxDrawBoxLightOperatorInstanceArrayMetaField);
array_impl!(CVfxDrawBoxLightCommandArrayMetaField);
array_impl!(igHashTraitsSDebugDrawBoltLookupPairArrayMetaField);
array_impl!(igWideCharArrayMetaField);
array_impl!(igHashTraitsSVfxBoltSharingKeyArrayMetaField);
array_impl!(CEntitySoundValueArrayMetaField);
array_impl!(CEntitySoundKeyArrayMetaField);
array_impl!(SBranchLevelArrayMetaField);
array_impl!(SLeafGroupArrayMetaField);
array_impl!(SAmbientAtmosphereEffectDataArrayMetaField);
array_impl!(SColorDataArrayMetaField);
array_impl!(CBoltedModelArrayMetaField);
array_impl!(CModelBoltInfoArrayMetaField);
array_impl!(CDeformerHelpersArrayMetaField);
array_impl!(CMarketplaceInventoryItemComparerArrayMetaField);
array_impl!(CNetworkPatcherScopedLoadArrayMetaField);
array_impl!(CNetworkErrorStatusArrayMetaField);
array_impl!(CCollectionVaultArrayMetaField);
array_impl!(CCloudToyDataArrayMetaField);
array_impl!(CTimeFormatHelperArrayMetaField);
array_impl!(CTagHashTraitsArrayMetaField);
array_impl!(CScopedPrecacheMemoryTrackerArrayMetaField);
array_impl!(CResourcePrecacherCurrentlyLoadingZoneScopeArrayMetaField);
array_impl!(CResourcePrecacherDestinationPoolScopeArrayMetaField);
array_impl!(COrgAnglesArrayMetaField);
array_impl!(PrintAreaArrayMetaField);
array_impl!(SHashTraitsEPlayerModeArrayMetaField);
array_impl!(SPlayerInfoArrayMetaField);
array_impl!(CGuiRacingConnectionListenerArrayMetaField);
array_impl!(CGuiConnectionListenerArrayMetaField);
array_impl!(CButtonMapArrayMetaField);
array_impl!(CHavokConstraintArrayMetaField);
array_impl!(CHavokRigidBodyIteratorArrayMetaField);
array_impl!(CHavokRigidBodyArrayMetaField);
array_impl!(SQueryDumpHelperArrayMetaField);
array_impl!(CHavokDeferredForceArrayMetaField);
array_impl!(igHashTraitsSEntityContactPairArrayMetaField);
array_impl!(CHavokBlockStreamAnalyzerArrayMetaField);
array_impl!(SHavokInPlaceAllocationArrayMetaField);
array_impl!(RigConstructionUtilsArrayMetaField);
array_impl!(CHavokAnimationUpdateBatchArrayMetaField);
array_impl!(CMinigameGlobalStateArrayMetaField);
array_impl!(CMinigamePlayerStateArrayMetaField);
array_impl!(SSupsensionSurfaceInfoArrayMetaField);
array_impl!(SSupsensionImpulseArrayMetaField);
array_impl!(SWheelHitInfoArrayMetaField);
array_impl!(SCollisionExtraResponseInfoArrayMetaField);
array_impl!(SVehiclePostCollisionRecordArrayMetaField);
array_impl!(SVehicleCollisionPhysicalResponsesArrayMetaField);
array_impl!(SVehicleCollisionSubRecordArrayMetaField);
array_impl!(CDotNetArgsArrayMetaField);
array_impl!(SUniqueToyFinderArrayMetaField);
array_impl!(SRequiredToyFinderArrayMetaField);
array_impl!(SPlayerHandleCharacterFinderArrayMetaField);
array_impl!(SPlayerIdCharacterFinderArrayMetaField);
array_impl!(SHashTraitsElementTypeArrayMetaField);
array_impl!(kTfbSpyroTag_ToyTypeHashTraitsArrayMetaField);
array_impl!(CToyDataSorterArrayMetaField);
array_impl!(CTfbSpyroTagArrayMetaField);
array_impl!(CQueryArrayMetaField);
array_impl!(CStaticOrderArrayMetaField);
array_impl!(CQueryMemoryArrayMetaField);
array_impl!(CSortByHealthArrayMetaField);
array_impl!(CSortByWeightedVisionArrayMetaField);
array_impl!(CSortByLeftToRightPositionArrayMetaField);
array_impl!(CSortByCenterDistanceArrayMetaField);
array_impl!(CSortByOriginDistanceArrayMetaField);
array_impl!(CSortByDotProductArrayMetaField);
array_impl!(CSortOrderArrayMetaField);
array_impl!(CQueryHArrayMetaField);
array_impl!(CCollisionArrayMetaField);
array_impl!(SHashTraitsTfbSpyroTag_ToyTypeArrayMetaField);
array_impl!(CEntityClosestToPointSorterFunctorArrayMetaField);
array_impl!(SClocktibleCompareArrayMetaField);
array_impl!(CNetworkDisableFieldReplicationComponentFinderArrayMetaField);
array_impl!(SSplashInstanceInformationArrayMetaField);
array_impl!(CTriggerVolumeComponentQueryFlagsArrayMetaField);
array_impl!(CBoltOnArrayMetaField);
array_impl!(SEnabledComponentFinderArrayMetaField);
array_impl!(CEntityComponentDataSortByInitializationOrderArrayMetaField);
array_impl!(CCameraShakeSystemArrayMetaField);
array_impl!(CTransformArrayMetaField);
array_impl!(CCameraModelArrayMetaField);
array_impl!(CConstraintArrayMetaField);
array_impl!(BehaviorDataMetaFieldPairArrayMetaField);
array_impl!(IEntityFactoryArrayMetaField);
array_impl!(CTeleportInfoArrayMetaField);
array_impl!(CSaveSlotDataArrayMetaField);
array_impl!(CSaveLoadArrayMetaField);
array_impl!(SRaceParticipantNintendoExclusiveCompareArrayMetaField);
array_impl!(SRaceParticipantDriverChallengeVehicleDataNullCompareArrayMetaField);
array_impl!(SRaceParticipantVehicleDataNullCompareArrayMetaField);
array_impl!(SRaceParticipantMapCompareArrayMetaField);
array_impl!(SRaceParticipantVehicleToyIdCompareArrayMetaField);
array_impl!(SRaceParticipantCharacterToyIdCompareArrayMetaField);
array_impl!(PreMagicMomentDataArrayMetaField);
array_impl!(CMagicMomentRegisteredCharacterArrayMetaField);
array_impl!(CEntityTagHashTraitsArrayMetaField);
array_impl!(CEntityIDArrayMetaField);
array_impl!(CEntityFrameCallbackSortArrayMetaField);
array_impl!(MeshVertexSpawnParamsArrayMetaField);
array_impl!(DisplayOptionsArrayMetaField);
array_impl!(ScopedIgnorePrecacheErrorsArrayMetaField);
array_impl!(UIParamsArrayMetaField);
array_impl!(SPlacementBufferArrayMetaField);
array_impl!(BodyInfoArrayMetaField);
array_impl!(SEntCostArrayMetaField);
array_impl!(SOverrideSteeringDataArrayMetaField);
array_impl!(SModelRotationOverrideDataArrayMetaField);
array_impl!(SFakeModelTurnSnapBackDataArrayMetaField);
array_impl!(SFindDistanceLevelArrayMetaField);
array_impl!(CHasFinishedPredicateArrayMetaField);
array_impl!(CCupSorterArrayMetaField);
array_impl!(CPositionSorterByPositionArrayMetaField);
array_impl!(CPositionSorterByProgressionArrayMetaField);
array_impl!(HistoryEntryArrayMetaField);
array_impl!(PlatformHistoryEntryArrayMetaField);
array_impl!(CPathDifficultyFunctorArrayMetaField);
array_impl!(CBurnSegmentComparerArrayMetaField);
array_impl!(SDispatchMessageByRecipientPredicateArrayMetaField);
array_impl!(SDispatchMessageByMemoryPoolPredicateArrayMetaField);
array_impl!(CDotNetEntityMessageComparerArrayMetaField);
array_impl!(CCameraSystemManagerCameraDataArrayMetaField);
array_impl!(CDeactivateAllOverrideCameraListenerArrayMetaField);
array_impl!(CCameraShakeArrayMetaField);
array_impl!(CCameraBlendArrayMetaField);
array_impl!(SBlockedLinkArrayMetaField);
array_impl!(CTetherArrayMetaField);
array_impl!(SFindVillainByVehicleArrayMetaField);
array_impl!(SFindVillainByDriverArrayMetaField);
array_impl!(SFindVillainArrayMetaField);
array_impl!(SNetProjectileSpawnInfoCollectorArrayMetaField);
array_impl!(CCollectibleWaypointArrayMetaField);
array_impl!(CGameStateArrayMetaField);
