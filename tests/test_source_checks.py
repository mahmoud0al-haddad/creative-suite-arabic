import hashlib
import importlib.util
import tempfile
import unittest
from pathlib import Path

script=Path(__file__).resolve().parent.parent/'scripts/check-source.py'
spec=importlib.util.spec_from_file_location('source_check',script)
source=importlib.util.module_from_spec(spec);spec.loader.exec_module(source)

class SourceChecksumTests(unittest.TestCase):
    def fixture(self,temp):
        root=Path(temp);file=root/'sources/example/Cargo.toml';file.parent.mkdir(parents=True);file.write_bytes(b'fixture')
        (root/'source-checksums.sha256').write_text(hashlib.sha256(b'fixture').hexdigest()+'  sources/example/Cargo.toml\n',encoding='utf8')
        return root,file

    def test_records_exact_tree_and_rejects_changed_or_extra_source(self):
        with tempfile.TemporaryDirectory() as temp:
            root,file=self.fixture(temp)
            self.assertEqual(source.verify(root),1)
            file.write_bytes(b'changed')
            with self.assertRaises(ValueError):source.verify(root)
            file.write_bytes(b'fixture')
            (file.parent/'extra.rs').write_bytes(b'not indexed')
            with self.assertRaises(ValueError):source.verify(root)

    def test_rejects_paths_outside_source_inputs(self):
        with tempfile.TemporaryDirectory() as temp:
            root,file=self.fixture(temp)
            (root/'source-checksums.sha256').write_text('0'*64+'  ../outside.rs\n',encoding='utf8')
            with self.assertRaises(ValueError):source.verify(root)

if __name__=='__main__':unittest.main()
