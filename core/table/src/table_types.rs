use async_trait::async_trait;

#[async_trait]
pub trait TableSchemaTrait {
    // crud operations for table
    async fn create_table(&self, table_schema: TableSchema) -> TableSchema;
    async fn get_table(&self, table_schema: TableSchema) -> Table;
    async fn update_table(&self, table_schema: TableSchema) -> Table;
    async fn delete_table(&self, table_schema: TableSchema) -> bool;

    // crud operations for one record
    async fn create_record(&self, row: Row) -> Row;
    async fn get_record(&self, row: Row, columns: Vec<Column>) -> Row;
    async fn update_record(&self, row: Row) -> Row;
    async fn delete_record(&self, row: Row) -> bool;

    // crud operations for
    async fn get_all_records(&self, table_schema: TableSchema) -> Vec<Row>;
}

pub struct TableSchema {
    pub name: String,
    pub columns: Vec<Column>,
}

pub struct Column {
    pub name: String,
    pub data_type: String,
    pub is_nullable: bool,
    pub is_primary_key: bool,
}

pub struct Row {
    pub row_id : usize,
    pub row_value : String,
    pub column: Column
}

pub struct Table {
    pub schema: TableSchema,
    pub rows: Vec<Row>,
}