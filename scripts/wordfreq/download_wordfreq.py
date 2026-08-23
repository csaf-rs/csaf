#!/usr/bin/env python3
"""Download and process word frequency data from pyspellchecker."""

import argparse
import gzip
import json
import sys
import urllib.request
from pathlib import Path

def download_and_process(url, top_n, output_file):
    """Download gzipped JSON, process, and save top N words by frequency."""
    print(f"Downloading from {url}...")
    
    try:
        # Download the gzipped file
        with urllib.request.urlopen(url) as response:
            gzipped_data = response.read()
    except Exception as e:
        print(f"Error downloading file: {e}", file=sys.stderr)
        return False
    
    print("Extracting gzipped data...")
    # Decompress
    json_data = gzip.decompress(gzipped_data).decode('utf-8')
    
    print("Parsing JSON...")
    # Parse JSON
    words_dict = json.loads(json_data)
    
    print(f"Sorting {len(words_dict)} words by frequency...")
    # Sort desc by frequency
    sorted_words = sorted(words_dict.items(), key=lambda x: x[1], reverse=True)
    
    # Get top N
    top_words = sorted_words[:top_n]

    output_file = Path(output_file)
    if output_file.is_dir():
        print(f"Error: output path '{output_file}' is a directory, not a file.", file=sys.stderr)
        return False
    output_file.parent.mkdir(parents=True, exist_ok=True)

    # Write to file
    print(f"Writing top {len(top_words)} words to {output_file}...")
    with open(output_file, 'w', encoding='utf-8') as f:
        for word, frequency in top_words:
            f.write(f"{word} {frequency}\n")

    return True


def main():
    parser = argparse.ArgumentParser(
        description="Download and process word frequency data from pyspellchecker."
    )
    parser.add_argument(
        "-u", "--url",
        required=True,
        help="URL to the gzipped word frequency JSON file"
    )
    parser.add_argument(
        "-n", "--top-n",
        type=int,
        required=True,
        help="Number of top words to extract"
    )
    parser.add_argument(
        "-o", "--output",
        required=True,
        help="Full path to write the output file to"
    )
    
    args = parser.parse_args()
    
    success = download_and_process(args.url, args.top_n, args.output)
    sys.exit(0 if success else 1)


if __name__ == "__main__":
    main()
