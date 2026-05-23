#!/usr/bin/env python3
import csv
import uuid

NAMESPACE = uuid.UUID("00000000-0000-0000-0000-000000000001")
COMPANY_ID = uuid.uuid5(NAMESPACE, "borne-demo-company")

tunisian_cities = [
    ("Tunis Central Station", "Avenue Habib Bourguiba, Tunis", 10.1815, 36.8065),
    ("Sousse Station", "Boulevard Haffouz, Sousse", 10.6369, 35.8254),
    ("Sfax Station", "Rue de la Republique, Sfax", 10.7603, 34.7406),
    ("Bizerte Station", "Avenue Habib Bouguiba, Bizerte", 9.8738, 37.2744),
    ("Gabes Station", "Rue de Tripoli, Gabes", 10.0972, 33.8815),
    ("Ariana Station", "Rue Ibn Khaldoun, Ariana", 10.1904, 36.8625),
    ("Gafsa Station", "Avenue de la Republique, Gafsa", 8.7842, 34.4250),
    ("Monastir Station", "Boulevard de l'Environnement, Monastir", 10.8261, 35.7643),
    ("Ben Arous Station", "Rue des Freres Bouadla, Ben Arous", 10.2230, 36.7533),
    ("Kairouan Station", "Avenue Ibn Sina, Kairouan", 10.0963, 35.6781),
    ("Nabeul Station", "Rue du 2 Mars, Nabeul", 10.7366, 36.4561),
    ("Mahdia Station", "Bordj El Kebir, Mahdia", 11.0622, 35.5047),
    ("Medenine Station", "Avenue de la Liberte, Medenine", 10.5053, 33.3540),
    ("Kasserine Station", "Rue de l'Independance, Kasserine", 8.8337, 35.1721),
    ("Jendouba Station", "Avenue Farhat Hached, Jendouba", 8.7802, 36.5013),
]

connectors = ["Type2", "CCS", "CHAdeMO", "Type2_Tethered"]
hours_options = [
    "24/7",
    "Mo-Fr 08:00-20:00; Sa 09:00-13:00",
    "Mo-Sa 07:00-22:00",
    "Mo-Fr 06:00-22:00; Sa-Su 08:00-20:00",
    "",
]

rows = []
for i in range(500):
    city_idx = i % len(tunisian_cities)
    base_name, base_addr, base_lng, base_lat = tunisian_cities[city_idx]
    offset_lng = ((i % 50) - 25) * 0.005
    offset_lat = (((i // 50) % 50) - 25) * 0.005
    station_id = uuid.uuid5(NAMESPACE, f"station-{i:04d}")
    name = f"{base_name} Borne {i+1}" if i >= len(tunisian_cities) else base_name
    addr = base_addr if i < len(tunisian_cities) else f"{base_addr}, Unit {i+1}"
    is_active = "TRUE" if (i % 7 != 0) else "FALSE"
    under_maint = "TRUE" if (i % 11 == 0) else "FALSE"
    hrs = hours_options[i % len(hours_options)]
    rows.append([
        station_id, COMPANY_ID, name, addr,
        f"{base_lng + offset_lng:.4f}", f"{base_lat + offset_lat:.4f}",
        is_active, under_maint, hrs
    ])

with open("bornemap/infrastructure/seed/stations.csv", "w", newline="") as f:
    writer = csv.writer(f)
    writer.writerow(["id", "company_id", "name", "address", "lng", "lat", "is_active", "under_maintenance", "opening_hours_osm"])
    writer.writerows(rows)

print(f"Generated {len(rows)} station rows")
