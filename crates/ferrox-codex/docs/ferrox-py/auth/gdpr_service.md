# GdprService

## 1. Overview (What does this do?)
The `GdprService` is a specialized service that abstracts complex commands required to comply with modern privacy laws (like the GDPR in Europe and the CCPA in California). It provides automated mechanisms for data portability (exporting user data) and the Right to be Forgotten (deleting or anonymizing user data).

## 2. Philosophy (Why does it exist?)
Compliance with data privacy laws is a strict legal requirement, not a feature. Leaving developers to manually write cascade deletions for user data is highly dangerous; it often results in orphaned records, corrupted relational constraints, or, worse, incomplete deletions that violate privacy laws. This service exists to centralize and automate secure data anonymization and extraction.

## 3. Target Audience (Who is it for?)
This service is essential for Data Protection Officers (DPOs), compliance engineers, and backend developers who are legally required to provide users with the ability to download their data or permanently delete their accounts from the SaaS platform.

## 4. Architecture (How does it work?)
- **Right to be Forgotten (Deletion)**: The `delete_user_data` method securely masks, anonymizes, or hard-deletes Personally Identifiable Information (PII) across the database. It triggers an internal event that other modules can listen to, ensuring data is scrubbed across all bounded contexts.
- **Data Portability**: The `export_user_data` method aggregates the entire user state and preferences into a readable JSON or CSV format, ready to be compressed into a `.zip` file and delivered to the user.

## 5. Installation / Setup
The `GdprService` is available out of the box when using `ferrox-py-auth`. No additional installation is required, but you must ensure your specific data repositories implement the required anonymization hooks if you have custom tables containing PII.

## 6. Quickstart (Usage)
```python
from ferrox_py_auth.services.gdpr_service import GdprService

async def handle_account_deletion(gdpr_service: GdprService, user_id: str):
    # Perform a compliant soft delete or hard delete of PII
    await gdpr_service.delete_user_data(user_id=user_id)
    return {"message": "Account successfully anonymized."}

async def handle_data_export(gdpr_service: GdprService, user_id: str):
    # Generate a full export of the user's data footprint
    dump = await gdpr_service.export_user_data(user_id=user_id)
    # Returns a comprehensive dict: { "email": "...", "identities": [...], "created_at": "..." }
    return dump
```

## 7. Ecosystem Integration
The `GdprService` integrates with the **CQRS and Event Dispatcher** from the core framework. When `delete_user_data` is called, it emits a `UserDeletedEvent`. Other modules (like `ferrox-py-commerce`) subscribe to this event to safely cancel active subscriptions and anonymize billing records without creating tightly coupled dependencies.
