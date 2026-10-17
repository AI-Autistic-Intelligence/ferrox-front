---
id: kyc-identity
title: KYC & Document Verification
sidebar_position: 2
---

# KYC & Document Verification

In modern FinTech, PropTech, or high-security SaaS platforms, knowing your customer (KYC) is a legal requirement. Ferrox abstracts away the complexities of integrating third-party KYC providers like Onfido or Stripe Identity, offering a pure-code, unified interface across all supported languages.

## Philosophy

Identity verification involves creating applicants, generating secure SDK upload links, and receiving asynchronous webhook updates. Ferrox standardizes these steps into the `KYCProviderInterface`, ensuring that switching from one provider to another requires zero changes to your application's business logic.

## Architecture

The module utilizes the following standardized DTOs:
- **`KYCApplicant`**: A unified representation of the user requesting verification.
- **`KYCVerificationResult`**: Standardizes provider-specific responses into clean Enums (`APPROVED`, `DECLINED`, `NEEDS_REVIEW`).
- **Webhook Signature Verification**: Every provider implements `verifyWebhookSignature()` to guarantee that incoming status updates are cryptografically signed and authentic.

## Quickstart (Onfido Integration)

Here is how you generate a secure document upload link using `ferrox-py` or `ferrox-php`:

```python
from ferrox.auth.kyc import OnfidoKYCProvider, KYCApplicant

provider = OnfidoKYCProvider(api_token="YOUR_ONFIDO_TOKEN")

# 1. Register the applicant
applicant = KYCApplicant(
    id="local_db_id",
    user_id="user_123",
    first_name="John",
    last_name="Doe",
    email="john@example.com",
    country_iso3="USA"
)
onfido_applicant_id = await provider.create_applicant(applicant)

# 2. Generate a secure link for the frontend SDK
sdk_link = await provider.generate_verification_link(
    provider_applicant_id=onfido_applicant_id,
    redirect_url="https://yourapp.com/kyc/complete"
)
print(f"Send this link to the user's frontend: {sdk_link}")
```

## Ecosystem Integration
The KYC module works perfectly with the **Cloud & IaC** module. For instance, when you need to archive the verified identity documents, you can use the Cloud module to generate S3 Presigned URLs, ensuring the files are stored securely without touching your application servers.
