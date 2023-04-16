use crate::prisma::{self, referendum};
use async_graphql::{ComplexObject, Enum, SimpleObject};

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Referendum {
    pub id: String,
    pub group_id: String,
    pub name: String,
    pub slug: String,
    pub question: String,
    pub answers: Vec<String>,
    pub participants: Participants,
    pub participant_names: Vec<String>,
    pub participant_roles: Vec<String>,
    pub description: String,
    pub status: Status,
    pub start_date: String,
    pub end_date: String,
}

#[ComplexObject]
impl Referendum {}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum Participants {
    All,
    ByName,
    ByRole,
}

impl From<prisma::Participants> for Participants {
    fn from(value: prisma::Participants) -> Self {
        match value {
            prisma::Participants::All => Participants::All,
            prisma::Participants::ByName => Participants::ByName,
            prisma::Participants::ByRole => Participants::ByRole,
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum Status {
    Planned,
    InProgress,
    Closed,
}

impl From<prisma::Status> for Status {
    fn from(value: prisma::Status) -> Self {
        match value {
            prisma::Status::Planned => Status::Planned,
            prisma::Status::InProgress => Status::InProgress,
            prisma::Status::Closed => Status::Closed,
        }
    }
}

impl From<referendum::Data> for Referendum {
    fn from(val: referendum::Data) -> Self {
        Self {
            id: val.id,
            name: val.name,
            group_id: val.group_id,
            slug: val.slug,
            question: val.question,
            answers: val.answers,
            participants: val.participants.into(),
            description: val.description,
            participant_names: val.participant_names,
            participant_roles: val.participant_roles,
            status: val.status.into(),
            start_date: val.start_date.to_string(),
            end_date: val.end_date.to_string(),
        }
    }
}
