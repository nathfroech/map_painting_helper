import tempfile
from unittest import mock

import pytest
from httpx import ASGITransport, AsyncClient


@pytest.fixture
async def async_client():
    with (
        tempfile.TemporaryDirectory(prefix="test_frontend_out") as tmp_static_dir,
        mock.patch("app.main.settings.static_dir", tmp_static_dir),
    ):
        from app.main import app  # noqa: PLC0415

        transport = ASGITransport(app=app)
        async with AsyncClient(transport=transport, base_url="http://test") as client:
            yield client
