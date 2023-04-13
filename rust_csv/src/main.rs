use std::{env, error::Error, process::ExitCode};

use csv;
use serde::{Deserialize, Serialize};

mod config;

fn read_from_file(p: &str) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(p)?;

    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() -> ExitCode {
    let cfg = config::ConfigBuilder::new().build();
    println!("{:?}", cfg);
    let result: Result<(), Box<dyn Error>>;

    match cfg.cmd {
        config::Cmd::Read => {
            let src = cfg.src.to_str().unwrap();
            result = read_from_file(&src);
        }
        config::Cmd::Write => {
            let dest = cfg.dest.to_str().unwrap();
            result = write_from_serde(&dest);
        }
    }
    if let Err(e) = result {
        eprintln!("{}", e);
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

fn write_from_serde(p: &str) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(p)?;
    // 1. write headers
    wtr.write_record(&["City", "State", "Population", "Latitude", "Longitude"])?;

    // 2. write records
    wtr.serialize(("Davidsons Landing", "AK", "", "65.2419444", "-165.2716667"))?;
    wtr.serialize(("Kenai", "AK", "7610", "60.5544444", "-151.2583333"))?;
    wtr.serialize(("Oakman", "AL", "", "33.7133333", "-87.3886111"))?;
    wtr.serialize(("Richards Crossroads", "AL", "", "31.7369444", "-85.2644444"))?;
    wtr.serialize(("Sandfort", "AL", "", "32.3380556", "-85.2233333"))?;
    wtr.serialize(("Selma", "AL", "18980", "32.4072222", "-87.0211111"))?;
    wtr.serialize((
        "Shadow Oaks Addition",
        "AR",
        "",
        "34.9555556",
        "-91.9475000",
    ))?;
    wtr.serialize(("Summerville", "AR", "", "33.5202778", "-92.3555556"))?;
    wtr.serialize(("El Mirage", "AZ", "32308", "33.6130556", "-112.3238889"))?;
    wtr.serialize(("Willow Springs", "AZ", "", "36.1894444", "-111.3930556"))?;
    wtr.serialize(("Colton", "CA", "52335", "34.0738889", "-117.3127778"))?;
    wtr.serialize(("Fontana", "CA", "169160", "34.0922222", "-117.4341667"))?;
    wtr.serialize((
        "Fountain Valley",
        "CA",
        "56133",
        "33.7091667",
        "-117.9527778",
    ))?;
    wtr.serialize(("Kings Beach", "CA", "", "39.2377778", "-120.0255556"))?;
    wtr.serialize(("Milpitas", "CA", "62636", "37.4283333", "-121.9055556"))?;
    wtr.serialize(("Mokelumne City", "CA", "", "38.2530556", "-121.4380556"))?;
    wtr.serialize(("Mount Eden", "CA", "", "37.6361111", "-122.0988889"))?;
    wtr.serialize(("San Clemente", "CA", "62272", "33.4269444", "-117.6111111"))?;
    wtr.serialize(("Seal Beach", "CA", "24404", "33.7413889", "-118.1038889"))?;
    wtr.serialize((
        "West Hollywood",
        "CA",
        "37031",
        "34.0900000",
        "-118.3608333",
    ))?;
    wtr.serialize(("Bridgeport", "CT", "139090", "41.1669444", "-73.2052778"))?;
    wtr.serialize(("Orange", "CT", "13860", "41.2783333", "-73.0261111"))?;
    wtr.serialize(("Azalea Park", "FL", "12347", "28.5408333", "-81.3008333"))?;
    wtr.serialize(("Bratt", "FL", "", "30.9655556", "-87.4275000"))?;
    wtr.serialize(("Cutler Ridge", "FL", "26831", "25.5802778", "-80.3469444"))?;
    wtr.serialize(("Dunn Creek", "FL", "", "30.4861111", "-81.5908333"))?;
    wtr.serialize(("South Daytona", "FL", "14451", "29.1655556", "-81.0047222"))?;
    wtr.serialize(("Brickhouse", "GA", "", "33.7750000", "-82.8108333"))?;
    wtr.serialize(("Lakeview Heights", "GA", "", "33.6188889", "-84.4505556"))?;
    wtr.serialize(("Perry", "GA", "11234", "32.4580556", "-83.7316667"))?;
    wtr.serialize(("Roswell", "GA", "77218", "34.0230556", "-84.3616667"))?;
    wtr.serialize(("Warfield", "GA", "", "33.2994444", "-83.3838889"))?;
    wtr.serialize(("Kirkman", "IA", "", "41.7286111", "-95.2650000"))?;
    wtr.serialize(("Travers", "ID", "", "42.6091667", "-113.7361111"))?;
    wtr.serialize(("Calhoun", "IL", "", "38.6502778", "-88.0436111"))?;
    wtr.serialize(("Cleone", "IL", "", "39.4230556", "-87.9075000"))?;
    wtr.serialize(("Deerfield", "IL", "19618", "42.1711111", "-87.8444444"))?;
    wtr.serialize(("Highbank Town", "IN", "", "38.5144444", "-87.1502778"))?;
    wtr.serialize(("Indianapolis", "IN", "773283", "39.7683333", "-86.1580556"))?;
    wtr.serialize(("Leona", "KS", "", "39.7872222", "-95.3213889"))?;
    wtr.serialize(("New Salem", "KS", "", "37.3105556", "-96.8950000"))?;
    wtr.serialize(("Flint Springs", "KY", "NULL", "37.3433333", "-86.7136111"))?;
    wtr.serialize(("Harvey", "LA", "22383", "29.9033333", "-90.0772222"))?;
    wtr.serialize(("Jennings", "LA", "10547", "30.2222222", "-92.6569444"))?;
    wtr.serialize(("King", "LA", "", "32.2405556", "-91.1213889"))?;
    wtr.serialize(("Opelousas", "LA", "22835", "30.5333333", "-92.0813889"))?;
    wtr.serialize(("Reading", "MA", "23441", "42.5255556", "-71.0958333"))?;
    wtr.serialize(("Mount Airy", "MD", "8714", "39.3761111", "-77.1550000"))?;
    wtr.serialize(("Auburn", "ME", "23488", "44.0977778", "-70.2316667"))?;
    wtr.serialize(("Ellsworth", "ME", "7055", "44.5433333", "-68.4200000"))?;
    wtr.serialize(("Sturgis", "MI", "11081", "41.7991667", "-85.4191667"))?;
    wtr.serialize((
        "Brooklyn Center",
        "MN",
        "27718",
        "45.0761111",
        "-93.3325000",
    ))?;
    wtr.serialize(("Coon Rapids", "MN", "62528", "45.1200000", "-93.2875000"))?;
    wtr.serialize(("Moark", "MO", "NULL", "36.3825000", "-89.9888889"))?;
    wtr.serialize(("Owens", "MO", "", "37.2188889", "-92.4027778"))?;
    wtr.serialize(("Natchez", "MS", "17118", "31.5602778", "-91.4030556"))?;
    wtr.serialize(("Rogers", "NE", "", "41.4652778", "-96.9147222"))?;
    wtr.serialize(("Hollis", "NH", "7711", "42.7430556", "-71.5922222"))?;
    wtr.serialize(("Bayonne", "NJ", "59878", "40.6686111", "-74.1147222"))?;
    wtr.serialize(("Belleville", "NJ", "36878", "40.7936111", "-74.1505556"))?;
    wtr.serialize(("Frenchtown", "NJ", "NULL", "40.5261111", "-75.0619444"))?;
    wtr.serialize(("Sharp", "NJ", "", "40.0922222", "-74.7427778"))?;
    wtr.serialize((
        "Los Ranchos de Albuquerque",
        "NM",
        "5184",
        "35.1619444",
        "-106.6422222",
    ))?;
    wtr.serialize(("Deerhead", "NY", "", "44.3522222", "-73.5436111"))?;
    wtr.serialize(("Howland", "NY", "", "43.0791667", "-76.6827778"))?;
    wtr.serialize(("Lake Grove", "NY", "10715", "40.8527778", "-73.1155556"))?;
    wtr.serialize(("Penfield Center", "NY", "", "43.1672222", "-77.4313889"))?;
    wtr.serialize(("Comet", "OH", "", "39.1158333", "-82.5511111"))?;
    wtr.serialize(("Little Mountain", "OH", "", "41.6402778", "-81.2819444"))?;
    wtr.serialize(("Mason", "OH", "30988", "39.3600000", "-84.3100000"))?;
    wtr.serialize(("Siverly", "OH", "", "39.3488889", "-82.5000000"))?;
    wtr.serialize(("Gladstone", "OR", "12249", "45.3808333", "-122.5936111"))?;
    wtr.serialize(("Gresham", "OR", "98851", "45.4983333", "-122.4302778"))?;
    wtr.serialize(("Ephrata", "PA", "13182", "40.1797222", "-76.1791667"))?;
    wtr.serialize(("Mount Airy", "PA", "", "41.0941667", "-79.5222222"))?;
    wtr.serialize(("Uhlerstown", "PA", "", "40.5252778", "-75.0736111"))?;
    wtr.serialize(("Weis Library", "PA", "", "42.0483333", "-80.1700000"))?;
    wtr.serialize(("Woodcock", "PA", "", "41.7547222", "-80.0858333"))?;
    wtr.serialize(("Rock Hill", "SC", "59766", "34.9247222", "-81.0252778"))?;
    wtr.serialize(("Summerville", "SC", "34958", "33.0183333", "-80.1758333"))?;
    wtr.serialize(("Wolfton", "SC", "", "33.5883333", "-80.9819444"))?;
    wtr.serialize(("Avenger Village", "TX", "", "32.4594444", "-100.4552778"))?;
    wtr.serialize(("Brashear", "TX", "", "33.1186111", "-95.7333333"))?;
    wtr.serialize(("Dumas Junction", "TX", "", "35.2127778", "-101.8019444"))?;
    wtr.serialize(("Edinburg", "TX", "60509", "26.3013889", "-98.1630556"))?;
    wtr.serialize((
        "Eichelberger Crossing",
        "TX",
        "",
        "31.6166667",
        "-97.3077778",
    ))?;
    wtr.serialize(("Euless", "TX", "53221", "32.8369444", "-97.0816667"))?;
    wtr.serialize(("Greenock", "TX", "", "31.7661111", "-97.3452778"))?;
    wtr.serialize(("Greenville", "TX", "25382", "33.1383333", "-96.1105556"))?;
    wtr.serialize((
        "Highland Village",
        "TX",
        "15365",
        "33.0916667",
        "-97.0463889",
    ))?;
    wtr.serialize(("Maxey Town", "TX", "", "31.4433333", "-94.1225000"))?;
    wtr.serialize(("Pharr", "TX", "60687", "26.1944444", "-98.1833333"))?;
    wtr.serialize(("Snyder", "TX", "10600", "32.7177778", "-100.9172222"))?;
    wtr.serialize(("Webster", "TX", "9038", "29.5375000", "-95.1180556"))?;
    wtr.serialize(("Wild Peach Village", "TX", "", "29.0833333", "-95.6336111"))?;
    wtr.serialize((
        "Misty Hills Numbers 1-7",
        "UT",
        "",
        "40.6416667",
        "-111.9955556",
    ))?;
    wtr.serialize((
        "Pleasant Grove",
        "UT",
        "24449",
        "40.3641667",
        "-111.7377778",
    ))?;
    wtr.serialize(("Rio Vista", "VA", "", "37.5688889", "-77.5230556"))?;
    wtr.serialize(("Tabernacle", "VA", "", "37.4230556", "-76.2966667"))?;
    wtr.serialize(("Cody", "WY", "9161", "44.5263889", "-109.0558333"))?;

    Ok(())
}
