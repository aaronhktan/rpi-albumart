use std::collections::HashMap;

use rocket::serde::Serialize;
use rocket::Request;

use rocket_dyn_templates::Template;

mod lastfm;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct TemplateContext<'r> {
    name: &'r str,
    track: &'r str,
    artist: &'r str,
    album: &'r str,
    album_art: &'r str,
    user: &'r str,
    key: &'r str,
}

#[get("/?<user>&<key>")]
pub async fn hello(user: Option<&str>, key: Option<&str>) -> Template {
    if let (Some(user), Some(key)) = (user, key) {
        if let Some(track) = lastfm::get_current_playing(user, key).await.unwrap() {
            println!("{:?}", track);
            return Template::render(
                "index",
                &TemplateContext {
                    name: user,
                    track: &track.name,
                    artist: &track.artist,
                    album: &track.album,
                    album_art: &track.album_art,
                    user: &user,
                    key: &key,
                },
            );
        }
        return Template::render(
            "index",
            &TemplateContext {
                name: "None",
                track: "None",
                artist: "None",
                album: "None",
                album_art: "None",
                user: &user,
                key: &key,
            },
        );
    }
    Template::render(
        "index",
        &TemplateContext {
            name: "None",
            track: "None",
            artist: "None",
            album: "None",
            album_art: "None",
            user: "None",
            key: "None",
        },
    )
}

#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Template {
    let mut map = HashMap::new();
    map.insert("path", req.uri().path().raw());
    Template::render("error/404", &map)
}
