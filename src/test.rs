use super::*;

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
        let mut correct = Notes::new();
        correct.extend_from_slice(&[0, 4, 7]).unwrap();
        assert_eq!(res, correct);
    }

    #[test]
    fn check_minor() {
        let chord: Chord = ChordBuilder::default()
            .root(0)
            .chord_type(ChordType::Minor)
            .build()
            .unwrap();

        let res = generate_notes_from_chord(chord);
        // C(-1) minor chord
        let mut correct = Notes::new();
        correct.extend_from_slice(&[0, 3, 7]).unwrap();
        assert_eq!(res, correct);
    }
   
    #[test]
    fn check_sixth() {
        let chord: Chord = ChordBuilder::default()
            .root(0)
            .coloring(Coloring::Sixth)
            .build()
            .unwrap();

        let res = generate_notes_from_chord(chord);
        // C(-1) major chord
        let mut correct = Notes::new();
        correct.extend_from_slice(&[0, 4, 7, 9]).unwrap();
        assert_eq!(res, correct);
    }

    #[test]
    fn check_minor_sixth() {
        let chord: Chord = ChordBuilder::default()
            .root(0)
            .chord_type(ChordType::Minor)
            .coloring(Coloring::Sixth)
            .build()
            .unwrap();

        let res = generate_notes_from_chord(chord);
        // C(-1) minor sixth chord
        let mut correct = Notes::new();
        correct.extend_from_slice(&[0, 3, 7, 9]).unwrap();
        assert_eq!(res, correct);
    } 

    #[test]
    fn check_dominant_seven() {
        let chord: Chord = ChordBuilder::default()
            .root(0)
            .chord_type(ChordType::Dominant)
            .coloring(Coloring::Seventh)
            .build()
            .unwrap();

        let res = generate_notes_from_chord(chord);
        // A major chord
        let mut correct = Notes::new();
        correct.extend_from_slice(&[0, 4, 7, 10]).unwrap();
        assert_eq!(res, correct);
    }

    #[test]
    fn check_minor_seventh() {
        let chord: Chord = ChordBuilder::default()
            .root(0)
            .chord_type(ChordType::Minor)
            .coloring(Coloring::Seventh)
            .build()
            .unwrap();

        let res = generate_notes_from_chord(chord);
        // A major chord
        let mut correct = Notes::new();
        correct.extend_from_slice(&[0, 3, 7, 10]).unwrap();
        assert_eq!(res, correct);
    }

    #[test]
    fn check_maj_seven() {
        let chord: Chord = ChordBuilder::default()
            .root(0)
            .coloring(Coloring::Seventh)
            .build()
            .unwrap();

        let res = generate_notes_from_chord(chord);
        // A major chord
        let mut correct = Notes::new();
        correct.extend_from_slice(&[0, 4, 7, 11]).unwrap();
        assert_eq!(res, correct);
    }

    #[test]
    fn check_ninth() {
        let chord: Chord = ChordBuilder::default()
            .root(0)
            .coloring(Coloring::Ninth)
            .build()
            .unwrap();

        let res = generate_notes_from_chord(chord);
        // A major chord
        let mut correct = Notes::new();
        correct.extend_from_slice(&[0, 4, 7, 11, 14]).unwrap();
        assert_eq!(res, correct);
    }

    #[test]
    fn check_eleventh() {
        let chord: Chord = ChordBuilder::default()
            .root(0)
            .coloring(Coloring::Eleventh)
            .build()
            .unwrap();

        let res = generate_notes_from_chord(chord);
        // A major chord
        let mut correct = Notes::new();
        correct.extend_from_slice(&[0, 4, 7, 11, 14, 17]).unwrap();
        assert_eq!(res, correct);
    }

    #[test]
    fn check_serialization() {
        let chord: Chord = ChordBuilder::default()
            .root(0)
            .build()
            .unwrap();

        let res = chord.into_serialized_data();
        assert_eq!(res, &[0, 0, 0, 0]);
    }

    #[test]
    fn check_deserialization() {
        let mut buf = SerializedData::new();
        let _ = buf.extend_from_slice(&[0, 0, 0, 0]); 

        let res = Chord::from_serialized_data(buf);
        assert_eq!(res, ChordBuilder::default()
            .root(0)
            .build()
            .unwrap())
    }

    #[test]
    fn check_serialization_deserialization() {
        let chord: Chord = ChordBuilder::default()
            .root(0)
            .build()
            .unwrap();
        
        let res: SerializedData = chord.into_serialized_data(); 
        assert_eq!(chord, Chord::from_serialized_data(res));
    }
}
