# Connectors

## 1. Overview (What does this do?)
The `connectors` module standardizes read and write access to external data sources. It provides a set of pre-built, reusable classes that can interact with Object Storage (S3, MinIO), local file systems, and tabular formats (CSV, Excel) using a unified programming interface.

## 2. Philosophy (Why does it exist?)
Interacting with external data sources usually involves writing boilerplate code using varying SDKs (e.g., `boto3` for S3, native `open()` for files, `requests` for APIs). This creates tightly coupled code that is difficult to test and impossible to swap out easily. By forcing all data I/O through a generic `BaseConnector` interface, the business logic remains entirely oblivious to where the data is actually coming from or going to, allowing developers to easily mock these interactions during unit testing.

## 3. Target Audience (Who is it for?)
This module is intended for Data Engineers writing ETL pipelines or backend developers who need to generate, upload, or process reports, backups, and user-uploaded files without wrestling with raw provider SDKs.

## 4. Architecture (How does it work?)
- **BaseConnector**: An abstract base class defining the strict I/O contract (`read` and `write` methods).
- **S3Connector**: Facilitates interaction with S3-compatible object storage (AWS, MinIO). It handles authentication securely and manages streaming byte uploads and downloads to prevent memory exhaustion on large files.
- **CsvConnector**: Simplifies handling CSV files, natively supporting chunking mechanisms (often wrapping Pandas or the native Python `csv` module) to process massive datasets sequentially.

## 5. Installation / Setup
The `BaseConnector` requires no external libraries. However, to use the specific implementations, you must install the respective drivers.

```bash
# For S3 connectivity
pip install boto3

# For advanced CSV/Excel manipulation
pip install pandas
```

## 6. Quickstart (Usage)
```python
from ferrox_py_utils.connectors.s3 import S3Connector

# 1. Initialize the connector (usually done within the IoC Container)
s3_connector = S3Connector(
    endpoint_url="http://localhost:9000",
    access_key="minioadmin",
    secret_key="minioadmin",
    bucket_name="bronze-layer"
)

# 2. Use the unified interface
def backup_database_dump(data_bytes: bytes):
    # The business logic only calls 'write'
    s3_connector.write(file_path="backups/db_dump.sql", data=data_bytes)

# 3. Read data back
raw_data = s3_connector.read(file_path="backups/db_dump.sql")
```

## 7. Ecosystem Integration
Connectors are explicitly designed to be registered as **Providers** in the core `ferrox-py` Dependency Injection Container. This ensures that the application only maintains one active connection pool to S3 or a remote FTP server, injecting it seamlessly into the **PipelineOrchestrator** steps when needed.
