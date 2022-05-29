use crate::config;
use crate::ApiError;
use crate::Watermark;
use actix_multipart::{Field, Multipart, MultipartError};
use actix_web::{
    error::PayloadError,
    web::{block, Data},
};
use image::{imageops::FilterType, GenericImage, GenericImageView, Pixel, Rgba};
use rand::{thread_rng, Rng};
use serde_json::{Map, Value};
use std::sync::Arc;

use futures::future::TryFutureExt;
use tokio::{fs, prelude::*, process::Command, stream::StreamExt};
use uuid::Uuid;

#[derive(Debug)]
pub struct KeyValue {
    pub key: String,
    pub value: String,
}

#[derive(Serialize, Debug)]
pub struct FileInfo {
    pub uuid: Uuid,
    pub name: Box<str>,
    pub extension: AttachmentExtension,
    pub width: u32,
    pub height: u32,
    pub size: usize,
    // pub hash: Box<str>,
}

#[derive(Debug)]
pub enum MultipartValue {
    KeyValue(KeyValue),
    FileInfo(FileInfo),
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum AttachmentExtension {
    #[serde(rename = "png")]
    Png,
    #[serde(rename = "jpeg")]
    Jpeg,
    #[serde(rename = "jpg")]
    Jpg,
    #[serde(rename = "gif")]
    Gif,
    // WebP,
    // Mp4,
    WebM,
    // Gif,
}

impl AttachmentExtension {
    pub fn from_str(s: &str) -> Option<Self> {
        use AttachmentExtension::*;

        match s {
            "png" => Some(Png),
            "jpg" => Some(Jpg),
            "jpeg" => Some(Jpeg),
            "gif" => Some(Gif),
            "webm" => Some(WebM),

            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        use AttachmentExtension::*;

        match self {
            Png => "png",
            Jpg => "jpg",
            Jpeg => "jpeg",
            Gif => "gif",
            WebM => "webm",
        }
    }
}

// #[derive(Debug)]
pub struct State {
    pub uploads_max: usize,
    pub uploads_cur: usize,
    pub values: Map<String, Value>,
    pub files: Vec<FileInfo>,
    pub total_acc: usize,
    pub watermark: Data<Watermark>,
}

impl State {
    pub fn new(uploads_max: usize, watermark: Data<Watermark>) -> Self {
        Self {
            watermark,
            uploads_max,
            uploads_cur: 0,
            values: Map::new(),
            files: Vec::with_capacity(uploads_max),
            total_acc: 0,
        }
    }

    pub async fn cleanup_files(&mut self) {
        for file in &self.files {
            fs::remove_file(format!(
                "uploads/full/_temp/{}.{}",
                file.uuid,
                file.extension.as_str()
            ))
            .await
            .expect("Failed to remove temp upload at cleanup");
            fs::remove_file(format!(
                "uploads/thumbs/{}.{}",
                file.uuid,
                config::THUMBNAIL_EXTENSION
            ))
            .await
            .expect("Failed to remove thumbnail at cleanup");
        }

        self.files.clear();
    }

