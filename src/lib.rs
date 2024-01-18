pub trait Signal {
    type Item<'a>
    where
        Self: 'a;

    fn next(&mut self) -> Self::Item<'_>;
}

pub fn hold<S, E>(signal: S, event: E) -> Hold<S, E>
where
    for<'a> S: 'a + Signal<Item<'a> = Option<E>>,
{
    Hold { signal, event }
}

pub struct Hold<S, E> {
    signal: S,
    event: E,
}

impl<S, E> Signal for Hold<S, E>
where
    for<'a> S: 'a + Signal<Item<'a> = Option<E>>,
{
    type Item<'a> = &'a E
    where
        Self: 'a;

    fn next(&mut self) -> Self::Item<'_> {
        if let Some(event) = self.signal.next() {
            self.event = event;
        }
        &self.event
    }
}
