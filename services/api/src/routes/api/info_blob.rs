use crate::data::{DatabaseFilter, DatabaseFiltered};
use crate::error::Result;
use crate::models::*;
use crate::util::api_response;

use warp::{filters::body, filters::path, filters::BoxedFilter, Filter, Reply};

fn all(series_id: SeriesId, source: DatabaseFiltered) -> Result<Vec<InfoBlob>> {
    source.all_infoblobs(series_id)
}

fn select(series_id: SeriesId, blob_id: InfoBlobId, source: DatabaseFiltered) -> Result<InfoBlob> {
    source.get_infoblob(series_id, blob_id)
}

fn update(
    series_id: SeriesId,
    blob_id: InfoBlobId,
    form: InfoBlobForm,
    mut source: DatabaseFiltered,
) -> Result<InfoBlob> {
    source.update_infoblob(series_id, blob_id, form)
}

fn new(series_id: SeriesId, form: InfoBlobForm, mut source: DatabaseFiltered) -> Result<InfoBlob> {
    source.new_infoblob(series_id, form)
}

fn delete(
    series_id: SeriesId,
    blob_id: InfoBlobId,
    mut source: DatabaseFiltered,
) -> Result<InfoBlob> {
    source.delete_infoblob(series_id, blob_id)
}

fn select_types(
    series_id: SeriesId,
    types: String,
    source: DatabaseFiltered,
) -> Result<Vec<InfoBlob>> {
    source.get_info_types(series_id, types.split("+").collect())
}

pub fn routes(data_filter: DatabaseFilter) -> BoxedFilter<(impl Reply,)> {
    let all = warp::filters::method::get2()
        .and(path!("info" / SeriesId))
        .and(path::end())
        .and(data_filter.clone())
        .map(all)
        .map(api_response);

    let new = warp::filters::method::post2()
        .and(path!("info" / SeriesId))
        .and(path::end())
        .and(body::json::<InfoBlobForm>())
        .and(data_filter.clone())
        .map(new)
        .map(api_response);

    let select = warp::filters::method::get2()
        .and(path!("info" / SeriesId / InfoBlobId))
        .and(path::end())
        .and(data_filter.clone())
        .map(select)
        .map(api_response);

    let delete = warp::filters::method::delete2()
        .and(path!("info" / SeriesId / InfoBlobId))
        .and(path::end())
        .and(data_filter.clone())
        .map(delete)
        .map(api_response);

    let update = warp::filters::method::put2()
        .and(path!("info" / SeriesId / InfoBlobId))
        .and(path::end())
        .and(body::content_length_limit(1024 * 64))
        .and(body::json::<InfoBlobForm>())
        .and(data_filter.clone())
        .map(update)
        .map(api_response);

    let select_types = warp::filters::method::get2()
        .and(path!("info" / SeriesId / "types" / String))
        .and(path::end())
        .and(data_filter.clone())
        .map(select_types)
        .map(api_response);

    warp::any()
        .and(
            all.or(new)
                .or(update)
                .or(select)
                .or(delete)
                .or(select_types),
        )
        .boxed()
}
