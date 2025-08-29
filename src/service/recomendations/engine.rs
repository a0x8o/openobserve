// Copyright 2025 OpenObserve Inc.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use config::{
    META_ORG_ID,
    meta::{search, search::SearchEventType, stream::StreamType},
};
use infra::{errors::Error, schema::STREAM_SETTINGS};
use o2_enterprise::enterprise::common::config::get_config as get_o2_config;

use crate::service::search as SearchService;

pub async fn get_query_data_from_usage(start_time: i64, end_time: i64) -> Result<search::Response, Error> {
    let trace_id = config::ider::generate_trace_id();
    let user_id = "query_reco_user".to_string();
    let sql =r#"SELECT request_body ,count(request_body) as r_count , max(response_time) as m_rs  ,org_id FROM \"usage\" WHERE event = 'Search' AND search_type != 'ui' group by request_body ,org_id  order by m_rs desc"#.to_string();

    let req = config::meta::search::Request {
        query: config::meta::search::Query {
            sql,
            from: 0,
            size: -1,
            start_time,
            end_time,
            quick_mode: false,
            query_type: "".to_string(),
            track_total_hits: false,
            uses_zo_fn: false,
            query_fn: None,
            action_id: None,
            skip_wal: false,
            streaming_output: false,
            streaming_id: None,
            histogram_interval: 0,
        },
        encoding: config::meta::search::RequestEncoding::Empty,
        regions: vec![],
        clusters: vec![],
        timeout: 0,
        search_type: Some(SearchEventType::Other),
        search_event_context: None,
        use_cache: false,
        local_mode: None,
    };
    let resp = SearchService::search(
        &trace_id,
        META_ORG_ID,
        StreamType::Logs,
        Some(user_id),
        &req,
    )
    .await?;

    println!("resp: {:?}", resp);
    Ok(resp)
}

pub async fn get_distinct_values(
    field_names: &[&str],
    stream_name: &str,
    org_id: &str,
    start_time: i64,
    end_time: i64
) -> Result<search::Response, Error> {
    let trace_id = config::ider::generate_trace_id();
    let user_id = "query_reco_user".to_string();
    let fields = field_names
        .iter()
        .map(|field| format!(" approx_distinct({field}) as {field} "))
        .collect::<Vec<String>>()
        .join(",");
    let fields = fields.strip_suffix(",").unwrap_or(&fields);

    let sql = format!("SELECT {fields} FROM \"{stream_name}\"");

    let req = config::meta::search::Request {
        query: config::meta::search::Query {
            sql,
            from: 0,
            size: -1,
            start_time,
            end_time,
            quick_mode: false,
            query_type: "".to_string(),
            track_total_hits: false,
            uses_zo_fn: false,
            query_fn: None,
            action_id: None,
            skip_wal: false,
            streaming_output: false,
            streaming_id: None,
            histogram_interval: 0,
        },
        encoding: config::meta::search::RequestEncoding::Empty,
        regions: vec![],
        clusters: vec![],
        timeout: 0,
        search_type: Some(SearchEventType::Other),
        search_event_context: None,
        use_cache: false,
        local_mode: None,
    };
    let resp = SearchService::search(
        &trace_id,
        org_id,
        StreamType::Logs,
        Some(user_id),
        &req,
    )
    .await?;

    println!("resp: {:?}", resp);
    Ok(resp)
}

async fn get_stream_settings() -> Result<(), Error> {
    let r = STREAM_SETTINGS.read().await;
    for (key, value) in r.iter() {
        let columns = key.split('/').collect::<Vec<&str>>();
        let org_id = columns[0];
        let stream_type = StreamType::from(columns[1]);
        let stream_name = columns[2];

    }
    Ok(())

}  

pub async fn get_recommendations()-> Result<(), Error>{
    let o2_config = get_o2_config();
    let end_time = chrono::Utc::now().timestamp_micros();
    let start_time =
        end_time - (o2_config.common.query_recommendations_interval * 60 * 1000 * 1000);
    log::info!("Stage 1: Getting query data from usage");
    let usage_resp = get_query_data_from_usage(start_time, end_time).await?;

    let q_hits = usage_resp.hits;
    if q_hits.is_empty(){
        log::info!("No queries found in usage");
        return Ok(());
    }
    //let usage_hits = o2_enterprise::enterprise::recommendations::RecommendationInputRecords::build_from_hits(q_hits)?;

    
    

    Ok(())

}
