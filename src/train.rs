use std::collections::BTreeMap;
use std::collections::HashSet;
use num_traits::cast::ToPrimitive;
use crate::conseq::longest_match;

pub fn train(inp: &str) -> Vec<(String, usize)> {
    let mut tbl = raw(inp);

    let v = rank_by_score(&tbl);

    v
}

pub fn rank_by_score(tbl: &BTreeMap<String, usize>) -> Vec<(String, usize)> {
    let mut v = tbl.iter().map(|(s,n)| (s.to_string(), *n)).collect::<Vec<_>>();
    v.sort_by_key(|a| -
		  ((a.1 as f32) * (a.0.len() as f32).sqrt()).to_isize().unwrap() );
    v
}

pub fn rank_by_len(tbl: &BTreeMap<String, usize>) -> Vec<(String, usize)> {
    let mut v = tbl.iter().map(|(s,n)| (s.to_string(), *n)).collect::<Vec<_>>();
    v.sort_by_key(|a| - (a.0.len() as isize));
    v
}

fn combine(tbl: &mut BTreeMap<String, usize>, ord: Vec<(String, usize)>) {

    let mut zeros = HashSet::<String>::new();
    for (s, n) in ord {
	if zeros.get(&s) != None {continue;}
	// let n = *tbl.get(&s).unwrap();
	for b in 1..s.len() {
	    let sub = &s[b..];
	    tbl.entry(sub.to_string()).and_modify(|c| {*c -= n} );
	    if tbl.get(sub)==Some(&0) {
		zeros.insert(sub.to_string());
	    }
	}
    }

    for s in zeros {
	tbl.remove(&s);
    }
}

pub fn raw(inp: &str) -> BTreeMap<String, usize> {

    let mut dict = BTreeMap::<String, usize>::new();
	
    for b in 1..inp.len() {
	if let Some(s) = longest_match( &inp[..b], &inp[b..] ) {
	    //println!("sent {} and {} and got {}", &inp[..b], &inp[b..], s);
	    dict.entry(s).and_modify(|c| {*c += 1} ).or_insert(1);
	}
    }

    dict
}



#[test]
fn raw_1() {
    let a = "aa";

    let tbl = raw(a);

    assert_eq!(format!("{:?}", tbl),
	       "{\"a\": 1}"); 
}

#[test]
fn raw_2() {
    let a = "aaaa";

    let tbl = raw(a);

    // (a, 2), (aa, 1)

    assert_eq!(format!("{:?}", tbl),
	       "{\"a\": 2, \"aa\": 1}"); 
}

#[test]
fn train_1() {
    let a = "abab";

    let v = train(a);

    // (a, 1), (ab, 1), (b, 1)

    assert_eq!(format!("{:?}", v),
	       "[(\"a\", 1), (\"ab\", 1), (\"b\", 1)]"); 
}
