import functools
import pathlib
import sys
import tempfile
import threading
from typing import TYPE_CHECKING, Protocol

from app.settings import settings


class ParserModule(Protocol):
    def parse_eu4(self, game_path: str) -> str: ...


class ParserImportError(ImportError):
    pass


_PARSER_LOAD_LOCK = threading.Lock()


@functools.lru_cache(maxsize=1)
def load_parser() -> ParserModule:
    if not settings.parser_module_path:
        msg = "Parser module path not set"
        raise ParserImportError(msg)
    if not settings.parser_module_path.is_dir():
        msg = "Parser module directory does not exist"
        raise ParserImportError(msg)

    parser_dll_dir: str
    pyd_path = settings.parser_module_path / "parser.pyd"
    dll_path = settings.parser_module_path / "parser.dll"

    with _PARSER_LOAD_LOCK:
        if pyd_path.is_file():
            parser_dll_dir = str(settings.parser_module_path)
        elif dll_path.is_file():
            pyd_bytes = dll_path.read_bytes()
            # Atomic write using a temporary file to avoid concurrent write issues
            with tempfile.NamedTemporaryFile(
                dir=settings.parser_module_path,
                delete=False,
                suffix=".tmp",
            ) as tmp:
                tmp.write(pyd_bytes)
                temp_pyd = pathlib.Path(tmp.name)
            try:
                temp_pyd.replace(pyd_path)
            finally:
                if temp_pyd.exists():
                    temp_pyd.unlink()
            parser_dll_dir = str(settings.parser_module_path)
        else:
            msg = "Parser module directory is empty"
            raise ParserImportError(msg)

    if parser_dll_dir not in sys.path:
        sys.path.insert(0, parser_dll_dir)

    try:
        if TYPE_CHECKING:
            parser = ParserModule()  # ty:ignore[call-non-callable]
        else:
            import parser  # noqa: PLC0415
    except ImportError as e:
        raise ParserImportError(str(e)) from e
    else:
        return parser
