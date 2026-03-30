use std::collections::HashMap;
use crate::dataset::{ColumnType, Dataset, Value, Row};
use crate::query::{Aggregation, Condition, Query};

// Make a helper function to see if each row has the condition:
fn row_matches_condition(dataset: &Dataset, row: &Row, condition: &Condition) -> bool {
    // use match function to compare the condtion:
    match condition {
        // if the condition is equal:
        Condition::Equal(column_name, expected_value) => {
            // get the column index of the filtered value:
            let column_index = dataset.column_index(column_name);

            // get the value of the index and see if it is equal to the expected value:
            row.get_value(column_index) == expected_value
        }

        // see if the condition matches the case where Not (Some condition):
        Condition::Not(inner_condition) => {
            // flip the result to False if it is true:
            !row_matches_condition(dataset, row, inner_condition)
        }

        // check if condition 1 and condition 2 is true:
        Condition::And(left_condition, right_condition) => {
            // Checks both conditions recursively, && means both needs to be true:
            row_matches_condition(dataset, row, left_condition)
                && row_matches_condition(dataset, row, right_condition)
        }

        // check if condition 1 or condition 2 is true:
        Condition::Or(left_condition, right_condition) => {
            // Checks both conditions recursively, || means at least one condition is true:
            row_matches_condition(dataset, row, left_condition)
                || row_matches_condition(dataset, row, right_condition)
        }
    }
}
pub fn filter_dataset(dataset: &Dataset, filter: &Condition) -> Dataset {

    // generate the columns from the input dataset, because they are the same:
    let input_columns = dataset.columns();

    // create a output dataset based on the copy of input dataset columns: 
    let mut output_dataset = Dataset::new(input_columns.clone());

    // iterate each row of the dataset:
    for each_row in dataset.iter() {
        // if it matches the condition, using the helper function I made:
        if row_matches_condition(dataset, each_row, filter) {

            // add the row into the output dataset
            output_dataset.add_row(each_row.clone());
        }
    }

    // return the output dataset:
    return output_dataset;
}

pub fn group_by_dataset(dataset: Dataset, group_by_column: &String) -> HashMap<Value, Dataset> {
    // find the column index of the column I want to group:
    let column_index = dataset.column_index(group_by_column);

    // get the columns of the input dataset:
    let input_columns = dataset.columns();

    // create a new HashMap to store the output:
    let mut groups: HashMap<Value, Dataset> = HashMap::new();

    // iterate each row of the dataset:
    for row in dataset.iter() {
        // let each group to accord to the row value of the column:
        let key = row.get_value(column_index).clone();

        // if the group does not contain the key:
        if !groups.contains_key(&key) {
            // insert the key to it with new dataset with correct column:
            groups.insert(key.clone(), Dataset::new(input_columns.clone()));
        }
        // Get the group and add the row:
        groups.get_mut(&key).unwrap().add_row(row.clone());
    }
    //return the HashMap contains all groups:
    return groups
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