    pub async fn untemp_files(&self) {
        for file in &self.files {
            fs::rename(
                format!(
                    "uploads/full/_temp/{}.{}",
                    file.uuid,
                    file.extension.as_str()
                ),
                format!("uploads/full/{}.{}", file.uuid, file.extension.as_str()),
            )
            .await
            .expect("Failed to untemp file");
        }
    }
}

pub async fn extract(
    mut multipart: Multipart,
    watermark: Data<Watermark>,
    max: usize,
) -> Result<State, ApiError> {
    let mut state = State::new(max, watermark);

    while let Some(field) = multipart.next().await {
        let f = field?;

        let res = process_field(&mut state, f).await;

        match res {
            Ok(v) => match v {
                MultipartValue::KeyValue(kv) => {
                    state
                        .values
                        .insert(kv.key, serde_json::Value::String(kv.value));
                }
                MultipartValue::FileInfo(file) => {
                    state.files.push(file);
                }
            },

            Err(e) => {
                state.cleanup_files().await;

                return Err(e);
            }
        };
    }

    Ok(state)
}

pub async fn process_field(state: &mut State, field: Field) -> Result<MultipartValue, ApiError> {
    let ct = field
        .content_disposition()
        .ok_or(MultipartError::NoContentType)?;

    let name = ct.get_name().ok_or(MultipartError::ParseContentType)?;

    Ok(match ct.get_filename() {
        Some(displayname) => proccess_file(state, name, displayname, field).await,

        None => proccess_kv(state, name, field).await,
    }?)
}

pub async fn proccess_file(
    state: &mut State,
    name: &str,
    displayname: &str,
    field: Field,
) -> Result<MultipartValue, ApiError> {
    if displayname.len() > 128 {
        return Err(ApiError::multipart_filename_too_long(name));
    }

    if state.uploads_cur == state.uploads_max {
        return Err(ApiError::multipart_files_limit());
    }

    state.uploads_cur += 1;

    if name.len() <= 4 || &name[0..4] != "file" {
        return Err(ApiError::multipart_file_bad_field_name(name));
    }

    let ext = get_extension_from_filename(&displayname).ok_or(MultipartError::ParseContentType)?;

    if ext.len() > 32 {
        return Err(ApiError::multipart_file_extension_too_long(name));
    }

    let extu = ext.to_uppercase();

    let extension = match extu.as_str() {
        "PNG" => AttachmentExtension::Png,
        "JPG" => AttachmentExtension::Jpg,
        "JPEG" => AttachmentExtension::Jpeg,
        "GIF" => AttachmentExtension::Gif,
        "WEBM" => AttachmentExtension::WebM,

        &_ => return Err(ApiError::multipart_unknown_extenstions(name, ext)),
    };

    let uuid = Uuid::new_v4();
    let filename = format!("{}.{}", uuid, ext);
    let path = Arc::new(format!("uploads/full/_temp/{}", filename));
    let tpath = format!("uploads/thumbs/{}.{}", uuid, config::THUMBNAIL_EXTENSION);

    let size = save_file(state, &path, field).await?;

    // FIXME: barebone
    tokio::time::delay_for(std::time::Duration::from_millis(100)).await;

    let save_thumb_path = path.clone();
    let webm_path = path.clone();
    let result = match extension {
        AttachmentExtension::Png
        | AttachmentExtension::Jpeg
        | AttachmentExtension::Jpg
        | AttachmentExtension::Gif => {
            let block_path = path.clone();
            match block(move || image::open(block_path.as_ref())).await {
                Ok(mut image) => {
                    let mut randomizer = thread_rng();
                    let (w, h) = image.dimensions();
                    if config::WATERMARK_ENABLED == true {
                        match extension {
                            AttachmentExtension::Png
                            | AttachmentExtension::Jpeg
                            | AttachmentExtension::Jpg => {
                                // add watermark and save file
                                // step 1. clone and resize the watermark
                                let watermark_resized = state
                                    .watermark
                                    .as_ref()
                                    .0
                                    .clone()
                                    .resize(
                                        w / config::WATERMARK_RATIO,
                                        h / config::WATERMARK_RATIO,
                                        FilterType::Triangle,
                                    )
                                    .to_rgba();
                                let (ww, wh) = watermark_resized.dimensions();

                                // fill pixels to image
                                let offset_w = 32;
                                let offset_h = 32;

                                if w > ww + offset_w && h > wh + offset_h {
                                    let x = w - ww - offset_w;
                                    let y = h - wh - offset_h;
                                    let r = (randomizer.gen(), randomizer.gen(), randomizer.gen());
                                    for (n, pixel) in watermark_resized.pixels().enumerate() {
                                        let mut pixel = pixel.clone();
                                        let nn = if n == 0 { 1 } else { n };
                                        if pixel.0 != [0, 0, 0, 0] {
                                            if config::WATERMARK_RANDOMIZE_COLOR {
                                                pixel = Rgba([r.0, r.1, r.2, 200])
                                            }

                                            let xa = x + (nn as u32 % ww);
                                            let ya = y + (nn as u32 / ww);
                                            let mut pix = image.get_pixel(xa, ya);
                                            pix.blend(&pixel);
                                            image.put_pixel(xa, ya, pix);
                                        }
                                    }

                                    image = block(move || {
                                        image.save(save_thumb_path.clone().as_ref()).map(|_| image)
                                    })
                                    .await
                                    .expect("failed to save watermarked file");
                                }
                            }
                            _ => {}
                        }
                    }

                    let thumb = image.thumbnail(config::THUMBNAIL_SIZE, config::THUMBNAIL_SIZE);

                    let _ = block(move || thumb.save(tpath))
                        .await
                        .expect("Failed to save thumbnail");

                    Some((w, h))
                }

                Err(e) => None,
            }
        }

        AttachmentExtension::WebM => {
            // FIXME: what are fuck, remove this shit
            let result_wh = Command::new("ffprobe")
                .arg("-v")
                .arg("error")
                .arg("-show_entries")
                .arg("stream=width,height")
                .arg("-of")
                .arg("default=noprint_wrappers=1:nokey=1")
                .arg(webm_path.as_ref())
                .output()
                .await?;

            let result_duration = Command::new("ffprobe")
                .arg("-v")
                .arg("error")
                .arg("-show_entries")
                .arg("format=duration")
                .arg("-of")
                .arg("default=noprint_wrappers=1:nokey=1")
                .arg(webm_path.as_ref())
                .output()
                .await?;

            if (result_wh.status.success() && result_duration.status.success()) {
                // get utf8 string
                let string_wh = String::from_utf8_lossy(&result_wh.stdout);
                // get duration of the video
                let mut lines = string_wh.lines();
                const ERR_MSG: &str = "Failed to parse ffmpeg w,h response";
                let mut next = move || {
                    let mut n = lines.next().unwrap();
                    while n == "N/A" {
                        n = lines.next().unwrap();
                    }

                    n
                };
                let (w, h): (u32, u32) = (
                    next().parse().expect(ERR_MSG),
                    next().parse().expect(ERR_MSG),
                );

                // get utf8 string
                let string_duration = String::from_utf8_lossy(&result_duration.stdout);
                let mut duration = string_duration.lines().next().unwrap().parse().expect("failed to parse ffmpeg duration response");

                // if duration is greater than 10 then 10, because hard decoding
                if (duration > 10.0) {
                    duration = 10.0;
                }

                // get random sec
                let sec = rand::thread_rng().gen_range(0.0, duration);
                let size = config::THUMBNAIL_SIZE.to_string();

                let result = Command::new("ffmpeg")
                    .arg("-ss")
                    .arg(sec.to_string())
                    .arg("-i")
                    .arg(webm_path.as_ref())
                    .arg("-vf")
                    .arg(["scale=", &size, ":-2,setsar=1:1"].concat())
                    .arg("-f")
                    .arg("image2")
                    .arg("-vframes")
                    .arg("1")
                    .arg(&tpath)
                    .output()
                    .await?;

                if (result.status.success()) {
                    let r = String::from_utf8_lossy(&result.stderr);
    
                    // check if there are errors
                    if (r.contains("File extends beyond end of segment.")) {
                        // shit
                        let result = Command::new("ffmpeg")
                        .arg("-i")
                        .arg(webm_path.as_ref())
                        .arg("-vf")
                        .arg(["scale=", &size, ":-2,setsar=1:1"].concat())
                        .arg("-f")
                        .arg("image2")
                        .arg("-vframes")
                        .arg("1")
                        .arg(&tpath)
                        .output()
                        .await?;

                        if (result.status.success()) {
                            Some((w, h))
                        } else {
                            None
                        }
                    } else {
                        Some((w, h))
                    }

                } else {
                    None
                }
            } else {
                None
            }
        }
    };

    if let Some((w, h)) = result {
        Ok(MultipartValue::FileInfo(FileInfo {
            uuid,
            extension,
            size: size,
            width: w,
            height: h,
            name: displayname.into(),
        }))
    } else {
        fs::remove_file(path.as_ref())
            .await
            .expect("failed to remove file");
        Err(ApiError::multipart_file_corrupted(name))
    }
}

pub async fn save_file(state: &mut State, path: &str, mut field: Field) -> Result<usize, ApiError> {
    println!("{:?}", path);
    let mut file = fs::File::create(path)
        .await
        .expect("Failed to create temp file");

    let mut acc = 0;

    while let Some(chunk) = field.next().await {
        let data = chunk?;

        let len = data.len();
        acc += len;

        state.total_acc += len;

        if acc > config::MAX_FILE_SIZE || state.total_acc > config::MAX_PAYLOAD_SIZE {
            fs::remove_file(path).await.expect("Failed to remove file");

            return Err(ApiError::multipart_field_overflow());
        }

        file.write_all(data.as_ref())
            .await
            .expect("Failed to write to file");
    }

    Ok(acc)
}

pub async fn proccess_kv(
    state: &mut State,
    name: &str,
    mut field: Field,
) -> Result<MultipartValue, ApiError> {
    let mut val = String::with_capacity(1024);
    let mut acc = 0;

    while let Some(chunk) = field.next().await {
        let data = chunk?;
        let len = data.len();

        acc += len;
        state.total_acc += len;

        if acc > config::MAX_FILE_SIZE || state.total_acc > config::MAX_PAYLOAD_SIZE {
            return Err(ApiError::multipart_field_overflow());
        }

        val += &String::from_utf8(data.to_vec())
            .map_err(|_| MultipartError::Payload(PayloadError::EncodingCorrupted))?;
    }

    Ok(MultipartValue::KeyValue(KeyValue {
        key: name.into(),
        value: val.into(),
    }))
}

fn get_extension_from_filename(filename: &str) -> Option<&str> {
    use std::ffi::OsStr;
    use std::path::Path;

    Path::new(filename).extension().and_then(OsStr::to_str)
}
