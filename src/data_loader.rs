//this module defines the Customer struct and handles CSV data loading and preprocessing
use chrono::NaiveDate;
use csv::ReaderBuilder;
use serde::Deserialize;
use std::{error::Error, fs::File};

#[derive(Debug, Deserialize,Clone)]
pub struct Customer {//struct representing one customer record from the dataset
	#[serde(rename = "ID")]
	pub id: u32,
	#[serde(rename = "Income")]
	pub income: Option<f64>,
	#[serde(rename = "Education")]
	pub education: String,
	#[serde(rename = "Marital_Status")]
	pub marital_status: String,
	#[serde(rename = "MntWines")]
	pub mntwines: f64,
	#[serde(rename = "MntFruits")]
	pub mntfruits: f64,
	#[serde(rename = "MntMeatProducts")]
	pub mntmeatproducts: f64,
	#[serde(rename = "MntFishProducts")]
	pub mntfishproducts: f64,
	#[serde(rename = "MntSweetProducts")]
	pub mntsweetproducts: f64,
	#[serde(rename = "MntGoldProds")]
	pub mntgoldprods: f64,
	#[serde(rename = "Recency")]
	pub recency: f64,
	#[serde(rename = "Dt_Customer", deserialize_with = "deserialize_date")]
	pub dt_customer: NaiveDate,
	#[serde(rename = "NumWebPurchases")]
	pub num_web_purchases: u32,
	#[serde(rename = "NumCatalogPurchases")]
	pub num_catalog_purchases: u32,
	#[serde(rename = "NumStorePurchases")]
	pub num_store_purchases: u32,
	#[serde(rename = "Year_Birth")]
	pub year_birth :u32,
	#[serde(rename = "Kidhome")]
	pub kidhome :u32,
	#[serde(rename = "Teenhome")]
	pub teenhome: u32,
	#[serde(rename = "AcceptedCmp1")]
	pub acceptedcmp1:u32,
	#[serde(rename = "AcceptedCmp2")]
	pub acceptedcmp2:u32,
	#[serde(rename = "AcceptedCmp3")]
	pub acceptedcmp3:u32,
	#[serde(rename = "AcceptedCmp4")]
	pub acceptedcmp4:u32,
	#[serde(rename = "AcceptedCmp5")]
	pub acceptedcmp5:u32,
	#[serde(rename = "Response")]
	pub response:u32,
}

impl Customer {
	//calculates total spending across all product categories
	pub fn total_spent(&self) -> f64 {
		self.mntwines
		+self.mntfruits
		+self.mntmeatproducts
		+self.mntfishproducts
		+self.mntsweetproducts
		+self.mntgoldprods
	}
//generates a numeric feature vector for clustering
//return a vector containing demographic, RFM and campaign data
	pub fn feature_vector(&self) -> Vec <f64> {
		let today = NaiveDate::from_ymd_opt(2025,4,1).unwrap();
		let days_as_customer = (today - self.dt_customer).num_days() as f64;
		let age = (2025 - self.year_birth) as f64;
		let num_children = (self.kidhome + self.teenhome) as f64;
//calculate frequency as total purchases divided by days since being a customer
		let frequency = (self.num_web_purchases+ self.num_store_purchases+self.num_catalog_purchases) as f64 / days_as_customer.max(1.0);
		let income =self.income.unwrap_or(0.0);
		let education_score = match self.education.as_str() {
			"PhD" => 3.0,
			"Master" => 2.0,
			"Graduation" => 1.0,
			_ => 0.0
		};//convert education level to numeric score
		let marital_score = match self.marital_status.as_str() {
			"Married" => 1.0,
			"Together" => 0.8,
			"Single" => 0.5,
			_ => 0.0,
		};
		let campaign_accepted = if self.acceptedcmp1 == 1 {
            		1.0
        	} else if self.acceptedcmp2 == 1 {
            		2.0
        	} else if self.acceptedcmp3 == 1 {
            		3.0
        	} else if self.acceptedcmp4 == 1 {
            		4.0
        	} else if self.acceptedcmp5 == 1 {
            		5.0
        	} else if self.response == 1 {
            		6.0
        	} else {
            		0.0 
        };
		vec![income,education_score, marital_score, self.recency as f64, frequency, self.total_spent(),num_children as f64, campaign_accepted,age]
	}
}

fn deserialize_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    	D: serde::Deserializer<'de>,
{
    	let s = String::deserialize(deserializer)?;
    	NaiveDate::parse_from_str(&s, "%d-%m-%Y").map_err(serde::de::Error::custom)
}

//load the CSV file
pub fn load_data(path: &str) -> Result<Vec<Customer>, Box<dyn Error>> {
    	let file = File::open(path)?;
    	let mut rdr = ReaderBuilder::new()
        	.has_headers(true)
        	.delimiter(b'\t')
        	.from_reader(file);

    	let mut result = Vec::new();

    	for record in rdr.deserialize::<Customer>() {
        	let customer = record?;
        	result.push(customer);
    	}

    	Ok(result)
}
