use derive_builder::Builder;

#[derive(Builder)]
pub struct Chord {
    pub root: u16,
    #[builder(default = "false")]
    pub minor: bool,
    #[builder(default = "false")]
    pub maj_seven: bool,
    #[builder(default = "false")]
    pub sixth: bool,
    #[builder(default = "false")]
    pub seventh: bool
}

pub type Notes = Vec<u16>;

pub fn generate_notes_from_chord(chord: Chord) -> Notes {
    let mut notes: Notes = Notes::with_capacity(6);
    notes.push(chord.root);
    notes.push(chord.root + 4);
    notes.push(chord.root + 7);

    if chord.minor {
        notes[1] -= 1;
    }

    if chord.sixth {
        notes.push(chord.root + 9);
    }
    else if chord.seventh {
        if chord.minor {
            notes.push(chord.root + 10);
        }
        else {
            notes.push(chord.root + 11);
        }
    }
    else if chord.maj_seven {
        notes.push(chord.root + 10);
    }

    notes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_builder() {
        let chord: Chord = ChordBuilder::default()
            .root(0)
            .build()
            .unwrap();

        assert_eq!(chord.root, 0);
    }

    #[test]
    fn check_major() {
        let chord: Chord = ChordBuilder::default()
            .root(0)
            .build()
            .unwrap();

        let res = generate_notes_from_chord(chord);
        // C(-1) major chord
        let correct = vec![0, 4, 7];
        assert_eq!(res, correct);
    }

    #[test]
    fn check_minor() {
        let chord: Chord = ChordBuilder::default()
            .root(0)
            .minor(true)
            .build()
            .unwrap();

        let res = generate_notes_from_chord(chord);
        // C(-1) minor chord
        let correct = vec![0, 3, 7];
        assert_eq!(res, correct);
    }
   
    #[test]
    fn check_sixth() {
        let chord: Chord = ChordBuilder::default()
            .root(0)
            .sixth(true)
            .build()
            .unwrap();

        let res = generate_notes_from_chord(chord);
        // C(-1) major chord
        let correct = vec![0, 4, 7, 9];
        assert_eq!(res, correct);
    }

    #[test]
    fn check_minor_sixth() {
        let chord: Chord = ChordBuilder::default()
            .root(0)
            .minor(true)
            .sixth(true)
            .build()
            .unwrap();

        let res = generate_notes_from_chord(chord);
        // C(-1) minor sixth chord
        let correct = vec![0, 3, 7, 9];
        assert_eq!(res, correct);
    } 

    #[test]
    fn check_dominant_seven() {
        let chord: Chord = ChordBuilder::default()
            .root(0)
            .maj_seven(true)
            .build()
            .unwrap();

        let res = generate_notes_from_chord(chord);
        // A major chord
        let correct = vec![0, 4, 7, 10];
        assert_eq!(res, correct);
    }

    #[test]
    fn check_maj_seven() {
        let chord: Chord = ChordBuilder::default()
            .root(0)
            .seventh(true)
            .build()
            .unwrap();

        let res = generate_notes_from_chord(chord);
        // A major chord
        let correct = vec![0, 4, 7, 11];
        assert_eq!(res, correct);
    }
}
