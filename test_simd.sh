#!/bin/bash

set -e

echo "📦 Checking if Rust nightly is installed..."
if ! rustup show | grep -q "nightly"; then
    echo "🚨 Rust nightly is not installed. Installing now..."
    rustup install nightly
else
    echo "✅ Rust nightly is already installed."
fi

echo
echo "🧱 Creating 'users' table..."
cargo run -- create-table users id:int name:string age:int

echo
echo "📥 Inserting rows into 'users' table..."
cargo run -- insert users 1 "Alice" 25
cargo run -- insert users 2 "Bob" 32
cargo run -- insert users 3 "Charlie" 43
cargo run -- insert users 4 "Daisy" 54
cargo run -- insert users 5 "Eve" 65
cargo run -- insert users 6 "Frank" 35
cargo run -- insert users 7 "Grace" 54
cargo run -- insert users 8 "Heidi" 19
cargo run -- insert users 9 "Ivan" 20
cargo run -- insert users 10 "Judy" 21
cargo run -- insert users 11 "Karl" 20
cargo run -- insert users 12 "Laura" 22

echo
echo "🔍 Scanning 'age' column..."
cargo run -- scan users age

echo
echo "🚀 Testing x86 SIMD filters..."
cargo run -- filter-simd-eq users age 54
cargo run -- filter-simd-not-eq users age 25
cargo run -- filter-simd-gt users age 30
cargo run -- filter-simd-lt users age 30
cargo run -- filter-simd-lt-eq users age 30
cargo run -- filter-simd-gt-eq users age 30
cargo run -- filter-simd-logical users age gt 25 age lt 54 or

echo
echo "⚡ Testing AVX2 SIMD filters..."
cargo run -- filter-simd-eq-avx users age 20
cargo run -- filter-simd-not-eq-avx users age 20
cargo run -- filter-simd-gt-avx users age 20
cargo run -- filter-simd-lt-avx users age 20
cargo run -- filter-simd-gt-eq-avx users age 20
cargo run -- filter-simd-lt-eq-avx users age 20

echo
echo "📋 Listing all tables..."
cargo run -- list-tables

echo
echo "✅ All tests completed successfully."