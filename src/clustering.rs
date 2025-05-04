//this module is for performing clustering and calculating the average features of each tier
use crate::data_loader::Customer;
use rand::Rng;

//calculates Euclidean distance between two vectors
//input:a,b: slices of equal length representing feature vectors
//output: euclidean distance
fn distance(a:&[f64],b:&[f64]) -> f64 {
	a.iter().zip(b.iter()).map(|(x,y)|(x-y).powi(2)).sum::<f64>().sqrt()
}

//performs K-means clustering
//inputs: customers, k:number of clusters, iterations:number of iterations to run the algorithm
//output: Vec<usize> cluster assignment for each customer
pub fn kmeans_cluster(customers: &[Customer], k:usize, iterations: usize) -> Vec<usize>{
	let mut rng = rand::thread_rng();
	let mut centers: Vec<Vec<f64>> = Vec::new();
	while centers.len() < k {
//initialize random cluster centers
		let idx =rng.gen_range(0..customers.len());
		let candidate = customers[idx].feature_vector();
		if !centers.contains(&candidate){
			centers.push(candidate);
		}
	}
	let mut assignments = vec![0;customers.len()];
	let feature_len = customers[0].feature_vector().len();
	for _ in 0..iterations{
		for (i, customer) in customers.iter().enumerate(){
			let fv = customer.feature_vector();
			let (best_cluster, _) = centers.iter().enumerate().map(|(idx,center)|(idx,distance(&fv, center))).min_by(|a,b| a.1.partial_cmp(&b.1).unwrap()).unwrap();
			assignments[i] = best_cluster;
		}
		let mut new_centers = vec![vec![0.0;feature_len];k];
		let mut counts = vec![0;k];
		for (i,customer) in customers.iter().enumerate(){
//recompute centers by averaging feature vectors in each cluster
			let fv = customer.feature_vector();
			let cluster = assignments[i];
			for j in 0..3{
				new_centers[cluster][j] += fv[j];
			}
			counts[cluster] += 1;
		}
		for i in 0..k{
			if counts[i] > 0{
				for j in 0..3 {
					new_centers[i][j] /= counts[i] as f64;
				}
			}
		}
		centers = new_centers;
	}
	let feature_names = ["Income",  "Education", "Marital Score","Recency","Frequency","Total Spend", "Number of Children", "Campaign Accepted", "Age"];
	let mut feature_sums = vec![vec![0.0;feature_len];k];
	let mut cluster_counts = vec![0;k];
	for (m,customer) in customers.iter().enumerate(){
		let cluster = assignments[m];
		let features = customer.feature_vector();
		for j in 0..feature_len{
			feature_sums[cluster][j] += features[j];
		}
		cluster_counts[cluster] += 1;
	}
	println!("\n Cluster feature averages");
	for cluster in 0..k{
		println!("Cluster {}", cluster);
		for j in 0..feature_len{
			let avg =feature_sums[cluster][j]/cluster_counts[cluster].max(1) as f64;
			println!("{:<16}:{:2}", feature_names[j],avg);
		}
		println!();
	}
	assignments
}