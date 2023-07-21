use chord_companion::*;
use heapless::Vec;
use postcard::{from_bytes, to_vec};

fn main() {
    println!("Init");

    let chord: Chord = ChordBuilder::default()
        .root(69)
        .chord_type(ChordType::Minor)
        .build()
        .unwrap();

    let res: Vec<u8, 5> = to_vec(&chord).unwrap();
    println!("{:?}", res);

    let chord_rebuilt: Chord = from_bytes(&res).unwrap();
    println!("{:?}", chord_rebuilt);
}
