//! MPD commands

use crate::{
    client::resp::{
        handlers::{MixedResponseResponse, OkResponse, RespMapResponse, ResponseHandler, Tracks},
        respmap_handlers::ListallResponse,
    },
    DatabaseVersion,
};

#[derive(Copy, Clone)]
pub struct Stats;
#[derive(Copy, Clone)]
pub struct Status;

#[derive(Copy, Clone)]
pub struct Setvol(pub u32);
#[derive(Copy, Clone)]
pub struct Repeat(pub bool);
#[derive(Copy, Clone)]
pub struct Random(pub bool);
#[derive(Copy, Clone)]
pub struct Consume(pub bool);

#[derive(Copy, Clone)]
pub struct PlayId(pub u32);
#[derive(Copy, Clone)]
pub struct PlayAtPosition(pub u32);
#[derive(Copy, Clone)]
pub struct QueueClear;
#[derive(Copy, Clone)]
pub struct QueueAdd<'a,'b>(pub &'a str, pub Option<&'b str>);
#[derive(Copy, Clone)]
pub struct QueueMoveId<'a>(pub u32, pub &'a str);
#[derive(Copy, Clone)]
pub struct QueueDeleteId(pub u32);

#[derive(Copy, Clone)]
pub struct Search<'a>(pub Option<&'a str>);
#[derive(Copy, Clone)]
pub struct PlaylistInfo;

#[derive(Copy, Clone)]
pub struct Stop;
#[derive(Copy, Clone)]
pub struct PlayPause(pub bool);
#[derive(Copy, Clone)]
pub struct Next;
#[derive(Copy, Clone)]
pub struct Prev;

#[derive(Copy, Clone)]
pub struct Rescan<'a>(pub Option<&'a str>);
#[derive(Copy, Clone)]
pub struct Update<'a>(pub Option<&'a str>);

#[derive(Copy, Clone)]
pub struct Idle;
#[derive(Copy, Clone)]
pub struct NoIdle;

#[derive(Copy, Clone)]
pub struct Listall<'a>(pub Option<&'a str>);
#[derive(Copy, Clone)]
pub struct ListallInfo<'a>(pub Option<&'a str>);

pub enum MpdCmdParameters {
    String(String),
    StringAndString(String, String),
    U32AndString(u32, String),
}
pub trait MpdCmd {
    /// The Command name
    const CMD: &'static str;
    /// The Response handler for this command
    type Handler: ResponseHandler;
    /// Optionally returns the commands argument as a String
    fn argument(&self) -> Option<MpdCmdParameters> {
        None
    }
    /// Creates the MPD command line for this command
    fn to_cmdline(&self) -> String {
        if let Some(arg) = self.argument() {
            match arg {
                MpdCmdParameters::String(arg) => format!("{} \"{}\"\n", Self::CMD, arg),
                MpdCmdParameters::StringAndString(arg0, arg1) => format!("{} \"{}\" \"{}\"\n", Self::CMD, arg0, arg1),
                MpdCmdParameters::U32AndString(arg0, arg1) => format!("{} {} \"{}\"\n", Self::CMD, arg0, arg1),
            }
        } else {
            format!("{}\n", Self::CMD)
        }
    }
}

impl<'a> MpdCmd for ListallInfo<'a> {
    const CMD: &'static str = "listallinfo";
    type Handler = MixedResponseResponse;

    fn argument(&self) -> Option<MpdCmdParameters> {
        match self.0 {
            Some(arg) => Some(MpdCmdParameters::String(arg.to_string())),
            None => None,
        }        
    }
}

impl<'a,'b> MpdCmd for QueueAdd<'a,'b> {
    const CMD: &'static str = "add";
    type Handler = OkResponse;

    fn argument(&self) -> Option<MpdCmdParameters> {
        match self.1 {
            Some(position) => Some(MpdCmdParameters::StringAndString(self.0.to_string(), position.to_string())),
            None => Some(MpdCmdParameters::String(self.0.to_string())),
        }
    }
}

impl<'a> MpdCmd for QueueMoveId<'a> {
    const CMD: &'static str = "moveid";
    type Handler = OkResponse;

    fn argument(&self) -> Option<MpdCmdParameters> {
        Some(MpdCmdParameters::U32AndString(self.0 as u32, self.1.to_string()))
    }
}

impl MpdCmd for QueueDeleteId {
    const CMD: &'static str = "deleteid";
    type Handler = OkResponse;

