use std::collections::HashMap;
use std::convert::From;

use chrono::NaiveDateTime;

use rocket::serde::Serialize;
use rocket::Request;

use rocket_dyn_templates::Template;

mod lastfm;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct NowPlayingContext<'r> {
    track: &'r str,
    artist: &'r str,
    album: &'r str,
    album_art: &'r str,
    user: &'r str,
    key: &'r str,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Album {
    name: String,
    album_art: String,
}

impl From<&lastfm::Album> for Album {
    // Is there a better way to implement this?
    // I don't want to put the Serialize macro into the lastfm module,
    // which is why I have to convert the lastfm::Album to tera::Album.
    fn from(album: &lastfm::Album) -> Self {
        Album {
            name: album.name.clone(),
            album_art: album.album_art.clone(),
        }
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct NotPlayingContext<'r> {
    scrobble_count: &'r u32,
    account_creation: &'r str,
    albums: Vec<Album>,
    user: &'r str,
    key: &'r str,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct ApiContext<'r> {
    user: Option<&'r str>,
    key: Option<&'r str>,
}

#[get("/?<user>&<key>")]
pub async fn hello(user: Option<&str>, key: Option<&str>) -> Template {
    if let (Some(user), Some(key)) = (user, key) {
        if let Some(track) = lastfm::get_current_playing(user, key).await.unwrap() {
            // User is listening to music! Render the now playing screen.
            println!("{:?}", track);
            return Template::render(
                "index",
                &NowPlayingContext {
                    track: &track.name,
                    artist: &track.artist,
                    album: &track.album,
                    album_art: &track.album_art,
                    user: &user,
                    key: &key,
                },
            );
        } else {
            // User is not listening to music: show them stats for albums in the past week.
            let user_stats = lastfm::get_user_stats(user, key);
            let album_stats = lastfm::get_top_albums(user, key, lastfm::QueryPeriod::SevenDay, &9);

            let user_stats = user_stats.await.unwrap();
            let album_stats = album_stats.await.unwrap();

            // Convert account creation epoch time to string
            let account_creation = NaiveDateTime::from_timestamp(user_stats.registered_time, 0)
                    .date()
                    .format("%-d %B %Y")
                    .to_string();

            // Convert lastfm::Album to tera::Album so the objects can be serialized by Rocket for Tera
            let albums = album_stats.iter().map(|album| Album::from(album)).collect();

            return Template::render(
                "not_playing",
                &NotPlayingContext {
                    scrobble_count: &user_stats.scrobble_count,
                    account_creation: &account_creation,
                    albums,
                    user: &user,
                    key: &key,
                },
            );
        }
    }

    // If the user and key are not passed in through the URL, prompt user for them
    Template::render("no_params", &ApiContext { user, key })
}

#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Template {
    let mut map = HashMap::new();
    map.insert("path", req.uri().path().raw());
    Template::render("error/404", &map)
}
