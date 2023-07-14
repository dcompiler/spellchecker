/// Adapted from https://codereview.stackexchange.com/questions/212568/find-the-longest-common-sequence-of-two-strings-in-rust

use criterion::Criterion;

fn longest_con_seq(s1: &str, s2: &str) -> String {
    let mut max: Option<String> = None; // Holds value of string with maximum length
    let mut current = String::new(); // String container to hold current longest value
    let mut s1_iter = s1.chars().peekable(); // Peekable iterator for string s1
    let mut s2_iter = s2.chars(); //Iterator for string s2
    let mut s2_prev_pos = s2_iter.clone(); // Iterator that holds position of previous location of first iterator
    let mut s1_prev_pos = s1_iter.clone(); // Peekable iterator used to make sure all possible combinations are located.

    loop {
        let s1_char = s1_iter.next(); // Get character in s1

        if current.is_empty() {
            // If no consecutive string found yet store location of iterator
            s1_prev_pos = s1_iter.clone()
        }

        match s1_char {
            Some(s1_char) => loop {
                match s2_iter.next() {
                    Some(s2_char) if s1_char == s2_char => {
                        current.push(s1_char);
                        s2_prev_pos = s2_iter.clone();
                        break;
                    }
                    Some(_) => continue,
                    None => {
                        s2_iter = s2_prev_pos.clone();
                        break;
                    }
                }
            },
            None => match s1_prev_pos.peek() {
                Some(_) => {
                    if max.as_ref().map_or(true, |s| s.len() < current.len()) {
                        max = Some(current.clone());
                    }
                    current.clear();

                    s1_iter = s1_prev_pos.clone();
                    s2_iter = s2.chars();
                }
                None => break,
            },
        }
    }

    max.unwrap_or_default()
}

fn criterion_benchmark(c: &mut Criterion) {
    let s1 = "GXTKAYB";
    let s2 = "AGGTAB";
    c.bench_function("Benchmark", move |b| b.iter(|| longest_con_seq(s1, s2)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

#[test]
fn example_1() {
    assert_eq!(longest_con_seq("ABAZDC", "BACBAD"), "ABAD");
}

#[test]
fn example_2() {
    assert_eq!(longest_con_seq("AGGTAB", "GXTKAYB"), "GTAB");
}

#[test]
fn example_3() {
    assert_eq!(longest_con_seq("aaaa", "aa"), "aa");
}

#[test]
fn example_4() {
    assert_eq!(longest_con_seq("ABBA", "ABJABA"), "ABBA");
}
