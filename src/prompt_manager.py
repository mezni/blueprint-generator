from pathlib import Path
from typing import Any

import yaml


class PromptManager:
    def __init__(self, prompt_path: str | Path) -> None:
        self.prompt_path = Path(prompt_path)

    def load(self) -> dict[str, Any]:
        if not self.prompt_path.exists():
            raise FileNotFoundError(
                f"Prompt file not found: {self.prompt_path}"
            )

        with self.prompt_path.open("r", encoding="utf-8") as file:
            prompt = yaml.safe_load(file)

        if not isinstance(prompt, dict):
            raise TypeError("Prompt configuration must be a YAML mapping.")

        return prompt

    def render_user_prompt(
        self,
        project_idea: str,
        state: str = "",
    ) -> str:
        prompt = self.load()

        template = prompt["user"]

        return template.format(
            project_idea=project_idea,
            state=state,
        )

    def get_system_prompt(self) -> str:
        prompt = self.load()

        return prompt["system"]