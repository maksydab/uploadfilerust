
#[macro_use] 


use std::fs::File;

use std::io::{Read, Error};

use rand::prelude::*;
use rocket::fs::TempFile;

//█████████████████████████████████████████
//█─█─█▄─▄▄─█▄─▄███▄─▄▄─█▄─▄▄─█▄─▄▄▀█─▄▄▄▄█
//█─▄─██─▄█▀██─██▀██─▄▄▄██─▄█▀██─▄─▄█▄▄▄▄─█
//█▄█▄█▄▄▄▄▄█▄▄▄▄▄█▄▄▄███▄▄▄▄▄█▄▄█▄▄█▄▄▄▄▄█



pub fn returnhtml(path:&str) -> String {
    let mut htmlfile = File::open(path).unwrap();
    let mut contents =String::new();
    htmlfile.read_to_string(&mut contents).unwrap();
    contents
}

const PARTSNAME: &[&str] = &[
    "ma","ba","bo","ka","ly","ja","po","pe","bu","wo","ce","we","hi","lo","pi",
    "ra","na","sa","ta","fa","zi","xu","qu","mi","li","do","go","no","ki","shi",
    "chi","phi","pho","plo","bra","cla","dra","tra","qua","sha","tha","sha","rio",
    "neo","zeo","cy","ly","vo","ko","pa","to","ri","mi","la","de","be","pe","jo"
];


pub fn randomid() -> String {
    let mut count=0;
    let mut name =String::new();
    let mut rng = rand::rng();
    loop  {
        
        let part= PARTSNAME[rng.random_range(0..PARTSNAME.len())];
        name.push_str(part);
        println!("{}",name);
        if count == 3 {
            break;
        }
        count += 1;
    }
    let mut bob = name.to_string();
    bob.push_str(&rng.random_range(1..99).to_string());
    bob
}
#[derive(FromForm)]
pub struct Upload<'r> { file: TempFile<'r> }
