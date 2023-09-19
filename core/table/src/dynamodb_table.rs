use crate::table_types::{Table, TableSchema, TableSchemaTrait};
use async_trait::async_trait;

pub struct DynamoDbTable {
}

#[async_trait]
impl TableSchemaTrait for DynamoDbTable {
    async fn create_table(&self, table_schema: TableSchema) -> TableSchema {
        todo!()
    }

    async fn get_table(&self, table_schema: TableSchema) -> Table {
        todo!()
    }

    async fn update_table(&self, table_schema: TableSchema) -> Table {
        todo!()
    }

    async fn delete_table(&self, table_schema: TableSchema) -> bool {
        todo!()
    }

    async fn create_record(&self, row: crate::table_types::Row) -> crate::table_types::Row {
        todo!()
    }

    async fn get_record(&self, row: crate::table_types::Row, columns: Vec<crate::table_types::Column>) -> crate::table_types::Row {
        todo!()
    }

    async fn update_record(&self, row: crate::table_types::Row) -> crate::table_types::Row {
        todo!()
    }

    async fn delete_record(&self, row: crate::table_types::Row) -> bool {
        todo!()
    }

    async fn get_all_records(&self, table_schema: TableSchema) -> Vec<crate::table_types::Row> {
        todo!()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creating_table() {
        assert_eq!(2 + 2, 4)
    }
}