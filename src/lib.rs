extern crate serde;
extern crate serde_json;
extern crate chrono;
extern crate reqwest;

#[macro_use]
extern crate serde_derive;

use chrono::DateTime;
use chrono::offset::Utc;
use chrono::TimeZone;
use reqwest::*;

mod ch_date_format;
mod ch_u8_format;

#[derive(Serialize, Deserialize, Debug)]
pub struct Address {
    #[serde(rename = "Careof")]
    #[serde(default)]
    pub care_of: String,
    
    #[serde(rename = "POBox")]
    #[serde(default)]
    pub po_box: String,
    
    #[serde(rename = "AddressLine1")]
    pub address_line1: String,

    #[serde(rename = "AddressLine2")]
    #[serde(default)]
    pub address_line2: String,

    #[serde(rename = "PostTown")]
    pub post_town: String,

    #[serde(rename = "County")]
    #[serde(default)]
    pub county: String,

    #[serde(rename = "Country")]
    #[serde(default)]
    pub country: String,

    #[serde(rename = "Postcode")]
    #[serde(default)]
    pub postcode: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreviousName {
    #[serde(rename = "CONDate")]
    #[serde(default="ch_date_format::empty_value")]
    #[serde(with = "ch_date_format")]
    pub date: DateTime<Utc>,

    #[serde(rename = "CompanyName")]
    pub name: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Accounts {
    #[serde(rename = "AccountRefDay")]
    #[serde(default)]
    #[serde(with = "ch_u8_format")]
    pub ref_day: u8,

    #[serde(rename = "AccountRefMonth")]
    #[serde(default)]
    #[serde(with = "ch_u8_format")]
    pub ref_month: u8,

    #[serde(rename = "NextDueDate")]
    #[serde(with = "ch_date_format")]
    pub next_due_date: DateTime<Utc>,

    #[serde(rename = "LastMadeUpDate")]
    #[serde(with = "ch_date_format")]
    pub last_made_up_date: DateTime<Utc>,

    #[serde(rename = "AccountCategory")]
    #[serde(default)]
    pub category: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    #[serde(rename = "NextDueDate")]
    #[serde(with = "ch_date_format")]
    next_due_date: DateTime<Utc>,

    #[serde(rename = "LastMadeUpDate")]
    #[serde(with = "ch_date_format")]
    last_made_up_date: DateTime<Utc>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mortgages {
    #[serde(rename = "NumMortCharges")]
    #[serde(with = "ch_u8_format")]
    num_charges: u8,

    #[serde(rename = "NumMortOutstanding")]
    #[serde(with = "ch_u8_format")]
    num_outstanding: u8,

    #[serde(rename = "NumMortPartSatisfied")]
    #[serde(with = "ch_u8_format")]
    num_part_satisfied: u8,

    #[serde(rename = "NumMortSatisfied")]
    #[serde(with = "ch_u8_format")]
    num_satisfied: u8
}

impl Mortgages {
    pub fn empty_value() -> Self {
        Mortgages {
            num_charges: 0,
            num_outstanding: 0,
            num_part_satisfied: 0,
            num_satisfied: 0
        }
    }

    pub fn is_empty(self) -> bool {
        self.num_charges == 0 && self.num_outstanding == 0 && self.num_part_satisfied == 0 && self.num_satisfied == 0
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SICcodes {
    #[serde(rename = "SicText")]
    pub text: Vec<String>
}

impl SICcodes {
    pub fn empty_value() -> Self {
        SICcodes {
            text: Vec::<String>::new()
        }
    }

    pub fn is_empty(self) -> bool {
        self.text.len() == 0
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LimitedPartnership {
    #[serde(rename = "SicTeNumGenPartnersxt")]
    pub num_gen_partners: u8,

    #[serde(rename = "NumLimPartners")]
    pub num_lim_partners: u8
}

impl LimitedPartnership {
    pub fn empty_value() -> Self {
        LimitedPartnership{
            num_gen_partners: 0,
            num_lim_partners: 0,
        }
    }

    pub fn is_empty(self) -> bool {
        self.num_gen_partners == 0 && self.num_lim_partners == 0
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Company {
    #[serde(rename = "CompanyName")]
    pub name: String,

    #[serde(rename = "CompanyNumber")]
    pub number: String,

    #[serde(rename = "RegAddress")]
    reg_address: Address,

    #[serde(rename = "CompanyCategory")]
    pub category: String,

    #[serde(rename = "CompanyStatus")]
    pub status: String,

    #[serde(rename = "CountryOfOrigin")]
    #[serde(default)]
    pub country_of_origin: String,

    #[serde(rename = "IncorporationDate")]
    #[serde(default="ch_date_format::empty_value")]
    #[serde(with = "ch_date_format")]
    pub incorporation_date: DateTime<Utc>,

    #[serde(rename = "RegistrationDate")]
    #[serde(default="ch_date_format::empty_value")]
    #[serde(with = "ch_date_format")]
    pub registration_date: DateTime<Utc>,

    #[serde(rename = "DissolutionDate")]
    #[serde(default="ch_date_format::empty_value")]
    #[serde(with = "ch_date_format")]
    pub dissolution_date: DateTime<Utc>,

    #[serde(rename = "PreviousName")]
    #[serde(default)]
    pub previous_name: Vec<PreviousName>,

    #[serde(rename = "Accounts")]
    pub accounts: Accounts,

    #[serde(rename = "Returns")]
    pub returns: Returns,

    #[serde(rename = "Mortgages")]
    #[serde(default="Mortgages::empty_value")]
    pub mortgages: Mortgages,

    #[serde(rename = "SICCodes")]
    #[serde(default="SICcodes::empty_value")]
    pub sic_codes: SICcodes,

    #[serde(rename = "LimitedPartnerships")]
    #[serde(default="LimitedPartnership::empty_value")]
    pub limited_partnership: LimitedPartnership
}

impl Company {
    pub fn new<a: API>(api: &a, number: &'static str) -> Result<Company>{
        api.get_company(number)
    }
}

pub trait API {
    fn get_company(self, number: &'static str) -> Result<Company>;
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonResponse {
    #[serde(rename = "primaryTopic")]
    primary_topic: Company
}

pub struct ChApi {}

impl API for ChApi {
    fn get_company(self, number: &'static str) -> Result<Company> {
        let url = format!("http://data.companieshouse.gov.uk/doc/company/{}.json", number);
        let res:JsonResponse = reqwest::get(&url)?.json()?;
        Ok(res.primary_topic)
    }
}

pub struct MockApi {}

impl API for MockApi {
    fn get_company(self, number: &'static str) -> Result<Company> {
        println!("{}", number);
        let company = Company {
            name: "TEST COMPANY LTD".to_string(),
            number: "01234567".to_string(),
            reg_address: Address{
                care_of: "".to_string(),
                po_box: "".to_string(),
                address_line1: "110 Test street".to_string(),
                address_line2: "".to_string(),
                post_town: "Test town".to_string(),
                county: "".to_string(),
                country: "".to_string(),
                postcode: "TS7 1NG ".to_string(),
            },
            category: "Private company limited by shares".to_string(),
            status: "Active".to_string(),
            country_of_origin: "England".to_string(),
            incorporation_date: Utc.datetime_from_str("2016-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
            registration_date: ch_date_format::empty_value(),
            dissolution_date: ch_date_format::empty_value(),
            previous_name: Vec::new(),
            accounts: Accounts {
                ref_day: 31,
                ref_month: 12,
                next_due_date: Utc.datetime_from_str("2018-09-30 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
                last_made_up_date: Utc.datetime_from_str("2016-12-31 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
                category: "MICRO".to_string()
            },
            returns: Returns {
                next_due_date: Utc.datetime_from_str("2019-02-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
                last_made_up_date: Utc.datetime_from_str("2017-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap()
            },
            mortgages: Mortgages::empty_value(),
            sic_codes: SICcodes::empty_value(),
            limited_partnership: LimitedPartnership::empty_value()
        };
        Ok(company)
    }
}

#[cfg(test)]
mod tests {
    use Company;
    use MockApi;

    #[derive(Debug)]
    struct GetCompanyTest {
        company_number: &'static str,
        expected_name: &'static str,
        expect_limited_partnership_empty: bool
    }

    #[test]
    fn get_company() {
        let api = MockApi{};
        let tests: Vec<GetCompanyTest> = vec![
            GetCompanyTest{
                company_number: "01234567",
                expected_name: "TEST COMPANY LTD",
                expect_limited_partnership_empty: true
            }
        ];
        for test in tests.iter() {
            let company = Company::new(&api, test.company_number).expect("not found");
            assert_eq!(company.name, test.expected_name);
            assert_eq!(company.limited_partnership.is_empty(), test.expect_limited_partnership_empty);
        }
    }
}
