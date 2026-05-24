from sqlalchemy.ext.asyncio import async_sessionmaker, create_async_engine, AsyncSession

from app.core.config import settings

engine = create_async_engine(
    settings.database_url,
    pool_size=5,
    max_overflow=10,
)

async_session = async_sessionmaker(engine, class_=AsyncSession, expire_on_commit=False)


async def get_session() -> AsyncSession:
    async with async_session() as session:
        try:
            yield session
        finally:
            await session.close()
