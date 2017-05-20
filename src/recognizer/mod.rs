#![cfg_attr(test, feature(test))]

#[cfg(test)] extern crate test;

pub mod nfa;

use self::nfa::NFA;
use self::nfa::CharacterClass;
use std::collections::BTreeMap;
use std::collections::btree_map;
use std::cmp::Ordering;
use std::ops::Index;



#[derive(Clone)]
struct Metadata {
    statics: u32,
    dynamics: u32,
    stars: u32,
    param_names: Vec<String>,
}

impl Metadata {
    pub fn new() -> Metadata {
        Metadata{ statics: 0, dynamics: 0, stars: 0, param_names: Vec::new() }
    }
}

impl Ord for Metadata {
    fn cmp(&self, other: &Metadata) -> Ordering {
        if self.stars > other.stars {
            Ordering::Less
        } else if self.stars < other.stars {
            Ordering::Greater
        } else if self.dynamics > other.dynamics {
            Ordering::Less
        } else if self.dynamics < other.dynamics {
            Ordering::Greater
        } else if self.statics > other.statics {
            Ordering::Less
        } else if self.statics < other.statics {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for Metadata {
    fn partial_cmp(&self, other: &Metadata) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Metadata {
    fn eq(&self, other: &Metadata) -> bool {
        self.statics == other.statics && self.dynamics == other.dynamics && self.stars == other.stars
    }
}

impl Eq for Metadata {}

#[derive(PartialEq, Clone, Debug)]
pub struct Params {
    map: BTreeMap<String, String>
}

impl Params {
    pub fn new() -> Params {
        Params{ map: BTreeMap::new() }
    }

    pub fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.map.get(key).map(|s| &s[..])
    }

    pub fn iter<'a>(&'a self) -> Iter<'a> {
        Iter(self.map.iter())
    }
}

impl<'a> Index<&'a str> for Params {
    type Output = String;
    fn index(&self, index: &'a str) -> &String {
        match self.map.get(index) {
            None => panic!(format!("params[{}] did not exist", index)),
            Some(s) => s,
        }
    }
}

impl<'a> IntoIterator for &'a Params {
    type IntoIter = Iter<'a>;
    type Item = (&'a str, &'a str);

    fn into_iter(self) -> Iter<'a> {
        self.iter()
    }
}

pub struct Iter<'a>(btree_map::Iter<'a, String, String>);

impl<'a> Iterator for Iter<'a> {
    type Item = (&'a str, &'a str);

    #[inline]
    fn next(&mut self) -> Option<(&'a str, &'a str)> {
        self.0.next().map(|(k, v)| (&**k, &**v))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

pub struct Match<T> {
    pub handler: T,
    pub params: Params
}

impl<T> Match<T> {
    pub fn new(handler: T, params: Params) -> Match<T> {
        Match{ handler: handler, params: params }
    }
}

#[derive(Clone)]
pub struct Router<T> {
    nfa: NFA<Metadata>,
    handlers: BTreeMap<usize, T>
}

impl<T> Router<T> {
    pub fn new() -> Router<T> {
        Router{ nfa: NFA::new(), handlers: BTreeMap::new() }
    }

    pub fn add(&mut self, mut route: &str, dest: T) {
        if route.len() != 0 && route.as_bytes()[0] == b'/' {
            route = &route[1..];
        }

        let nfa = &mut self.nfa;
        let mut state = 0;
        let mut metadata = Metadata::new();

        for (i, segment) in route.split('/').enumerate() {
            if i > 0 { state = nfa.put(state, CharacterClass::valid_char('/')); }

            if segment.len() > 0 && segment.as_bytes()[0] == b':' {
                state = process_dynamic_segment(nfa, state);
                metadata.dynamics += 1;
                metadata.param_names.push(segment[1..].to_string());
            } else if segment.len() > 0 && segment.as_bytes()[0] == b'*' {
                state = process_star_state(nfa, state);
                metadata.stars += 1;
                metadata.param_names.push(segment[1..].to_string());
            } else {
                state = process_static_segment(segment, nfa, state);
                metadata.statics += 1;
            }
        }

        nfa.acceptance(state);
        nfa.metadata(state, metadata);
        self.handlers.insert(state, dest);
    }

    pub fn recognize<'a>(&'a self, mut path: &str) -> Result<Match<&'a T>, String> {
        if path.len() != 0 && path.as_bytes()[0] == b'/' {
            path = &path[1..];
        }

        let nfa = &self.nfa;
        let result = nfa.process(path, |index| nfa.get(index).metadata.as_ref().unwrap());

        match result {
            Ok(nfa_match) => {
                let mut map = Params::new();
                let state = &nfa.get(nfa_match.state);
                let metadata = state.metadata.as_ref().unwrap();
                let param_names = metadata.param_names.clone();

                for (i, capture) in nfa_match.captures.iter().enumerate() {
                    map.insert(param_names[i].to_string(), capture.to_string());
                }

                let handler = self.handlers.get(&nfa_match.state).unwrap();
                Ok(Match::new(handler, map))
            },
            Err(str) => Err(str)
        }
    }
}

