use super::*;
const START_LAT: f64 = 4.63874255;
const START_LON: f64 = -74.085237757545;

fn add_marker_map(marker_id: &str, image: &str, map_id: &str, shiny: &mut CustomServer, session: &mut CustomSession) {
    shiny.f64_store.insert(marker_id.to_string(), vec![START_LAT, START_LON]);
    run_js(session, &format!(r#"
        var icon = new L.icon({{
            iconUrl: '{image}',
            shadowSize: [0, 0],
            iconSize:     [48, 48], // size of the icon
            iconAnchor:   [24, 24]
        }});
        window['{marker_id}'] = L.marker([{START_LAT}, {START_LON}], {{ icon: icon }}).addTo(window['{map_id}']);
    "#))
}

fn move_marker_map(marker_id: &str, map_id: &str, shiny: &mut CustomServer, session: &mut CustomSession) {
    let mut rng = rand::thread_rng();
    let mut new_coords: Vec<f64> = vec![0.0, 0.0];
    new_coords[0] +=  rng.gen_range(-0.0005..0.0005) + shiny.f64_store.get(marker_id).unwrap()[0];
    new_coords[1] +=  rng.gen_range(-0.0005..0.0005) + shiny.f64_store.get(marker_id).unwrap()[1];
    shiny.f64_store.insert(marker_id.to_string(), new_coords.clone());
    run_js(session, &format!(r#"window['{marker_id}'].setLatLng(new L.latLng({n1}, {n2})); "#, n1 = new_coords[0], n2 = new_coords[1]));

}

pub fn server(id: &str, shiny: &mut CustomServer, session: &mut CustomSession, init: bool) {
    let ns = NS!(id);
    let map_id = ns("map");
    if init {
        run_js(session, &format!(r#"
            window['{map_id}'] = L.map('{map_id}').setView([{START_LAT}, {START_LON}], 17);
            L.tileLayer('https://{{s}}.tile.openstreetmap.de/{{z}}/{{x}}/{{y}}.png', {{ maxZoom: 19, attribution: '© OpenStreetMap' }}).addTo(window['{map_id}']);
        "#));
        add_marker_map(&ns("person1"), "https://upload.wikimedia.org/wikipedia/commons/3/35/Tux.svg", &map_id, shiny, session);
        add_marker_map(&ns("person2"), "https://wiki.postgresql.org/images/a/a4/PostgreSQL_logo.3colors.svg", &map_id, shiny, session);
        add_marker_map(&ns("person3"), "https://files.brandlogos.net/svg/WMXi7xYVyY/docker-moby-logo-sAMSSLCB_brandlogos.net.svg", &map_id, shiny, session);
        add_marker_map(&ns("person4"), "https://rustacean.net/assets/rustacean-flat-noshadow.svg", &map_id, shiny, session);
        return;
    }
    run_js(session, &format!(r#"window['{map_id}'].invalidateSize()"#));
    if shiny.input.get_u64(&ns("toggle:shiny.action")).unwrap_or(0) % 2 != 0 {
        move_marker_map(&ns("person1"), &map_id, shiny, session);
        move_marker_map(&ns("person2"), &map_id, shiny, session);
        move_marker_map(&ns("person3"), &map_id, shiny, session);
        move_marker_map(&ns("person4"), &map_id, shiny, session);
        return;
    }
}
