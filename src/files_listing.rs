use crate::resource::Resource;

use actix_files::Directory;
use actix_web::{HttpRequest, Responder, dev::ServiceResponse};
use askama::Template;
use askama_web::WebTemplate;
use std::{io, path::Path};
use urlencoding;

#[derive(Template, WebTemplate)]
#[template(path = "layout.html")]
struct PageTemplate<'a> {
    title: &'a str,
    folders: &'a [Resource],
    files: &'a [Resource],
}

#[derive(Template, WebTemplate)]
#[template(path = "main.html")]
struct ContentTemplate<'a> {
    title: &'a str,
    folders: &'a [Resource],
    files: &'a [Resource],
}

fn collect_items(
    directory: &Directory,
    base_url: &Path,
) -> io::Result<(Vec<Resource>, Vec<Resource>)> {
    let mut folders = Vec::new();
    let mut files = Vec::new();

    if let Some(parent) = Resource::parent(base_url) {
        folders.push(parent);
    }

    for read_result in directory.path.read_dir()? {
        if let Ok((resource, is_dir)) = Resource::from_read_result(read_result, base_url, directory)
        {
            if is_dir {
                folders.push(resource);
            } else {
                files.push(resource);
            }
        }
    }

    folders.sort();
    files.sort();

    Ok((folders, files))
}

pub(crate) fn render(directory: &Directory, request: &HttpRequest) -> io::Result<ServiceResponse> {
    let path = request.path();
    let title = urlencoding::decode(path).unwrap_or_default();
    let base_url = Path::new(path);
    let (folders, files) = collect_items(directory, base_url)?;

    let response = if !request.headers().contains_key("HX-Request") {
        PageTemplate {
            title: &title,
            folders: &folders,
            files: &files,
        }
        .respond_to(request)
    } else {
        ContentTemplate {
            title: &title,
            folders: &folders,
            files: &files,
        }
        .respond_to(request)
    };

    Ok(ServiceResponse::new(request.clone(), response))
}
