//! A module with methods for `Country` and other associated functions and interfaces.

use std::fmt;

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

impl fmt::Debug for Country {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "Country: {{code: {}, name: {}}}", self.code, self.name);

    }
}
