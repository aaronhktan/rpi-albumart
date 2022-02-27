use reqwest::Error;

#[derive(Debug)]
pub struct Track {
    pub name: String,
    pub album: String,
    pub artist: String,
    pub album_art: String,
}

pub async fn get_current_playing(username: &str, api_key: &str) -> Result<Option<Track>, Error> {
    let request_url = format!(
        "https://ws.audioscrobbler.com/\
                            2.0/?method=user.getrecenttracks\
                            &user={username}&api_key={api_key}\
                            &limit=2&format=json",
        username = username,
        api_key = api_key
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
