use chord_companion::*;

fn main() {
    println!("Init");

    let chord: Chord = ChordBuilder::default()
        .root(69)
        .minor(false)
        .build()
        .unwrap();

    println!("{:?}", generate_notes_from_chord(chord));
}
