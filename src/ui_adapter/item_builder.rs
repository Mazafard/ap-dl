use crate::aparat::models::{AparatPlaylistInfo, AparatVideoInfo};
use crate::state::InternalTask;
use crate::MediaItem;

pub fn build_playlist_items(playlist: &AparatPlaylistInfo) -> (Vec<MediaItem>, Vec<InternalTask>) {
    let mut items = Vec::new();
    let mut tasks = Vec::new();

    for (idx, video) in playlist.items.iter().enumerate() {
        let item_id = format!("{}_{}", video.hash, idx);
        items.push(MediaItem {
            id: item_id.clone().into(),
            index_label: format!("{:02}", idx + 1).into(),
            title: video.title.clone().into(),
            duration: video.duration.clone().into(),
            resolution: "Auto".into(),
            file_size: "Auto".into(),
            progress: 0.0,
            speed_text: "Queued".into(),
            status_code: 0,
            selected: true,
        });
        tasks.push(InternalTask {
            id: item_id,
            video_hash: video.hash.clone(),
            title: video.title.clone(),
            direct_url: None,
            candidate_urls: Vec::new(),
        });
    }
    (items, tasks)
}

pub fn build_single_video_item(video: &AparatVideoInfo) -> (MediaItem, InternalTask) {
    let v_hash = video.hash.clone();
    let v_title = video.title.clone();
    let first_url = video.qualities.first().map(|q| q.url.clone());
    let candidate_urls = video.qualities.first().map(|q| q.urls.clone()).unwrap_or_default();
    let item_id = format!("{}_0", v_hash);

    let item = MediaItem {
        id: item_id.clone().into(),
        index_label: "01".into(),
        title: v_title.clone().into(),
        duration: video.formatted_duration.clone().into(),
        resolution: video.qualities.first().map(|q| q.label.clone()).unwrap_or_else(|| "Default".into()).into(),
        file_size: "Auto".into(),
        progress: 0.0,
        speed_text: "Queued".into(),
        status_code: 0,
        selected: true,
    };
    let task = InternalTask { id: item_id, video_hash: v_hash, title: v_title, direct_url: first_url, candidate_urls };
    (item, task)
}
