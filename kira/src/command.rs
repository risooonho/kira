use crate::{
	arrangement::{Arrangement, ArrangementId},
	audio_stream::{AudioStream, AudioStreamId},
	group::{Group, GroupId},
	instance::{
		Instance, InstanceId, PauseInstanceSettings, ResumeInstanceSettings, StopInstanceSettings,
	},
	metronome::{Metronome, MetronomeId},
	mixer::{
		effect::{Effect, EffectId, EffectSettings},
		SubTrackId, Track, TrackIndex,
	},
	parameter::{tween::Tween, ParameterId},
	playable::PlayableId,
	sequence::{SequenceInstance, SequenceInstanceId},
	sound::{Sound, SoundId},
	tempo::Tempo,
	value::Value,
};

#[derive(Debug, Clone)]
pub(crate) enum ResourceCommand {
	AddSound(Sound),
	RemoveSound(SoundId),
	AddArrangement(Arrangement),
	RemoveArrangement(ArrangementId),
}

#[derive(Debug, Clone)]
pub(crate) enum InstanceCommand {
	Play(InstanceId, Instance),
	SetInstanceVolume(InstanceId, Value<f64>),
	SetInstancePitch(InstanceId, Value<f64>),
	SetInstancePanning(InstanceId, Value<f64>),
	SeekInstance(InstanceId, f64),
	SeekInstanceTo(InstanceId, f64),
	PauseInstance(InstanceId, PauseInstanceSettings),
	ResumeInstance(InstanceId, ResumeInstanceSettings),
	StopInstance(InstanceId, StopInstanceSettings),
	PauseInstancesOf(PlayableId, PauseInstanceSettings),
	ResumeInstancesOf(PlayableId, ResumeInstanceSettings),
	StopInstancesOf(PlayableId, StopInstanceSettings),
	PauseInstancesOfSequence(SequenceInstanceId, PauseInstanceSettings),
	ResumeInstancesOfSequence(SequenceInstanceId, ResumeInstanceSettings),
	StopInstancesOfSequence(SequenceInstanceId, StopInstanceSettings),
	PauseGroup(GroupId, PauseInstanceSettings),
	ResumeGroup(GroupId, ResumeInstanceSettings),
	StopGroup(GroupId, StopInstanceSettings),
}

#[derive(Debug, Clone)]
pub(crate) enum MetronomeCommand {
	AddMetronome(MetronomeId, Metronome),
	RemoveMetronome(MetronomeId),
	SetMetronomeTempo(MetronomeId, Value<Tempo>),
	StartMetronome(MetronomeId),
	PauseMetronome(MetronomeId),
	StopMetronome(MetronomeId),
}

pub(crate) enum SequenceCommand {
	StartSequenceInstance(SequenceInstanceId, SequenceInstance),
	MuteSequenceInstance(SequenceInstanceId),
	UnmuteSequenceInstance(SequenceInstanceId),
	PauseSequenceInstance(SequenceInstanceId),
	ResumeSequenceInstance(SequenceInstanceId),
	StopSequenceInstance(SequenceInstanceId),
	PauseGroup(GroupId),
	ResumeGroup(GroupId),
	StopGroup(GroupId),
}

#[derive(Debug)]
pub(crate) enum MixerCommand {
	AddSubTrack(Track),
	RemoveSubTrack(SubTrackId),
	AddEffect(TrackIndex, Box<dyn Effect>, EffectSettings),
	SetEffectEnabled(TrackIndex, EffectId, bool),
	RemoveEffect(TrackIndex, EffectId),
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum ParameterCommand {
	AddParameter(ParameterId, f64),
	RemoveParameter(ParameterId),
	SetParameter(ParameterId, f64, Option<Tween>),
}

#[derive(Debug, Clone)]
pub(crate) enum GroupCommand {
	AddGroup(GroupId, Group),
	RemoveGroup(GroupId),
}

#[derive(Debug)]
pub(crate) enum StreamCommand {
	AddStream(AudioStreamId, TrackIndex, Box<dyn AudioStream>),
	RemoveStream(AudioStreamId),
}

pub(crate) enum Command {
	Resource(ResourceCommand),
	Instance(InstanceCommand),
	Metronome(MetronomeCommand),
	Sequence(SequenceCommand),
	Mixer(MixerCommand),
	Parameter(ParameterCommand),
	Group(GroupCommand),
	Stream(StreamCommand),
}

impl From<ResourceCommand> for Command {
	fn from(command: ResourceCommand) -> Self {
		Self::Resource(command)
	}
}

impl From<InstanceCommand> for Command {
	fn from(command: InstanceCommand) -> Self {
		Self::Instance(command)
	}
}

impl From<MetronomeCommand> for Command {
	fn from(command: MetronomeCommand) -> Self {
		Self::Metronome(command)
	}
}

impl From<SequenceCommand> for Command {
	fn from(command: SequenceCommand) -> Self {
		Self::Sequence(command)
	}
}

impl From<MixerCommand> for Command {
	fn from(command: MixerCommand) -> Self {
		Self::Mixer(command)
	}
}

impl From<ParameterCommand> for Command {
	fn from(command: ParameterCommand) -> Self {
		Self::Parameter(command)
	}
}

impl From<GroupCommand> for Command {
	fn from(command: GroupCommand) -> Self {
		Self::Group(command)
	}
}

impl From<StreamCommand> for Command {
	fn from(command: StreamCommand) -> Self {
		Self::Stream(command)
	}
}
