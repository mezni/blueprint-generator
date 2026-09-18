import pytest
from pydantic import ValidationError

from models import ProjectName


def test_project_name_accepts_valid_name() -> None:
    result = ProjectName(name="KnowledgeHub")

    assert result.name == "KnowledgeHub"


def test_project_name_rejects_empty_name() -> None:
    with pytest.raises(ValidationError):
        ProjectName(name="")