from fastapi import HTTPException
from sqlalchemy.ext.asyncio import AsyncSession

from app.users.models import User
from app.users.repository import UserRepository
from app.users.schemas import UserCreate, UserUpdate


class UserService:
    def __init__(self, session: AsyncSession):
        self._repo = UserRepository(session)

    async def list_users(self) -> list[User]:
        return await self._repo.list_all()

    async def get_user(self, user_id: int) -> User:
        user = await self._repo.get_by_id(user_id)
        if not user:
            raise HTTPException(status_code=404, detail="User not found")
        return user

    async def create_user(self, data: UserCreate) -> User:
        existing = await self._repo.get_by_username(data.username)
        if existing:
            raise HTTPException(status_code=409, detail="Username already taken")
        existing_email = await self._repo.get_by_email(data.email)
        if existing_email:
            raise HTTPException(status_code=409, detail="Email already taken")

        user = User(
            username=data.username,
            email=data.email,
            role=data.role,
            is_active=data.is_active,
        )
        return await self._repo.create(user)

    async def update_user(self, user_id: int, data: UserUpdate) -> User:
        user = await self.get_user(user_id)

        if data.username is not None and data.username != user.username:
            existing = await self._repo.get_by_username(data.username)
            if existing:
                raise HTTPException(status_code=409, detail="Username already taken")
            user.username = data.username

        if data.email is not None and data.email != user.email:
            existing_email = await self._repo.get_by_email(data.email)
            if existing_email:
                raise HTTPException(status_code=409, detail="Email already taken")
            user.email = data.email

        if data.role is not None:
            user.role = data.role

        if data.is_active is not None:
            user.is_active = data.is_active

        return await self._repo.update(user)

    async def toggle_active(self, user_id: int) -> User:
        user = await self.get_user(user_id)
        user.is_active = not user.is_active
        return await self._repo.update(user)

    async def delete_user(self, user_id: int) -> None:
        await self.get_user(user_id)
        await self._repo.delete(user_id)
