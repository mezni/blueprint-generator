from datetime import datetime
from pydantic import BaseModel, Field


class ChargerResponse(BaseModel):
    id: int
    station_id: int | None
    connector: str
    power_kw: float | None
    status: str
    is_active: bool
    created_at: datetime
    updated_at: datetime

    model_config = {"from_attributes": True}


class ChargerCreate(BaseModel):
    station_id: int | None = None
    connector: str = Field(..., pattern=r"^(CCS|Type2|CHAdeMO|GBT)$")
    power_kw: float | None = None
    status: str = Field(default="available", pattern=r"^(available|occupied|offline|maintenance)$")
    is_active: bool = True


class ChargerUpdate(BaseModel):
    station_id: int | None = None
    connector: str | None = Field(default=None, pattern=r"^(CCS|Type2|CHAdeMO|GBT)$")
    power_kw: float | None = None
    status: str | None = Field(default=None, pattern=r"^(available|occupied|offline|maintenance)$")
    is_active: bool | None = None