    fn argument(&self) -> Option<MpdCmdParameters> {
        Some(MpdCmdParameters::String(self.0.to_string()))
    }
}

impl<'a> MpdCmd for Listall<'a> {
    const CMD: &'static str = "listall";
    type Handler = RespMapResponse<ListallResponse>;

    fn argument(&self) -> Option<MpdCmdParameters> {
        match self.0 {
            Some(arg) => Some(MpdCmdParameters::String(arg.to_string())),
            None => None,
        }
    }
}

impl<'a> MpdCmd for Update<'a> {
    const CMD: &'static str = "update";
    type Handler = RespMapResponse<DatabaseVersion>;

    fn argument(&self) -> Option<MpdCmdParameters> {
        match self.0 {
            Some(arg) => Some(MpdCmdParameters::String(arg.to_string())),
            None => None,
        }        
    }
}

impl<'a> MpdCmd for Rescan<'a> {
    const CMD: &'static str = "rescan";
    type Handler = RespMapResponse<DatabaseVersion>;

    fn argument(&self) -> Option<MpdCmdParameters> {
        match self.0 {
            Some(arg) => Some(MpdCmdParameters::String(arg.to_string())),
            None => None,
        }        
    }
}

impl<'a> MpdCmd for Search<'a> {
    const CMD: &'static str = "search";
    type Handler = Tracks;
    fn argument(&self) -> Option<MpdCmdParameters> {
        match self.0 {
            Some(arg) => Some(MpdCmdParameters::String(arg.to_string())),
            None => None,
        }        
    }
}

impl MpdCmd for PlaylistInfo {
    const CMD: &'static str = "playlistinfo";
    type Handler = Tracks;
}

impl MpdCmd for Repeat {
    const CMD: &'static str = "repeat";
    type Handler = OkResponse;
    fn argument(&self) -> Option<MpdCmdParameters> {
        Some(MpdCmdParameters::String((self.0 as u32).to_string()))
    }
}

impl MpdCmd for Random {
    const CMD: &'static str = "random";
    type Handler = OkResponse;
    fn argument(&self) -> Option<MpdCmdParameters> {
        Some(MpdCmdParameters::String((self.0 as u32).to_string()))
    }
}

impl MpdCmd for Consume {
    const CMD: &'static str = "consume";
    type Handler = OkResponse;
    fn argument(&self) -> Option<MpdCmdParameters> {
        Some(MpdCmdParameters::String((self.0 as u32).to_string()))
    }
}

impl MpdCmd for PlayPause {
    const CMD: &'static str = "pause";
    type Handler = OkResponse;
    fn argument(&self) -> Option<MpdCmdParameters> {
        Some(MpdCmdParameters::String((self.0 as u32).to_string()))
    }
}

impl MpdCmd for Next {
    const CMD: &'static str = "next";
    type Handler = OkResponse;
}
impl MpdCmd for Prev {
    const CMD: &'static str = "prev";
    type Handler = OkResponse;
}

impl MpdCmd for QueueClear {
    const CMD: &'static str = "clear";
    type Handler = OkResponse;
}

impl MpdCmd for NoIdle {
    const CMD: &'static str = "noidle";
    type Handler = OkResponse;
}

impl MpdCmd for Idle {
    const CMD: &'static str = "idle";
    type Handler = RespMapResponse<crate::Subsystem>;
}

impl MpdCmd for Stats {
    const CMD: &'static str = "stats";
    type Handler = RespMapResponse<crate::Stats>;
}

impl MpdCmd for Status {
    const CMD: &'static str = "status";
    type Handler = RespMapResponse<crate::Status>;
}

impl MpdCmd for Setvol {
    const CMD: &'static str = "setvol";
    type Handler = OkResponse;

    fn argument(&self) -> Option<MpdCmdParameters> {
        Some(MpdCmdParameters::String(self.0.to_string()))
    }
}

impl MpdCmd for Stop {
    const CMD: &'static str = "stop";
    type Handler = OkResponse;
}

impl MpdCmd for PlayId {
    const CMD: &'static str = "playid";
    type Handler = OkResponse;

    fn argument(&self) -> Option<MpdCmdParameters> {
        Some(MpdCmdParameters::String(self.0.to_string()))
    }
}

impl MpdCmd for PlayAtPosition {
    const CMD: &'static str = "play";
    type Handler = OkResponse;

    fn argument(&self) -> Option<MpdCmdParameters> {
        Some(MpdCmdParameters::String(self.0.to_string()))
    }
}
