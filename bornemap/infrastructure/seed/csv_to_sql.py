#!/usr/bin/env python3
import csv
import sys

print("-- Auto-generated full seed: 500 stations + chargers")
print("-- Run: python3 infrastructure/seed/csv_to_sql.py > migrations/20260523_0004_seed_synthetic.sql")
print()

with open("bornemap/infrastructure/seed/stations.csv", "r") as f:
    reader = csv.DictReader(f)
    rows = list(reader)

print("INSERT INTO station_domain.companies (id, name, is_test)")
print("VALUES ('00000000-0000-0000-0000-000000000001', 'BorneMap Demo Operator', TRUE)")
print("ON CONFLICT (id) DO NOTHING;")
print()

connector_types = ["Type2", "CCS", "CHAdeMO", "Type2_Tethered"]
charger_idx = 0

station_values = []
charger_values = []

for i, row in enumerate(rows):
    sid = row["id"]
    cid = row["company_id"]
    name = row["name"].replace("'", "''")
    address = row["address"].replace("'", "''")
    lng = row["lng"]
    lat = row["lat"]
    is_active = row["is_active"]
    under_maint = row["under_maintenance"]
    hrs = row["opening_hours_osm"].replace("'", "''") if row["opening_hours_osm"] else ""

    hrs_sql = f"'{hrs}'" if hrs else "NULL"

    station_values.append(
        f"    ('{sid}', '{cid}', '{name}', '{address}', "
        f"ST_MakePoint({lng}, {lat})::geography, {is_active}, {under_maint}, {hrs_sql}, TRUE)"
    )

    num_chargers = (i % 4) + 1
    for j in range(num_chargers):
        charger_idx += 1
        ch_id = f"{charger_idx:010d}-0000-0000-0000-000000000000"
        conn = connector_types[j % len(connector_types)]
        power = [22.0, 50.0, 62.5, 7.4, 150.0][j % 5]
        ch_active = "TRUE" if (charger_idx % 5 != 0) else "FALSE"
        charger_values.append(
            f"    ('{ch_id}', '{sid}', '{conn}', {power:.2f}, {ch_active}, TRUE)"
        )

print("INSERT INTO station_domain.stations (id, company_id, name, address, location, is_active, under_maintenance, opening_hours_osm, is_test)")
print("VALUES")
print(",\n".join(station_values))
print("ON CONFLICT (id) DO NOTHING;")
print()

print("INSERT INTO station_domain.chargers (id, station_id, connector, power_kw, is_active, is_test)")
print("VALUES")
print(",\n".join(charger_values))
print("ON CONFLICT (id) DO NOTHING;")
