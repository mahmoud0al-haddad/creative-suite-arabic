import hashlib
import json
import subprocess
import sys
import tempfile
import unittest
import zipfile
from pathlib import Path

SCRIPT = Path(__file__).resolve().parent.parent / 'scripts/prepare-public-payload.py'
APPS = ['cadcraft', 'deckcraft', 'designcraft', 'effectcraft', 'filmcraft', 'gridcraft',
        'lightcraft', 'pdfcraft', 'photocraft', 'soundcraft', 'vectorcraft', 'wordcraft']

class PublicPayloadTests(unittest.TestCase):
    def fixture(self, temp):
        stage, dest, archive = Path(temp)/'stage', Path(temp)/'public', Path(temp)/'sources.zip'
        stage.mkdir()
        def put(name, data=b'synthetic reviewed file'):
            path = stage/name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(data)
            return hashlib.sha256(data).hexdigest()
        status = {'launcher_sha256': put('artcraft-launcher.exe'), 'applications': {}}
        put('portable.txt')
        put('vcruntime140.dll')
        put('launcher-native-ui-probe.exe')
        for name in APPS:
            relative = f'apps/{name}/{name}.exe'
            status['applications'][name] = {'version':'0.1.0','arabic_reviewed':True,'modified_gui_sha256':put(relative),'executables':[relative]}
            put(f'apps/{name}/vcruntime140.dll')
            put(f'apps/{name}/support-open.dll')
        put('licenses/open/NOTICE.txt')
        put('licenses/microsoft-runtime/NOTICE.txt')
        for name in ['app.ron','app-en.ron','personal-suite.txt','data/installed.json','control.key']:
            put('ArtCraftLauncherData/'+name)
        (stage/'package-status.json').write_text(json.dumps(status), encoding='utf8')
        with zipfile.ZipFile(archive,'w') as z:
            z.writestr('source-manifest.json',json.dumps({'repositories':[{'name':name} for name in APPS+['craft-launcher']]}))
        return stage,dest,archive

    def run_assembler(self, stage, dest, archive):
        return subprocess.run([sys.executable,str(SCRIPT),'--reviewed-stage',str(stage),'--destination',str(dest),'--public-source-zip',str(archive)],capture_output=True,text=True)

    def test_filters_crt_private_control_and_probe_but_preserves_support(self):
        with tempfile.TemporaryDirectory() as temp:
            stage,dest,archive=self.fixture(temp)
            result=self.run_assembler(stage,dest,archive)
            self.assertEqual(result.returncode,0,result.stderr)
            self.assertFalse(list(dest.rglob('vcruntime140.dll')))
            self.assertFalse((dest/'launcher-native-ui-probe.exe').exists())
            self.assertFalse((dest/'ArtCraftLauncherData/control.key').exists())
            self.assertFalse((dest/'licenses/microsoft-runtime').exists())
            self.assertTrue((dest/'apps/wordcraft/support-open.dll').is_file())
            self.assertTrue((stage/'vcruntime140.dll').is_file())
            self.assertNotEqual(self.run_assembler(stage,dest,archive).returncode,0)

    def test_refuses_unreviewed_or_tampered_gui_before_creating_destination(self):
        with tempfile.TemporaryDirectory() as temp:
            stage,dest,archive=self.fixture(temp)
            (stage/'apps/wordcraft/wordcraft.exe').write_bytes(b'not reviewed')
            self.assertNotEqual(self.run_assembler(stage,dest,archive).returncode,0)
            self.assertFalse(dest.exists())

if __name__=='__main__':
    unittest.main()
