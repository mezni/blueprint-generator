from collections.abc import Callable
from typing import Any

from pydantic import BaseModel

from models import GenerateBlueprintInput
from project_planner import ProjectPlanner


class Tool:
    def __init__(
        self,
        name: str,
        description: str,
        input_model: type[BaseModel],
        function: Callable[..., Any],
    ) -> None:
        self.name = name
        self.description = description
        self.input_model = input_model
        self.function = function

    def execute(self, arguments: dict[str, Any]) -> Any:
        validated_arguments = self.input_model.model_validate(
            arguments
        )

        return self.function(
            **validated_arguments.model_dump()
        )


class ToolRegistry:
    def __init__(self) -> None:
        self._tools: dict[str, Tool] = {}

    def register(self, tool: Tool) -> None:
        self._tools[tool.name] = tool

    def get(self, name: str) -> Tool:
        if name not in self._tools:
            raise ValueError(f"Tool not found: {name}")

        return self._tools[name]

    def execute(
        self,
        name: str,
        arguments: dict[str, Any],
    ) -> Any:
        tool = self.get(name)

        return tool.execute(arguments)


def create_blueprint_tool(
    planner: ProjectPlanner,
) -> Tool:
    return Tool(
        name="generate_blueprint",
        description="Generate a technical project blueprint.",
        input_model=GenerateBlueprintInput,
        function=planner.generate_blueprint,
    )