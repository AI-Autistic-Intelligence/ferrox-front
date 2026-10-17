---
id: universal-oauth
title: Universal OAuth
sidebar_position: 1
---

# Universal OAuth

Ferrox offers a standardized, enterprise-grade Universal OAuth module. Forget about writing boilerplate code to handle different provider quirks (like GitHub putting emails in a separate endpoint). Ferrox handles the complexities of social logins so you can focus on building your app.

## Philosophy

Our goal is to make Ferrox the "Pure Code WordPress". Authentication shouldn't take weeks. We abstracted OAuth2 and OpenID Connect flows into a unified `OAuthProvider` interface. Regardless of whether the user logs in with Google, GitHub, Apple, or Microsoft, you receive a standardized `FerroxIdentity` object.

## Architecture

The module is built around two primary components:
1. **`OAuthProviderInterface`**: Defines the contract for fetching authorization URLs, exchanging codes, and retrieving user profiles.
2. **`FerroxIdentity`**: A normalized DTO representing a user's identity, making it simple to link multiple social accounts to a single user in your database.

## Quickstart (Python / PHP)

Here is how you can use the Google OAuth provider out of the box in `ferrox-py` or `ferrox-php`:

```python
# ferrox-py Example
from ferrox.auth.oauth import GoogleOAuthProvider

provider = GoogleOAuthProvider(client_id="YOUR_ID", client_secret="YOUR_SECRET")

# 1. Redirect user to Google
auth_url = provider.get_authorization_url(state="secure_random_state", redirect_uri="https://yourapp.com/callback")
print(f"Go to: {auth_url}")

# 2. In your callback endpoint, exchange the code
token = await provider.exchange_code(code="req_code", redirect_uri="https://yourapp.com/callback")

# 3. Get the standardized identity
identity = await provider.fetch_user_profile(token)
print(f"Welcome, {identity.first_name}! (Provider: {identity.provider.value})")
```

## Supported Providers
- **Google** (OpenID Connect)
- **GitHub** (Standard OAuth2, with automatic `/emails` fetching)
- **Apple** (Sign in with Apple) *(Coming soon to all languages)*
- **Microsoft / Azure AD** *(Coming soon to all languages)*

## Ecosystem Integration
The OAuth module integrates natively with `ferrox-auth` to generate your internal access tokens (like Paseto/Argon2 sessions) once the social identity is verified.
