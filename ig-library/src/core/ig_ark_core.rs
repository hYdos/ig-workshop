use crate::core::ig_ark_core::EGame::*;
use crate::core::meta::ig_metadata_manager::{igMetaFieldInfo, igMetadataManager};
use crate::core::meta::ig_xml_metadata::load_xml_metadata;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::sync::Arc;
use serde::Serialize;
use crate::core::ig_core_platform::IG_CORE_PLATFORM;
use crate::core::meta::field::r#impl::ig_int_meta_field::igIntMetaField;
use crate::core::meta::field::r#impl::ig_memory_ref_meta_field::igMemoryRefMetaField;
use crate::core::meta::field::r#impl::ig_object_ref_meta_field::igObjectRefMetaField;
use crate::core::meta::field::r#impl::ig_size_type_meta_field::igSizeTypeMetaField;
use crate::core::meta::field::r#impl::ig_string_meta_field::igStringMetaField;
use crate::util::ig_name::igNameMetaField;

/// Contains reflection metadata information. Stands for Application Runtime Kernel.
pub struct igArkCore {
    pub metadata_manager: igMetadataManager
}

impl igArkCore {
    pub fn new(game: EGame, platform: IG_CORE_PLATFORM) -> Self {
        let metadata_path = PathBuf::from(format!("ArkCore/{:?}/", game));
        let xml_metadata = load_xml_metadata(&metadata_path).unwrap_or_else(|_| panic!("Failed to find metadata at path {}", metadata_path.display()));
        let mut metadata_manager = igMetadataManager::new(xml_metadata.0, xml_metadata.1, xml_metadata.2, platform);
        register_metafields(&mut metadata_manager);
        igArkCore { metadata_manager }
    }
}

