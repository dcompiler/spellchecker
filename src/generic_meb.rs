use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
enum Token<T: Eq> {
	Tok(char),
	EndTok(T),
	End
}

const BMAX : usize = 11;
const _: () = assert!(BMAX <= u16::MAX as usize, "too large");

use std::collections::HashMap;
use std::hash::Hash;

trait Unsgned {}

impl Unsgned for u8 {}
impl Unsgned for u16 {}
impl Unsgned for u32 {}
impl Unsgned for u64 {}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
struct Sash<T: Unsgned>(pub T);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct Meb<T, H> 
where
    T: Eq + Hash,
    H: Unsgned,
{
	prefix: T,
	mem: [Token<T>; BMAX],
	seqs: Vec<(u16, Sash<H>)>,
	succs: HashMap<T, Box<Meb<T, H>>>
}

use toml;

#[cfg(test)]

mod tests {
	use super::*;
	
	#[test]
	fn test_meb_toml() {
		let prefix = 16u32;
		
		//let first_word: [Token<u32>; 10] = std::array::from_fn(|i| {
            //Token::Tok(i as u32)
        //});
        let first_word: [Token<u32>; 4] = "abcd".chars()
						.map(|c| Token::Tok(c)).collect::<Vec<_>>().
						try_into().expect("Wrong length");
		let mut mem = [Token::End; BMAX];
		mem[..4].copy_from_slice(&first_word);
		
        let seqs = vec![
            (0u16, Sash(42u32))
        ];

        let mut succs = HashMap::new();

        let meb = Meb {
            prefix,
            mem,
            seqs,
            succs,
        };
        
        let encoded = toml::to_string(&meb).expect("Meb toml serialize failed");
        println!("{:?}", encoded);
        let decoded: Meb<u32, u32> = toml::from_str(&encoded).expect("Meb toml deserialize failed");

        assert_eq!(meb, decoded);


	}
}
