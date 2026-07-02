from http import HTTPStatus
from typing import Never
from unittest import mock

from httpx import AsyncClient

from app.parser_loader import ParserImportError


class TestParseEU4Data:
    url = "/api/eu4/parse-data"

    async def test_returns_503_when_parser_unavailable(self, async_client: AsyncClient) -> None:
        with mock.patch(
            "app.main.load_parser",
            side_effect=ParserImportError("Parser module not available"),
        ):
            response = await async_client.post(self.url)

        assert response.status_code == HTTPStatus.SERVICE_UNAVAILABLE
        data = response.json()
        assert data["error"].startswith("Parser module not available")

    async def test_returns_500_when_parser_fails(self, async_client: AsyncClient) -> None:
        with mock.patch("app.main.load_parser", return_value=mock.MagicMock()) as patched_parser:

            def raise_exception(*args, **kwargs) -> Never:
                raise Exception("Parser failed")  # noqa: EM101, TRY002, TRY003

            patched_parser.return_value.parse_eu4 = mock.MagicMock(side_effect=raise_exception)
            response = await async_client.post(self.url)

            assert response.status_code == HTTPStatus.INTERNAL_SERVER_ERROR
            patched_parser.return_value.parse_eu4.assert_called_once()
            data = response.json()
            assert data["error"].startswith("Parser execution failed")

    async def test_runs_parser(self, async_client: AsyncClient) -> None:
        with mock.patch("app.main.load_parser", return_value=mock.MagicMock()) as patched_parser:
            patched_parser.return_value.parse_eu4.return_value = '{"test": "data"}'
            response = await async_client.post(self.url)

            assert response.status_code == HTTPStatus.OK
            patched_parser.return_value.parse_eu4.assert_called_once()
            data = response.json()
            assert data == {"test": "data"}
