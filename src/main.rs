mod data_loader;
mod clustering;
use clustering::kmeans_cluster;

fn main() {
    	let path = "/Users/cher/customer_segmentation/src/data.csv";

    	let customers = match data_loader::load_data(path) {
        	Ok(customers) => {
            	println!("Loaded {} customers", customers.len());
		customers
        	},
        	Err(e) => {
            	eprintln!("Failed to load data: {}", e);
		return;
        	}
    	};
	let k=3;
	let iterations =5;
	let assignments = kmeans_cluster(&customers, k, iterations);
	let mut totals = vec![0.0;k];
	let mut counts = vec![0;k];
	for (i,customer) in customers.iter().enumerate(){
		let cluster = assignments [i];
		totals[cluster] += customer.total_spent();
		counts[cluster] += 1;
	}
	println!("\n Cluster summary:");
	for n in 0..k{
		let avg_spend = if counts [n] >0 {
			totals[n]/counts[n] as f64
		}else{0.0};
		println!("Cluster {} -> {} customers | Avg Total Spend:{:.2}", n, counts[n], avg_spend)
	};
	
}

