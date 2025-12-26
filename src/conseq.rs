/// Adapted from https://codereview.stackexchange.com/questions/212568/find-the-longest-common-sequence-of-two-strings-in-rust

use criterion::Criterion;

pub fn longest_match(s1: &str, s2: &str) -> Option<String> {
    let m1 = longest_s2_in_s1(s1, s2);
    let m2 = longest_s2_in_s1(s2, s1);

    match (m1, m2) {
	(None, None) => None,
	(Some(s1), None) => Some(s1),
	(None, Some(s2)) => Some(s2),
	(Some(s1), Some(s2)) => if s1.len() >= s2.len() {Some(s1)} else {Some(s2)}
    }
}


/// Longest match of s2 in s1
fn longest_s2_in_s1(s1: &str, s2: &str) -> Option<String> {    
    let mut max: Option<String> = None; 
    let mut current = String::new(); 
    let mut s1_outer = s1.chars().peekable();

    while s1_outer.peek() != None {

	current.clear();

	let mut s1_inner = s1_outer.clone();
	let mut s2_iter = s2.chars(); 

	while s1_inner.peek() != None {
            if let Some(s1_char) = s1_inner.next() {
		match s2_iter.next() {
                    Some(s2_char) if s1_char == s2_char => 
			current.push(s1_char),
		    _ => break,
		}
	    }
	}
	
	if !current.is_empty() && max.as_ref().map_or(true, |s| s.len() < current.len()) {
            max = Some(current.clone());
        }

	s1_outer.next();
    }

    max

    // max.unwrap_or_default()
}

fn criterion_benchmark(c: &mut Criterion) {
    let s1 = "GXTKAYB";
    let s2 = "TKAB";
    c.bench_function("Benchmark", move |b| b.iter(|| longest_match(s1, s2)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

#[test]
fn t1() {
    assert_eq!(longest_match("BABAZDC", "BAZBAD"), Some("BAZ".to_string()));
}

#[test]
fn t2() {
    assert_eq!(longest_match("AGGTAB", "GTAYB"), Some("GTA".to_string()));
}

#[test]
fn t3() {
    assert_eq!(longest_match("aaaa", "aa"), Some("aa".to_string()));
}

#[test]
fn t4() {
    assert_eq!(longest_match("ABBA", "ABBA"), Some("ABBA".to_string()));
}

#[test]
fn t5() {
    assert_eq!(longest_match("ABCD", "EF"), None);
}

#[test]
fn t6() {
    assert_eq!(longest_match("a", "a"), Some("a".to_string()));
}

#[test]
fn t7() {
    assert_eq!(longest_match("cabcabb", "abb"), Some("abb".to_string()));
}

#[test]
fn t8() {
    assert_eq!(longest_match("abb", "cabcabb"), Some("abb".to_string()));
}
