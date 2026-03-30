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
    todo!("Implement this!");
}

pub fn aggregate_dataset(dataset: HashMap<Value, Dataset>, aggregation: &Aggregation) -> HashMap<Value, Value> {
    todo!("Implement this!");
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