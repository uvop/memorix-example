# flake8: noqa
import typing
import os

if typing.TYPE_CHECKING:
    from dataclasses import dataclass
else:
    from memorix_client_redis import dataclass

from enum import Enum
from memorix_client_redis import (
    MemorixBase,
    MemorixCacheAll,
    MemorixPubSubAll,
    MemorixTaskAll,
)


class System(str, Enum):
    NODE = "NODE"
    DENO = "DENO"
    BUN = "BUN"
    RUST = "RUST"
    PYTHON = "PYTHON"


class MemorixTask(MemorixTaskAll.Base):
    def __init__(self, api: MemorixBase) -> None:
        super().__init__(api=api)

        self.pass_ball = MemorixTaskAll.ItemTTTT[System, int](
            api=api,
            id="pass_ball",
            payload_class=int,
        )

class Memorix(MemorixBase):
    def __init__(self) -> None:
        super().__init__(redis_url=os.environ["REDIS_URL"])


        self._namespace_name_tree = []
        self.task = MemorixTask(self)
