#!/usr/bin/env bash

rsync_url() {
    if [ $# -ne 2 ]; then
        echo "Usage: rsync_url <URL> <destination>" >&2
        return 2
    fi

    url="$1"
    dest="$2"

    # Create a temporary file
    tmp=$(mktemp) || {
        echo "Error: mktemp failed" >&2
        return 1
    }

    # Ensure the temporary file is removed on function exit
    trap 'rm -f "$tmp"' RETURN

    # Download the URL to the temporary file using curl or wget
    if command -v curl >/dev/null 2>&1; then
        curl -fsSL "$url" -o "$tmp" || {
            echo "Error: download via curl failed" >&2
            return 1
        }
    elif command -v wget >/dev/null 2>&1; then
        wget -qO "$tmp" "$url" || {
            echo "Error: download via wget failed" >&2
            return 1
        }
    else
        echo "Error: neither curl nor wget is installed." >&2
        return 1
    fi

    # Sync the temporary file to the destination with rsync and checksum
    rsync -c "$tmp" "$dest" || {
        echo "Error: rsync failed" >&2
        return 1
    }

    return 0
}

rsync -c csaf/csaf_2.0/json_schema/csaf_json_schema.json csaf-rs/assets/csaf_2.0_json_schema.json
rsync -c csaf/csaf_2.1/json_schema/csaf.json csaf-rs/assets/csaf_2.1_json_schema.json
rsync -c ssvc/data/schema/v1/Decision_Point-1-0-1.schema.json csaf-rs/assets/decision_point_1.0.1_json_schema.json
rsync_url https://www.iana.org/assignments/language-subtag-registry/language-subtag-registry csaf-rs/assets/language-subtag-registry.txt
rsync -cr --delete ssvc/data/json/decision_points/ csaf-rs/assets/ssvc_decision_points/
