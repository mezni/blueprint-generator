from llm_client import LLMClient
from models import ProjectBlueprint
from prompt_manager import PromptManager


class ProjectPlanner:
    def __init__(
        self,
        llm_client: LLMClient,
        prompt_manager: PromptManager,
    ) -> None:
        self.llm_client = llm_client
        self.prompt_manager = prompt_manager

    def generate_blueprint(
        self,
        project_idea: str,
    ) -> ProjectBlueprint:
        if not project_idea.strip():
            raise ValueError(
                "Project idea cannot be empty."
            )

        messages = [
            {
                "role": "system",
                "content": self.prompt_manager.get_system_prompt(),
            },
            {
                "role": "user",
                "content": self.prompt_manager.render_user_prompt(
                    project_idea
                ),
            },
        ]

        return self.llm_client.structured_output(
            messages=messages,
            response_model=ProjectBlueprint,
        )