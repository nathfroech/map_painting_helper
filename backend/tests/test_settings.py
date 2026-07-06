import pathlib
from unittest import mock

import pytest

from app.settings import _get_default_parser_target


class TestGetDefaultParserTarget:
    @pytest.mark.parametrize(
        ("platform", "expected_result"),
        [
            ("Windows", pathlib.Path("D:/project/parser/target/x86_64-pc-windows-gnullvm/release")),
            ("Linux", None),
            ("Darwin", None),
            ("Other", None),
        ],
    )
    def test_get_target_for_platform(self, platform: str, expected_result: pathlib.Path | None):
        with mock.patch("platform.system", return_value=platform):
            result = _get_default_parser_target(pathlib.Path("D:/project"))

        if expected_result is None:
            assert result is None
        else:
            assert result == expected_result
