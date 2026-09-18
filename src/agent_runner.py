from agent import ProjectPlannerAgent


class AgentRunner:
    def __init__(
        self,
        agent: ProjectPlannerAgent,
        max_iterations: int = 5,
    ) -> None:
        self.agent = agent
        self.max_iterations = max_iterations

    def run(self, project_idea: str) -> None:
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
                return

            if action.action == "generate_blueprint":
                state = (
                    "The blueprint generation action "
                    "has been selected."
                )

        raise RuntimeError(
            "Agent exceeded maximum iterations."
        )