# Models (Pydantic / Mongo)

## 1. Overview (What does this do?)
The Models module exports the base Data Transfer Objects (DTOs) and Database Schemas used to represent Users and their authentication states. These models are statically validated using Pydantic and are designed to be natively serialized/deserialized into MongoDB or adapted for SQLAlchemy ORMs.

## 2. Philosophy (Why does it exist?)
A fragmented definition of what constitutes a "User" leads to bugs across different microservices. By providing strict, centrally defined Pydantic models for `User` and `UserIdentity`, `ferrox-py-auth` ensures that every part of the application agrees on the structure of the identity data, ensuring data integrity and fast validation at the API boundaries.

## 3. Target Audience (Who is it for?)
This module is for developers integrating the Auth module who need to understand the underlying data structure of the User object, or those who need to extend the base models with custom application-specific fields (like `company_name` or `avatar_url`).

## 4. Architecture (How does it work?)
- **UserIdentity**: A sub-document representing an external Identity Provider (IdP) link. It stores the `provider` (e.g., "google", "apple", "facebook", "local"), the `provider_id` (the alphanumeric unique identifier from the IdP), and auditing timestamps like `last_login`.
- **User**: The primary aggregate root model. It contains the main `email` address, manages critical boolean flags (`is_email_verified`, `is_active`, `is_locked`), holds a list of `UserIdentity` objects, and contains an array of `roles` (strings) used by the RBAC engine.

## 5. Installation / Setup
These models require `pydantic` (installed by default with the core framework). If you intend to use them directly with a NoSQL database, the `motor` driver from the core Data Component is recommended.

## 6. Quickstart (Usage)
```python
from ferrox_py_auth.models.user import User, UserIdentity
from datetime import datetime

# Creating an instance of a User manually (usually handled by AuthService)
identity = UserIdentity(
    provider="google",
    provider_id="sub_123456789",
    last_login=datetime.utcnow()
)

new_user = User(
    email="test@example.com",
    is_email_verified=True,
    is_active=True,
    roles=["user", "editor"],
    identities=[identity]
)

# The model will automatically validate its schema upon instantiation
print(new_user.model_dump_json())
```

## 7. Ecosystem Integration
These models are utilized by the **Validation Pipes** (Layer 5) to validate incoming request bodies containing user data. They are also intrinsically linked to the **Data Component**, where they act as the primary interface between the `AuthService` and the database repository.
