use serde::{Deserialize, Serialize};
use std::io;
use std::io::Stdout;
use csv::Writer;
use structopt::StructOpt;

const TRAIT_EYE: &str = "01 _ Eye";
const TRAIT_DECORATION: &str = "02 _ Decoration";
const TRAIT_ARMS: &str = "03 _ Arms";
const TRAIT_LEGS: &str = "04 _ Legs";
const TRAIT_BODY: &str = "05 _ Body";
const TRAIT_CHIP: &str = "06 _ Chip";

#[derive(StructOpt)]
struct Cli {
    /// Input format type: JSON or CSV
    #[structopt(short, long, default_value="JSON")]
    input: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Attribute {
    trait_type: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Metadata {
    name: String,
    description: String,
    edition: u32,
    attributes: Vec<Attribute>
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Amare {
    name: String,
    description: String,
    edition: u32,
    eye: String,
    decoration: String,
    arms: String,
    legs: String,
    body: String,
    chip: String,
}

fn main() {
    let args: Cli = Cli::from_args();
    let mut input = String::new();

    if args.input == "CSV" {
        let mut rdr = csv::Reader::from_reader(io::stdin());
        let mut mds: Vec<Metadata> = Vec::new();
        for result in rdr.deserialize() {
            let record: Amare = result.unwrap();
            let md = amare_to_metadata(&record);
            mds.push(md);
        }
        mds.sort_by_key(|m| m.edition);
        println!("{}", serde_json::to_string(&mds).unwrap());
    } else {
        let mut wtr = csv::Writer::from_writer(io::stdout());
        let mut mds: Vec<Metadata> = serde_json::from_reader(io::stdin()).unwrap();
        mds.sort_by_key(|m| m.edition);

        for md in mds {
            let amare = metadata_to_amare(&md);
            wtr.serialize(amare).unwrap();
            String::clear(&mut input);
        }
    }
}

fn metadata_to_amare(v: &Metadata) -> Amare {
    let mut a = Amare::default();

    for attr in &v.attributes {
        match attr.trait_type.as_ref() {
            TRAIT_EYE => a.eye = attr.value.to_string(),
            TRAIT_DECORATION => a.decoration = attr.value.to_string(),
            TRAIT_ARMS => a.arms = attr.value.to_string(),
            TRAIT_LEGS => a.legs = attr.value.to_string(),
            TRAIT_BODY => a.body = attr.value.to_string(),
            TRAIT_CHIP => a.chip = attr.value.to_string(),
            _ => {}
        }
    }

    a.name = v.name.to_owned();
    a.description = v.description.to_owned();
    a.edition = v.edition;

    a
}

fn amare_to_metadata(v: &Amare) -> Metadata {
    Metadata {
        name: v.name.to_string(),
        description: v.description.to_string(),
        edition: v.edition,
        attributes: vec![
            Attribute{ trait_type: TRAIT_EYE.to_string(), value: v.eye.to_string()},
            Attribute{ trait_type: TRAIT_DECORATION.to_string(), value: v.decoration.to_string()},
            Attribute{ trait_type: TRAIT_ARMS.to_string(), value: v.arms.to_string()},
            Attribute{ trait_type: TRAIT_LEGS.to_string(), value: v.legs.to_string()},
            Attribute{ trait_type: TRAIT_BODY.to_string(), value: v.body.to_string()},
            Attribute{ trait_type: TRAIT_CHIP.to_string(), value: v.chip.to_string()},
        ]
    }
}
