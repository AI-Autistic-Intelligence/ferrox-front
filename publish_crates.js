/**
 * Topological Crate Publishing Script for ferrox-front
 * 
 * Publishes crates to crates.io in strict topological dependency order:
 * Level 0: ferrox-front-core, ferrox-front-macro
 * Level 1: ferrox-front-security, ferrox-front-charts, ferrox-front-ws
 * Level 2: ferrox-front-router, ferrox-front-ui
 * Level 3: ferrox-front-templates
 */

const { execSync } = require('child_process');
const path = require('path');

const PUBLISH_ORDER = [
  'crates/ferrox-front-core',
  'crates/ferrox-front-macro',
  'crates/ferrox-front-security',
  'crates/ferrox-front-charts',
  'crates/ferrox-front-ws',
  'crates/ferrox-front-router',
  'crates/ferrox-front-ui',
  'crates/ferrox-front-templates',
];

const isDryRun = process.argv.includes('--dry-run');

console.log(`Starting ${isDryRun ? 'DRY-RUN' : 'LIVE'} publish sequence for ferrox-front...\n`);

for (const cratePath of PUBLISH_ORDER) {
  const crateName = path.basename(cratePath);
  console.log(`📦 Publishing [${crateName}] from ${cratePath}...`);

  const cmd = isDryRun
    ? `cargo publish --allow-dirty --dry-run -p ${crateName}`
    : `cargo publish --allow-dirty -p ${crateName}`;

  try {
    execSync(cmd, { stdio: 'inherit' });
    console.log(`✅ Success for ${crateName}\n`);
    if (!isDryRun) {
      // Sleep 15s to allow crates.io index propagation
      console.log('⏳ Waiting 15s for crates.io index propagation...');
      execSync('sleep 15 || timeout /t 15', { stdio: 'ignore' });
    }
  } catch (error) {
    console.error(`❌ Failed to publish ${crateName}:`, error.message);
    process.exit(1);
  }
}

console.log('🎉 All ferrox-front crates published successfully!');
