"""Read-only verification of sources/ and build-inputs/ against source-checksums.sha256."""
import argparse, hashlib, re
from pathlib import Path

def verify(root):
    root=root.resolve()
    entries=(root/'source-checksums.sha256').read_text(encoding='utf-8-sig').splitlines()
    recorded=set()
    for line in entries:
        if not line.strip():continue
        match=re.fullmatch(r'([0-9a-fA-F]{64})  (.+)',line)
        if not match:raise ValueError('Malformed checksum row')
        relative=Path(match[2])
        if relative.is_absolute() or '..' in relative.parts or relative.parts[0] not in ('sources','build-inputs'):
            raise ValueError('Checksum path outside source inputs')
        path=(root/relative).resolve()
        if not path.is_relative_to(root) or not path.is_file():raise ValueError(f'Missing/escaping source input: {relative}')
        name=relative.as_posix()
        if name in recorded:raise ValueError(f'Duplicate checksum row: {relative}')
        recorded.add(name)
        with path.open('rb') as handle:
            actual=hashlib.file_digest(handle,'sha256').hexdigest()
        if actual!=match[1].lower():raise ValueError(f'Source checksum mismatch: {relative}')
    actual_files={p.relative_to(root).as_posix() for folder in ['sources','build-inputs'] for p in (root/folder).rglob('*') if p.is_file() and not any(part in ('.git','.build','target','__pycache__') for part in p.relative_to(root).parts)}
    if not recorded or actual_files!=recorded:
        raise ValueError(f'Source file set differs from checksum index: {len(actual_files-recorded)} unrecorded, {len(recorded-actual_files)} absent')
    return len(recorded)

def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root',type=Path,default=Path(__file__).resolve().parent.parent)
    args=parser.parse_args()
    print(f'Verified {verify(args.root)} exported source/build-input checksums. No files changed.')

if __name__=='__main__':main()
