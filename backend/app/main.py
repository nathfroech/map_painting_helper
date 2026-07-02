import json

from fastapi import FastAPI
from fastapi.responses import JSONResponse

from .parser_loader import ParserImportError, load_parser
from .settings import settings

app = FastAPI()
app.frontend("/", directory=settings.static_dir)


@app.post("/api/eu4/parse-data")
async def parse_eu4_data() -> JSONResponse:
    try:
        parser = load_parser()
    except ParserImportError as e:
        return JSONResponse(
            {"error": f"Parser module not available: {e}"},
            status_code=503,
        )

    try:
        result = parser.parse_eu4(str(settings.eu4_game_path))
        return JSONResponse(json.loads(result))
    except Exception as e:  # noqa: BLE001
        return JSONResponse(
            {"error": f"Parser execution failed: {e}"},
            status_code=500,
        )
