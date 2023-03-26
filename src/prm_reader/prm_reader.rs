use std::io::{BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::process;

pub fn read() -> HashMap<String,String> { 
    
    let mut params = HashMap::new();

    let f = File::open(r#"D:\prm\cnfg.txt"#).unwrap_or_else(|err| {
        println!("Unable to open paramter file due the error: {}",err);
        process::exit(1);
    });
    let f = BufReader::new(f);

    for line in f.lines() {
        let  temp = line.expect("Unable to read the elements from prm file").to_string();
        let mut splitter=temp.splitn(2, '=');
        let first = splitter.next().expect("Unable to split the key from file").trim().to_string();
        let second = splitter.next().expect("Unable to split the value from file").trim().to_string();
        params.insert(
        first,
        second,
         );
    }
    return params;

}

pub fn find_value(a:&HashMap<String,String>,b:String)-> String{
    a.get(&b).expect("Unable to find the Value from Hash map").to_string()
}