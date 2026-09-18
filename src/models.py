from pydantic import BaseModel, Field


class ProjectName(BaseModel):
    name: str = Field(
        min_length=2,
        max_length=100,
        description="A concise and professional IT project name.",
    )