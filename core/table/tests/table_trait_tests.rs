// use dynamodb_table::DynamoDbTable;
use table::dynamodb_table::DynamoDbTable;


fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn test_creating_table() {
    // DynamoDbTable::create_table();
    let result = add(2, 2);
    assert_eq!(result, 4);
}

#[test]
fn test_get_table(){

}


#[test]
fn test_update_table(){

}


#[test]
fn test_delete_table(){

}


#[test]
fn test_create_record(){

}


#[test]
fn test_get_record(){

}


#[test]
fn test_update_record(){

}


#[test]
fn test_delete_record(){

}


#[test]
fn test_get_all_records(){

}