/// Registers all built in meta fields to the [core::meta::field::ig_metafield_registry::igMetafieldRegistry]
fn register_metafields(imm: &mut igMetadataManager) {
    imm.meta_field_registry.register::<igIntMetaField>(Arc::from("igIntMetaField"), Arc::new(igIntMetaField));
    imm.meta_field_registry.register::<igStringMetaField>(Arc::from("igStringMetaField"), Arc::new(igStringMetaField));
    imm.meta_field_registry.register::<igNameMetaField>(Arc::from("igNameMetaField"), Arc::new(igNameMetaField));
    imm.meta_field_registry.register::<igSizeTypeMetaField>(Arc::from("igSizeTypeMetaField"), Arc::new(igSizeTypeMetaField));
    imm.meta_field_registry.register::<igObjectRefMetaField>(Arc::from("igObjectRefMetaField"), Arc::new(igObjectRefMetaField));
    imm.meta_field_registry.register_complex::<igMemoryRefMetaField>(Arc::from("igMemoryRefMetaField"), |ark_field, imm, _metafield_registry, platform| {
        let raw_internal_metafield = &ark_field.ark_info.read().unwrap().clone().ig_memory_ref_info.unwrap();
        // TODO: i need a better system for this. so many types here it really is ugly but the oop side of this makes it hard to work through
        let updated_internal_metafield = igMetaFieldInfo {
            ark_info: raw_internal_metafield.clone(),
            _type: raw_internal_metafield.read().unwrap().clone()._type,
            name: raw_internal_metafield.read().unwrap().clone().name,
            size: imm.calculate_size(&raw_internal_metafield.read().unwrap(), &platform),
            offset: raw_internal_metafield.read().unwrap().clone().offset, // should always be 0 but just in case.
        };
        Arc::new(igMemoryRefMetaField(Arc::new(updated_internal_metafield)))
    });
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum EGame {
    EV_None = -1,
    EV_ZooCube,
    EV_HootersRoadTrip,
    EV_DogsPlayingPoker,
    EV_EnigmaRisingTide,
    EV_CrashNitroKart,
    EV_SpiderMan2,
    EV_LupinSenseiColumbusNoIsanWaAkeNiSomaru,
    EV_YuGiOhTheDawnOfDestiny,
    EV_GraffitiKingdom,
    EV_XMenLegends,
    EV_GradiusV,
    EV_ShamanKingPowerOfSpirit,
    EV_UltimateSpiderMan,
    EV_XMenLegendsIIRiseOfApocalypse,
    EV_TonyHawksAmericanSk8land,
    EV_DigimonWorld4,
    EV_SpiderManBattleForNewYork,
    EV_MarvelUltimateAlliance,
    EV_TonyHawksDownhillJam,
    EV_TransformersAutobots,
    EV_TransformersDecpticons,
    EV_TonyHawksProvingGround,
    EV_ShrekTheThird,
    EV_BeautfilKatamari,
    EV_LupinSenseiLupinNiWaShiOZenigataNiWaKoiO,
    EV_SpiderMan3_DS,
    EV_WanganMidnightMaximumTune3,
    EV_BackyardBasketball2007,
    EV_SpiderMan3_HC,
    EV_OperationDarkness,
    EV_MadagascarTMEscape2AfricaTMTheGameTM,
    EV_SkylandersSpyrosAdventure,
    EV_SkylandersSpyrosAdventure_3DS,
    EV_HatsuneMikuProjectDiva,
    EV_HatsuneMikuProjectDiva2nd,
    EV_HatsuneMikuProjectDivaExtend,
    EV_SkylandersBattlegrounds,
    EV_SkylandersCloudPatrol,
    EV_SkylandersGiants,
    EV_SkylandersGiants_3DS,
    EV_SkylandersLostIslands,
    EV_SkylandersSwapForce,
    EV_SkylandersSwapForce_3DS,
    EV_SkylandersTrapTeam,
    EV_SkylandersTrapTeam_3DS,
    EV_SkylandersSuperchargers,
    EV_SkylandersSuperchargersIos,
    EV_SkylandersImaginators,
    EV_SkylandersImaginatorsSwitch,
    EV_CrashNSaneTrilogy,
    EV_CrashTeamRacingNitroFueled,
    EV_Count = 51,
}

impl Display for EGame {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EV_ZooCube => f.write_str("Zoo Cube"),
            EV_HootersRoadTrip => f.write_str("Hooters Road Trip"),
            EV_DogsPlayingPoker => f.write_str("Dogs Playing Poker"),
            EV_EnigmaRisingTide => f.write_str("Enigma Rising Tide"),
            EV_CrashNitroKart => f.write_str("Crash Nitro Kart"),
            EV_SpiderMan2 => f.write_str("Spider-Man 2"),
            EV_LupinSenseiColumbusNoIsanWaAkeNiSomaru => f.write_str("Lupin Sansei: Columbus no Isan wa Akenisomaru"),
            EV_YuGiOhTheDawnOfDestiny => f.write_str("Yu-Gi-Oh! The Dawn of Destiny"),
            EV_GraffitiKingdom => f.write_str("Graffiti Kingdom"),
            EV_XMenLegends => f.write_str("X-Men Legends"),
            EV_GradiusV => f.write_str("Gradius V"),
            EV_ShamanKingPowerOfSpirit => f.write_str("Shaman King: Power of Spirit"),
            EV_UltimateSpiderMan => f.write_str("Ultimate Spider-Man"),
            EV_XMenLegendsIIRiseOfApocalypse => f.write_str("X-Men Legends II: Rise of Apocalypse"),
            EV_TonyHawksAmericanSk8land => f.write_str("Tony Hawk's American Sk8land"),
            EV_DigimonWorld4 => f.write_str("Digimon World 4"),
            EV_SpiderManBattleForNewYork => f.write_str("Spider-Man: Battle for New York"),
            EV_MarvelUltimateAlliance => f.write_str("Marvel: Ultimate Alliance"),
            EV_TonyHawksDownhillJam => f.write_str("Tony Hawk's Downhill Jam"),
            EV_TransformersAutobots => f.write_str("Transformers: Autobots"),
            EV_TransformersDecpticons => f.write_str("Transformers: Decepticons"),
            EV_TonyHawksProvingGround => f.write_str("Tony Hawk's Proving Ground"),
            EV_ShrekTheThird => f.write_str("Shrek the Third"),
            EV_BeautfilKatamari => f.write_str("Beautiful Katamari"),
            EV_LupinSenseiLupinNiWaShiOZenigataNiWaKoiO => f.write_str("Lupin Sansei: Lupin ni wa Shi o, Zenigata ni wa Koi o"),
            EV_SpiderMan3_DS => f.write_str("SpiderMan 3 (DS)"),
            EV_WanganMidnightMaximumTune3 => f.write_str("Wangan Midnight: Maximum Tune 3"),
            EV_BackyardBasketball2007 => f.write_str("Backyard Basketball 2007"),
            EV_OperationDarkness => f.write_str("Operation Darkness"),
            EV_MadagascarTMEscape2AfricaTMTheGameTM => f.write_str("Madagascar: Escape 2 Africa"),
            EV_SkylandersSpyrosAdventure => f.write_str("Skylanders: Spryro's Adventure"),
            EV_SkylandersSpyrosAdventure_3DS => f.write_str("Skylanders: Spyro's Adventure 3DS"),
            EV_HatsuneMikuProjectDiva => f.write_str("HatsuneMiku Project Diva"),
            EV_HatsuneMikuProjectDiva2nd => f.write_str("HatsuneMiku Project Diva: 2nd"),
            EV_HatsuneMikuProjectDivaExtend => f.write_str("HatsuneMiku Project Diva: Extend"),
            EV_SkylandersBattlegrounds => f.write_str("Skylanders: Battlegrounds"),
            EV_SkylandersCloudPatrol => f.write_str("Skylanders: CloudPatrol"),
            EV_SkylandersGiants => f.write_str("Skylanders: Giants"),
            EV_SkylandersGiants_3DS => f.write_str("Skylanders: Giants 3DS"),
            EV_SkylandersLostIslands => f.write_str("Skylanders: Lost Islands"),
            EV_SkylandersSwapForce => f.write_str("Skylanders: Swap Force"),
            EV_SkylandersSwapForce_3DS => f.write_str("Skylanders: Swap Force 3DS"),
            EV_SkylandersTrapTeam => f.write_str("Skylanders: Trap Team"),
            EV_SkylandersTrapTeam_3DS => f.write_str("Skylanders: Trap Team 3DS"),
            EV_SkylandersSuperchargers => f.write_str("Skylanders: Superchargers"),
            EV_SkylandersSuperchargersIos => f.write_str("Skylanders: Superchargers v1.2.2 (iOS) "),
            EV_SkylandersImaginators => f.write_str("Skylanders: Imaginators"),
            EV_SkylandersImaginatorsSwitch => f.write_str("Skylanders: Imaginators (Switch)"),
            EV_CrashNSaneTrilogy => f.write_str("Crash N' Sane Trilogy"),
            EV_CrashTeamRacingNitroFueled => f.write_str("Crash Team Racing: Nitro Fueled"),
            _ => f.write_str(format!("{:?}", self).as_str()),
        }
    }
}

