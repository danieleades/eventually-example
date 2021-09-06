use eventually::Aggregate;

mod item;
pub use item::Item;

mod state;
pub use state::State;

mod command;
pub use command::Command;

mod event;
pub use event::Event;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("order has already been created")]
    AlreadyCreated,

    #[error("order has already been created")]
    NotYetCreated,

    #[error("order can't be edited anymore")]
    NotEditable,

    #[error("order has already been cancelled")]
    AlreadyCompleted,

    #[error("order has already been ccompleted")]
    AlreadyCancelled,
}

pub struct OrderAggregate;

impl Aggregate for OrderAggregate {
    type Id = String;

    type State = State;

    type Event = Event;

    type Command = Command;

    type Error = Error;

    fn apply(state: Self::State, event: Self::Event) -> Result<Self::State, Self::Error> {
        match (state, event) {
            (State::NotStarted, Event::Created { id, at }) => Ok(State::create(id, at)),
            (State::NotStarted, _) => Err(Error::NotYetCreated),
            (_, Event::Created { id: _, at: _ }) => Err(Error::AlreadyCreated),
            (State::Editable(mut state), Event::ItemAdded { item, at }) => {
                state.add_item(item, at);
                Ok(state.into())
            }
            (State::Editable(state), Event::Completed { at }) => Ok(state.complete(at).into()),
            (State::Editable(state), Event::Cancelled { at }) => Ok(state.cancel(at).into()),
            (_, Event::ItemAdded { item: _, at: _ }) => Err(Error::NotEditable),
            (State::Complete(_), Event::Completed { at }) => todo!(),
            (State::Complete(_), Event::Cancelled { at }) => todo!(),
            (State::Cancelled(_), Event::Completed { at }) => todo!(),
            (State::Cancelled(_), Event::Cancelled { at }) => todo!(),
        }
    }

    fn handle<'a, 's: 'a>(
        &'a self,
        id: &'s Self::Id,
        state: &'s Self::State,
        command: Self::Command,
    ) -> futures::future::BoxFuture<'a, Result<Option<Vec<Self::Event>>, Self::Error>>
    where
        Self: Sized,
    {
        match (state, command) {
            (State::NotStarted, Command::Create) => todo!(),
            (State::NotStarted, Command::AddItem(_)) => todo!(),
            (State::NotStarted, Command::Complete) => todo!(),
            (State::NotStarted, Command::Cancel) => todo!(),
            (State::Editable(_), Command::Create) => todo!(),
            (State::Editable(_), Command::AddItem(_)) => todo!(),
            (State::Editable(_), Command::Complete) => todo!(),
            (State::Editable(_), Command::Cancel) => todo!(),
            (State::Complete(_), Command::Create) => todo!(),
            (State::Complete(_), Command::AddItem(_)) => todo!(),
            (State::Complete(_), Command::Complete) => todo!(),
            (State::Complete(_), Command::Cancel) => todo!(),
            (State::Cancelled(_), Command::Create) => todo!(),
            (State::Cancelled(_), Command::AddItem(_)) => todo!(),
            (State::Cancelled(_), Command::Complete) => todo!(),
            (State::Cancelled(_), Command::Cancel) => todo!(),
        }
    }
}
