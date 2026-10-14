"""Static placeholder checks for recognized Arabic catalogs; native tests remain authoritative."""
import argparse, collections, json, re
from pathlib import Path

QUOTED = r'"(?:[^"\\]|\\.)*"'
PAIR = re.compile(r'\(\s*(' + QUOTED + r')\s*,\s*(' + QUOTED + r')\s*[,)]', re.S)
TRIPLE = re.compile(r'\(\s*(' + QUOTED + r')\s*,\s*(' + QUOTED + r')\s*,\s*(' + QUOTED + r')\s*[,)]', re.S)
MATCH = re.compile(r'(' + QUOTED + r'(?:\s*\|\s*' + QUOTED + r')*)\s*=>\s*(' + QUOTED + r')', re.S)

def slots(text):
    text = text.replace('{{', '').replace('}}', '')
    return collections.Counter(m[1] for m in re.finditer(r'\{([A-Za-z_][A-Za-z0-9_]*|[0-9]*)(?::[^{}]*)?\}', text))

def rows(path):
    text = path.read_text(encoding='utf-8-sig')
    if path.suffix == '.tsv':
        for line in text.splitlines():
            if not line or line.startswith('#'):
                continue
            cells = line.split('\t')
            if len(cells) == 3 and cells[0] != '@id':
                yield cells[1], cells[2], 'plural' if cells[0] == '@plural' else 'plain'
    elif path.suffix == '.json':
        for key, value in json.loads(text).items():
            if isinstance(value, str):
                yield key, value, 'plain'
    else:
        if path.name == 'arabic.rs':
            for match in MATCH.finditer(text):
                for key in re.findall(QUOTED, match[1]):
                    try:
                        yield json.loads(key), json.loads(match[2]), 'plain'
                    except json.JSONDecodeError:
                        continue
            return
        if path.name not in ('ar.rs', 'ar_messages.rs'):
            found = re.search(r'const ARABIC[^=]*=\s*&\[(.*?)\n\];', text, re.S)
            if not found:
                return
            text = found[1]
        matches = list(TRIPLE.finditer(text))
        occupied = set()
        for match in matches:
            occupied.update(range(match.start(), match.end()))
            try:
                yield json.loads(match[2]), json.loads(match[3]), 'plain'
            except json.JSONDecodeError:
                continue
        for match in PAIR.finditer(text):
            if match.start() in occupied:
                continue
            try:
                yield json.loads(match[1]), json.loads(match[2]), 'plain'
            except json.JSONDecodeError:
                continue

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root', type=Path, default=Path(__file__).resolve().parent.parent)
    args = parser.parse_args()
    source = args.root / 'sources'
    if not source.is_dir():
        raise SystemExit('Missing exported sources/ directory.')
    files, count, errors = [], 0, []
    for path in sorted(source.rglob('*')):
        if not path.is_file() or 'vendor' in path.parts:
            continue
        if path.name in ('ar.tsv', 'ar.rs', 'arabic.rs', 'ar_messages.rs', 'ar.json', 'ar-formats.json') or (path.suffix == '.rs' and path.name in ('i18n.rs', 'ui.rs')):
            found = list(rows(path))
            if not found:
                continue
            files.append(str(path.relative_to(args.root)).replace('\\', '/'))
            for english, arabic, kind in found:
                count += 1
                okay = slots(english) == slots(arabic)
                if kind == 'plural':
                    expected = slots(english.split('|')[0])
                    forms = arabic.split('|')
                    okay = len(forms) == 6
                    for index, form in enumerate(forms):
                        actual = slots(form)
                        # Arabic zero/one/two forms can express the number through grammar.
                        equivalent = expected.copy()
                        if index < 3 and not actual.get('n'):
                            equivalent.pop('n', None)
                        okay = okay and actual == equivalent
                if not okay:
                    errors.append({'file': files[-1], 'source': english, 'source_slots': dict(slots(english)), 'arabic_slots': dict(slots(arabic))})
    print(json.dumps({'recognized_catalogs': files, 'rows_checked': count, 'placeholder_errors': errors,
                     'scope': 'Recognized catalog structures only; @id entries require native key/source tests. Arabic six-form plurals are checked per form, with grammatical zero/one/two quantity omission. Coverage, grammar, ordering and dynamic UI require native tests.'}, ensure_ascii=False, indent=2))
    raise SystemExit(1 if errors else 0)

if __name__ == '__main__':
    main()
