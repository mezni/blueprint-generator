-- osm2pgsql Lua filter for EV charging stations
-- Converts OSM amenity=charging_station nodes into our schema

function osm2pgsql_process_node(object)
    if object.tags.amenity == "charging_station" then
        local osm_id = object.osm_id
        local station_id = string.format("stn-%08x", osm_id)
        local operator = object.tags.operator or "Unknown"
        local name = object.tags.name or (operator .. " Charging Station")
        name = name:gsub("'", "''")

        -- Ensure OSM Import partner exists
        object:insert([[
            INSERT INTO partners (id, name, type, contact_email, is_live)
            VALUES ('prt-00000000', 'OSM Import', 'Business', 'osm@borne-map.tn', false)
            ON CONFLICT (id) DO NOTHING;
        ]])

        -- Insert station
        object:insert(string.format([[
            INSERT INTO stations (id, name, partner_id, geom, status, is_live, updated_at)
            VALUES (
                '%s', '%s',
                'prt-00000000',
                ST_SetSRID(ST_MakePoint(%.7f, %.7f), 4326)::geography,
                'Available', false, NOW()
            )
            ON CONFLICT (id) DO UPDATE SET
                name = EXCLUDED.name,
                geom = EXCLUDED.geom;
        ]], station_id, name, object.lon, object.lat))

        -- Insert chargers based on socket tags
        local sockets = {
            { tag = "socket:type2",       plug = "Type2",  power = 22 },
            { tag = "socket:type2_combo", plug = "CCS2",   power = 50 },
            { tag = "socket:chademo",     plug = "CHAdeMO", power = 50 },
            { tag = "socket:ccs",         plug = "CCS2",   power = 120 },
            { tag = "socket:type3",       plug = "Type3",  power = 22 },
            { tag = "socket:tesla",       plug = "Tesla",  power = 120 },
        }

        local charger_count = 0
        for _, s in ipairs(sockets) do
            if object.tags[s.tag] and object.tags[s.tag] ~= "0" then
                charger_count = charger_count + 1
                local charger_id = string.format("chg-%08x%02x", osm_id, charger_count)
                object:insert(string.format([[
                    INSERT INTO chargers (id, station_id, plug_type, power_output, status, is_live, updated_at)
                    VALUES ('%s', '%s', '%s', %d, 'Available', false, NOW())
                    ON CONFLICT (id) DO NOTHING;
                ]], charger_id, station_id, s.plug, s.power))
            end
        end

        -- Fallback: one generic charger if no socket tags
        if charger_count == 0 then
            local charger_id = string.format("chg-%08x001", osm_id)
            object:insert(string.format([[
                INSERT INTO chargers (id, station_id, plug_type, power_output, status, is_live, updated_at)
                VALUES ('%s', '%s', 'Type2', 22, 'Available', false, NOW())
                ON CONFLICT (id) DO NOTHING;
            ]], charger_id, station_id))
        end
    end
end
