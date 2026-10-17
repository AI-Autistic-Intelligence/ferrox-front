---
id: cloud-iac
title: Cloud & IaC 
sidebar_position: 3
---

# Cloud & IaC (Infrastructure as Code)

Ferrox is not just a framework for business logic; it's a complete ecosystem. The Cloud & IaC module allows your code to interact securely with Cloud Storage (AWS S3, Google Cloud Storage) and to generate Infrastructure as Code (Terraform) dynamically.

## Philosophy

Whether you are building a multi-tenant SaaS that needs to spin up isolated databases on the fly, or just securely handling file uploads, you shouldn't have to write raw HTTP requests or string-concatenate Terraform files. Ferrox provides clean abstractions to manage infrastructure directly from your codebase.

## Cloud Storage 

The `CloudStorageProvider` standardizes file operations across providers. A killer feature is the built-in support for **Presigned URLs**, allowing you to let users upload files directly to S3 from the browser, saving your backend's bandwidth.

### S3 Quickstart
```python
from ferrox.utils.cloud import AWSS3Provider

provider = AWSS3Provider(
    region_name="eu-central-1",
    aws_access_key_id="KEY",
    aws_secret_access_key="SECRET"
)

# Generate a temporary URL for the frontend to upload a file directly
upload_url = await provider.generate_presigned_url(
    bucket="my-ferrox-bucket",
    path="uploads/user_avatar.png",
    expiration_seconds=600 # 10 minutes
)
```

## Infrastructure Generation (Terraform)

The `TerraformGenerator` enables you to define infrastructure using pure code. It compiles valid HCL (HashiCorp Configuration Language) that can be applied by the Terraform CLI.

### VPC and Database Quickstart
```python
from ferrox.utils.cloud import AWSTerraformGenerator

generator = AWSTerraformGenerator()

# Generate a secure VPC
vpc_hcl = generator.generate_vpc(name="ferrox_prod_vpc", cidr_block="10.0.0.0/16")

# Generate an RDS Database instance
db_hcl = generator.generate_database(
    name="ferrox_db",
    engine="postgres",
    version="15.3",
    instance_class="db.t3.micro"
)

# Now you can write these strings to a .tf file and run `terraform apply`!
```

## Ecosystem Integration
This module is the backbone of the Ferrox ecosystem. It can be paired with `ferrox-kyc` for storing verified identity documents, or with `ferrox-commerce` for serving digital downloads securely.
