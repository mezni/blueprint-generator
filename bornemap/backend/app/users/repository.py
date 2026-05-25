from sqlalchemy import select, delete
from sqlalchemy.ext.asyncio import AsyncSession
from app.users.models import User


class UserRepository:
    def __init__(self, session: AsyncSession):
        self._session = session

    async def list_all(self) -> list[User]:
        result = await self._session.execute(select(User).order_by(User.id))
        return list(result.scalars().all())

    async def get_by_id(self, user_id: int) -> User | None:
        result = await self._session.execute(select(User).where(User.id == user_id))
        return result.scalar_one_or_none()

    async def get_by_username(self, username: str) -> User | None:
        result = await self._session.execute(select(User).where(User.username == username))
        return result.scalar_one_or_none()

    async def get_by_email(self, email: str) -> User | None:
        result = await self._session.execute(select(User).where(User.email == email))
        return result.scalar_one_or_none()

    async def create(self, user: User) -> User:
        self._session.add(user)
        await self._session.flush()
        await self._session.refresh(user)
        return user

    async def update(self, user: User) -> User:
        await self._session.flush()
        await self._session.refresh(user)
        return user

    async def delete(self, user_id: int) -> None:
        await self._session.execute(delete(User).where(User.id == user_id))
