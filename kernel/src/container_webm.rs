extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn container_webm_init() -> i32 {
    0
}

pub extern "C" fn container_webm_exit() -> i32 {
    0
}

pub struct WebMContainer {
    tracks: Vec<Track>,
    cues: Vec<CuePoint>,
    segments: Vec<Segment>,
    cluster: Cluster,
}

impl WebMContainer {
    pub fn new() -> Self {
        WebMContainer {
            tracks: Vec::new(),
            cues: Vec::new(),
            segments: Vec::new(),
            cluster: Cluster::new(),
        }
    }

    pub fn add_track(&mut self, track: Track) {
        self.tracks.push(track);
    }

    pub fn add_cue_point(&mut self, cue: CuePoint) {
        self.cues.push(cue);
    }

    pub fn add_segment(&mut self, segment: Segment) {
        self.segments.push(segment);
    }

    pub fn get_tracks(&self) -> &Vec<Track> {
        &self.tracks
    }

    pub fn get_cues(&self) -> &Vec<CuePoint> {
        &self.cues
    }
}

pub struct Track {
    track_number: u64,
    track_type: String,
    codec_id: String,
}

impl Track {
    pub fn new(track_number: u64, track_type: String, codec_id: String) -> Self {
        Track {
            track_number,
            track_type,
            codec_id,
        }
    }

    pub fn get_track_number(&self) -> u64 {
        self.track_number
    }

    pub fn get_track_type(&self) -> &str {
        &self.track_type
    }

    pub fn get_codec_id(&self) -> &str {
        &self.codec_id
    }
}

pub struct CuePoint {
    time: u64,
    track_positions: Vec<TrackPosition>,
}

impl CuePoint {
    pub fn new(time: u64) -> Self {
        CuePoint {
            time,
            track_positions: Vec::new(),
        }
    }

    pub fn add_track_position(&mut self, position: TrackPosition) {
        self.track_positions.push(position);
    }

    pub fn get_time(&self) -> u64 {
        self.time
    }
}

pub struct TrackPosition {
    track_number: u64,
    cluster_position: u64,
    block_number: u64,
}

impl TrackPosition {
    pub fn new(track_number: u64, cluster_position: u64, block_number: u64) -> Self {
        TrackPosition {
            track_number,
            cluster_position,
            block_number,
        }
    }

    pub fn get_track_number(&self) -> u64 {
        self.track_number
    }

    pub fn get_cluster_position(&self) -> u64 {
        self.cluster_position
    }

    pub fn get_block_number(&self) -> u64 {
        self.block_number
    }
}

pub struct Segment {
    timecode_scale: u64,
    duration: Option<u64>,
}

impl Segment {
    pub fn new(timecode_scale: u64, duration: Option<u64>) -> Self {
        Segment {
            timecode_scale,
            duration,
        }
    }

    pub fn get_timecode_scale(&self) -> u64 {
        self.timecode_scale
    }

    pub fn get_duration(&self) -> Option<u64> {
        self.duration
    }
}

pub struct Cluster {
    timecode: u64,
    blocks: Vec<Block>,
}

impl Cluster {
    pub fn new() -> Self {
        Cluster {
            timecode: 0,
            blocks: Vec::new(),
        }
    }

    pub fn set_timecode(&mut self, timecode: u64) {
        self.timecode = timecode;
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    pub fn get_timecode(&self) -> u64 {
        self.timecode
    }
}

pub struct Block {
    track_number: u64,
    timestamp: u64,
    data: Vec<u8>,
}

impl Block {
    pub fn new(track_number: u64, timestamp: u64, data: Vec<u8>) -> Self {
        Block {
            track_number,
            timestamp,
            data,
        }
    }

    pub fn get_track_number(&self) -> u64 {
        self.track_number
    }

    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn get_data(&self) -> &Vec<u8> {
        &self.data
    }
}
