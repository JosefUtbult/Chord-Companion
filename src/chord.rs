use postcard::{from_bytes, to_vec};
use derive_builder::Builder;
use serde::{self, Serialize, Serializer, ser::SerializeStruct, Deserialize};
// use postcard::{from_bytes, to_vec};
use heapless::Vec;

/// Indicates the type of chord, affecting the third and seventh
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum ChordType {
    Major = 0,
    Dominant,
    Minor
}

/// Indicates the coloring of the chord, appending notes as necessary
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum Coloring {
    None = 0,
    Sixth,
    Seventh,
    Ninth,
    Eleventh,
}

/// A struct that defines a chord by its root note, its inversion,
/// type and coloring. This struct can be serialized and deserialized
/// for storage and transmission between devices
/// 
/// A Chord can be made by setting each variable in it manually. The
/// following example is an `Amaj7` chord
/// ```
/// use chord_companion::*;
/// 
/// let chord = Chord {
///     root: 69,
///     inversion: 0,
///     chord_type: ChordType::Major,
///     coloring: Coloring::Seventh
/// };
/// ```
/// The type of the data produced by serializing a Chord object into the
/// postcard standard
pub type SerializedData = Vec<u8, 4>;

/// 
/// Or by using the `ChordBuilder` type
/// ```
/// use chord_companion::*;
/// 
/// let chord: Chord = ChordBuilder::default()
///     .root(69)
///     .coloring(Coloring::Seventh)
///     .build()
///     .unwrap();
/// ```
/// 
/// A Chord can also be serialized and deserialized in order to store
/// it as bytes or to transmit it. This serialization is done by the 
/// [serde](https://docs.rs/serde/latest/serde/index.html) framework,
/// and is serialized to the
/// [postcard](https://docs.rs/postcard/latest/postcard/) standard
/// ```
/// use chord_companion::*;
/// use postcard::{from_bytes, to_vec};
/// 
/// let chord: Chord = ChordBuilder::default()
///     .root(0)
///     .build()
///     .unwrap();
/// 
/// let res: SerializedData = to_vec(&chord).unwrap();
/// assert_eq!(res, &[0, 0, 0, 0]);
/// assert_eq!(chord, from_bytes(&res).unwrap());
/// ```
#[derive(Builder, Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct Chord {
    /// Root note of the chord, described by the [MIDI numbering
    /// system](https://computermusicresource.com/midikeys.html)
    pub root: u8,
    /// Inversion of the chord going from the lowest note up.
    /// Inversion 0 is the base chord with the root as the lowest
    /// note. Inversion 2 is the base chord with the fifth as
    /// the lowest note, followed by the root and the the third one
    /// octave higher
    #[builder(default = "0")]
    pub inversion: u8,
    /// The type of chord according to the `ChordType` enum
    #[builder(default = "ChordType::Major")]
    pub chord_type: ChordType,
    /// The coloring of the chord according to the `Coloring` enum
    #[builder(default = "Coloring::None")]
    pub coloring: Coloring
}

/// Serialize the audio from a Chord object into a 4 byte vector
impl Chord {
    #[allow(unused)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Chord", 5)?;
        s.serialize_field("root", &self.root)?;
        s.serialize_field("inversion", &self.inversion)?;
        s.serialize_field("chord_type", &self.chord_type)?;
        s.serialize_field("coloring", &self.coloring)?;
        s.end()
    }

    #[allow(unused)]
    pub fn into_serialized_data(&self) -> SerializedData {
        to_vec(self).unwrap()
    }

    #[allow(unused)]
    pub fn from_serialized_data(serialized_data: SerializedData) -> Chord {
        from_bytes(&serialized_data).unwrap()
    }
}
