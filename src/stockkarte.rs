use direction;
use uom::si::mass::kilogram;
// via: https://community.hiveeyes.org/t/kategorien-fur-bob-stockkarte/877/3
//
#[derive(Debug)]
pub struct Stockkarte {
    pub inventorials: Inventorials,
    pub brood: Brood,
    pub state: State,
    pub health: Health,
    pub queen_reproduction: QueenReproduction,
    pub treatment: Treatment,
    pub frames_for_brood: u8,
    pub varroa_diaper_count: u8,
}

#[derive(Debug)]
enum AddedQueen {
    Cell,
    Virgin,
    Copulated,
}

#[derive(Debug)]
pub struct ChangedFrames {
    brood: i8,
    feed: i8,
    empty: i8,
    poll: i8,
    construction_with_foundation: i8,
    construction_without_foundation: i8,
    construction_with_foundation_strip: i8,
}

#[derive(Debug)]
pub struct Treatment {
    queen_added: AddedQueen,
    changed_frames: ChangedFrames,
    bees: AddOrRemove,
    feeded_solid: kilogram,
    feeded_liquid: kilogram,
    queen_excluder: AddOrRemove,
    intensity: ReviewIntensity,
    broke_queen_cells: bool,
    cut_drone_breed: bool,
    varroa_treatment: VarroaTreatment,
}

#[derive(Debug)]
pub enum VarroaTreatment {
    Apistan(AmountChoices),
    FormicAcid(AmountChoices),
    OrganicAcid(AmountChoices),
    OxalicAcid(AmountChoices),
    PowderedSuggar(AmountChoices),
    Other(String, AmountChoices),
}

#[derive(Debug)]
pub enum ReviewIntensity {
    NoReview = 0,
    OnlyCoverOpened = 1,
    Tilted = 2,
    HoneyOnly = 4,
    BroodPartly = 8,
    Complete = 16,
    Divided = 32,
}

#[derive(Debug)]
enum AddOrRemove {
    Added = 1,
    Not = 0,
    Removed = -1,
}

#[derive(Debug)]
pub struct QueenReproduction {
    pub test_queen_cups: AmountChoices,
    pub open_queen_cells: u8,
    pub capped_queen_cells: u8,
    pub hatched_queen_cells: u8,
    pub swarmed: bool,
}

#[derive(Debug)]
pub struct Brood {
    pub eggs: AmountChoices,
    pub open_brood: AmountChoices,
    pub capped_brood: AmountChoices,
    pub drone_brood: AmountChoices,
    pub brood_frames_amount: u8,
}

#[derive(Debug)]
pub struct Health {
    pub varroa_mits: AmountChoices,
    pub crippled_wings: AmountChoices,
    pub hatch_interruped: AmountChoices,
    pub holeyd_brood_cells: AmountChoices,
    pub complete_hive_lost: bool,
}

#[derive(Debug)]
pub struct State {
    pub strength: StateChoices,
    pub meekness: StateChoices,
    pub air_traffic: AmountChoices,
    pub feed_amount: AmountChoices,
}

#[derive(Debug)]
pub enum AmountChoices {
    None_ = 0,
    Some_ = 1,
    Many = 2,
    ALot = 3,
}

#[derive(Debug)]
pub enum StateChoices {
    Well = 1,
    Okay = 0,
    Bad = -1,
}

#[derive(Debug)]
pub struct Inventorials {
    pub name: String,
    pub race: MeliferaRace,
    pub queens_birth: time::Date,
    pub hive_frame: HiveFrame,
    pub location: Locations,
    pub entrance_direction: direction::Direction,
    pub brood_levels: u8,
}

#[derive(Debug)]
enum Locations {
    Urban,
    GardenColony,
    Rural,
    Forest,
    Agricultural,
}

#[derive(Debug)]
enum HiveFrame {
    // via: https://de.wikipedia.org/wiki/R%C3%A4hmchen
    Langstroth,
    DadantMod,
    DadantBlatt,
    DeutschNormal,
    Zander,
    BritishStandard,
    SchweizerMaß,
}

#[derive(Debug)]
enum MeliferaRace {
    // via https://de.wikipedia.org/wiki/Rassen_der_Westlichen_Honigbiene
    Adami,
    Carnica,
    Cecropia,
    Cypria,
    Iberiensis,
    Ligustica,
    Macedonica,
    Mellifera,
    Ruttneri,
    Siciliana,
    Sossimai,
    Taurica,
    Anatoliaca,
    Artemisia,
    Caucasia,
    Pomonella,
    Remipes,
    Sinisxinyuan,
    Syriaca,
    Adansonii,
    Capensis,
    Intermissa,
    Jemenitica,
    Lamarckii,
    Litorea,
    Monticola,
    Sahariensis,
    Scutellata,
    Simensis,
    Unicolor,
}
