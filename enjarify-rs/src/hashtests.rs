// Copyright 2016 Google Inc. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// use std::io::prelude::*;
// use std::fs::File;

extern crate crypto;
use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;

use rayon::prelude::*;

use strings::*;
use jvm::optimization::options::Options;
use super::{read, translate};

fn hash(s: &bstr) -> BString {
    let mut res = vec![0; 32];
    let mut h = Sha256::new();
    h.input(s);
    h.result(&mut res);
    res
}

fn hexdigest(s: &bstr) -> String {
    let mut h = Sha256::new();
    h.input(s);
    h.result_str()
}

pub fn main() {
    let testfiles = (1..8).map(|test| read(format!("../tests/test{}/classes.dex", test))).collect::<Vec<_>>();

    let outputs: Vec<Vec<(BString, String)>> = (0..(7*256)).into_par_iter().map(|ind| {
        let ti = (ind / 256) as usize;
        let bits = ind % 256;
        let dexes = &testfiles[ti..ti+1];

        let results = translate(Options::from(bits as u8), dexes);
        let output = results.into_iter().map(|(_, res)| {
            let cls = res.unwrap();
            let digest = hexdigest(&cls);
            (cls, digest)
        }).collect();
        output
    }).collect();

    let mut fullhash = vec![];
    for (ind, pairs) in outputs.into_iter().enumerate() {
        let bits = ind % 256;
        if bits==0 {println!("test{}", (ind/256)+1);}

        for (cls, digest) in pairs {
            println!("{:08b} {}", bits, digest);
            fullhash.extend(cls);
            fullhash = hash(&fullhash);
        }
    }
    println!("done!\nFinal hash: {}", hexdigest(&fullhash));
}
