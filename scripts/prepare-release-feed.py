"""Create the update feed only after the reviewed complete installer has been built."""
import argparse
import json
from pathlib import Path
from release_contract import FEED_NAME, make_feed, verify_installed_metadata

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--payload', type=Path, required=True)
    parser.add_argument('--setup', type=Path, required=True)
    parser.add_argument('--destination', type=Path, required=True)
    args = parser.parse_args()
    metadata = json.loads((args.payload / 'ArtCraftLauncherData/data/bundle.json').read_text(encoding='utf-8'))
    installed = json.loads((args.payload / 'ArtCraftLauncherData/data/installed.json').read_text(encoding='utf-8-sig'))
    verify_installed_metadata(args.payload, metadata, installed)
    feed = make_feed(metadata, args.setup)
    if args.destination.name != FEED_NAME or args.destination.exists():
        raise ValueError('Use a new destination named ' + FEED_NAME)
    args.destination.parent.mkdir(parents=True, exist_ok=True)
    args.destination.write_text(json.dumps(feed, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    print(f'Feed prepared for {metadata["tag"]}, revision {metadata["revision"]}; installer and every component SHA-256 verified')

if __name__ == '__main__':
    main()
