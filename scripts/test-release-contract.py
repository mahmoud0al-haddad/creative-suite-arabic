"""Packaging boundary regressions using synthetic files, without running any application."""
import copy
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
import shutil
import zipfile

from release_contract import (APPS, FEED_NAME, SETUP_NAME, contained_executable,
                              make_feed, make_metadata, sha, validate_metadata,
                              verify_installed_metadata)


class ReleaseContractTests(unittest.TestCase):
    def setUp(self):
        self.directory = tempfile.TemporaryDirectory(prefix="Arabic O'Brien ")
        self.addCleanup(self.directory.cleanup)
        self.root = Path(self.directory.name) / 'العربية'
        self.data = self.root / 'ArtCraftLauncherData/data'
        self.data.mkdir(parents=True)
        launcher = self.root / 'artcraft-launcher.exe'
        launcher.write_bytes(b'synthetic launcher; never executed')
        self.status = {'launcher_version': '0.4.1', 'launcher_sha256': sha(launcher), 'launcher_arabic_reviewed': True, 'applications': {}}
        self.installed = {'apps': {}}
        for name in APPS:
            relative = f'apps/{name}/{name}.exe'
            binary = self.root / relative
            binary.parent.mkdir(parents=True)
            binary.write_text('synthetic application ' + name, encoding='utf-8')
            self.status['applications'][name] = {'version': '0.7.0', 'arabic_reviewed': True,
                                                'executables': [relative], 'modified_gui_sha256': sha(binary)}
            self.installed['apps'][name] = {'version': '0.7.0', 'path': '../../' + relative}
        self.metadata = make_metadata(self.root, self.status, 'v2026.10-ar.1', 1)
        self.setup = self.root / SETUP_NAME
        self.setup.write_bytes(b'synthetic complete setup; never executed')

    def test_complete_inventory_and_feed_match_real_file_bytes(self):
        verify_installed_metadata(self.root, self.metadata, self.installed)
        feed = make_feed(self.metadata, self.setup)
        self.assertEqual(feed['installer']['sha256'], sha(self.setup))
        self.assertEqual(feed['installer']['size'], self.setup.stat().st_size)
        self.assertEqual(feed['metadata']['applications'].keys(), APPS)
        self.assertIn('/mahmoud0al-haddad/creative-suite-arabic/releases/download/v2026.10-ar.1/', feed['installer']['url'])

    def test_source_review_does_not_accept_changed_or_unreviewed_files(self):
        for fault in ['review', 'application', 'launcher', 'launcher_review']:
            status = copy.deepcopy(self.status)
            if fault == 'review':
                status['applications']['pdfcraft']['arabic_reviewed'] = False
            elif fault == 'application':
                status['applications']['pdfcraft']['modified_gui_sha256'] = '0' * 64
            elif fault == 'launcher':
                status['launcher_sha256'] = '0' * 64
            else:
                status['launcher_arabic_reviewed'] = False
            with self.subTest(fault=fault), self.assertRaises(ValueError):
                make_metadata(self.root, status, 'v2026.10-ar.1', 1)

    def test_rejects_unsupported_or_partial_metadata(self):
        for fault in ['missing_app', 'extra_app', 'revision_zero', 'boolean_revision', 'foreign', 'platform', 'unknown', 'bad_tag', 'oversized_tag', 'hash', 'version']:
            metadata = copy.deepcopy(self.metadata)
            if fault == 'missing_app':
                del metadata['applications']['figcraft']
            elif fault == 'extra_app':
                metadata['applications']['unreviewedcraft'] = metadata['applications']['figcraft'].copy()
            elif fault == 'revision_zero':
                metadata['revision'] = 0
            elif fault == 'boolean_revision':
                metadata['revision'] = True
            elif fault == 'foreign':
                metadata['install_id'] = 'another-installation'
            elif fault == 'platform':
                metadata['platform'] = 'linux-x64'
            elif fault == 'unknown':
                metadata['unexpected'] = 'not supported'
            elif fault == 'bad_tag':
                metadata['tag'] = 'v2026.13-ar.1'
            elif fault == 'oversized_tag':
                metadata['tag'] = 'v2026.10-ar.4294967296'
            elif fault == 'hash':
                metadata['applications']['pdfcraft']['sha256'] = 'not a checksum'
            else:
                metadata['launcher']['version'] = '0.4.1-beta'
            with self.subTest(fault=fault), self.assertRaises(ValueError):
                validate_metadata(metadata)

    def test_absolute_escape_wrong_name_and_missing_paths_are_rejected(self):
        for path in ['C:/outside/pdfcraft.exe', r'\\server\share\pdfcraft.exe', '/outside/pdfcraft.exe', '../../../pdfcraft.exe', '../../apps/figcraft/figcraft.exe', '../../apps/pdfcraft/missing.exe']:
            with self.subTest(path=path), self.assertRaises(ValueError):
                contained_executable(self.root, self.data, path, 'pdfcraft')

    def test_installed_record_and_binary_changes_are_rejected(self):
        for fault in ['record_version', 'record_escape', 'missing_app', 'metadata_hash']:
            installed = copy.deepcopy(self.installed)
            metadata = copy.deepcopy(self.metadata)
            if fault == 'record_version':
                installed['apps']['pdfcraft']['version'] = '0.0.0'
            elif fault == 'record_escape':
                installed['apps']['pdfcraft']['path'] = '../../../pdfcraft.exe'
            elif fault == 'missing_app':
                del installed['apps']['pdfcraft']
            else:
                metadata['applications']['pdfcraft']['sha256'] = '0' * 64
            with self.subTest(fault=fault), self.assertRaises(ValueError):
                verify_installed_metadata(self.root, metadata, installed)
        (self.root / 'apps/pdfcraft/pdfcraft.exe').write_bytes(b'modified after review')
        with self.assertRaises(ValueError):
            verify_installed_metadata(self.root, self.metadata, self.installed)

    def test_feed_requires_named_nonempty_complete_installer(self):
        wrong = self.root / 'wrong-setup.exe'
        wrong.write_bytes(b'setup')
        with self.assertRaises(ValueError):
            make_feed(self.metadata, wrong)
        self.setup.write_bytes(b'')
        with self.assertRaises(ValueError):
            make_feed(self.metadata, self.setup)

    def test_feed_cli_round_trip_and_existing_destination_guard(self):
        (self.data / 'bundle.json').write_text(json.dumps(self.metadata), encoding='utf-8')
        (self.data / 'installed.json').write_text(json.dumps(self.installed), encoding='utf-8')
        destination = self.root / 'release' / FEED_NAME
        command = [sys.executable, str(Path(__file__).with_name('prepare-release-feed.py')),
                   '--payload', str(self.root), '--setup', str(self.setup), '--destination', str(destination)]
        result = subprocess.run(command, capture_output=True, text=True, encoding='utf-8')
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(json.loads(destination.read_text(encoding='utf-8')), make_feed(self.metadata, self.setup))
        first = destination.read_bytes()
        refused = subprocess.run(command, capture_output=True, text=True, encoding='utf-8')
        self.assertNotEqual(refused.returncode, 0)
        self.assertEqual(first, destination.read_bytes())

    def prepare_synthetic_stage(self, wrong_pin=False):
        self.status['launcher_upstream_commit'] = 'c' * 40
        for item in self.status['applications'].values():
            item['upstream_commit'] = 'a' * 40
        (self.root / 'package-status.json').write_text(json.dumps(self.status), encoding='utf-8')
        (self.root / 'portable.txt').write_text('isolated test fixture', encoding='utf-8')
        config = self.data.parent
        for lang, filename in [('ar', 'app.ron'), ('en', 'app-en.ron')]:
            (config / filename).write_text(json.dumps({'settings': f'(language:"{lang}")'}), encoding='utf-8')
        (config / 'personal-suite.txt').write_text('synthetic suite', encoding='utf-8')
        (self.data / 'installed.json').write_text(json.dumps(self.installed), encoding='utf-8')
        licenses = self.root / 'licenses'
        licenses.mkdir()
        (licenses / 'NOTICE.txt').write_text('synthetic fixture notice', encoding='utf-8')
        (self.root / 'apps/pdfcraft/vcruntime140.dll').write_bytes(b'synthetic excluded runtime')
        (self.root / 'apps/pdfcraft/debug.pdb').write_bytes(b'synthetic excluded symbols')
        source = Path(self.directory.name) / 'source.zip'
        repositories = [{'name': name, 'releasedTag': 'v0.7.0', 'upstreamCommit': 'a' * 40} for name in sorted(APPS)]
        repositories.append({'name': 'craft-launcher', 'releasedTag': 'v0.4.1', 'upstreamCommit': 'c' * 40})
        if wrong_pin:
            repositories[0]['upstreamCommit'] = 'b' * 40
        with zipfile.ZipFile(source, 'w') as archive:
            archive.writestr('public/source-manifest.json', json.dumps({'repositories': repositories}))
        destination = Path(self.directory.name) / 'public-payload'
        command = [sys.executable, str(Path(__file__).with_name('prepare-public-payload.py')),
                   '--reviewed-stage', str(self.root), '--destination', str(destination),
                   '--public-source-zip', str(source), '--tag', 'v2026.10-ar.1', '--revision', '1']
        result = subprocess.run(command, capture_output=True, text=True, encoding='utf-8')
        return result, destination

    def test_public_payload_cli_and_installation_verifier_round_trip(self):
        result, payload = self.prepare_synthetic_stage()
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertFalse((payload / 'apps/pdfcraft/vcruntime140.dll').exists())
        self.assertFalse((payload / 'apps/pdfcraft/debug.pdb').exists())
        installation = Path(self.directory.name) / 'isolated-installation'
        shutil.copytree(payload, installation)
        report = Path(self.directory.name) / 'verified.json'
        command = [sys.executable, str(Path(__file__).with_name('verify-public-payload.py')),
                   '--payload', str(payload), '--installation', str(installation),
                   '--language', 'ar', '--report', str(report)]
        checked = subprocess.run(command, capture_output=True, text=True, encoding='utf-8')
        self.assertEqual(checked.returncode, 0, checked.stderr)
        evidence = json.loads(report.read_text(encoding='utf-8'))
        self.assertEqual(evidence['reviewed_applications'], 13)
        self.assertEqual(evidence['bundle_revision'], 1)
        (installation / 'apps/pdfcraft/pdfcraft.exe').write_bytes(b'modified installed GUI')
        rejected = subprocess.run(command, capture_output=True, text=True, encoding='utf-8')
        self.assertNotEqual(rejected.returncode, 0)

    def test_public_payload_refuses_a_different_frozen_source_pin(self):
        result, destination = self.prepare_synthetic_stage(wrong_pin=True)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('public source pin differ', result.stderr)
        self.assertFalse(destination.exists())


if __name__ == '__main__':
    unittest.main()
