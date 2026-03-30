use std::collections::HashMap;
use crate::dataset::{ColumnType, Dataset, Value, Row};
use crate::query::{Aggregation, Condition, Query};

pub fn filter_dataset(dataset: &Dataset, filter: &Condition) -> Dataset {
    todo!("Implement this!");
}

pub fn group_by_dataset(dataset: Dataset, group_by_column: &String) -> HashMap<Value, Dataset> {
    todo!("Implement this!");
}

fn sum_and_count_integers(dataset: &Dataset, column_name: &String) -> (i32, i32) { // helper function to make aggregate_dataset less cluttered
    let col_index = dataset.column_index(column_name); // keep track of column index
    let mut sum = 0; // create mutable sum 
    let mut count = 0; // create mutable count
    
    for row in dataset.iter() { // loop through each row of the dataset
        if let Value::Integer(val) = row.get_value(col_index) { // only reach the below code if the value is an integer (i32), ignore strings
            sum += *val; // add value to running sum
            count += 1; // increase count
        }
    }
    
    (sum, count) // returns both the sum and count
}

pub fn aggregate_dataset(dataset: HashMap<Value, Dataset>, aggregation: &Aggregation) -> HashMap<Value, Value> {
    let mut result: HashMap<Value, Value> = HashMap::new(); // create hashmap to store result

    for (group_key, group_dataset) in dataset.into_iter() { // iterate over the dataset, using .into_iter() to take ownership/avoid copying the whole thing
        let aggregated_value = match aggregation { // determine which type of aggregation to perform
            Aggregation::Count(_column_name) => { 
                let count = group_dataset.iter().count() as i32; // count the rows in the group
                Value::Integer(count)
            },
            Aggregation::Sum(column_name) => { 
                let (sum, _count) = sum_and_count_integers(&group_dataset, column_name); // calls the helper function above to find the sum
                Value::Integer(sum)
            },
            
            Aggregation::Average(column_name) => {
                let (sum, count) = sum_and_count_integers(&group_dataset, column_name); // use helper function again to get the count and the sum
                let avg = if count > 0 { sum / count } else { 0 }; // use count and sum to take the average
                Value::Integer(avg)
            }
        };
        result.insert(group_key, aggregated_value);
    }
    result

}

pub fn compute_query_on_dataset(dataset: &Dataset, query: &Query) -> Dataset {
    let filtered = filter_dataset(dataset, query.get_filter());
    let grouped = group_by_dataset(filtered, query.get_group_by());
    let aggregated = aggregate_dataset(grouped, query.get_aggregate());

    // Create the name of the columns.
    let group_by_column_name = query.get_group_by();
    let group_by_column_type = dataset.column_type(group_by_column_name);
    let columns = vec![
        (group_by_column_name.clone(), group_by_column_type.clone()),
        (query.get_aggregate().get_result_column_name(), ColumnType::Integer),
    ];

    // Create result dataset object and fill it with the results.
    let mut result = Dataset::new(columns);
    for (grouped_value, aggregation_value) in aggregated {
        result.add_row(Row::new(vec![grouped_value, aggregation_value]));
    }
    return result;
}