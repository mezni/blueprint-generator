from datetime import datetime
from pydantic import BaseModel, EmailStr, Field


class UserResponse(BaseModel):
    id: int
    username: str
    email: str
    role: str
    is_active: bool
    created_at: datetime
    updated_at: datetime

    model_config = {"from_attributes": True}


class UserCreate(BaseModel):
    username: str = Field(..., min_length=2, max_length=100)
    email: EmailStr
    role: str = Field(default="viewer", pattern=r"^(admin|editor|viewer)$")
    password: str | None = Field(default=None, min_length=6)
    is_active: bool = True


class UserUpdate(BaseModel):
    username: str | None = Field(default=None, min_length=2, max_length=100)
    email: EmailStr | None = None
    role: str | None = Field(default=None, pattern=r"^(admin|editor|viewer)$")
    password: str | None = Field(default=None, min_length=6)
    is_active: bool | None = None
