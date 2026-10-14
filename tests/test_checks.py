import importlib.util
import tempfile
import unittest
from pathlib import Path

def module(name):
    path = Path(__file__).resolve().parent.parent / 'scripts' / name
    spec = importlib.util.spec_from_file_location(name, path)
    value = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(value)
    return value

catalog = module('check-translations.py')
upstream = module('check-upstream.py')

class CatalogTests(unittest.TestCase):
    def test_mixed_named_and_anonymous_slots_preserve_arguments(self):
        self.assertEqual(catalog.slots('Saved {} to {path}; {{literal}}'), catalog.slots('حُفظ {} في {path}؛ {{literal}}'))
        self.assertNotEqual(catalog.slots('Saved {} to {path}'), catalog.slots('حُفظ {}'))

    def test_precision_consumer_keeps_anonymous_grammar_slot(self):
        self.assertEqual(catalog.slots('{} duplicate{} skipped'), catalog.slots('عدد الملفات المتخطاة: {}{:.0}'))

    def test_rust_match_aliases_and_escaped_quotes_are_parsed(self):
        with tempfile.TemporaryDirectory() as temp:
            path = Path(temp) / 'arabic.rs'
            path.write_text('"INST" | "inst" => "آلة",\n"Version {}" => "الإصدار {}",', encoding='utf8')
            self.assertEqual(list(catalog.rows(path)), [('INST', 'آلة', 'plain'), ('inst', 'آلة', 'plain'), ('Version {}', 'الإصدار {}', 'plain')])

    def test_plural_context_is_preserved_and_named_ids_are_not_source_text(self):
        with tempfile.TemporaryDirectory() as temp:
            path = Path(temp) / 'ar.tsv'
            path.write_text('@plural\t{n} file|{n} files\tلا ملفات|ملف|ملفان|{n} ملفات|{n} ملفًا|{n} ملف\n@id\tapp.update\tتحديث {v}\n', encoding='utf8')
            values = list(catalog.rows(path))
            self.assertEqual(len(values), 1)
            self.assertEqual(values[0][2], 'plural')

class UpstreamTests(unittest.TestCase):
    def test_read_only_report_marks_change_without_mutation(self):
        pins = {'repositories': [{'name': 'one', 'upstream_version': '0.5.0'}, {'name': 'two', 'upstream_version': '0.6.0'}]}
        calls = []
        def fake(entry):
            calls.append(entry['name'])
            return {'tag_name': 'v0.6.0', 'html_url': 'https://github.com/storytold/example/releases/tag/v0.6.0'}
        rows = upstream.report(pins, fake)
        self.assertEqual(calls, ['one', 'two'])
        self.assertFalse(rows[0]['same_tag'])
        self.assertTrue(rows[1]['same_tag'])
        self.assertEqual(pins['repositories'][0]['upstream_version'], '0.5.0')

    def test_failure_does_not_look_like_unchanged_release(self):
        def broken(_):
            raise ValueError('do not publish a private error detail')
        row = upstream.report({'repositories': [{'name': 'one', 'upstream_version': '0.5.0'}]}, broken)[0]
        self.assertEqual(row, {'application': 'one', 'error': 'ValueError'})

    def test_invalid_upstream_schema_is_reported_as_failed(self):
        row = upstream.report({'repositories': [{'name': 'one', 'upstream_version': '0.5.0'}]}, lambda _: {'message': 'bad schema'})[0]
        self.assertEqual(row, {'application': 'one', 'error': 'ValueError'})

if __name__ == '__main__':
    unittest.main()
