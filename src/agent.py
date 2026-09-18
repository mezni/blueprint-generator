from llm_client import LLMClient
from models import AgentAction
from prompt_manager import PromptManager


class ProjectPlannerAgent:
    def __init__(
        self,
        llm_client: LLMClient,
        prompt_manager: PromptManager,
    ) -> None:
        self.llm_client = llm_client
        self.prompt_manager = prompt_manager

    def decide(
        self,
        project_idea: str,
        state: str,
    ) -> AgentAction:
        messages = [
            {
                "role": "system",
                "content": self.prompt_manager.get_system_prompt(),
            },
            {
                "role": "user",
                "content": self.prompt_manager.render_user_prompt(
                    project_idea=project_idea,
                    state=state,
                ),
            },
        ]

        return self.llm_client.structured_output(
            messages=messages,
            response_model=AgentAction,
        )