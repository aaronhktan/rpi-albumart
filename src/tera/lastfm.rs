use std::fmt;

use reqwest::Error;

#[derive(Debug)]
pub struct Track {
    pub name: String,
    pub album: String,
    pub artist: String,
    pub album_art: String,
}

#[derive(Debug)]
pub struct UserStats {
    pub scrobble_count: u32,
    pub registered_time: i64,
}

#[derive(Debug)]
pub struct Album {
    pub name: String,
    pub album_art: String,
}

#[allow(dead_code)]
// Only QueryPeriod::SevenDay is used right now
pub enum QueryPeriod {
    Overall,
    SevenDay,
    OneMonth,
    ThreeMonths,
    SixMonths,
    TwelveMonths,
}

impl fmt::Display for QueryPeriod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QueryPeriod::Overall => write!(f, "overall"),
            QueryPeriod::SevenDay => write!(f, "7day"),
            QueryPeriod::OneMonth => write!(f, "1month"),
            QueryPeriod::ThreeMonths => write!(f, "3month"),
            QueryPeriod::SixMonths => write!(f, "6month"),
            QueryPeriod::TwelveMonths => write!(f, "12month"),
        }
    }
}

pub async fn get_current_playing(username: &str, api_key: &str) -> Result<Option<Track>, Error> {
    let request_url = format!(
        "https://ws.audioscrobbler.com/\
                            2.0/?method={method}\
                            &user={username}&api_key={api_key}\
                            &format=json",
        method = "user.getrecenttracks",
        username = username,
        api_key = api_key,
    );

    let response = reqwest::get(&request_url).await?;
    let json = response.json::<serde_json::Value>().await?;

    if json["recenttracks"]["track"][0]["@attr"]["nowplaying"] != "true" {
        return Ok(None);
    }

    let name = String::from(json["recenttracks"]["track"][0]["name"].as_str().unwrap());
    let album = String::from(
        json["recenttracks"]["track"][0]["album"]["#text"]
            .as_str()
            .unwrap(),
    );
    let artist = String::from(
        json["recenttracks"]["track"][0]["artist"]["#text"]
            .as_str()
            .unwrap(),
    );
    let art = String::from(
        json["recenttracks"]["track"][0]["image"][3]["#text"]
            .as_str()
            .unwrap(),
    );

    Ok(Some(Track {
        name: name,
        album: album,
        artist: artist,
        album_art: art,
    }))
}

pub async fn get_user_stats(username: &str, api_key: &str) -> Result<UserStats, Error> {
    let request_url = format!(
        "https://ws.audioscrobbler.com/\
                            2.0/?method={method}\
                            &user={username}&api_key={api_key}\
                            &format=json",
        method = "user.getinfo",
        username = username,
        api_key = api_key,
    );

    let response = reqwest::get(&request_url).await?;
    let json = response.json::<serde_json::Value>().await?;

    let scrobble_count = json["user"]["playcount"]
        .as_str()
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let registered_time = json["user"]["registered"]["#text"]
        .as_i64()
        .unwrap();
    println!("{}", format!("Scrobbles: {}", scrobble_count));

    Ok(UserStats { scrobble_count, registered_time })
}

pub async fn get_top_albums(
    username: &str,
    api_key: &str,
    period: QueryPeriod,
    limit: &u8,
) -> Result<Vec<Album>, Error> {
    let request_url = format!(
        "https://ws.audioscrobbler.com/\
                            2.0/?method={method}\
                            &user={username}&api_key={api_key}\
                            &period={period}&limit={limit}\
                            &format=json",
        method = "user.gettopalbums",
        username = username,
        api_key = api_key,
        period = period.to_string(),
        limit = limit.to_string(),
    );

    let response = reqwest::get(&request_url).await?;
    let json = response.json::<serde_json::Value>().await?;

    let mut albums = Vec::new();
    let json_albums = json["topalbums"]["album"].as_array().unwrap();

    for album in json_albums {
        albums.push(Album {
            name: String::from(album["name"].as_str().unwrap()),
            album_art: String::from(album["image"][3]["#text"].as_str().unwrap()),
        });

        if albums.len() >= *limit as usize {
            break;
        }
    }

    Ok(albums)
}
