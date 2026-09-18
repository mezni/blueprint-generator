from collections.abc import Callable
from typing import Any

from project_planner import ProjectPlanner


class Tool:
    def __init__(
        self,
        name: str,
        description: str,
        function: Callable[..., Any],
    ) -> None:
        self.name = name
        self.description = description
        self.function = function

    def execute(self, **kwargs: Any) -> Any:
        return self.function(**kwargs)


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
        **kwargs: Any,
    ) -> Any:
        tool = self.get(name)
        return tool.execute(**kwargs)


def create_blueprint_tool(
    planner: ProjectPlanner,
) -> Tool:
    return Tool(
        name="generate_blueprint",
        description="Generate a technical project blueprint.",
        function=planner.generate_blueprint,
    )