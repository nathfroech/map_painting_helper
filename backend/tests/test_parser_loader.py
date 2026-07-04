import pathlib
import sys
from typing import Any
from unittest import mock

import pytest

from app.parser_loader import ParserImportError, load_parser
from app.settings import settings

PARSER_SETTING = "app.parser_loader.settings.parser_module_path"


@pytest.fixture
def clean_test_parser():
    orig_path = sys.path[:]
    sys.modules.pop("parser", None)

    yield

    sys.path = orig_path
    sys.modules.pop("parser", None)
    assert not any("test_files" in path for path in sys.path)


@pytest.fixture
def parser_pyd(clean_test_parser: None):
    test_parser_dir = settings.backend_dir / "tests" / "test_files" / "test_parser_module_pyd"

    with mock.patch(PARSER_SETTING, test_parser_dir):
        yield test_parser_dir


@pytest.fixture
def parser_dll(clean_test_parser: None):
    test_parser_dir = settings.backend_dir / "tests" / "test_files" / "test_parser_module_dll"
    sys.modules.pop("parser", None)

    with mock.patch(PARSER_SETTING, test_parser_dir):
        yield test_parser_dir

    (test_parser_dir / "parser.pyd").unlink(missing_ok=True)


@pytest.fixture
def mocked_parser_import(clean_test_parser: None):
    orig_import = __import__

    def import_mock(name: str, *args: Any, **kwargs: Any) -> Any:  # noqa: ANN401
        if name == "parser":
            mock_parser = mock.MagicMock()
            mock_parser.parse_eu4 = mock.Mock(return_value='{"test": "data"}')
            return mock_parser
        return orig_import(name, *args, **kwargs)

    with mock.patch("builtins.__import__", side_effect=import_mock):
        yield


class TestLoadParser:
    def test_raises_error_on_empty_setting(self):
        with (
            mock.patch(PARSER_SETTING, None),
            pytest.raises(ParserImportError, match="Parser module path not set"),
        ):
            load_parser()

    def test_raises_error_on_nonexistent_directory(self):
        with (
            mock.patch(PARSER_SETTING, pathlib.Path("nonexistent_directory")),
            pytest.raises(ParserImportError, match="Parser module directory does not exist"),
        ):
            load_parser()

    def test_loads_pyd_file(self, parser_pyd: pathlib.Path):
        with pytest.raises(ParserImportError):
            load_parser()

        assert not (parser_pyd / "parser.dll").exists()

    def test_loads_dll_file(self, parser_dll: pathlib.Path):
        with pytest.raises(ParserImportError):
            load_parser()

        assert (parser_dll / "parser.pyd").exists()
        assert (parser_dll / "parser.dll").exists()

    def test_load_parser_success(self, parser_pyd: pathlib.Path, mocked_parser_import: None):
        parser = load_parser()

        assert hasattr(parser, "parse_eu4")
        assert callable(parser.parse_eu4)
