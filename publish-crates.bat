@echo off
echo Pubblicazione su crates.io iniziata...
echo Assicurati di aver fatto 'cargo login' prima!

cd crates\ferrox-front-core
cargo publish
cd ..\..\

cd crates\ferrox-front-macro
cargo publish
cd ..\..\

cd crates\ferrox-front-ui
cargo publish
cd ..\..\

cd crates\ferrox-front-charts
cargo publish
cd ..\..\

cd crates\ferrox-front-security
cargo publish
cd ..\..\

cd crates\ferrox-front-ws
cargo publish
cd ..\..\

cd crates\ferrox-front-templates
cargo publish
cd ..\..\

echo.
echo Tutti i crate sono stati pubblicati!
