from datetime import datetime
from pydantic import BaseModel, Field


class StationResponse(BaseModel):
    id: int
    partner_id: int | None
    name: str
    operator: str | None
    address: str | None
    location: dict | None
    plug_types: list[str] | None
    speed_kw: float | None
    is_active: bool
    created_at: datetime
    updated_at: datetime

    model_config = {"from_attributes": True}


class StationCreate(BaseModel):
    partner_id: int | None = None
    name: str = Field(..., min_length=1, max_length=255)
    operator: str | None = Field(default=None, max_length=255)
    address: str | None = Field(default=None, max_length=500)
    plug_types: list[str] | None = None
    speed_kw: float | None = None
    is_active: bool = True


class StationUpdate(BaseModel):
    partner_id: int | None = None
    name: str | None = Field(default=None, min_length=1, max_length=255)
    operator: str | None = Field(default=None, max_length=255)
    address: str | None = Field(default=None, max_length=500)
    plug_types: list[str] | None = None
    speed_kw: float | None = None
    is_active: bool | None = None
