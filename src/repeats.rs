use std::collections::BTreeMap;
use num_traits::cast::ToPrimitive;
use crate::train::rank_by_score;
use crate::train::rank_by_len;

// For debugging
// const MIN_PATTERN_LEN : usize = 1;

const MIN_PATTERN_LEN : usize = 2;


pub fn train_repeats(inp: &str) -> Vec<(String, usize)> {
    let tbl = raw(inp);

    println!("inp len {} tbl size {}", inp.len(), tbl.len());

    let tbl2 = dedup(tbl);

    let v = rank_by_score(&tbl2);

    v
}

fn rep_cnt(pattern: &str, base: &str) -> usize {
    let plen = pattern.len();

    // println!("p {}, b {}", pattern, base);

    if base.len() < pattern.len() {return 0;}

    (0..base.len()-plen+1).fold(0, |cnt, i| if pattern[0..plen]==base[i..i+plen] {cnt+1} else {cnt} )
}

fn raw(inp: &str) -> BTreeMap<String, usize> {

    let mut dict = BTreeMap::<String, usize>::new();

    for p in 0..inp.len() {
	for b in p+1..inp.len() {
	    for pend in p+MIN_PATTERN_LEN..inp.len() {
		let pattern = &inp[p..pend];
		let reps = rep_cnt(&pattern, &inp[b..]);
		if  reps > 0 {
		    // dedup here
		    dict.entry(pattern.to_string()).or_insert(reps+1);
		}
		else {
		    break;
		}
	    }
	}
    }

    dict
}

fn dedup(tbl: BTreeMap<String, usize>) -> BTreeMap<String, usize> {
    let mut tbl2 = BTreeMap::<String, usize>::new();
    let ord_vec = rank_by_len(&tbl);
    for (s, n) in ord_vec {
	let n2 = tbl2.iter().fold(n as i32, |nn, (k,v)| if k.contains(&s) {nn - *v as i32} else {nn} );
	if n2 > 0 {tbl2.insert(s.to_string(), n2 as usize);}
	else if n2 < 0 {println!("neg cnt {} {}", s, n2)}
    }
    tbl2
}

//#[test]
//fn train_1() {
    //let a = "abcccabb";

    //let v = train_repeats(a);

    //assert_eq!(format!("{:?}", v),
	       //"[(\"b\", 3), (\"c\", 3), (\"a\", 2), (\"ab\", 2), (\"cc\", 2)]"); 
//}


//#[test]
//fn raw_a2() {
    //// Need MIN_PATTERN_LEN=1
    //let a = "aa";

    //let tbl = raw(a);

    //assert_eq!(format!("{:?}", tbl),
	       //"{\"a\": 2}"); 
//}

//#[test]
//fn raw_a3() {
    //let a = "aaa";

    //let tbl = raw(a);

    //// (a, 3)

    //assert_eq!(format!("{:?}", tbl),
	       //"{\"a\": 3, \"aa\": 2}"); 
//}

//// Want a pattern to cross the base (to allow its full size) but counting becomes a bit tricky.
//#[test]
//fn raw_a4() {
    //let a = "aaaa";

    //let tbl = raw(a);

    //// (a, 3), (aa, 2)

    //assert_eq!(format!("{:?}", tbl),
	       //"{\"a\": 4, \"aa\": 3, \"aaa\": 2}"); 
//}

#[test]
fn rc1() {
    let a = "a";
    let b = "b";
    assert_eq!(&a[0..a.len()]==&b[0..b.len()], false);
    assert_eq!((0..10).fold(0, |s,i| if i<3 {s+1} else {s}), 3);
    assert_eq!(rep_cnt("a", "bbbbb"), 0);
    assert_eq!(rep_cnt("abb", "cabcccabb"), 1);
}
