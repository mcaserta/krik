use std::path::Path;
use warp::Filter;

pub fn serve_static_files(
    output_dir: impl AsRef<Path>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let output_dir = output_dir.as_ref().to_path_buf();

    // Serve files from output directory
    warp::fs::dir(output_dir.clone())
        .or(warp::path::end().and(warp::fs::file(output_dir.join("index.html"))))
}
