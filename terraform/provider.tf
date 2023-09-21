provider "aws" {
    access_key                  = "AKIATYUVCWESJE23SS6E"
    region                      = "us-east-1"
    secret_key                  = "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"
    skip_credentials_validation = true
    skip_metadata_api_check     = true
    skip_requesting_account_id  = true

    endpoints {
        dynamodb = "http://localhost:8000/"
    }
}