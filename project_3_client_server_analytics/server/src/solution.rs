use analytics_lib::{dataset::{self, Dataset}, query::Query};

pub fn hello() -> String {
    println!("hello called");
    return String::from("hello");
}

pub fn slow_rpc(input_dataset: &Dataset) -> Dataset {
    println!("using slow_rpc");
    return input_dataset.clone()
}

pub fn fast_rpc(input_dataset: &Dataset, query: Query) -> Dataset {
    println!("fast_rpc called");
    
    // process the dataset and query using the compute query on dataset function:
    let output_dataset = analytics_lib::solution::compute_query_on_dataset(input_dataset, &query);

    // returnt the output dataset:
    return output_dataset
}