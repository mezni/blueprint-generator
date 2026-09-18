from typing import Any

import httpx


class LLMClient:
    BASE_URL = "https://openrouter.ai/api/v1/chat/completions"

    def __init__(
        self,
        api_key: str,
        model: str,
        temperature: float = 0.3,
        max_tokens: int = 2000,
        timeout: float = 60.0,
    ) -> None:
        self.api_key = api_key
        self.model = model
        self.temperature = temperature
        self.max_tokens = max_tokens
        self.timeout = timeout

    def chat(self, messages: list[dict[str, str]]) -> str:
        headers = {
            "Authorization": f"Bearer {self.api_key}",
            "Content-Type": "application/json",
        }

        payload: dict[str, Any] = {
            "model": self.model,
            "messages": messages,
            "temperature": self.temperature,
            "max_tokens": self.max_tokens,
        }

        try:
            response = httpx.post(
                self.BASE_URL,
                headers=headers,
                json=payload,
                timeout=self.timeout,
            )

            response.raise_for_status()

        except httpx.TimeoutException as exc:
            raise RuntimeError("LLM request timed out.") from exc

        except httpx.HTTPStatusError as exc:
            raise RuntimeError(
                f"LLM request failed with status {exc.response.status_code}."
            ) from exc

        except httpx.RequestError as exc:
            raise RuntimeError("Unable to reach LLM provider.") from exc

        data = response.json()

        try:
            message = data["choices"][0]["message"]
        except (KeyError, IndexError, TypeError) as exc:
            raise RuntimeError("Unexpected LLM response format.") from exc

        content = message.get("content")
        if content is None:
            content = message.get("reasoning")

        if content is None:
            raise RuntimeError("Unexpected LLM response format.")

        return content