use std::collections::HashMap;
use std::io::{Error, Result, BufReader, BufRead};
use std::fs::File;

/// Represents a country on earth that is recognised in the icao codes.
pub struct Country {
    /// The [icao code](http://www.avcodes.co.uk/icaonat.asp) of the country
    pub code: String,
    /// The name of the country
    pub name: String,
}

impl Country {
    /// Constructor for the `Country` struct.
    ///
    /// # Examples
    ///
    /// ```
    /// # use oldnav_lib::navdata::country::Country;
    /// let new_country = Country::new("AG", "Solomon Islands");
    /// assert_eq!(new_country.code, "AG".to_string());
    /// assert_eq!(new_country.name, "Solomon Islands".to_string());
    /// ```
    pub fn new<S: Into<String>>(code: S, name: S) -> Country {
        return Country {
            code: code.into(),
            name: name.into(),
        };
    }
}


/// Read countries in from a txt file.
///
/// Basically it just maps ICAO codes to the country names,
/// see icao_countries.txt for an example file format.
///
/// # Examples
///
/// ```rust,no_run
/// # use oldnav_lib::navdata::country::{Country, read_countries};
/// # use std::collections::HashMap;
/// let countries_map = read_countries("some/path/to/icao_countries.txt").unwrap();
/// let country = countries_map.get("AG").unwrap();
/// println!("Code: {}, Name: {}", country.code, country.name);
///
/// ```
pub fn read_countries(file_path: &str) -> Result<HashMap<String, Country>> {
    let mut cm = HashMap::new();

    let f: File = match File::open(file_path) {
        Ok(v) => v,
        Err(e) => {
            return Err(Error::new(e.kind(),
                                  format!("Unable to open countries map file: {}", file_path)))
        }
    };

    let bf = BufReader::new(&f);

    for line in bf.lines() {
        let l = try!(line);
        let split: Vec<&str> = l.split("\t").collect();
        let country_code = split[0].to_string();
        let country_name = split[1].to_string();

        cm.insert(country_code.clone(),
                  Country::new(country_code, country_name));
    }

    return Ok(cm);
}
