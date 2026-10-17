# Ferrox-Py-Auth Overview

## 1. Overview (What does this do?)
The `ferrox-py-auth` package provides a unified, enterprise-ready Identity, Access Management (IAM), and Data Privacy (GDPR) solution designed specifically to plug into the core `ferrox-py` framework. 

## 2. Philosophy (Why does it exist?)
In complex distributed systems and modern SaaS architectures, authentication is rarely as simple as a single "Users" SQL table. It requires dynamic extension to support Single Sign-On (Google, Apple, Microsoft), Multi-Factor Authentication, and a decoupled data architecture to scale securely. This package exists to encapsulate all that complexity into a secure, reusable module so developers can focus on business logic rather than writing boilerplate auth code.

## 3. Target Audience (Who is it for?)
This package is built for application architects and backend engineers tasked with building secure platforms that require strict user identity tracking, varied permission levels (RBAC), and legal compliance with international privacy laws.

## 4. Architecture (How does it work?)
The Auth package seamlessly hooks into Layer 3 (Threat Engine) and Layer 4 (Auth Guards) of the `ferrox-py` Onion Request Pipeline. It provides the practical implementation for parsing tokens, validating user state, and persisting user data.
Integrated features include:
- Management of multiple users with SSO identities linked to the same aggregate entity.
- Granular Role-Based Access Control (RBAC).
- Ready-to-use GDPR tools (Right to be Forgotten, Data Portability).

## 5. Installation / Setup
Install the package alongside the core framework:
```bash
pip install ferrox-py-auth
```
Ensure that cryptographic keys used for token generation (JWT/PASETO) are injected securely via environment variables.

## 6. Quickstart (Usage)
To enable IAM across your application, register the module in your root configuration:

```python
from ferrox_py.core.container import Container
from ferrox_py_auth import AuthModule

container = Container()
container.register_module(AuthModule)
# Authentication and RBAC layers are now active globally.
```

## 7. Ecosystem Integration
The Auth module is a fundamental prerequisite for many other ecosystem packages. For instance, `ferrox-py-commerce` requires a guaranteed, authenticated `User` context to bind payment methods and track subscription ownership securely.
