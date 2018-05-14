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
mod ch_date_format_option;
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
    #[serde(default)]
    #[serde(with = "ch_date_format_option")]
    pub date: Option<DateTime<Utc>>,

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
    #[serde(with = "ch_date_format_option")]
    pub next_due_date: Option<DateTime<Utc>>,

    #[serde(rename = "LastMadeUpDate")]
    #[serde(with = "ch_date_format_option")]
    pub last_made_up_date: Option<DateTime<Utc>>,

    #[serde(rename = "AccountCategory")]
    #[serde(default)]
    pub category: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    #[serde(rename = "NextDueDate")]
    #[serde(with = "ch_date_format")]
    next_due_date: DateTime<Utc>,

    #[serde(rename = "LastMadeUpDate")]
    #[serde(with = "ch_date_format_option")]
    last_made_up_date: Option<DateTime<Utc>>
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

#[derive(Serialize, Deserialize, Debug)]
pub struct SICcodes {
    #[serde(rename = "SicText")]
    pub text: Vec<String>
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct LimitedPartnership {
    #[serde(rename = "SicTeNumGenPartnersxt")]
    pub num_gen_partners: u8,

    #[serde(rename = "NumLimPartners")]
    pub num_lim_partners: u8
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
    #[serde(default)]
    #[serde(with = "ch_date_format_option")]
    pub incorporation_date: Option<DateTime<Utc>>,

    #[serde(rename = "RegistrationDate")]
    #[serde(default)]
    #[serde(with = "ch_date_format_option")]
    pub registration_date: Option<DateTime<Utc>>,

    #[serde(rename = "DissolutionDate")]
    #[serde(default)]
    #[serde(with = "ch_date_format_option")]
    pub dissolution_date: Option<DateTime<Utc>>,

    #[serde(rename = "PreviousName")]
    #[serde(default)]
    pub previous_name: Vec<PreviousName>,

    #[serde(rename = "Accounts")]
    pub accounts: Accounts,

    #[serde(rename = "Returns")]
    pub returns: Returns,

    #[serde(rename = "Mortgages")]
    #[serde(default)]
    pub mortgages: Option<Mortgages>,

    #[serde(rename = "SICCodes")]
    #[serde(default)]
    pub sic_codes: Option<SICcodes>,

    #[serde(rename = "LimitedPartnerships")]
    #[serde(default)]
    pub limited_partnership: Option<LimitedPartnership>
}

impl Company {
    pub fn new<A: API>(api: &A, number: &'static str) -> Result<Company>{
        api.get_company(number)
    }
}

pub trait API {
    fn get_company(&self, number: &'static str) -> Result<Company>;
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonResponse {
    #[serde(rename = "primaryTopic")]
    primary_topic: Company
}

pub struct ChApi {}

impl API for ChApi {
    fn get_company(&self, number: &'static str) -> Result<Company> {
        let url = format!("http://data.companieshouse.gov.uk/doc/company/{}.json", number);
        let res:JsonResponse = reqwest::get(&url)?.json()?;
        Ok(res.primary_topic)
    }
}

pub struct MockApi {}

impl API for MockApi {
    fn get_company(&self, number: &'static str) -> Result<Company> {
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
            incorporation_date: Some(Utc.datetime_from_str("2016-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap()),
            registration_date: None,
            dissolution_date: None,
            previous_name: Vec::new(),
            accounts: Accounts {
                ref_day: 31,
                ref_month: 12,
                next_due_date: Some(Utc.datetime_from_str("2018-09-30 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap()),
                last_made_up_date: Some(Utc.datetime_from_str("2016-12-31 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap()),
                category: Some("MICRO".to_string())
            },
            returns: Returns {
                next_due_date: Utc.datetime_from_str("2019-02-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
                last_made_up_date: Some(Utc.datetime_from_str("2017-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap())
            },
            mortgages: None,
            sic_codes: None,
            limited_partnership: None
        };
        Ok(company)
    }
}

#[cfg(test)]
mod tests {
    use Company;
    use LimitedPartnership;
    use MockApi;

    #[derive(Debug)]
    struct GetCompanyTest {
        company_number: &'static str,
        expected_name: &'static str,
        expect_limited_partnership: Option<LimitedPartnership>
    }

    #[test]
    fn get_company() {
        let api = MockApi{};
        let tests: Vec<GetCompanyTest> = vec![
            GetCompanyTest{
                company_number: "01234567",
                expected_name: "TEST COMPANY LTD",
                expect_limited_partnership: None
            }
        ];
        for test in tests.iter() {
            let company = Company::new(&api, test.company_number).expect("not found");
            assert_eq!(company.name, test.expected_name);
            assert_eq!(company.limited_partnership, test.expect_limited_partnership);
        }
    }
}
