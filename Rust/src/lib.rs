#![allow(non_snake_case)]
//#![allow(unused)]

// use std::path::Path;
// use std::fs::File;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

// use lazy_static::lazy_static;

use serde::Deserialize;
// use serde::Deserialize;
// use serde::de::Error;
//use serde_json::{Map, Value};

#[derive(Debug, Deserialize)]
struct RawMarkerEntry {
    compulsoryFlag: bool,
    level: String,
    highestNumberSuffix: String,
    nestsFlag: bool,
    hasContent: String,
    occursIn: String,
    printedFlag: bool,
    closed: String,
    deprecatedFlag: bool,
    description: Option<String>,
    nameEnglish: String,
}
#[derive(Debug, Deserialize)]
struct RawUSFMMarkers {
    rawMarkerDict: HashMap<String, RawMarkerEntry>,
}

#[derive(Debug, Clone)]
struct MarkerEntry {
    compulsoryFlag: bool,
    level: String,
    highestNumberSuffix: Option<u8>,
    nestsFlag: bool,
    hasContent: String,
    occursIn: String,
    printedFlag: bool,
    closed: String,
    deprecatedFlag: bool,
    description: Option<String>,
    nameEnglish: String,
}
#[derive(Debug, Clone)]
// This is public so we can return it
pub struct USFMMarkers {
    markerDict: HashMap<String, MarkerEntry>,
    conversionDict: HashMap<String, String>,
}

impl USFMMarkers {
    pub fn marker_to_standard_marker(
        &self,
        original_marker: &str,
    ) -> String {
        // Returns a standard marker, i.e., s->s1, q->q1, etc.
        // if ! &self.USFMNumberDict.contains_key(usfm_num_str) {

        // if original_marker in self.__DataDict['conversionDict']: return self.__DataDict['conversionDict'][marker]
        // #else
        // if marker in self.__DataDict['combinedMarkerDict']: return marker
        // #else must be something wrong
        // raise KeyError
        // Ok(original_marker.to_string())
        if self.conversionDict.contains_key(original_marker) {return self.conversionDict[original_marker].to_string();} else {return original_marker.to_string();};
    }
}

#[cfg(test)]
mod tests {
    #[test]

    fn it_works() {
        use crate::{load_from_json, USFMMarkers};
        // let data_folderpath = Path::new("/home/robert/Programming/WebDevelopment/OpenScriptures/BibleOrgSys/BibleOrgSys/DataFiles/");
        let data_folderpath = String::from("/srv/Documents/FreelyGiven/BibleOrgSys/BibleOrgSys/DataFiles/");
        let usfm_markers: USFMMarkers = load_from_json(&data_folderpath).unwrap();
        assert_eq!(usfm_markers.markerDict["c"].nameEnglish, "Chapter numberr");
    }
}

/*
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
*/

pub fn get_paragraph_markers() -> Vec<String> {
    let paragraph_markers: Vec<_> = vec!["p", "q1", "q2", "q3", "q4"]
            .into_iter()
            .map(String::from)
            .collect();
    paragraph_markers
}


pub fn load_from_json(data_folderpath: &String) -> Result<USFMMarkers, Box<dyn Error>> {
    println!("  In bos_usfm_markers::load_from_json()…");

    let filepath = data_folderpath.clone() + "DerivedFiles/USFM3Markers_Tables.json";
    // let the_file = File::open(filepath)?;
    let mut owned_string: String = "Something went wrong reading the USFM markers file: ".to_owned();
    owned_string.push_str(&filepath);
    let the_file_contents = fs::read_to_string(filepath).expect(&owned_string);
    let raw_usfm_markers: RawUSFMMarkers =
        serde_json::from_str(&the_file_contents).expect("USFM markers JSON was not well-formatted");
    // print_type_of(&parsed_json); // serde_json::value::Value
    // print_type_of(&parsed_json["allAbbreviationsDict"]); // serde_json::value::Value

    let mut markerDict = HashMap::new();
    let mut conversionDict: HashMap<String, String> = HashMap::new();
    for (marker, rawMarkerEntry) in raw_usfm_markers.rawMarkerDict {
        // println!("{} / {:?}", marker, rawMarkerEntry);
        let markerEntry = MarkerEntry {
            compulsoryFlag: rawMarkerEntry.compulsoryFlag,
            level: rawMarkerEntry.level,
            highestNumberSuffix: if rawMarkerEntry.highestNumberSuffix == "None" {
                None
            } else {
                Some(rawMarkerEntry.highestNumberSuffix.parse::<u8>().unwrap())
            },
            nestsFlag: rawMarkerEntry.nestsFlag,
            hasContent: rawMarkerEntry.hasContent,
            occursIn: rawMarkerEntry.occursIn,
            printedFlag: rawMarkerEntry.printedFlag,
            closed: rawMarkerEntry.closed,
            deprecatedFlag: rawMarkerEntry.deprecatedFlag,
            description: rawMarkerEntry.description,
            nameEnglish: rawMarkerEntry.nameEnglish,
        };
        markerDict.insert(marker.clone(), markerEntry);
        if rawMarkerEntry.highestNumberSuffix != "None" {
            // We have some extra work to do
            if marker.ends_with("-s") || marker.ends_with("-e") {
                //assert marker in ("qt-s","qt-e") // Only ones we know of so far
                // Numerical suffix can't just be appended to the end of these
                let len = marker.len();
                conversionDict.insert(marker.clone(), format!("{}1{}", &marker[..len-2], &marker[len-2..]));
            } else {
                // not a milestone start/end marker
                conversionDict.insert(marker.clone(), marker + "1");
            }
        }
    }

    // Display a few results to the user
    println!(
        "    Loaded markers data for {:?} USFM markers.",
        markerDict.len()
    );
    println!(
        "      Marker 's' is {:?}: {:?}",
        markerDict["s"].nameEnglish, markerDict["s"].description,
    );
    println!("    Conversion dict is {:?}", conversionDict);

    Ok(USFMMarkers {
        markerDict: markerDict,
        conversionDict: conversionDict,
    })
}
