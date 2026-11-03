"""Prepare a separate public payload without proprietary Microsoft runtime redistribution."""
import argparse, hashlib, json, shutil, zipfile
from pathlib import Path
from release_contract import APPS, make_metadata, verify_installed_metadata

CRT = {x.lower() for x in (
    'concrt140.dll', 'msvcp140.dll', 'msvcp140_1.dll', 'msvcp140_2.dll',
    'msvcp140_atomic_wait.dll', 'msvcp140_codecvt_ids.dll', 'vccorlib140.dll',
    'vcruntime140.dll', 'vcruntime140_1.dll', 'vcruntime140_threads.dll')}

def sha(path):
    digest = hashlib.sha256()
    with path.open('rb') as handle:
        for block in iter(lambda: handle.read(1024 * 1024), b''):
            digest.update(block)
    return digest.hexdigest()

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--reviewed-stage', type=Path, required=True)
    parser.add_argument('--destination', type=Path, required=True)
    parser.add_argument('--public-source-zip', type=Path, required=True)
    parser.add_argument('--tag', required=True)
    parser.add_argument('--revision', type=int, required=True)
    args = parser.parse_args()
    source, dest, source_zip = (p.resolve() for p in (args.reviewed_stage, args.destination, args.public_source_zip))
    if dest.exists() or dest == source or dest.is_relative_to(source) or source.is_relative_to(dest):
        raise ValueError('Destination must be a new directory separate from the reviewed stage.')
    if not source_zip.is_file():
        raise ValueError('Supply the audited public source ZIP.')
    with zipfile.ZipFile(source_zip) as archive:
        if archive.testzip() is not None:
            raise ValueError('Public source ZIP has a CRC error.')
        names = archive.namelist()
        if any(Path(n).is_absolute() or '..' in Path(n).parts for n in names):
            raise ValueError('Unsafe member path in public source archive.')
        if not any(n.endswith('source-manifest.json') for n in names):
            raise ValueError('Public source ZIP must contain the audited source-manifest.json.')
        manifests = [n for n in names if n.endswith('source-manifest.json')]
        if len(manifests) != 1:
            raise ValueError('Expected exactly one public source manifest.')
        public_manifest = json.loads(archive.read(manifests[0]))
        if {entry['name'] for entry in public_manifest['repositories']} != APPS | {'craft-launcher'}:
            raise ValueError('Public source manifest must contain the matching fourteen workspaces.')
        if any(Path(n).name.lower() in CRT for n in names):
            raise ValueError('Runtime DLL found in source archive.')
    status = json.loads((source / 'package-status.json').read_text(encoding='utf-8-sig'))
    applications = status['applications']
    if set(applications) != APPS or not all(a.get('arabic_reviewed') for a in applications.values()):
        raise ValueError('Thirteen reviewed applications are required.')
    metadata = make_metadata(source, status, args.tag, args.revision)
    public_pins = {entry['name']: entry for entry in public_manifest['repositories']}
    for name, item in applications.items():
        if (public_pins[name]['upstreamCommit'] != item['upstream_commit']
                or public_pins[name]['releasedTag'] != 'v' + item['version']):
            raise ValueError(f'Reviewed application and public source pin differ: {name}')
    if (public_pins['craft-launcher']['upstreamCommit'] != status['launcher_upstream_commit']
            or public_pins['craft-launcher']['releasedTag'] != 'v' + status['launcher_version']):
        raise ValueError('Reviewed launcher and public source pin differ')
    if sha(source / 'artcraft-launcher.exe') != status['launcher_sha256']:
        raise ValueError('Reviewed launcher hash mismatch.')
    for name, app in applications.items():
        executable = (source / app['executables'][0]).resolve()
        if not executable.is_relative_to(source) or sha(executable) != app['modified_gui_sha256']:
            raise ValueError(f'Reviewed application hash mismatch: {name}')
    removed = []
    def ignored(directory, names):
        ignore = []
        for name in names:
            path = Path(directory) / name
            if path.is_symlink():
                raise ValueError('Symlinks are not accepted in the reviewed payload.')
            if name.lower() in CRT or name.lower().endswith('.pdb'):
                ignore.append(name)
                removed.append(str(path.relative_to(source)).replace('\\', '/'))
        return ignore
    dest.mkdir(parents=True)
    for name in ['artcraft-launcher.exe', 'portable.txt']:
        shutil.copy2(source / name, dest / name)
    shutil.copytree(source / 'apps', dest / 'apps', ignore=ignored)
    shutil.copytree(source / 'licenses', dest / 'licenses', ignore=lambda directory, names: [n for n in names if n.lower() == 'microsoft-runtime'])
    config = dest / 'ArtCraftLauncherData'
    (config / 'data').mkdir(parents=True)
    for name in ['app.ron', 'app-en.ron', 'personal-suite.txt']:
        shutil.copy2(source / 'ArtCraftLauncherData' / name, config / name)
    shutil.copy2(source / 'ArtCraftLauncherData/data/installed.json', config / 'data/installed.json')
    (config / 'data/bundle.json').write_text(json.dumps(metadata, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    installed = json.loads((config / 'data/installed.json').read_text(encoding='utf-8-sig'))
    verify_installed_metadata(dest, metadata, installed)
    docs = Path(__file__).resolve().parent.parent
    for lang, readme in [('AR', 'README.md'), ('EN', 'README.en.md')]:
        shutil.copy2(docs / readme, dest / f'README-{lang}.txt')
    (dest / 'sources').mkdir()
    shutil.copy2(source_zip, dest / 'sources/Creative-Suite-Arabic-Sources.zip')
    (dest / 'validation').mkdir()
    summary = {
        'edition': 'Independent Arabic/English edition',
        'release_tag': args.tag,
        'bundle_revision': args.revision,
        'launcher_sha256': status['launcher_sha256'],
        'applications': {n: {'upstream_version': a['version'], 'gui_sha256': a['modified_gui_sha256'], 'reviewed': True} for n, a in applications.items()},
        'runtime': {'distribution': 'No Microsoft CRT DLLs or redistributable installer bundled', 'prerequisite': 'Independently installed x64 v14 runtime >= 14.44.35207.0'},
        'source_archive_sha256': sha(source_zip),
        'checks': 'Reviewed pinned GUI build/test and Arabic/English native workflows; public prerequisite installer needs its separate clean-machine validation record.',
        'limitations': 'Representative workflows, not every feature/device/plugin. Original optional PdfCraft cargo-deny audit was skipped.',
        'excluded_runtime_and_symbols': sorted(removed),
    }
    (dest / 'validation/public-review-summary.json').write_text(json.dumps(summary, ensure_ascii=False, indent=2), encoding='utf-8')
    (dest / 'validation/THIRD-PARTY-RUNTIME.txt').write_text(
        "Microsoft Visual C++ x64 v14 Runtime 14.44.35207.0 or newer is an independently installed prerequisite.\n"
        "This public package contains no Microsoft CRT DLLs or redistributable installer.\n"
        "Official download and license information: https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist\n",
        encoding='utf-8')
    if any(p.name.lower() in CRT for p in dest.rglob('*')):
        raise ValueError('Public payload unexpectedly contains a CRT DLL.')
    print(json.dumps({'applications': len(APPS), 'removed_runtime_or_symbols': len(removed), 'public_source_sha256': sha(source_zip)}, indent=2))

if __name__ == '__main__':
    main()
