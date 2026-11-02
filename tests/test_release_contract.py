"""Include the published contract's existing security regressions in PR checks."""
import importlib.util
import sys
from pathlib import Path

scripts=Path(__file__).resolve().parent.parent/'scripts'
sys.path.insert(0,str(scripts))
spec=importlib.util.spec_from_file_location('published_release_contract_tests',scripts/'test-release-contract.py')
contract_tests=importlib.util.module_from_spec(spec)
spec.loader.exec_module(contract_tests)

def load_tests(loader,standard_tests,pattern):
    return loader.loadTestsFromModule(contract_tests)
