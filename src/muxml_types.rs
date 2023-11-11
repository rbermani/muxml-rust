use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct WorkElement {
    pub work_title: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct IdentificationElement {
    pub creator: CreatorElement,
    pub encoding: EncodingElement,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct ScorePart {
    #[serde(rename = "@id")]
    pub id: String,
    pub part_name: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct PartListElement {
    pub score_part: Vec<ScorePart>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct KeyElement {
    pub fifths: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct TimeElement {
    pub beats: String,
    pub beat_type: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct ClefElement {
    #[serde(rename = "@number")]
    pub number: String,
    pub sign: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum DynamicsValue {
    Ppp,
    Pp,
    P,
    F,
    Ff,
    Fff,
    Mp,
    Mf,
    Sf,
    Rf,
    N,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct DynamicsElement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamics: Option<DynamicsValue>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct WordsElement {
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum DirectionType {
    Dynamics(DynamicsElement),
    Segno,
    Coda,
    Words(WordsElement),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct DirectionTypeElement {
    #[serde(rename = "$value")]
    pub direction_type: DirectionType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct SoundElement {
    #[serde(rename = "@dynamics", skip_serializing_if = "Option::is_none")]
    pub dynamics: Option<f32>,
    #[serde(rename = "@tempo", skip_serializing_if = "Option::is_none")]
    pub tempo: Option<f32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct DirectionElement {
    pub direction_type: DirectionTypeElement,
    pub staff: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<SoundElement>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct AttributesElement {
    pub divisions: String,
    pub key: KeyElement,
    pub time: TimeElement,
    pub staves: String,
    pub clef: Vec<ClefElement>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct PitchElement {
    pub step: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alter: Option<String>,
    pub octave: i8,
}

#[derive(Clone, Copy, Default, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum TupletType {
    #[default]
    Start,
    Stop,
    None,
}

pub type SlurType = TupletType;
pub type TiedType = TupletType;

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct TupletElement {
    #[serde(rename = "@type")]
    pub r#type: TupletType,
    #[serde(rename = "@number")]
    pub number: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct TiedElement {
    #[serde(rename = "@type")]
    pub r#type: TiedType,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct SlurElement {
    #[serde(rename = "@type")]
    pub r#type: SlurType,
    #[serde(rename = "@number")]
    pub number: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum ArticulationValue {
    #[default]
    None,
    Accent,
    StrongAccent,
    Staccato,
    Staccatissimo,
    Tenuto,
    DetachedLegato,
    Stress,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct ArticulationElement {
    pub articulations: ArticulationValue,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum Notations {
    Tied(TiedElement),
    Slur(SlurElement),
    Tuplet(TupletElement),
    Articulations(ArticulationElement),
    Dynamics,
    Fermata,
    Arpeggiate,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct NotationsElement {
    #[serde(rename = "$value")]
    pub notations: Vec<Notations>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum PitchRest {
    Rest,
    Pitch(PitchElement),
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct TimeModificationElement {
    pub actual_notes: String,
    pub normal_notes: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct ChordElement {}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct DotElement {}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct GraceElement {
    #[serde(rename = "@slash")]
    pub slash: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct NoteElement {
    #[serde(skip_serializing_if = "Option::is_none")]
    chord: Option<ChordElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grace: Option<GraceElement>,
    #[serde(rename = "$value")]
    pitch_or_rest: PitchRest,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<String>,
    voice: String,
    #[serde(rename = "type")]
    r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    stem: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dot: Option<DotElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_modification: Option<TimeModificationElement>,
    staff: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    beam: Option<Vec<BeamElement>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notations: Option<NotationsElement>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct BackupElement {
    pub duration: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum MeasureDirectionNote {
    None,
    Direction(DirectionElement),
    Barline(BarlineElement),
    Note(NoteElement),
    Backup(BackupElement),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct EndingElement {
    #[serde(rename = "@number", skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct RepeatElement {
    #[serde(rename = "@direction", skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct BarlineElement {
    #[serde(rename = "@location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending: Option<EndingElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat: Option<RepeatElement>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct BeamElement {
    #[serde(rename = "@number")]
    pub number: String,
    #[serde(rename = "$text")]
    pub value: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Measure {
    #[serde(rename = "@number")]
    pub number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<AttributesElement>,
    #[serde(rename = "$value")]
    pub direction_note: Vec<MeasureDirectionNote>,
}

impl Default for Measure {
    fn default() -> Measure {
        Measure {
            number: "1".to_string(),
            attributes: None,
            direction_note: vec![],
        }
    }
}

#[derive(Debug, Serialize, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Part {
    #[serde(rename = "@id")]
    pub id: String,
    pub measure: Vec<Measure>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "score-partwise")]
#[serde(rename_all = "kebab-case")]
pub struct ScorePartWise {
    #[serde(rename = "@version")]
    pub version: String,
    pub work: WorkElement,
    pub identification: IdentificationElement,
    pub part_list: PartListElement,
    pub part: Vec<Part>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct SupportsElement {
    #[serde(rename = "@element")]
    pub element: String,
    #[serde(rename = "@type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct EncodingElement {
    pub software: String,
    pub encoding_date: String,
    pub supports: Vec<SupportsElement>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CreatorElement {
    #[serde(rename = "@type")]
    pub r#type: String,
    #[serde(rename = "$value")]
    pub value: String,
}
