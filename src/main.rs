use std::io::{BufRead,BufReader};
use std::fs::File;
use std::collections::HashMap;
use stardata::{Star, Equatorial};
use stardata::Name;
use stardata::Constellation;



fn main(){
    let finename = "./hip_constellation_line_star.csv";
    let file = File::open(finename).expect(&format!("Cannot Open {}",finename));
    let buffered_reader = BufReader::new(file);
    let stars : HashMap<usize,Star> = buffered_reader.lines().map(|line|{
        let line_str = line.expect("read error");
        let v:Vec<&str> = line_str.split_terminator(',').collect();
        let hip         = v[0].parse::<usize>().expect("parse_error");
        let r_hour      = v[1].parse::<usize>().expect("parse_error");
        let r_minuite   = v[2].parse::<usize>().expect("parse_error");
        let r_second    = v[3].parse::<f32>().expect("parse_error");
        let d_degree    = v[4].parse::<isize>().expect("parse_error");
        let north;
        let d_degree = if d_degree < 0 {
            north = false;
            -d_degree as usize
        }else{
            north = true;
            d_degree as usize
        };
        let d_minuite   = v[5].parse::<usize>().expect("parse_error");
        let d_second    = v[6].parse::<f32>().expect("parse_error");
        let magnitude   = v[7].parse::<f32>().expect("parse_error");
        (hip,Star::new(hip,Equatorial::new(r_hour,r_minuite,r_second,north,d_degree,d_minuite,d_second),magnitude,Name::default()))
    }).collect();

    let mut constellations: HashMap<String,Constellation> = HashMap::new();

    for wrapped_line in BufReader::new(File::open("./hip_constellation_line.csv").expect("Cannot Open file")).lines(){
        let line = wrapped_line.unwrap();
        let v:Vec<&str> = line.split_terminator(',').collect();
        let abb_name = v[0].to_owned();
        let start   = stars.get(&v[1].parse::<usize>().expect("parse error")).unwrap();
        let end     = stars.get(&v[2].parse::<usize>().expect("parse error")).unwrap();
        match constellations.get_mut(&abb_name){
            Some(constellation) => {
                constellation.add(start,end);
            },
            None => {
                let mut new_constellation = Constellation::new(Name::new(Some(abb_name.clone()),None));
                new_constellation.add(start,end);
                constellations.insert(abb_name, new_constellation);
            }
        }
    }
    
    println!("{:?}",constellations.get("Ori").unwrap());
}