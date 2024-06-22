use std::fs;

use anyhow::Error;
use colored::Colorize;
use serde_json::Value;

use crate::{CsvOpts, FileFormat};

pub fn csv2file(opts: CsvOpts) -> Result<String, Error> {
    let output = match opts.output {
        Some(output) => output,
        None => {
            let output = format!("output.{}", opts.format);
            output
        }
    };
    println!("{} {}", "Output file: ".blue(), output.blue());
    let mut rdr = csv::Reader::from_path(opts.input)?;
    let mut ret = Vec::with_capacity(128);
    let headers = rdr.headers()?.clone();

    for result in rdr.records() {
        let record = result?;
        // header.iter()  使用headers的迭代器
        // record.iter()  使用record的迭代器
        // zip()  将两个迭代器合并成一个元组
        // collect::<Value>()  将元组转换为Value类型
        let player = headers.iter().zip(record.iter()).collect::<Value>();
        ret.push(player);
    }

    let output_str: String = match opts.format {
        FileFormat::Json => {
            let json = serde_json::to_string_pretty(&ret)?;
            fs::write(output, &json)?;
            json.clone()
        }
        FileFormat::Yaml => {
            let yaml = serde_yaml::to_string(&ret)?;
            fs::write(output, &yaml)?;
            yaml.clone()
        }
    };

    Ok(output_str)
}
