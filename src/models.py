from typing import Literal

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
    project_name: str = Field(min_length=2, max_length=100)
    description: str = Field(min_length=10)
    business_problem: str = Field(min_length=10)

    functional_requirements: list[str] = Field(
        min_length=1
    )

    non_functional_requirements: list[str] = Field(
        min_length=1
    )

    technology_stack: list[Technology] = Field(
        min_length=1
    )

    implementation_plan: list[str] = Field(
        min_length=1
    )


class GenerateBlueprintInput(BaseModel):
    project_idea: str = Field(
        min_length=10,
        max_length=5000,
        description="The IT project idea to turn into a technical blueprint.",
    )


class AgentAction(BaseModel):
    action: Literal[
        "generate_blueprint",
        "finish",
    ]

    reason: str = Field(min_length=1)