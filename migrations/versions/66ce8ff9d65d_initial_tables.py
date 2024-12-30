"""initial_tables

Revision ID: 66ce8ff9d65d
Revises: 0
Create Date: 2024-12-30 09:35:59.931681

"""
from typing import Sequence, Union

from alembic import op
import sqlalchemy as sa


# revision identifiers, used by Alembic.
revision: str = '66ce8ff9d65d'
down_revision: Union[str, None] = None
branch_labels: Union[str, Sequence[str], None] = None
depends_on: Union[str, Sequence[str], None] = None


def upgrade():
    op.create_table(
        "sector_groups",
        sa.Column("id", sa.String(26), primary_key=True),
        sa.Column("name", sa.String(100), nullable=False),
        sa.Column("code", sa.Integer, nullable=False, unique=True),
    )
    op.create_table(
        "categories",
        sa.Column("id", sa.String(26), primary_key=True),
        sa.Column("name", sa.String(100), nullable=False),
    )
    op.create_table(
        "sectors",
        sa.Column("id", sa.String(26), primary_key=True),
        sa.Column("name", sa.String(100), nullable=False),
        sa.Column("code", sa.String(4), nullable=False, unique=True),
        sa.Column("sector_group_id", sa.String(26), sa.ForeignKey(
            "sector_groups.id"), nullable=False),
        sa.Column("category_id", sa.String(26), sa.ForeignKey(
            "categories.id"), nullable=False),
    )
    op.create_table(
        "brands",
        sa.Column("id", sa.String(26), primary_key=True),
        sa.Column("name", sa.String(100), nullable=False),
        sa.Column("code", sa.String(4), nullable=False, unique=True),
        sa.Column("sector_id", sa.String(26), sa.ForeignKey(
            "sectors.id"), nullable=False),
    )
    op.create_table(
        "ir_reports",
        sa.Column("id", sa.String(26), primary_key=True),
        sa.Column("year", sa.Integer, nullable=False),
        sa.Column("month", sa.Integer, nullable=False),
        sa.Column("brand_id", sa.String(26), sa.ForeignKey(
            "brands.id"), nullable=False),
        sa.UniqueConstraint("brand_id", "year", "month",
                            name="uq_ir_reports_brand_year_month"),
    )
    op.create_table(
        "ir_scores",
        sa.Column("id", sa.String(26), primary_key=True),
        sa.Column("brand_id", sa.String(26), sa.ForeignKey(
            "brands.id"), nullable=False),
        sa.Column("created_at", sa.DateTime, nullable=False),
    )
    op.create_table(
        "users",
        sa.Column("id", sa.String(26), primary_key=True),
        sa.Column("name", sa.String(100), nullable=False, unique=True),
        sa.Column("nickname", sa.String(100), nullable=False),
    )
    op.create_table(
        "credentials",
        sa.Column("id", sa.String(26), primary_key=True),
        sa.Column("user_id", sa.String(26), sa.ForeignKey(
            "users.id"), nullable=False, unique=True),
        sa.Column("hashed_password", sa.String(256), nullable=False),
    )
    op.create_table(
        "investors",
        sa.Column("id", sa.String(26), primary_key=True),
        sa.Column("user_id", sa.String(26), sa.ForeignKey(
            "users.id"), nullable=False, unique=True),
    )
    op.create_table(
        "portfolios",
        sa.Column("id", sa.String(26), primary_key=True),
        sa.Column("investor_id", sa.String(26), sa.ForeignKey(
            "investors.id"), nullable=False),
        sa.Column("name", sa.String(100), nullable=False),
    )
    op.create_table(
        "portfolio_items",
        sa.Column("id", sa.String(26), primary_key=True),
        sa.Column("portfolio_id", sa.String(26), sa.ForeignKey(
            "portfolios.id"), nullable=False),
        sa.Column("brand_id", sa.String(26), sa.ForeignKey(
            "brands.id"), nullable=False),
        sa.Column("amount", sa.Integer, nullable=False),
        sa.UniqueConstraint("portfolio_id", "brand_id",
                            name="uq_portfolio_items_portfolio_brand"),
    )


def downgrade():
    op.drop_table("portfolio_items")
    op.drop_table("portfolios")
    op.drop_table("investors")
    op.drop_table("credentials")
    op.drop_table("users")
    op.drop_table("ir_scores")
    op.drop_table("ir_reports")
    op.drop_table("brands")
    op.drop_table("sectors")
    op.drop_table("categories")
    op.drop_table("sector_groups")
