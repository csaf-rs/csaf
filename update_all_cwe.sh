#!/usr/bin/env bash
set -euxo pipefail

versions=(
  v1.0
  v1.0.1
  v1.1
  v1.2
  v1.3
  v1.4
  v1.5
  v1.6
  v1.7
  v1.8
  v1.8.1
  v1.9
  v1.10
  v1.11
  v1.12
  v1.13
  v2.0
  v2.1
  v2.2
  v2.3
  v2.4
  v2.5
  v2.6
  v2.7
  v2.8
  v2.9
  v2.10
  v2.11
  v2.12
  v3.0
  v3.1
  v3.2
  v3.3
  v3.4
  v3.4.1
  v4.0
  v4.1
  v4.2
  v4.3
  v4.4
  v4.5
  v4.6
  v4.7
  v4.8
  v4.9
  v4.10
  v4.11
  v4.12
  v4.13
  v4.14
  v4.15
  v4.16
  v4.17
  v4.18
  v4.19
  v4.19.1
  latest
)

for version in "${versions[@]}"; do
  ./update_cwe.sh "$version"
done
