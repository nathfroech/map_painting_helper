from http import HTTPStatus

from httpx import ASGITransport, AsyncClient

from app.main import app


async def test_root() -> None:
    transport = ASGITransport(app=app)
    async with AsyncClient(transport=transport, base_url="http://test") as client:
        response = await client.get("/")
        assert response.status_code == HTTPStatus.OK
        assert response.json() == {"message": "Hello, World!"}
