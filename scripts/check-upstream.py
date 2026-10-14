"""Read-only upstream release comparison; no issue creation, merge or release publication."""
import argparse, datetime, json, os, re, urllib.error, urllib.request
from pathlib import Path

def report(pins, fetch):
    result = []
    for entry in pins['repositories']:
        name = entry['name']
        try:
            latest = fetch(entry)
            if not isinstance(latest, dict) or not isinstance(latest.get('tag_name'), str) or not latest['tag_name']:
                raise ValueError('Upstream release response has no valid tag')
            tag = latest.get('tag_name', '')
            result.append({'application': name, 'pinned_version': entry['upstream_version'], 'upstream_latest_tag': tag,
                           'same_tag': tag.lstrip('v') == entry['upstream_version'], 'url': latest.get('html_url', '')})
        except (urllib.error.URLError, TimeoutError, ValueError, KeyError) as error:
            result.append({'application': name, 'error': type(error).__name__})
    return result

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--pins', type=Path, default=Path(__file__).resolve().parent.parent / 'source-pins.json')
    parser.add_argument('--output', type=Path, default=Path('upstream-report.json'))
    args = parser.parse_args()
    pins = json.loads(args.pins.read_text(encoding='utf-8-sig'))
    token = os.environ.get('GITHUB_TOKEN')
    def fetch(entry):
        match = re.fullmatch(r'https://github\.com/([A-Za-z0-9_.-]+)/([A-Za-z0-9_.-]+?)(?:\.git)?', entry['upstream_url'])
        if not match:
            raise ValueError('Unrecognized upstream URL')
        url = f'https://api.github.com/repos/{match[1]}/{match[2]}/releases/latest'
        headers = {'Accept': 'application/vnd.github+json', 'User-Agent': 'Creative-Suite-Arabic-Upstream-Report', 'X-GitHub-Api-Version': '2022-11-28'}
        if token:
            headers['Authorization'] = f'Bearer {token}'
        with urllib.request.urlopen(urllib.request.Request(url, headers=headers), timeout=30) as response:
            data = response.read(2 * 1024 * 1024 + 1)
            if len(data) > 2 * 1024 * 1024:
                raise ValueError('Upstream response too large')
            return json.loads(data)
    rows = report(pins, fetch)
    args.output.write_text(json.dumps({'checked_utc': datetime.datetime.now(datetime.timezone.utc).isoformat(), 'read_only': True, 'applications': rows}, indent=2), encoding='utf-8')
    summary = '| Application | Pinned | Latest upstream tag | Result |\n| --- | --- | --- | --- |\n'
    for row in rows:
        safe = lambda value: str(value).replace('\n', ' ').replace('\r', ' ').replace('|', '\\|')
        summary += f"| {safe(row['application'])} | {safe(row.get('pinned_version', '—'))} | {safe(row.get('upstream_latest_tag', '—'))} | {'check failed' if 'error' in row else 'unchanged' if row['same_tag'] else 'review needed'} |\n"
    summary += '\nRead-only report. No code, translation, issue, merge or release was changed.\n'
    if os.environ.get('GITHUB_STEP_SUMMARY'):
        with open(os.environ['GITHUB_STEP_SUMMARY'], 'a', encoding='utf-8') as handle:
            handle.write(summary)
    print(summary)
    raise SystemExit(1 if any('error' in row for row in rows) else 0)

if __name__ == '__main__':
    main()
