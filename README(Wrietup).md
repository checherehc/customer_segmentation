Project Overview
Goal:
 To segment customers into clusters based on purchasing behavior and demographic traits using K-means clustering, in order to uncover meaningful consumer profiles.
Dataset:
Source: Kaggle
Size: 2,240 customer records × 29 columns
Format: CSV
Data Processing
Loading into Rust:
Used the csv crate to read the datasets
Customer struct implements serde::Deserialize for structured parsing.
Cleaning / Transformation:
Parsed date field Dt_Customer using chrono::NaiveDate
Transformed features
Age from birth year
num_of_children from Kidhome + Teenhome
accepted_campaign_index for which campaign they accepted first
Converted relevant features into numerical vectors for clustering.
Code Structure
Modules:
main.rs: Program entry point, runs the clustering and prints summary.
data_loader.rs: Handles reading and preprocessing the dataset.
clustering.rs: Implements K-means and analysis functions.
Key Structs & Functions:
Customer Struct
Represents one customer.
Inputs: Parsed CSV row
Outputs: Populated Rust struct
Key fields: income, education, marital_status, acceptedcmp1...
Method:
feature_vector() → [f64] features for clustering
kmeans_cluster(customers, k, iterations)
Performs unsupervised K-means clustering.
Inputs: List of Customer, number of clusters, iterations
Output: Vec<usize> cluster index for each customer
Logic:
Randomly initializes cluster centers
Iteratively assigns customers to closest center
Recalculates centers based on means
Prints cluster feature averages
Test

Test descriptions:
test_distance_symmetry: Confirms distance metric is symmetric.
test_clustering_returns_valid_assignments: Validates clustering assigns all customers correctly.
Result

Interpretation:
High spending tier has way higher income than other two tiers, around 2 times higher than middle tier and more than 3 times higher than low tier
High spending tier is more valuable than other tiers not only because they spend more, but also because they spend more frequently(0.004), 4 times higher than the low tier.
High spending consumers have a lower chance to have children. Therefore, campaign can detour from advertising to a household with children. 
High spending consumers are older than consumers in other two tiers
Cluster 0: high spending, high income, high education, no children, older audience, responsive to campaign, frequent purchaser
Cluster 1: low spending, low income, younger audience, usually have 1 child, not sensitive to campaign
Cluster 2: middle spending, middle income, purchase more recently, usually have 1-2 child, mildly responsive to campaign
Instructions
To build: cargo build – release
To run: cargo run –release
Command-Line Input:
None required — reads from data.csv
Expected Runtime:
< 5 seconds for 2,240 rows with 3 clusters and 10 iterations.
AI-Assistance Disclosure and Other Citations
Use of ChatGPT:
Debugging serde errors:
I encountered issues when deserializing CSV rows into the Customer struct. ChatGPT explained that I needed to use #[serde(rename = "...")] to match field names in the CSV (which were in PascalCase or camelCase) with my struct fields (which were in snake_case).
Clarifying NaiveDate parsing with chrono crate:
ChatGPT helped me apply a manual deserializer for Dt_Customer due to its custom date format (dd-mm-yyyy).


