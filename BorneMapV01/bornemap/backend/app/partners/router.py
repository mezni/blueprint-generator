from fastapi import APIRouter, Depends
from sqlalchemy.ext.asyncio import AsyncSession

from app.core.dependencies import get_db
from app.partners.schemas import PartnerResponse, PartnerCreate, PartnerUpdate
from app.partners.service import PartnerService

router = APIRouter(prefix="/api/v1/partners", tags=["partners"])


@router.get("", response_model=list[PartnerResponse])
async def list_partners(db: AsyncSession = Depends(get_db)):
    service = PartnerService(db)
    return await service.list_partners()


@router.post("", response_model=PartnerResponse, status_code=201)
async def create_partner(data: PartnerCreate, db: AsyncSession = Depends(get_db)):
    service = PartnerService(db)
    return await service.create_partner(data)


@router.get("/{partner_id}", response_model=PartnerResponse)
async def get_partner(partner_id: int, db: AsyncSession = Depends(get_db)):
    service = PartnerService(db)
    return await service.get_partner(partner_id)


@router.patch("/{partner_id}", response_model=PartnerResponse)
async def update_partner(partner_id: int, data: PartnerUpdate, db: AsyncSession = Depends(get_db)):
    service = PartnerService(db)
    return await service.update_partner(partner_id, data)


@router.patch("/{partner_id}/toggle-active", response_model=PartnerResponse)
async def toggle_partner_active(partner_id: int, db: AsyncSession = Depends(get_db)):
    service = PartnerService(db)
    return await service.toggle_active(partner_id)


@router.delete("/{partner_id}", status_code=204)
async def delete_partner(partner_id: int, db: AsyncSession = Depends(get_db)):
    service = PartnerService(db)
    await service.delete_partner(partner_id)