impl TryFrom<String> for EGame {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "EV_None" => Ok(EV_None),
            "EV_ZooCube" => Ok(EV_ZooCube),
            "EV_HootersRoadTrip" => Ok(EV_HootersRoadTrip),
            "EV_DogsPlayingPoker" => Ok(EV_DogsPlayingPoker),
            "EV_EnigmaRisingTide" => Ok(EV_EnigmaRisingTide),
            "EV_CrashNitroKart" => Ok(EV_CrashNitroKart),
            "EV_SpiderMan2" => Ok(EV_SpiderMan2),
            "EV_LupinSenseiColumbusNoIsanWaAkeNiSomaru" => Ok(EV_LupinSenseiColumbusNoIsanWaAkeNiSomaru),
            "EV_YuGiOhTheDawnOfDestiny" => Ok(EV_YuGiOhTheDawnOfDestiny),
            "EV_GraffitiKingdom" => Ok(EV_GraffitiKingdom),
            "EV_XMenLegends" => Ok(EV_XMenLegends),
            "EV_GradiusV" => Ok(EV_GradiusV),
            "EV_ShamanKingPowerOfSpirit" => Ok(EV_ShamanKingPowerOfSpirit),
            "EV_UltimateSpiderMan" => Ok(EV_UltimateSpiderMan),
            "EV_XMenLegendsIIRiseOfApocalypse" => Ok(EV_XMenLegendsIIRiseOfApocalypse),
            "EV_TonyHawksAmericanSk8land" => Ok(EV_TonyHawksAmericanSk8land),
            "EV_DigimonWorld4" => Ok(EV_DigimonWorld4),
            "EV_SpiderManBattleForNewYork" => Ok(EV_SpiderManBattleForNewYork),
            "EV_MarvelUltimateAlliance" => Ok(EV_MarvelUltimateAlliance),
            "EV_TonyHawksDownhillJam" => Ok(EV_TonyHawksDownhillJam),
            "EV_TransformersAutobots" => Ok(EV_TransformersAutobots),
            "EV_TransformersDecpticons" => Ok(EV_TransformersDecpticons),
            "EV_TonyHawksProvingGround" => Ok(EV_TonyHawksProvingGround),
            "EV_ShrekTheThird" => Ok(EV_ShrekTheThird),
            "EV_BeautfilKatamari" => Ok(EV_BeautfilKatamari),
            "EV_LupinSenseiLupinNiWaShiOZenigataNiWaKoiO" => Ok(EV_LupinSenseiLupinNiWaShiOZenigataNiWaKoiO),
            "EV_SpiderMan3_DS" => Ok(EV_SpiderMan3_DS),
            "EV_WanganMidnightMaximumTune3" => Ok(EV_WanganMidnightMaximumTune3),
            "EV_BackyardBasketball2007" => Ok(EV_BackyardBasketball2007),
            "EV_SpiderMan3_HC" => Ok(EV_SpiderMan3_HC),
            "EV_OperationDarkness" => Ok(EV_OperationDarkness),
            "EV_MadagascarTMEscape2AfricaTMTheGameTM" => Ok(EV_MadagascarTMEscape2AfricaTMTheGameTM),
            "EV_SkylandersSpyrosAdventure" => Ok(EV_SkylandersSpyrosAdventure),
            "EV_SkylandersSpyrosAdventure_3DS" => Ok(EV_SkylandersSpyrosAdventure_3DS),
            "EV_HatsuneMikuProjectDiva" => Ok(EV_HatsuneMikuProjectDiva),
            "EV_HatsuneMikuProjectDiva2nd" => Ok(EV_HatsuneMikuProjectDiva2nd),
            "EV_HatsuneMikuProjectDivaExtend" => Ok(EV_HatsuneMikuProjectDivaExtend),
            "EV_SkylandersBattlegrounds" => Ok(EV_SkylandersBattlegrounds),
            "EV_SkylandersCloudPatrol" => Ok(EV_SkylandersCloudPatrol),
            "EV_SkylandersGiants" => Ok(EV_SkylandersGiants),
            "EV_SkylandersGiants_3DS" => Ok(EV_SkylandersGiants_3DS),
            "EV_SkylandersLostIslands" => Ok(EV_SkylandersLostIslands),
            "EV_SkylandersSwapForce" => Ok(EV_SkylandersSwapForce),
            "EV_SkylandersSwapForce_3DS" => Ok(EV_SkylandersSwapForce_3DS),
            "EV_SkylandersTrapTeam" => Ok(EV_SkylandersTrapTeam),
            "EV_SkylandersTrapTeam_3DS" => Ok(EV_SkylandersTrapTeam_3DS),
            "EV_SkylandersSuperchargers" => Ok(EV_SkylandersSuperchargers),
            "EV_SkylandersImaginators" => Ok(EV_SkylandersImaginators),
            "EV_CrashNSaneTrilogy" => Ok(EV_CrashNSaneTrilogy),
            "EV_CrashTeamRacingNitroFueled" => Ok(EV_CrashTeamRacingNitroFueled),
            "EV_Count" => Ok(EV_Count),
            _ => Err(()),
        }
    }
}
