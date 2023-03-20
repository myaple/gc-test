fn main() {
    println!("Hello, world!");
}

#[test]
fn test_feature() {
    use geojson::GeoJson as GeoGeoJson;
    use geojson::quick_collection;
    use geozero::geojson::GeoJson;
    use geozero::wkb::Ewkb;
    use geozero::{CoordDimensions, ToWkb, ToJson};
    use geo_types::{Geometry, GeometryCollection};

    let gj_str = r#"{ 
            "type" : "Feature", 
            "properties" : {  
                "capacity" : "10", 
                "type" : "U-Rack",
                "mount" : "Surface"
            }, 
            "geometry" : { 
                "type" : "Point", 
                "coordinates" : [ -71.073283, 42.417500 ] 
            }
        }"#; 

    let geogeojson = gj_str.to_string().parse::<GeoGeoJson>().unwrap();

    println!("\ngeojson crate itself: {}", geogeojson);

    let geojson = GeoJson(gj_str);
    let ewkb = geojson.to_ewkb(CoordDimensions::xy(), None).unwrap();

    println!("\newkb conversion: {}", Ewkb(ewkb).to_json().unwrap());

    let mut geocol: GeometryCollection<f64> = quick_collection(&geogeojson).unwrap();

    let ge: Geometry<f64> = Geometry::GeometryCollection(geocol);

    let gewkb = ge.to_ewkb(CoordDimensions::xy(), None).unwrap();
    println!("\ngc ewkb conversion: {}", Ewkb(gewkb).to_json().unwrap());
}


#[test]
fn test_fc() {
    use geojson::GeoJson as GeoGeoJson;
    use geojson::quick_collection;
    use geozero::geojson::GeoJson;
    use geozero::wkb::Ewkb;
    use geozero::{CoordDimensions, ToWkb, ToJson};
    use geo_types::{Geometry, GeometryCollection};

    let gj_str = r#"{
        "type" : "FeatureCollection",
        "features" : [{ 
            "type" : "Feature", 
            "properties" : {  
                "capacity" : "10", 
                "type" : "U-Rack",
                "mount" : "Surface"
            }, 
            "geometry" : { 
                "type" : "Point", 
                "coordinates" : [ -71.073283, 42.417500 ] 
            }
        },
           { "type" : "Feature", 
            "properties" : {  
                "capacity" : "10", 
                "type" : "U-Rack2",
                "mount" : "Surface"
            }, 
            "geometry" : { 
                "type" : "Point", 
                "coordinates" : [ -51.073283, 22.417500 ] 
            }
        }
        ]
    }"#;

    let geogeojson = gj_str.to_string().parse::<GeoGeoJson>().unwrap();

    println!("\ngeojson crate itself: {}", geogeojson);

    let geojson = GeoJson(gj_str);
    let ewkb = geojson.to_ewkb(CoordDimensions::xy(), None).unwrap();

    println!("\newkb conversion: {}", Ewkb(ewkb).to_json().unwrap());

    let mut geocol: GeometryCollection<f64> = quick_collection(&geogeojson).unwrap();

    let ge: Geometry<f64> = Geometry::GeometryCollection(geocol);

    let gewkb = ge.to_ewkb(CoordDimensions::xy(), None).unwrap();
    println!("\ngc ewkb conversion: {}", Ewkb(gewkb).to_json().unwrap());
}

#[test]
fn test_geom() {
    use geojson::GeoJson as GeoGeoJson;
    use geojson::quick_collection;
    use geozero::geojson::GeoJson;
    use geozero::wkb::Ewkb;
    use geozero::{CoordDimensions, ToWkb, ToJson};
    use geo_types::{Geometry, GeometryCollection};

    let gj_str = r#"{
                "type" : "Point", 
                "coordinates" : [ -71.073283, 42.417500 ] 
            }
    "#;

    let geogeojson = gj_str.to_string().parse::<GeoGeoJson>().unwrap();

    println!("\ngeojson crate itself: {}", geogeojson);

    let geojson = GeoJson(gj_str);
    let ewkb = geojson.to_ewkb(CoordDimensions::xy(), None).unwrap();

    println!("\newkb conversion: {}", Ewkb(ewkb).to_json().unwrap());

    let mut geocol: GeometryCollection<f64> = quick_collection(&geogeojson).unwrap();

    let ge: Geometry<f64> = Geometry::GeometryCollection(geocol);

    let gewkb = ge.to_ewkb(CoordDimensions::xy(), None).unwrap();
    println!("\ngc ewkb conversion: {}", Ewkb(gewkb).to_json().unwrap());
}
