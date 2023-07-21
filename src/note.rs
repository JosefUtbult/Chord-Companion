use heapless::Vec;

use crate::chord::*;

pub type Notes = Vec<u8, 8>;

/// Create a `Notes` object from a `Chord` object, containing each
/// MIDI-note in a chord described by the `Chord` object
pub fn generate_notes_from_chord(chord: Chord) -> Notes {
    let mut notes: Notes = Notes::new();

    macro_rules! push_note {
        ($interval:tt) => {
            notes.push(chord.root + $interval).unwrap();
        };
    }

    // Add root note
    push_note!(0);

    // Apply third
    if chord.chord_type == ChordType::Minor {
        push_note!(3);
    }
    else {
        push_note!(4);
    }

    // Apply fifth
    push_note!(7);

    // Add further coloring
    if chord.coloring == Coloring::Sixth {
        push_note!(9);
    }
    
    else if chord.coloring == Coloring::Seventh ||
        chord.coloring == Coloring::Ninth ||
        chord.coloring == Coloring::Eleventh {
        if chord.chord_type == ChordType::Major {
            push_note!(11);
        }
        else {
            push_note!(10);
        }
    }

    if chord.coloring == Coloring::Ninth ||
        chord.coloring == Coloring::Eleventh {
        push_note!(14);
    }

    if chord.coloring == Coloring::Eleventh {
        push_note!(17);
    }

    // Invert the chord structure
    for _i in 0..chord.inversion {
        let note = notes.swap_remove(0) + 12;
        push_note!(note);
    } 

    notes
}