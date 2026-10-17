# AuthService (Authentication & SSO)

## 1. Overview (What does this do?)
The `AuthService` is a singleton service registered within the Ferrox IoC container that handles all aspects of user registration, login, and identity verification. It provides out-of-the-box support for traditional email/password flows as well as external Single Sign-On (SSO) integrations.

## 2. Philosophy (Why does it exist?)
Modern web applications rarely rely solely on basic username/password authentication anymore. Users expect seamless logins via Google, Apple, or GitHub. The `AuthService` exists to abstract the complexities of linking multiple external identities to a single user account and to enforce secure registration flows (like Double Opt-In) by default.

## 3. Target Audience (Who is it for?)
This service is for backend developers who need to implement a secure user registration and login system without dealing with the low-level mechanics of hashing passwords, generating secure tokens, or verifying OAuth2 payloads manually.

## 4. Architecture (How does it work?)
- **Standard Registration**: Handles traditional credentials. It automatically initiates a "Double Opt-In" process, meaning the account remains in a locked, unverified state until a confirmation email link is clicked.
- **SSO Integration (OIDC/OAuth2)**: Handles login/registration via external Identity Providers (IdPs). The identity (e.g., a Google ID) is saved inside an `identities` sub-document associated with the user. If a user registers via SSO, the "email verified" status is automatically inherited from the trusted IdP, bypassing the email opt-in step.

## 5. Installation / Setup
The `AuthService` is automatically registered in your IoC container when you import and register the `AuthModule` from `ferrox-py-auth`. You may need to configure external OAuth2 credentials (Client IDs and Secrets) in your environment variables for SSO functionality.

## 6. Quickstart (Usage)
```python
from ferrox_py_auth.services.auth_service import AuthService

# Example: Resolving the service from the container and registering via SSO
async def handle_google_callback(auth_service: AuthService, google_payload: dict):
    # Registration via SSO (Bypasses Email Validation automatically)
    user = await auth_service.register_via_sso(
        email=google_payload["email"],
        provider="google",
        provider_id=google_payload["sub"]
    )
    return user
```

## 7. Ecosystem Integration
The `AuthService` leverages the core **Data Component** to persist the `User` models to the database. Upon successful login, it interacts with the **Security Component** to generate a secure PASETO or JWT token, which is then returned to the client to be used in the Authorization header of subsequent requests.
