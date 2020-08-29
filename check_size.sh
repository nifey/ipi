#!/bin/bash
rm -r compress
wasm-pack build --target=web --release

mkdir compress
cp index.html compress
cp -r pkg compress
rm compress/pkg/{*.ts,README.md,package.json}

zip -r compress compress -9
echo -n "Size of compressed zip: "
ls -lh | grep "compress.zip" | cut -d" " -f 6
