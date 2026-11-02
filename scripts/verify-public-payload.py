"""Verify a separate installed public suite against its reviewed complete payload."""
from pathlib import Path
import argparse
import json
import re
import zipfile

from release_contract import APPS, sha, verify_installed_metadata


def verify(payload, installation, language):
    payload, installation = Path(payload).resolve(), Path(installation).resolve()
    if payload == installation:
        raise ValueError('Payload and installation must be distinct')
    data = installation / 'ArtCraftLauncherData/data'
    installed = json.loads((data / 'installed.json').read_text(encoding='utf-8-sig'))
    metadata = json.loads((data / 'bundle.json').read_text(encoding='utf-8-sig'))
    summary = json.loads((payload / 'validation/public-review-summary.json').read_text(encoding='utf-8-sig'))
    if set(summary['applications']) != APPS:
        raise ValueError('Thirteen reviewed manifest applications required')
    if (summary['release_tag'] != metadata['tag'] or summary['bundle_revision'] != metadata['revision']
            or summary['launcher_sha256'] != metadata['launcher']['sha256']):
        raise ValueError('Installed bundle metadata differs from the reviewed payload')
    verify_installed_metadata(installation, metadata, installed)
    for name, app in summary['applications'].items():
        component = metadata['applications'][name]
        if not app['reviewed'] or app['upstream_version'] != component['version'] or app['gui_sha256'] != component['sha256']:
            raise ValueError(f'Reviewed application metadata differs: {name}')
    checked = {}
    for file in sorted(payload.rglob('*')):
        if not file.is_file():
            continue
        relative = file.relative_to(payload)
        if relative.as_posix() in ['ArtCraftLauncherData/app.ron', 'ArtCraftLauncherData/app-en.ron']:
            continue
        target = installation / relative
        if not target.is_file() or sha(file) != sha(target):
            raise ValueError(f'Installed payload mismatch: {relative}')
        checked[relative.as_posix()] = sha(target)
    prefs = json.loads((installation / 'ArtCraftLauncherData/app.ron').read_text(encoding='utf-8-sig'))
    if not re.search(r'\blanguage\s*:\s*"' + language + r'"', prefs['settings']):
        raise ValueError('Selected initial language mismatch')
    archive = installation / 'sources/Creative-Suite-Arabic-Sources.zip'
    if sha(archive) != summary['source_archive_sha256']:
        raise ValueError('Public source archive hash mismatch')
    with zipfile.ZipFile(archive) as source:
        if source.testzip() is not None:
            raise ValueError('Public source ZIP CRC failed')
        manifests = [n for n in source.namelist() if n.endswith('source-manifest.json')]
        if len(manifests) != 1:
            raise ValueError('Expected one public source manifest')
        provenance = json.loads(source.read(manifests[0]))
        if {entry['name'] for entry in provenance['repositories']} != APPS | {'craft-launcher'}:
            raise ValueError('Public source manifest must contain fourteen matching workspaces')
        pins = {entry['name']: entry for entry in provenance['repositories']}
        for name, component in metadata['applications'].items():
            if pins[name]['releasedTag'] != 'v' + component['version']:
                raise ValueError(f'Installed component and public source release differ: {name}')
        if pins['craft-launcher']['releasedTag'] != 'v' + metadata['launcher']['version']:
            raise ValueError('Installed launcher and public source release differ')
    forbidden = {'concrt140.dll', 'msvcp140.dll', 'msvcp140_1.dll', 'msvcp140_2.dll',
                 'msvcp140_atomic_wait.dll', 'msvcp140_codecvt_ids.dll', 'vccorlib140.dll',
                 'vcruntime140.dll', 'vcruntime140_1.dll', 'vcruntime140_threads.dll'}
    if any(p.name.lower() in forbidden or p.suffix.lower() == '.pdb' for p in installation.rglob('*')):
        raise ValueError('Microsoft CRT file or debug symbols found in public installation')
    return {'installer_language': language, 'reviewed_applications': len(APPS),
            'files_verified': len(checked), 'bundle_revision': metadata['revision'],
            'manifest_paths_relative_contained': True, 'public_source_crc': 'pass',
            'crt_omitted': True, 'payload_sha256': checked}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--payload', type=Path, required=True)
    parser.add_argument('--installation', type=Path, required=True)
    parser.add_argument('--language', choices=['ar', 'en'], required=True)
    parser.add_argument('--report', type=Path, required=True)
    args = parser.parse_args()
    report = verify(args.payload, args.installation, args.language)
    args.report.parent.mkdir(parents=True, exist_ok=True)
    args.report.write_text(json.dumps(report, ensure_ascii=False, indent=2), encoding='utf-8')
    print(f'{args.language}: verified {report["files_verified"]} files, thirteen reviewed GUI paths, bundle metadata, source archive and selected preferences')


if __name__ == '__main__':
    main()
