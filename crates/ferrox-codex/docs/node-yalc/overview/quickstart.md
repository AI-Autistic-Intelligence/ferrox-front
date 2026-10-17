---
id: quickstart
title: Quickstart Guide & Workspace Integration
sidebar_position: 2
---

# Quickstart Guide & Workspace Integration

This quickstart guide demonstrates how to install and integrate **Node-YALC** packages into any Node.js application, serverless function, or backend service.

---

## 1. Installation

Install individual `@node-yalc` packages from your private Yalc registry or npm monorepo setup:

```bash
# Core Utilities, Errors, and Logging
npm install @node-yalc/logger @node-yalc/errors @node-yalc/utils @node-yalc/common

# AWS Cloud Automation Helpers
npm install @node-yalc/aws-helpers

# Type Definitions & Contracts
npm install @node-yalc/interfaces @node-yalc/types @node-yalc/types-extends --save-dev
```

---

## 2. Step 1: Initialize Structured Logger & Correlation Tracking

Initialize the Pino-backed `YalcLogger` in your application entry point (`src/index.ts`):

```typescript
import { YalcLogger } from '@node-yalc/logger';
import { AppEnvEnum } from '@node-yalc/common';

const logger = new YalcLogger({
  serviceName: 'order-processor',
  environment: AppEnvEnum.PRODUCTION,
  level: 'info',
  redactPaths: ['req.headers.authorization', 'user.password', 'creditCard.cvv'],
});

logger.info('Order Processor initialized successfully');
```

---

## 3. Step 2: Handle Errors with Standardized Taxonomy

Use `@node-yalc/errors` to throw type-safe exceptions with structured error codes and HTTP status mappings:

```typescript
import { NotFoundError, BadRequestError } from '@node-yalc/errors';

export function processOrder(orderId: string, items: any[]) {
  if (!orderId) {
    throw new BadRequestError('Order ID must be specified', {
      errorCode: 'ERR_ORDER_INVALID_ID',
    });
  }

  const order = findOrder(orderId);
  if (!order) {
    throw new NotFoundError(`Order with ID ${orderId} does not exist`, {
      errorCode: 'ERR_ORDER_NOT_FOUND',
      context: { orderId },
    });
  }

  return order;
}
```

---

## 4. Step 3: Fetch Cached AWS SSM Parameters

Use `@node-yalc/aws-helpers` to retrieve encrypted parameters from AWS Systems Manager (SSM) with TTL memory caching:

```typescript
import { getSSMParameter } from '@node-yalc/aws-helpers';

export async function getDatabasePassword(): Promise<string> {
  // Fetches from AWS SSM Parameter Store with KMS decryption
  // Caches result in memory for 300 seconds (5 minutes)
  const password = await getSSMParameter('/prod/database/password', {
    decrypt: true,
    ttlSeconds: 300,
    region: 'us-east-1',
  });

  return password;
}
```

---

## 5. Step 4: Execute Throttled Concurrent Async Workers

Use `runConcurrently()` from `@node-yalc/utils` to process asynchronous jobs with strict concurrency limits:

```typescript
import { runConcurrently } from '@node-yalc/utils';
import { YalcLogger } from '@node-yalc/logger';

const logger = new YalcLogger({ serviceName: 'batch-worker' });

async function processOrderBatch(orderIds: string[]) {
  logger.info(`Starting batch processing for ${orderIds.length} orders...`);

  const results = await runConcurrently(
    orderIds,
    async (orderId) => {
      logger.debug(`Processing order ${orderId}...`);
      return await processOrderInDatabase(orderId);
    },
    {
      concurrency: 5, // Maximum 5 simultaneous workers
      stopOnError: false, // Continue executing remaining queue items on error
    }
  );

  const succeeded = results.filter(r => r.status === 'fulfilled');
  const failed = results.filter(r => r.status === 'rejected');

  logger.info(`Batch processing completed. Succeeded: ${succeeded.length}, Failed: ${failed.length}`);
}
```

---

## 6. Verification

Run your entry script:

```bash
npx ts-node src/index.ts
```

Inspect the structured JSON log output:

```json
{
  "level": 30,
  "time": 1726689600000,
  "pid": 12345,
  "hostname": "worker-node-1",
  "serviceName": "order-processor",
  "environment": "production",
  "msg": "Order Processor initialized successfully"
}
```

---

## 7. Next Steps

- Explore deep package documentation for aws-helpers, logger, and utils.


---

## 🔗 Cross-References

To see how this module integrates with the rest of the Ferrox architecture, refer to the following documentation:

- [Database & TypeORM](../../../nestjs-yalc/docs/databases/database.md)
- [Node-YALC Errors](../fundamentals/errors.md)
- [Observability & Logger](../../../nestjs-yalc/docs/observability/logger.md)
