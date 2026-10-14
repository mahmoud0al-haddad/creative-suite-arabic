"""Verify the pinned public build inputs against the release source manifest."""
from pathlib import Path
import argparse,hashlib,json

def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root',type=Path,default=Path(__file__).resolve().parent.parent)
    args=parser.parse_args()
    root=args.root.resolve()
    manifest=json.loads((root/'source-manifest.json').read_text(encoding='utf-8-sig'))
    pins=json.loads((root/'source-pins.json').read_text(encoding='utf-8-sig'))
    declared={r['name']:r['upstream_commit'] for r in pins['repositories']}
    exported={r['name']:r['upstreamCommit'] for r in manifest['repositories']}
    if len(exported)!=13 or exported!=declared:raise ValueError('Source pins differ from export provenance')
    checked=set()
    for item in manifest['files']:
        relative=item['path']
        original=root/relative
        file=original.resolve()
        if original.is_symlink() or not file.is_relative_to(root) or relative in checked:raise ValueError('Invalid or repeated manifest path')
        if not file.is_file() or file.stat().st_size!=item['bytes']:raise ValueError(f'Missing or changed input: {relative}')
        with file.open('rb') as handle:actual=hashlib.file_digest(handle,'sha256').hexdigest()
        if actual!=item['sha256']:raise ValueError(f'Checksum mismatch: {relative}')
        checked.add(relative)
    print(f'Verified {len(checked)} source/input files and all thirteen upstream pins.')

if __name__=='__main__':main()
