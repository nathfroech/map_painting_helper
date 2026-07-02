import pathlib
import platform

from pydantic import Field
from pydantic_settings import BaseSettings, SettingsConfigDict

__all__ = ["settings"]


def _get_default_parser_target(root_dir: pathlib.Path) -> pathlib.Path | None:
    if platform.system() == "Windows":
        cargo_build_target = "x86_64-pc-windows-gnullvm"
    elif platform.system() in ("Linux", "Darwin"):
        # TODO: Define proper values for Linux and macOS.  # noqa: FIX002
        return None
    else:
        return None

    return root_dir / "parser" / "target" / cargo_build_target / "release"


class Settings(BaseSettings):
    model_config = SettingsConfigDict(case_sensitive=True)

    backend_dir: pathlib.Path = pathlib.Path(__file__).resolve().parents[1]
    root_dir: pathlib.Path = backend_dir.parent
    frontend_dir: pathlib.Path = root_dir / "frontend"
    static_dir: pathlib.Path = frontend_dir / "out"

    parser_module_path: pathlib.Path | None = Field(
        validation_alias="PARSER_MODULE_PATH",
        default=_get_default_parser_target(root_dir),
    )
    eu4_game_path: pathlib.Path | None = Field(validation_alias="EU4_GAME_PATH", default=None)


settings = Settings()