fn process_static_segment<T>(segment: &str, nfa: &mut NFA<T>, mut state: usize) -> usize {
    for char in segment.chars() {
        state = nfa.put(state, CharacterClass::valid_char(char));
    }

    state
}

fn process_dynamic_segment<T>(nfa: &mut NFA<T>, mut state: usize) -> usize {
    state = nfa.put(state, CharacterClass::invalid_char('/'));
    nfa.put_state(state, state);
    nfa.start_capture(state);
    nfa.end_capture(state);

    state
}

fn process_star_state<T>(nfa: &mut NFA<T>, mut state: usize) -> usize {
    state = nfa.put(state, CharacterClass::any());
    nfa.put_state(state, state);
    nfa.start_capture(state);
    nfa.end_capture(state);

    state
}

#[test]
fn basic_router() {
    let mut router = Router::new();

    router.add("/thomas", "Thomas".to_string());
    router.add("/tom", "Tom".to_string());
    router.add("/wycats", "Yehuda".to_string());

    let m = router.recognize("/thomas").unwrap();

    assert_eq!(*m.handler, "Thomas".to_string());
    assert_eq!(m.params, Params::new());
}

#[test]
fn root_router() {
    let mut router = Router::new();
    router.add("/", 10);
    assert_eq!(*router.recognize("/").unwrap().handler, 10)
}

#[test]
fn empty_path() {
  let mut router = Router::new();
  router.add("/", 12);
  assert_eq!(*router.recognize("").unwrap().handler, 12)
}

#[test]
fn empty_route() {
  let mut router = Router::new();
  router.add("", 12);
  assert_eq!(*router.recognize("/").unwrap().handler, 12)
}

#[test]
fn ambiguous_router() {
    let mut router = Router::new();

    router.add("/posts/new", "new".to_string());
    router.add("/posts/:id", "id".to_string());

    let id = router.recognize("/posts/1").unwrap();

    assert_eq!(*id.handler, "id".to_string());
    assert_eq!(id.params, params("id", "1"));

    let new = router.recognize("/posts/new").unwrap();
    assert_eq!(*new.handler, "new".to_string());
    assert_eq!(new.params, Params::new());
}

#[test]
fn ambiguous_router_b() {
    let mut router = Router::new();

    router.add("/posts/:id", "id".to_string());
    router.add("/posts/new", "new".to_string());

    let id = router.recognize("/posts/1").unwrap();

    assert_eq!(*id.handler, "id".to_string());
    assert_eq!(id.params, params("id", "1"));

    let new = router.recognize("/posts/new").unwrap();
    assert_eq!(*new.handler, "new".to_string());
    assert_eq!(new.params, Params::new());
}

#[test]
fn multiple_params() {
    let mut router = Router::new();

    router.add("/posts/:post_id/comments/:id", "comment".to_string());
    router.add("/posts/:post_id/comments", "comments".to_string());

    let com = router.recognize("/posts/12/comments/100").unwrap();
    let coms = router.recognize("/posts/12/comments").unwrap();

    assert_eq!(*com.handler, "comment".to_string());
    assert_eq!(com.params, two_params("post_id", "12", "id", "100"));

    assert_eq!(*coms.handler, "comments".to_string());
    assert_eq!(coms.params, params("post_id", "12"));
    assert_eq!(coms.params["post_id"], "12".to_string());
}

#[test]
fn star() {
    let mut router = Router::new();

    router.add("*foo", "test".to_string());
    router.add("/bar/*foo", "test2".to_string());

    let m = router.recognize("/test").unwrap();
    assert_eq!(*m.handler, "test".to_string());
    assert_eq!(m.params, params("foo", "test"));

    let m = router.recognize("/foo/bar").unwrap();
    assert_eq!(*m.handler, "test".to_string());
    assert_eq!(m.params, params("foo", "foo/bar"));

    let m = router.recognize("/bar/foo").unwrap();
    assert_eq!(*m.handler, "test".to_string());
    assert_eq!(m.params, params("foo", "bar/foo"));
}

#[bench]
fn benchmark(b: &mut test::Bencher) {
    let mut router = Router::new();
    router.add("/posts/:post_id/comments/:id", "comment".to_string());
    router.add("/posts/:post_id/comments", "comments".to_string());
    router.add("/posts/:post_id", "post".to_string());
    router.add("/posts", "posts".to_string());
    router.add("/comments", "comments2".to_string());
    router.add("/comments/:id", "comment2".to_string());

    b.iter(|| {
        router.recognize("/posts/100/comments/200")
    });
}

#[allow(dead_code)]
fn params(key: &str, val: &str) -> Params {
    let mut map = Params::new();
    map.insert(key.to_string(), val.to_string());
    map
}

#[allow(dead_code)]
fn two_params(k1: &str, v1: &str, k2: &str, v2: &str) -> Params {
    let mut map = Params::new();
    map.insert(k1.to_string(), v1.to_string());
    map.insert(k2.to_string(), v2.to_string());
    map
}
