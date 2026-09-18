from agent import ProjectPlannerAgent
from tools import ToolRegistry


class AgentRunner:
    def __init__(
        self,
        agent: ProjectPlannerAgent,
        tools: ToolRegistry,
        max_iterations: int = 5,
    ) -> None:
        self.agent = agent
        self.tools = tools
        self.max_iterations = max_iterations

    def run(self, project_idea: str):
        state = "No actions have been performed."

        for iteration in range(self.max_iterations):
            print(f"\nIteration {iteration + 1}")

            action = self.agent.decide(
                project_idea=project_idea,
                state=state,
            )

            print(f"Action: {action.action}")
            print(f"Reason: {action.reason}")

            if action.action == "finish":
                print("Agent finished.")
                return state

            if action.action == "generate_blueprint":
                blueprint = self.tools.execute(
                    "generate_blueprint",
                    project_idea=project_idea,
                )

                state = (
                    "Blueprint generated successfully.\n"
                    f"Project name: {blueprint.project_name}"
                )

                print(
                    f"Generated: {blueprint.project_name}"
                )

        raise RuntimeError(
            "Agent exceeded maximum iterations."
        )