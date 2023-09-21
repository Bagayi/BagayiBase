resource "aws_dynamodb_table" "tenant_table_schemas" {
    name = "TenantTableSchemas"
    billing_mode = "PAY_PER_REQUEST"

    hash_key = "id"
    range_key = "project"

    attribute {
      name = "id"
      type = "S"
    }

    attribute {
      name = "project"
      type = "S"
    }
}

resource "aws_dynamodb_table" "table_schema"{
    name = "TableSchema"
    billing_mode = "PAY_PER_REQUEST"

    hash_key = "id"
    range_key = "project_id"

    attribute {
         name = "id"
         type = "S"
    }

    attribute {
      name = "project_id"
      type = "S"
    }
}

resource "aws_dynamodb_table" "table_schema_columns" {
    name = "TableSchemaColumn"
    billing_mode = "PAY_PER_REQUEST"

    hash_key = "id"
    range_key = "table_id"

    attribute {
      name = "id"
      type = "S"
    }

    attribute {
      name = "table_id"
      type = "S"
    }
}

resource "aws_dynamodb_table" "table_data" {
    name = "TableData"
    billing_mode = "PAY_PER_REQUEST"

    hash_key = "id"
    range_key = "column_id"

    attribute {
        name = "id"
        type = "S" 
    }

    attribute {
        name = "column_id"
        type = "S"
    }
}