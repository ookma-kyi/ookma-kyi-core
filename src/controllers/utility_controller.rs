use actix_files::NamedFile;
use actix_web::Result;

pub async fn robotstxt_file() -> Result<NamedFile> {
    Ok(NamedFile::open("src/public/robots.txt")?)
}