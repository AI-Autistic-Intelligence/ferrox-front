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
    ? `cargo publish --dry-run -p ${crateName}`
    : `cargo publish -p ${crateName}`;

  let published = false;
  let attempts = 0;
  const maxAttempts = 4;

  while (!published && attempts < maxAttempts) {
    attempts++;
    try {
      const output = execSync(cmd, { stdio: 'pipe', encoding: 'utf8' });
      console.log(output);
      console.log(`✅ Success for ${crateName}\n`);
      published = true;
      if (!isDryRun) {
        console.log('⏳ Waiting 20s for crates.io index propagation...');
        execSync('sleep 20 || timeout /t 20', { stdio: 'ignore' });
      }
    } catch (error) {
      const out = (error.stdout || '') + '\n' + (error.stderr || '') + '\n' + error.message;
      console.log(out);
      if (out.includes('already exists') || out.includes('is already uploaded')) {
        console.log(`ℹ️ ${crateName} is already published on crates.io, continuing...\n`);
        published = true;
      } else if (out.includes('failed to select a version for the requirement') && attempts < maxAttempts) {
        console.log(`⏳ crates.io index not yet updated for dependencies of ${crateName}. Retrying in 20s (attempt ${attempts}/${maxAttempts})...`);
        execSync('sleep 20 || timeout /t 20', { stdio: 'ignore' });
      } else {
        console.error(`❌ Failed to publish ${crateName}:`, error.message);
        process.exit(1);
      }
    }
  }
}

console.log('🎉 All ferrox-front crates published successfully!');
