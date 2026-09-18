from pydantic import BaseModel, Field


class ProjectName(BaseModel):
    name: str = Field(
        min_length=2,
        max_length=100,
        description="A concise and professional IT project name.",
    )


class Technology(BaseModel):
    name: str = Field(min_length=1)
    purpose: str = Field(min_length=1)


class ProjectBlueprint(BaseModel):
    project_name: str = Field(
        min_length=2,
        max_length=100,
    )

    description: str = Field(
        min_length=10,
    )

    business_problem: str = Field(
        min_length=10,
    )

    functional_requirements: list[str] = Field(
        min_length=1,
    )

    non_functional_requirements: list[str] = Field(
        min_length=1,
    )

    technology_stack: list[Technology] = Field(
        min_length=1,
    )

    implementation_plan: list[str] = Field(
        min_length=1,
    )