use serde_derive::Deserialize;
use std::fmt;

//
// General
//

/// Link icon do display in header
#[derive(Default, Deserialize, Debug, Clone)]
pub struct Hyperlink {
    pub name: String,
    pub target: String,
}

impl Hyperlink {
    pub fn new(name: &str, target: &str) -> Self {
        Self {
            name: name.into(),
            target: target.into(),
        }
    }
}

//
// Resume/CV
//

/// Link icon do display in header
#[derive(Default, Deserialize, Debug)]
pub struct CvLink {
    pub hyperlink: Hyperlink,
    pub icon: String,
}

/// Schema.org locality
#[derive(Default, Deserialize, Debug)]
pub struct Locality {
    pub name: String,
    pub postal_code: String,
    pub state: AddressRegion,
}

/// Schema.org addressRegion
#[derive(Default, Deserialize, Debug)]
pub struct AddressRegion {
    pub abbreviation: String,
    pub full_name: String,
    pub country: String,
}

/// Schema.org Postal Address
#[derive(Default, Deserialize, Debug)]
pub struct Address {
    pub street: String,
    pub line2: Option<String>,
    pub locality: Locality,
}

/// Month
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Month {
    Jan = 1,
    Feb,
    Mar,
    Apr,
    May,
    Jun,
    Jul,
    Aug,
    Sep,
    Oct,
    Nov,
    Dec,
}

impl Default for Month {
    fn default() -> Self {
        Self::Jan
    }
}

impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Date
#[derive(Default, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct MonthYear {
    pub year: i16,
    pub month: Month,
}

/// Type of degree
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum DegreeType {
    Cert,
    BS,
}

impl DegreeType {
    pub fn full_name(&self) -> &str {
        use DegreeType::*;
        match self {
            Cert => "Certificate",
            BS => "Bachelor of Science",
        }
    }
    pub fn abbreviation(&self) -> &str {
        use DegreeType::*;
        match self {
            Cert => "Cert.",
            BS => "B.S.",
        }
    }
}

impl Default for DegreeType {
    fn default() -> Self {
        Self::BS
    }
}

/// Single degree
#[derive(Default, Deserialize, Debug)]
pub struct Degree {
    pub degree_type: DegreeType,
    pub graduation_date: MonthYear,
    pub expected: bool,
    pub gpa: f32,
    pub subject: String,
}

/// Education entry
#[derive(Default, Deserialize, Debug)]
pub struct School {
    pub name: String,
    pub address: Address,
    pub degrees: Vec<Degree>,
}

/// Intro
#[derive(Default, Deserialize, Debug)]
pub struct Intro {
    pub one_liner: String,
    pub about: String,
    pub skills: String,
    pub techs: String,
}

/// CV Header
#[derive(Default, Deserialize, Debug)]
pub struct CvHeader {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub subtitle: String,
    pub links: Vec<CvLink>,
    pub address: Address,
}

impl CvHeader {
    pub fn mail_to(&self) -> String {
        format!("mailto:{}", self.email)
    }
}

/// CV Projects
#[derive(Default, Deserialize, Debug)]
pub struct CvProject {
    pub name: String,
    pub synopsis: String,
}

/// Employment Entry
#[derive(Default, Deserialize, Debug)]
pub struct CvEmployment {
    pub title: String,
    pub employer: String,
    pub begin_date: MonthYear,
    pub end_date: Option<MonthYear>,
    pub current: bool,
    pub address: Address,
    pub bullets: Vec<String>,
}

/// CV/Resume
#[derive(Default, Deserialize, Debug)]
pub struct CV {
    pub header: CvHeader,
    pub education: Vec<School>,
    pub intro: Intro,
    pub projects: Vec<CvProject>,
    pub employment: Vec<CvEmployment>,
}

/// Single project
#[derive(Default, Deserialize, Debug)]
pub struct Project {
    pub demo: String,
    pub repo: String,
    pub name: String,
    pub language: String,
    pub synopsis: String,
}

/// Projects
#[derive(Default, Deserialize, Debug)]
pub struct Projects {
    pub projects: Vec<Project>,
}
