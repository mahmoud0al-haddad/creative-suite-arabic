"""The published Arabic suite contract, shared by packaging and release tooling."""
from pathlib import Path, PureWindowsPath
import hashlib
import re

REPOSITORY = 'mahmoud0al-haddad/creative-suite-arabic'
EDITION = 'creative-suite-arabic-community'
PLATFORM = 'windows-x64'
INSTALL_ID = '149F3153-387D-4B57-9478-74333EAA5D28'
SETUP_NAME = 'Creative-Suite-Arabic-Windows-x64-Setup.exe'
FEED_NAME = 'arabic-suite-release.json'
MAX_SETUP_SIZE = 2 * 1024 * 1024 * 1024
APPS = frozenset({'cadcraft', 'deckcraft', 'designcraft', 'effectcraft', 'figcraft',
                  'filmcraft', 'gridcraft', 'lightcraft', 'pdfcraft', 'photocraft',
                  'soundcraft', 'vectorcraft', 'wordcraft'})
METADATA_FIELDS = {'schema', 'edition', 'platform', 'install_id', 'revision', 'tag', 'launcher', 'applications'}

def sha(path):
    with Path(path).open('rb') as source:
        return hashlib.file_digest(source, 'sha256').hexdigest()

def valid_tag(tag):
    match = re.fullmatch(r'v([0-9]{4})\.([0-9]{1,2})-ar\.([0-9]+)', tag) if isinstance(tag, str) else None
    return bool(match and 2020 <= int(match[1]) <= 9999 and 1 <= int(match[2]) <= 12 and 1 <= int(match[3]) <= 2**32 - 1)

def valid_component(value):
    return (isinstance(value, dict) and set(value) == {'version', 'sha256'}
            and isinstance(value['version'], str)
            and bool(re.fullmatch(r'(?:0|[1-9][0-9]*)\.(?:0|[1-9][0-9]*)\.(?:0|[1-9][0-9]*)', value['version']))
            and isinstance(value['sha256'], str) and bool(re.fullmatch(r'[0-9a-fA-F]{64}', value['sha256'])))

def validate_metadata(metadata):
    if not isinstance(metadata, dict) or set(metadata) != METADATA_FIELDS:
        raise ValueError('Unsupported Arabic bundle metadata fields')
    if (type(metadata['schema']) is not int or metadata['schema'] != 1
            or metadata['edition'] != EDITION or metadata['platform'] != PLATFORM
            or metadata['install_id'] != INSTALL_ID):
        raise ValueError('Unsupported Arabic bundle edition or platform')
    if type(metadata['revision']) is not int or not 1 <= metadata['revision'] <= 2**64 - 1 or not valid_tag(metadata['tag']):
        raise ValueError('Published bundles require a positive monotonic revision and valid calendar tag')
    if not isinstance(metadata['applications'], dict) or set(metadata['applications']) != APPS:
        raise ValueError('All thirteen current applications are required')
    if not valid_component(metadata['launcher']) or not all(valid_component(app) for app in metadata['applications'].values()):
        raise ValueError('Every component requires its stable version and SHA-256')
    return metadata

def contained_executable(root, base, relative, name):
    if not isinstance(relative, str) or PureWindowsPath(relative).drive or PureWindowsPath(relative).root or Path(relative).is_absolute():
        raise ValueError(f'Absolute or invalid application path: {name}')
    path = (Path(base) / relative.replace('\\', '/')).resolve()
    if not path.is_relative_to(Path(root).resolve()) or path.name != name + '.exe' or not path.is_file():
        raise ValueError(f'Application path escapes its installation or has the wrong filename: {name}')
    return path

def make_metadata(root, status, tag, revision):
    root = Path(root).resolve()
    if status.get('launcher_arabic_reviewed') is not True:
        raise ValueError('Only the reviewed Arabic launcher can become a public bundle')
    if set(status['applications']) != APPS or not all(item.get('arabic_reviewed') is True for item in status['applications'].values()):
        raise ValueError('Only the complete reviewed Arabic stage can become a public bundle')
    launcher = root / 'artcraft-launcher.exe'
    if sha(launcher) != status['launcher_sha256'].lower():
        raise ValueError('Reviewed launcher changed before metadata generation')
    applications = {}
    for name, item in sorted(status['applications'].items()):
        if not item.get('executables'):
            raise ValueError(f'Missing reviewed executable: {name}')
        executable = contained_executable(root, root, item['executables'][0], name)
        actual = sha(executable)
        if actual != item['modified_gui_sha256'].lower():
            raise ValueError(f'Reviewed application changed before metadata generation: {name}')
        applications[name] = {'version': item['version'], 'sha256': actual}
    metadata = {'schema': 1, 'edition': EDITION, 'platform': PLATFORM, 'install_id': INSTALL_ID,
                'revision': revision, 'tag': tag, 'launcher': {'version': status['launcher_version'], 'sha256': sha(launcher)},
                'applications': applications}
    return validate_metadata(metadata)

def verify_installed_metadata(root, metadata, installed):
    validate_metadata(metadata)
    root = Path(root).resolve()
    if set(installed['apps']) != APPS:
        raise ValueError('Installed application inventory differs from the complete bundle')
    for name, component in metadata['applications'].items():
        record = installed['apps'][name]
        executable = contained_executable(root, root / 'ArtCraftLauncherData/data', record['path'], name)
        if record['version'] != component['version'] or sha(executable) != component['sha256'].lower():
            raise ValueError(f'Installed application version or SHA-256 differs: {name}')
    if sha(root / 'artcraft-launcher.exe') != metadata['launcher']['sha256'].lower():
        raise ValueError('Installed launcher SHA-256 differs')

def make_feed(metadata, setup):
    validate_metadata(metadata)
    setup = Path(setup)
    if setup.name != SETUP_NAME or not setup.is_file() or not 0 < setup.stat().st_size <= MAX_SETUP_SIZE:
        raise ValueError('Feed requires the complete named installer within the launcher download bound')
    return {'metadata': metadata, 'installer': {'name': SETUP_NAME,
            'url': f'https://github.com/{REPOSITORY}/releases/download/{metadata["tag"]}/{SETUP_NAME}',
            'size': setup.stat().st_size, 'sha256': sha(setup)}}
